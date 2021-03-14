use super::{Map, Rect, TileType, Position, spawner, SHOW_MAPGEN_VISUALIZER};
mod simple_map;
#[allow(unused_imports)]
use simple_map::SimpleMapBuilder;
mod common;
use common::*;
use specs::prelude::*;
mod bsp_dungeon;
#[allow(unused_imports)]
use bsp_dungeon::BspDungeonBuilder;
mod bsp_interior;
#[allow(unused_imports)]
use bsp_interior::BspInteriorBuilder;
mod cellular_automata;
#[allow(unused_imports)]
use cellular_automata::CellularAutomataBuilder;
#[allow(unused_imports)]
mod drunkards_walk;
use drunkards_walk::{DrunkardsWalkBuilder};

pub trait MapBuilder {
	fn build_map(&mut self);
	fn spawn_entities(&mut self, ecs: &mut World);
	fn get_map(&mut self) -> Map;
	fn get_starting_position(&mut self) -> Position;
	fn get_snapshot_history(&self) -> Vec<Map>;
	fn take_snapshot(&mut self);
}

pub fn random_builder(new_depth: i32) -> Box<dyn MapBuilder> {
	let mut rng = rltk::RandomNumberGenerator::new();
	let builder = rng.roll_dice(1, 3);
	match builder {
		1 => Box::new(BspDungeonBuilder::new(new_depth)),
        2 => Box::new(BspInteriorBuilder::new(new_depth)),
        3 => Box::new(CellularAutomataBuilder::new(new_depth)),
        4 => Box::new(DrunkardsWalkBuilder::open_area(new_depth)),
        5 => Box::new(DrunkardsWalkBuilder::open_halls(new_depth)),
        6 => Box::new(DrunkardsWalkBuilder::winding_passages(new_depth)),
        _ => Box::new(SimpleMapBuilder::new(new_depth))
	}
}