use super::{Map, Rect, TileType, Position, spawner};
mod simple_map;
use simple_map::SimpleMapBuilder;
mod common;
use common::*;
use specs::prelude::*;

trait MapBuilder {
	fn build(new_depth: i32) -> Map;
}

pub fn build_random_map(new_depth: i32) -> Map {
	SimpleMapBuilder::build(new_depth)
}