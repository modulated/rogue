use super::{Map, Rect, TileType, Position, spawner};
mod simple_map;
use simple_map::SimpleMapBuilder;
mod common;
use common::*;
use specs::prelude::*;
mod bsp_dungeon;
use bsp_dungeon::BspDungeonBuilder;

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
	let builder = rng.roll_dice(1, 2);
	match builder {

		1 => Box::new(BspDungeonBuilder::new(new_depth)),
		_ => Box::new(SimpleMapBuilder::new(new_depth))
	}
}