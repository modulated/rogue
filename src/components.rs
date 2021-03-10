use specs::{prelude::*, saveload::SimpleMarker, saveload::ConvertSaveload, saveload::Marker};
use specs::error::NoError;
use serde::{Serialize, Deserialize};
use specs_derive::*;
use super::map::Map;
use rltk::{RGB};

pub fn register(ecs: &mut World) {
	ecs.register::<Position>();
	ecs.register::<Renderable>();
	ecs.register::<Player>();
	ecs.register::<Viewshed>();
	ecs.register::<Monster>();
	ecs.register::<Name>();
	ecs.register::<BlocksTile>();
	ecs.register::<CombatStats>();
	ecs.register::<WantsToMelee>();
	ecs.register::<SufferDamage>();
	ecs.register::<Item>();
	ecs.register::<Potion>();
	ecs.register::<WantsToPickupItem>();
	ecs.register::<InBackpack>();
	ecs.register::<WantsToUseItem>();
	ecs.register::<WantsToDropItem>();
	ecs.register::<Consumable>();
	ecs.register::<ProvidesHealing>();
	ecs.register::<Ranged>();
	ecs.register::<InflictsDamage>();
	ecs.register::<AreaOfEffect>();
	ecs.register::<Confusion>();
	ecs.register::<SimpleMarker<SerializeMe>>();
	ecs.register::<SerializationHelper>();
}

#[derive(Component, ConvertSaveload)]
pub struct Position {
	pub x: i32,
	pub y: i32,
}

#[derive(Component, ConvertSaveload)]
pub struct Renderable {
	pub glyph: rltk::FontCharType,
	pub fg: RGB,
	pub bg: RGB,
	pub render_order: i32
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Player {}

#[derive(Component, ConvertSaveload)]
pub struct Viewshed {
	pub visible_tiles : Vec<rltk::Point>,
	pub range : i32,
	pub dirty : bool
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Monster {}

#[derive(Component, Debug, ConvertSaveload)]
pub struct Name {
	pub name : String
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlocksTile {}

#[derive(Component, Debug, ConvertSaveload)]
pub struct CombatStats {
	pub max_hp : i32,
	pub hp : i32,
	pub defense : i32,
	pub power : i32
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct WantsToMelee {
	pub target : Entity
}

#[derive(Component, Debug, ConvertSaveload)]
pub struct SufferDamage {
	pub amount : Vec<i32>
}

impl SufferDamage {
	pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
		if let Some(suffering) = store.get_mut(victim) {
			suffering.amount.push(amount);
		} else {
			let dmg = SufferDamage { amount : vec![amount] };
			store.insert(victim, dmg).expect("Unable to insert damage");
		}
	}
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Item {}

#[derive(Component, Debug, ConvertSaveload)]
pub struct Potion {
	pub heal_amount: i32
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct InBackpack {
	pub owner: Entity
}

#[derive(Component, ConvertSaveload)]
pub struct WantsToPickupItem {
	pub collected_by: Entity,
	pub item: Entity
}

#[derive(Component, ConvertSaveload)]
pub struct WantsToUseItem {
	pub item: Entity,
	pub target: Option<rltk::Point>
}

#[derive(Component, ConvertSaveload)]
pub struct WantsToDropItem {
	pub item: Entity
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Consumable {}

#[derive(Component, Debug, ConvertSaveload)]
pub struct ProvidesHealing {
	pub heal_amount: i32
}

#[derive(Component, Debug, ConvertSaveload)]
pub struct Ranged {
	pub range: i32
}

#[derive(Component, Debug, ConvertSaveload)]
pub struct InflictsDamage {
	pub damage: i32
}

#[derive(Component, Debug, ConvertSaveload)]
pub struct AreaOfEffect {
	pub radius: i32
}

#[derive(Component, Debug, ConvertSaveload)]
pub struct Confusion {
	pub duration: i32
}


// Serialization Helper Code
pub struct SerializeMe;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
	pub map : Map
}