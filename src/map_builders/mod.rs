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
mod drunkards_walk;
#[allow(unused_imports)]
use drunkards_walk::DrunkardsWalkBuilder;
mod maze;
#[allow(unused_imports)]
use maze::MazeBuilder;
mod dla;
#[allow(unused_imports)]
use dla::DLABuilder;
mod voronoi;
#[allow(unused_imports)]
use voronoi::VoronoiBuilder;

pub trait MapBuilder {
	fn build_map(&mut self);
	fn spawn_entities(&mut self, ecs: &mut World);
	fn get_map(&self) -> Map;
	fn get_starting_position(&self) -> Position;
	fn get_snapshot_history(&self) -> Vec<Map>;
	fn take_snapshot(&mut self);
}

pub fn random_builder(new_depth: i32) -> Box<dyn MapBuilder> {
	let mut rng = rltk::RandomNumberGenerator::new();
	let builder = rng.roll_dice(1, 12);
	match builder {
		1 => Box::new(BspDungeonBuilder::new(new_depth)),
		2 => Box::new(BspInteriorBuilder::new(new_depth)),
		3 => Box::new(CellularAutomataBuilder::new(new_depth)),
		4 => Box::new(DrunkardsWalkBuilder::open_area(new_depth)),
		5 => Box::new(DrunkardsWalkBuilder::open_halls(new_depth)),
		6 => Box::new(DrunkardsWalkBuilder::winding_passages(new_depth)),
		7 => Box::new(DrunkardsWalkBuilder::fat_passages(new_depth)),
		8 => Box::new(DrunkardsWalkBuilder::fearful_symmetry(new_depth)),
		9 => Box::new(MazeBuilder::new(new_depth)),
		10 => Box::new(DLABuilder::walk_inwards(new_depth)),
		11 => Box::new(DLABuilder::walk_outwards(new_depth)),
		12 => Box::new(DLABuilder::central_attractor(new_depth)),
		13 => Box::new(DLABuilder::insectoid(new_depth)),
		14 => Box::new(VoronoiBuilder::pythagoras(new_depth)),
		15 => Box::new(VoronoiBuilder::manhattan(new_depth)),
		_ => Box::new(SimpleMapBuilder::new(new_depth))
	}
	// Box::new(DLABuilder::new(new_depth))
}