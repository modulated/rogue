use specs::prelude::*;
use super::{WantsToPickupItem, Name, InBackpack, Map, Position, GameLog, CombatStats, SufferDamage, WantsToUseItem, Consumable, WantsToDropItem, ProvidesHealing, InflictsDamage, AreaOfEffect, Confusion, Equippable, Equipped};

pub struct InventorySystem {}

impl<'a> System<'a> for InventorySystem {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		ReadExpect<'a, Entity>,
		WriteExpect<'a, GameLog>,
		WriteStorage<'a, WantsToPickupItem>,
		WriteStorage<'a, Position>,
		ReadStorage<'a, Name>,
		WriteStorage<'a, InBackpack>
	);

	fn run(&mut self, data: Self::SystemData) {
		let (player_entity, mut gamelog, mut wants_pickup, mut positions, names, mut backpack) = data;

		for pickup in wants_pickup.join() {
			positions.remove(pickup.item);
			backpack.insert(pickup.item, InBackpack{ owner: pickup.collected_by}).expect("Unable to insert into backpack!");

			if pickup.collected_by == *player_entity {
				gamelog.entries.push(format!("You pick up the {}.", names.get(pickup.item).unwrap().name));
			}
		}

		wants_pickup.clear();
	}
}

pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
	type SystemData = (
		ReadExpect<'a, Entity>,
		WriteExpect<'a, GameLog>,
		ReadExpect<'a, Map>,
		Entities<'a>,
		WriteStorage<'a, WantsToUseItem>,
		ReadStorage<'a, Name>,
		ReadStorage<'a, InflictsDamage>,
		ReadStorage<'a, Consumable>,
		WriteStorage<'a, CombatStats>,
		ReadStorage<'a, ProvidesHealing>, 
		WriteStorage<'a, SufferDamage>,
		ReadStorage<'a, AreaOfEffect>,
		WriteStorage<'a, Confusion>,
		WriteStorage<'a, InBackpack>,
		WriteStorage<'a, Equipped>,
		ReadStorage<'a, Equippable>
	);

	fn run(&mut self, data: Self::SystemData) {
		let (player_entity, 
			mut gamelog, 
			map, 
			entities, 
			mut wants_use, 
			names, 
			inflicts_damage, 
			consumables, 
			mut combat_stats, 
			healing, 
			mut suffer_damage, 
			aoe, 
			mut confused, 
			mut backpack, 
			mut equipped,
			equippable
		) = data;

		for (entity, useitem) in (&entities, &wants_use).join() {
			let mut used_item = true;
			// let consumable = consumables.get(useitem.item);

			// Calculate item targets
			let mut targets: Vec<Entity> = Vec::new();
			match useitem.target {
				None => targets.push(*player_entity),
				Some(target) => {
					let area_effect = aoe.get(useitem.item);
					match area_effect {
						None => {
							// Single target
							let idx = map.xy_idx(target.x, target.y);
							for mob in map.tile_content[idx].iter() {
								targets.push(*mob);
							}
						}
						Some(area_effect) => {
							// AoE
							let mut affected_tiles = rltk::field_of_view(target, area_effect.radius, &*map);
							affected_tiles.retain(|p| p.x > 0 && p.x < map.width - 1 && p.y > 0 && p.y < map.height -1);
							for tile_idx in affected_tiles.iter() {
								let idx = map.xy_idx(tile_idx.x, tile_idx.y);
								for mob in map.tile_content[idx].iter() {
									targets.push(*mob);
								}
							}
						}
					}
				}
			}
			
			let item_equippable = equippable.get(useitem.item);
			match item_equippable {
				None => {}
				Some(can_equip) => {
					let target_slot = can_equip.slot;
					let target = targets[0];

					// Remove any item in targets relevant slot
					let mut to_unequip: Vec<Entity> = Vec::new();
					for (item_entity, already_equipped, name) in (&entities, &equipped, &names).join() {
						if already_equipped.owner == target && already_equipped.slot == target_slot {
							to_unequip.push(item_entity);
							if target == *player_entity {
								gamelog.entries.push(format!("You unequip the {}.", name.name));
							}
						}
					}
					for item in to_unequip.iter() {
						equipped.remove(*item);
						backpack.insert(*item, InBackpack{ owner: target }).expect("Unable to insert item into backpack.");
					}

					// Wield item
					equipped.insert(useitem.item, Equipped{ owner: target, slot: target_slot }).expect("Unable to wield item.");
					backpack.remove(useitem.item);
					if target == *player_entity {
						gamelog.entries.push(format!("You equip the {}.", names.get(useitem.item).unwrap().name));
					}
				}
			}

			// If heals, provide healing
			let item_heals = healing.get(useitem.item);
			match item_heals {
				None => {}
				Some(healer) => {
					for target in targets.iter() {
						let stats = combat_stats.get_mut(*target);
						if let Some(stats) = stats {
							stats.hp = i32::min(stats.max_hp, stats.hp + healer.heal_amount);
							if entity == *player_entity {
								gamelog.entries.push(format!("You drink the {}, healing {} health.", names.get(useitem.item).unwrap().name, healer.heal_amount));
							}
						}
						used_item = true;
					}
					
				}
			}

			// If inflicts damage, apply to target cell
			let item_damages = inflicts_damage.get(useitem.item);
			match item_damages {
				None => {}
				Some(damage) => {					
					used_item = false;
					for mob in targets.iter() {
						SufferDamage::new_damage(&mut suffer_damage, *mob, damage.damage);
						if entity == *player_entity {
							let mob_name = names.get(*mob).unwrap();
							let item_name = names.get(useitem.item).unwrap();
							gamelog.entries.push(format!("You use {} on {}, inflicting {} damage.", item_name.name, mob_name.name, damage.damage));
						}

						used_item = true;
					}
				}
			}

			// If adds confusion, calculate
			let mut add_confusion = Vec::new();
			{
				let causes_confusion = confused.get(useitem.item);
				match causes_confusion {
					None => {}
					Some(confusion) => {
						used_item = false;
						for mob in targets.iter() {
							add_confusion.push((*mob, confusion.duration));
							if entity == *player_entity {
								let mob_name = names.get(*mob).unwrap();
								let item_name = names.get(useitem.item).unwrap();
								gamelog.entries.push(format!("You use {} on {}, confusing them.", item_name.name, mob_name.name));							
							}
							used_item = true;
						}
					}
				}
			}
			for mob in add_confusion.iter() {
				confused.insert(mob.0, Confusion{duration: mob.1}).expect("Unable to apply confusion");
			}

			// Delete the item if successfully used
			if used_item {
				let consumable = consumables.get(useitem.item);
				match consumable {
					None => {}
					Some(_) => {
						entities.delete(useitem.item).expect("Delete failed.");
					}
				}
			}
		}
		wants_use.clear();
	}
}

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
	type SystemData = (
		ReadExpect<'a, Entity>,
		WriteExpect<'a, GameLog>,
		Entities<'a>,
		WriteStorage<'a, WantsToDropItem>,
		ReadStorage<'a, Name>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, InBackpack>
	);

	fn run(&mut self, data: Self::SystemData) {
		let (player_entity, mut gamelog, entities, mut wants_drop, names, mut positions, mut backpack) = data;

		for (entity, to_drop) in (&entities, &wants_drop).join() {
			let mut dropper_pos: Position = Position {x: 0, y: 0};
			{
				let dropped_pos = positions.get(entity).unwrap();
				dropper_pos.x = dropped_pos.x;
				dropper_pos.y = dropped_pos.y;
			}
			positions.insert(to_drop.item, Position{ x: dropper_pos.x, y: dropper_pos.y}).expect("Unable to insert position for dropped item");
			backpack.remove(to_drop.item);

			if entity == *player_entity {
				gamelog.entries.push(format!("You drop the {}.", names.get(to_drop.item).unwrap().name));
			}
		}
		wants_drop.clear();
	}
}