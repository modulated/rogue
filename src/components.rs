use specs::{prelude::*, saveload::SimpleMarker, saveload::ConvertSaveload, saveload::Marker};
use specs::error::NoError;
use serde::{Serialize, Deserialize};
use specs_derive::*;
use super::map::Map;
use rltk::{RGB};

pub fn register(ecs: &mut World) {
	// Ser/Deser
	ecs.register::<SimpleMarker<SerializeMe>>();
	ecs.register::<SerializationHelper>();
	// Game Components
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
	ecs.register::<WantsToRemoveItem>();
	ecs.register::<WantsToDropItem>();
	ecs.register::<Consumable>();
	ecs.register::<ProvidesHealing>();
	ecs.register::<Ranged>();
	ecs.register::<InflictsDamage>();
	ecs.register::<MagicMapper>();
	ecs.register::<AreaOfEffect>();
	ecs.register::<Confusion>();
	ecs.register::<Equippable>();
	ecs.register::<Equipped>();
	ecs.register::<MeleePowerBonus>();
	ecs.register::<DefenseBonus>();
	ecs.register::<Hidden>();
	ecs.register::<EntryTrigger>();
	ecs.register::<EntityMoved>();
	ecs.register::<SingleActivation>();
	ecs.register::<ParticleLifetime>();

}

// ********************************************************************************
// Serialization Components
// ********************************************************************************
pub struct SerializeMe;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
	pub map : Map
}

// ********************************************************************************
// Game Components
// ********************************************************************************

#[derive(Component, ConvertSaveload, Clone, Copy)]
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

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct WantsToRemoveItem {
	pub item : Entity
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

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct MagicMapper {}

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum EquipmentSlot { Melee, Shield }

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Equippable {
	pub slot: EquipmentSlot
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Equipped {
	pub owner: Entity,
	pub slot: EquipmentSlot
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct MeleePowerBonus {
	pub power: i32
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct DefenseBonus {
	pub defense: i32
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Hidden {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct EntryTrigger {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct EntityMoved {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SingleActivation {}

#[derive(Component,Serialize, Deserialize, Clone)]
pub struct ParticleLifetime {
	pub lifetime_ms: f32
}



