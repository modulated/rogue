use specs::prelude::*;
use super::{WantsToPickupItem, Name, InBackpack, Position, GameLog, CombatStats, WantsToDrinkPotion, Consumable, WantsToDropItem};

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
	type SystemData = ( ReadExpect<'a, Entity>,
		WriteExpect<'a, GameLog>,
		Entities<'a>,
		WriteStorage<'a, WantsToDrinkPotion>,
		ReadStorage<'a, Name>,
		ReadStorage<'a, Consumable>,
		WriteStorage<'a, CombatStats>
	);

	fn run(&mut self, data: Self::SystemData) {
		let (player_entity, mut gamelog, entities, mut wants_drink, names, consumables, mut combat_stats) = data;

		for (entity, drink, stats) in (&entities, &wants_drink, &mut combat_stats).join() {
			let consumable = consumables.get(useitem.item);
			match consumable {
				None => {}
				Some(_) => {					
					entities.delete(useitem.potion).expect("Delete item failed");
				}
			}
		}
		wants_drink.clear();
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