use super::{Map, Rect, TileType, Position, spawner::spawn_entity, SHOW_MAPGEN_VISUALIZER};
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
mod wfc;
#[allow(unused_imports)]
use wfc::WFCBuilder;
mod prefab_builder;
#[allow(unused_imports)]
use prefab_builder::PrefabBuilder;
mod room_based_spawner;
use room_based_spawner::RoomBasedSpawner;
mod room_based_stairs;
use room_based_stairs::RoomBasedStairs;
mod room_based_starting_position;
use room_based_starting_position::RoomBasedStartingPosition;

pub struct BuilderMap {
	pub spawn_list: Vec<(usize, String)>,
	pub map: Map,
	pub starting_position: Option<Position>,
	pub rooms: Option<Vec<Rect>>,
	pub history: Vec<Map>
}

impl BuilderMap {
	fn take_snapshot(&mut self) {
		if SHOW_MAPGEN_VISUALIZER {
			let mut snapshot = self.map.clone();
			for v in snapshot.revealed_tiles.iter_mut() {
				*v = true;
			}
			self.history.push(snapshot);
		}
	}
}

pub struct BuilderChain {
	starter: Option<Box<dyn InitialMapBuilder>>,
	builders: Vec<Box<dyn MetaMapBuilder>>,
	pub build_data: BuilderMap
}

impl BuilderChain {
	pub fn new(new_depth: i32) -> BuilderChain {
		BuilderChain {
			starter: None,
			builders: Vec::new(),
			build_data: BuilderMap {
				spawn_list: Vec::new(),
				map: Map::new(new_depth),
				starting_position: None,
				rooms: None,
				history: Vec::new()
			}
		}
	}

	pub fn start_with(&mut self, starter: Box<dyn InitialMapBuilder>) {
		match self.starter {
			None => self.starter = Some(starter),
			Some(_) => panic!("There can only be one starting builder.")
		}
	}

	pub fn with(&mut self, metabuilder: Box<dyn MetaMapBuilder>) {
		self.builders.push(metabuilder);
	}

	pub fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator) {
		match &mut self.starter {
			None => panic!("Cannot build without an initial starting builder."),
			Some(starter) => {
				starter.build_map(rng, &mut self.build_data);
			}
		}

		for metabuilder in self.builders.iter_mut() {
			metabuilder.build_map(rng, &mut self.build_data);
		}
	}

	pub fn spawn_entities(&mut self, ecs: &mut World) {
		for entity in self.build_data.spawn_list.iter() {
			spawn_entity(ecs, &(&entity.0, &entity.1));
		}
	}
}

pub trait MapBuilder {
	fn build_map(&mut self);
	fn get_map(&self) -> Map;
	fn get_starting_position(&self) -> Position;
	fn get_snapshot_history(&self) -> Vec<Map>;
	fn take_snapshot(&mut self);
	fn get_spawn_list(&self) -> &Vec<(usize, String)>;	

	fn spawn_entities(&mut self, ecs: &mut World) {
		for entity in self.get_spawn_list().iter() {
			spawn_entity(ecs, &(&entity.0, &entity.1));
		}
	}
}

pub trait InitialMapBuilder {
	fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator, build_data: &mut BuilderMap);
}

pub trait MetaMapBuilder {
	fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator, build_data: &mut BuilderMap);
}

pub fn random_builder(new_depth: i32, _rng: &mut rltk::RandomNumberGenerator) -> BuilderChain {
	let mut builder = BuilderChain::new(new_depth);
	builder.start_with(BspDungeonBuilder::new());
	builder.with(RoomBasedSpawner::new());
	builder.with(RoomBasedStartingPosition::new());
	builder.with(RoomBasedStairs::new());
	builder
}