use specs::prelude::*;
use super::{WantsToPickupItem, Name, InBackpack, Position, GameLog, CombatStats, WantsToUseItem, Consumable, WantsToDropItem, ProvidesHealing};

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
		Entities<'a>,
		WriteStorage<'a, WantsToUseItem>,
		ReadStorage<'a, Name>,
		ReadStorage<'a, Consumable>,
		WriteStorage<'a, CombatStats>,
		ReadStorage<'a, ProvidesHealing>
	);

	fn run(&mut self, data: Self::SystemData) {
		let (player_entity, mut gamelog, entities, mut wants_use, names, consumables, mut combat_stats, healing) = data;

		for (entity, useitem) in (&entities, &wants_use).join() {
			let mut used_item = true;

			let consumable = consumables.get(useitem.item);
			
			// If heals, provide healing
			let item_heals = healing.get(useitem.item);
			match item_heals {
				None => {}
				Some(healer) => {
					used_item = false;
					let stats = combat_stats.get_mut(*player_entity);
					if let Some(stats) = stats {
						stats.hp = i32::min(stats.max_hp, stats.hp + healer.heal_amount);
						if entity == *player_entity {
							gamelog.entries.push(format!("You drink the {}, healing {} health.", names.get(useitem.item).unwrap().name, healer.heal_amount));
						}
						used_item = true;
					}
					
				}
			}

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