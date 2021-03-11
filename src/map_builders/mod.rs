use super::{Map, Rect, TileType, Position, spawner};
mod simple_map;
use simple_map::SimpleMapBuilder;
mod common;
use common::*;
use specs::prelude::*;

pub trait MapBuilder {
	fn build_map(&mut self, new_depth: i32) -> (Map, Position);
	fn spawn_entities(&mut self, map: &mut Map, ecs: &mut World, new_depth: i32);
}

pub fn random_builder(new_depth: i32) -> Box<dyn MapBuilder> {
	Box::new(SimpleMapBuilder::new(new_depth))
}