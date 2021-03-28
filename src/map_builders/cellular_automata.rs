use super::{MapBuilder, Map, TileType, Position, super::spawner::spawn_region, SHOW_MAPGEN_VISUALIZER, generate_voronoi_spawn_regions, remove_unreachable_areas_returning_most_distant};
use rltk::RandomNumberGenerator;
use std::collections::HashMap;

pub struct CellularAutomataBuilder {
	map: Map,
	starting_position: Position,
	depth: i32,
	history: Vec<Map>,
	noise_areas: HashMap<i32, Vec<usize>>,
	spawn_list: Vec<(usize, String)>
}

impl MapBuilder for CellularAutomataBuilder {
	fn get_map(&self) -> Map {
		self.map.clone()
	}

	fn get_starting_position(&self) -> Position {
		self.starting_position.clone()
	}

	fn get_snapshot_history(&self) -> Vec<Map> {
		self.history.clone()
	}

	fn get_spawn_list(&self) -> &Vec<(usize, String)> {
		&self.spawn_list
	}

	fn build_map(&mut self) {
		self.build();
	}

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

#[allow(dead_code)]
impl CellularAutomataBuilder {
	pub fn new(new_depth: i32) -> CellularAutomataBuilder {
		CellularAutomataBuilder {
			map: Map::new(new_depth),
			starting_position: Position {x: 0, y: 0},
			depth: new_depth,
			history: Vec::new(),
			noise_areas: HashMap::new(),
			spawn_list: Vec::new()
		}
	}

	fn build(&mut self) {
		let mut rng = RandomNumberGenerator::new();

		// Randomise the map - 55% to be filled
		for y in 1..self.map.height - 1 {
			for x in 1..self.map.width - 1 {
				let roll = rng.roll_dice(1, 100);
				let idx = self.map.xy_idx(x, y);
				if roll > 55 {
					self.map.tiles[idx] = TileType::Floor
				} else {
					self.map.tiles[idx] = TileType::Wall
				}
			}
		}
		self.take_snapshot();

		// Iteratively apply cellular automata rules
		for _i in 0..15 {
			let mut newtiles = self.map.tiles.clone();

			for y in 1..self.map.height - 1 {
				for x in 1..self.map.width - 1 {
					let idx = self.map.xy_idx(x, y);
					let mut neighbors = 0;
					if self.map.tiles[idx - 1] == TileType::Wall { neighbors += 1; }
					if self.map.tiles[idx + 1] == TileType::Wall { neighbors += 1; }
					if self.map.tiles[idx - self.map.width as usize] == TileType::Wall { neighbors += 1; }
					if self.map.tiles[idx + self.map.width as usize] == TileType::Wall { neighbors += 1; }
					if self.map.tiles[idx - (self.map.width as usize - 1)] == TileType::Wall { neighbors += 1; }
					if self.map.tiles[idx - (self.map.width as usize + 1)] == TileType::Wall { neighbors += 1; }
					if self.map.tiles[idx + (self.map.width as usize - 1)] == TileType::Wall { neighbors += 1; }
					if self.map.tiles[idx + (self.map.width as usize + 1)] == TileType::Wall { neighbors += 1; }
					
					if neighbors > 4 || neighbors == 0 {
						newtiles[idx] = TileType::Wall;
					} else {
						newtiles[idx] = TileType::Floor;
					}
				}
			}

			self.map.tiles = newtiles.clone();
			self.take_snapshot();			
		}

		// Find start pos
		self.starting_position = Position{ x: self.map.width / 2, y: self.map.height / 2 };
		let mut start_idx = self.map.xy_idx(self.starting_position.x, self.starting_position.y);
		while self.map.tiles[start_idx] != TileType::Floor {
			self.starting_position.x -= 1;
			start_idx = self.map.xy_idx(self.starting_position.x, self.starting_position.y);
		}

		// Find all reachable areas
		let exit_tile = remove_unreachable_areas_returning_most_distant(&mut self.map, start_idx);
		self.map.tiles[exit_tile] = TileType::DownStairs;
		self.take_snapshot();

		// Build noise map for spawning entities
		self.noise_areas = generate_voronoi_spawn_regions(&self.map, &mut rng);

		for area in self.noise_areas.iter() {
			spawn_region(&self.map, &mut rng, area.1, self.depth, &mut self.spawn_list);
		}
	}
}