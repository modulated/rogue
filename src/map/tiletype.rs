use serde::{Serialize, Deserialize};

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize, Hash, Eq)]
pub enum TileType {
	Wall, 
	Floor, 
	DownStairs,
	Road,
	Gravel,
	Grass,
	ShallowWater,
	DeepWater,
	WoodFloor,
	Bridge
}

impl TileType {
	pub fn is_walkable(&self) -> bool {
		match self {
			TileType::Floor | TileType::DownStairs | TileType::Road | TileType::Grass | TileType::ShallowWater | TileType::WoodFloor | TileType::Bridge | TileType::Gravel => true,
			_ => false
		}
	}

	pub fn is_opaque(&self) -> bool {
		match self {
			TileType::Wall => true,
			_ => false
		}
	}

	pub fn move_cost(&self) -> f32 {
		match self {
			TileType::Road => 0.8,
			TileType::Grass => 1.1,
			TileType::ShallowWater => 1.2,
			_ => 1.0
		}
	}
}