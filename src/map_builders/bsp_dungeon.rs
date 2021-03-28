use rltk::RandomNumberGenerator as Rng;
use super::{Map, TileType, Rect, BuilderMap, InitialMapBuilder, apply_room_to_map};

pub struct BspDungeonBuilder {	
	rects: Vec<Rect>	
}

impl InitialMapBuilder for BspDungeonBuilder {
	#[allow(dead_code)]
	fn build_map(&mut self, rng: &mut Rng, build_data: &mut BuilderMap) {
		self.build(rng, build_data);
	}	
}

impl BspDungeonBuilder {
	#[allow(dead_code)]
	pub fn new() -> Box<BspDungeonBuilder> {
		Box::new(BspDungeonBuilder {
			rects: Vec::new()
		})
	}

	fn build(&mut self, rng: &mut Rng, build_data: &mut BuilderMap) {		
		let mut rooms: Vec<Rect> = Vec::new();
		self.rects.clear();
		self.rects.push( Rect::new(2, 2, build_data.map.width-5, build_data.map.height-5) ); // Start with a single map-sized rectangle
		let first_room = self.rects[0];
		self.add_subrects(first_room); // Divide the first room

		// Up to 240 times, we get a random rectangle and divide it. If its possible to squeeze a
		// room in there, we place it and add it to the rooms list.
		let mut n_rooms = 0;
		while n_rooms < 240 {
			let rect = self.get_random_rect(rng);
			let candidate = self.get_random_sub_rect(rect, rng);

			if self.is_possible(candidate, &build_data.map) {
				apply_room_to_map(&mut build_data.map, &candidate);
				rooms.push(candidate);
				self.add_subrects(rect);
				build_data.take_snapshot();
			}

			n_rooms += 1;
		}
		build_data.rooms = Some(rooms);
	}


	fn add_subrects(&mut self, rect : Rect) {
		let width = i32::abs(rect.x1 - rect.x2);
		let height = i32::abs(rect.y1 - rect.y2);
		let half_width = i32::max(width / 2, 1);
		let half_height = i32::max(height / 2, 1);

		self.rects.push(Rect::new( rect.x1, rect.y1, half_width, half_height ));
		self.rects.push(Rect::new( rect.x1, rect.y1 + half_height, half_width, half_height ));
		self.rects.push(Rect::new( rect.x1 + half_width, rect.y1, half_width, half_height ));
		self.rects.push(Rect::new( rect.x1 + half_width, rect.y1 + half_height, half_width, half_height ));
	}

	fn get_random_rect(&mut self, rng : &mut Rng) -> Rect {
		if self.rects.len() == 1 { return self.rects[0]; }
		let idx = (rng.roll_dice(1, self.rects.len() as i32)-1) as usize;
		self.rects[idx]
	}

	fn get_random_sub_rect(&self, rect : Rect, rng : &mut Rng) -> Rect {
		let mut result = rect;
		let rect_width = i32::abs(rect.x1 - rect.x2);
		let rect_height = i32::abs(rect.y1 - rect.y2);

		let w = i32::max(3, rng.roll_dice(1, i32::min(rect_width, 10))-1) + 1;
		let h = i32::max(3, rng.roll_dice(1, i32::min(rect_height, 10))-1) + 1;

		result.x1 += rng.roll_dice(1, 6)-1;
		result.y1 += rng.roll_dice(1, 6)-1;
		result.x2 = result.x1 + w;
		result.y2 = result.y1 + h;

		result
	}

	fn is_possible(&self, rect : Rect, map: &Map) -> bool {
		let mut expanded = rect;
		expanded.x1 -= 2;
		expanded.x2 += 2;
		expanded.y1 -= 2;
		expanded.y2 += 2;		

		for y in expanded.y1 ..= expanded.y2 {
			for x in expanded.x1 ..= expanded.x2 {
				if x > map.width-2 { return false; }
				if y > map.height-2 { return false; }
				if x < 1 { return false; }
				if y < 1 { return false; }
				
				let idx = map.xy_idx(x, y);
				if map.tiles[idx] != TileType::Wall {
					return false;
				}
				
			}
		}

		true
	}
}