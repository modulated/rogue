use super::{MetaMapBuilder, InitialMapBuilder, BuilderMap, TileType};
use rltk::RandomNumberGenerator as Rng;

pub struct CellularAutomataBuilder {}

impl InitialMapBuilder for CellularAutomataBuilder {
	#[allow(dead_code)]
	fn build_map(&mut self, rng: &mut Rng, build_data: &mut BuilderMap) {
		self.build(rng, build_data);
	}
}

impl MetaMapBuilder for CellularAutomataBuilder {
	#[allow(dead_code)]
	fn build_map(&mut self, _rng: &mut Rng, build_data: &mut BuilderMap) {
		self.apply_iteration(build_data);
	}
}

#[allow(dead_code)]
impl CellularAutomataBuilder {
	pub fn new() -> Box<CellularAutomataBuilder> {
		Box::new(CellularAutomataBuilder{})
	}

	fn build(&mut self, rng: &mut Rng, build_data: &mut BuilderMap) {		
		// Randomise the map - 55% to be filled
		for y in 1..build_data.map.height - 1 {
			for x in 1..build_data.map.width - 1 {
				let roll = rng.roll_dice(1, 100);
				let idx = build_data.map.xy_idx(x, y);
				if roll > 55 {
					build_data.map.tiles[idx] = TileType::Floor
				} else {
					build_data.map.tiles[idx] = TileType::Wall
				}
			}
		}
		build_data.take_snapshot();

		// Iteratively apply cellular automata rules
		for _i in 0..15 {
			self.apply_iteration(build_data);
		}
	}

	fn apply_iteration(&mut self, build_data: &mut BuilderMap) {
		let mut newtiles = build_data.map.tiles.clone();

		for y in 1..build_data.map.height - 1 {
			for x in 1..build_data.map.width - 1 {
				let idx = build_data.map.xy_idx(x, y);
				let mut neighbors = 0;
				if build_data.map.tiles[idx - 1] == TileType::Wall { neighbors += 1; }
				if build_data.map.tiles[idx + 1] == TileType::Wall { neighbors += 1; }
				if build_data.map.tiles[idx - build_data.map.width as usize] == TileType::Wall { neighbors += 1; }
				if build_data.map.tiles[idx + build_data.map.width as usize] == TileType::Wall { neighbors += 1; }
				if build_data.map.tiles[idx - (build_data.map.width as usize - 1)] == TileType::Wall { neighbors += 1; }
				if build_data.map.tiles[idx - (build_data.map.width as usize + 1)] == TileType::Wall { neighbors += 1; }
				if build_data.map.tiles[idx + (build_data.map.width as usize - 1)] == TileType::Wall { neighbors += 1; }
				if build_data.map.tiles[idx + (build_data.map.width as usize + 1)] == TileType::Wall { neighbors += 1; }
				
				if neighbors > 4 || neighbors == 0 {
					newtiles[idx] = TileType::Wall;
				} else {
					newtiles[idx] = TileType::Floor;
				}
			}
		}

		build_data.map.tiles = newtiles.clone();
		build_data.take_snapshot();		
	}
}

