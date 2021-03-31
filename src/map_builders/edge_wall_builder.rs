use super::{MetaMapBuilder, BuilderMap, TileType};
use rltk::RandomNumberGenerator as Rng;

pub struct EdgeWallBuilder {}

impl MetaMapBuilder for EdgeWallBuilder {
	fn build_map(&mut self, rng: &mut Rng, build_data : &mut BuilderMap)  {
		self.build(rng, build_data);
	}
}

impl EdgeWallBuilder {
	#[allow(dead_code)]
	pub fn new() -> Box<Self> {
		Box::new(Self {})
	}

	fn build(&mut self, _rng : &mut Rng, build_data : &mut BuilderMap) {
		let width: usize = build_data.map.width as usize;
		let height: usize = build_data.map.height as usize;

		for x in 0..width {
			build_data.map.tiles[x] = TileType::Floor;
			build_data.map.tiles[x + ((height - 1) * width)] = TileType::Floor;
		}

		for y in 0..height {
			build_data.map.tiles[y] = TileType::Floor;
			build_data.map.tiles[y + ((width - 1) * height)] = TileType::Floor;
		}

		build_data.take_snapshot();
	}
}