use rltk::{ RGB, Rltk, BaseMap, Algorithm2D, Point };
use specs::prelude::*;
use serde::{Serialize, Deserialize};

pub const MAPWIDTH : usize = 80;
pub const MAPHEIGHT : usize = 43;
pub const MAPCOUNT : usize = MAPHEIGHT * MAPWIDTH;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize, Hash, Eq)]
pub enum TileType {
	Wall, Floor, DownStairs
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Map {
	pub tiles : Vec<TileType>,
	pub width : i32,
	pub height : i32,
	pub revealed_tiles : Vec<bool>,
	pub visible_tiles : Vec<bool>,
	pub blocked : Vec<bool>,
	pub depth: i32,

	#[serde(skip_serializing)]
	#[serde(skip_deserializing)]
	pub tile_content : Vec<Vec<Entity>>
}

impl Map {
	pub fn new(new_depth: i32) -> Map {
		Map {
			tiles : vec![TileType::Wall; MAPCOUNT],
			width : MAPWIDTH as i32,
			height: MAPHEIGHT as i32,
			revealed_tiles : vec![false; MAPCOUNT],
			visible_tiles : vec![false; MAPCOUNT],
			blocked : vec![false; MAPCOUNT],
			depth: new_depth,
			tile_content : vec![Vec::new(); MAPCOUNT]			
		}
	}

	pub fn xy_idx(&self, x: i32, y: i32) -> usize {
		(y as usize * self.width as usize) + x as usize
	}


	fn is_exit_valid(&self, x:i32, y:i32) -> bool {
		if x < 1 || x > self.width-1 || y < 1 || y > self.height-1 { return false; }
		let idx = self.xy_idx(x, y);
		!self.blocked[idx]
	}

	pub fn populate_blocked(&mut self) {
		for (i,tile) in self.tiles.iter_mut().enumerate() {
			self.blocked[i] = *tile == TileType::Wall;
		}
	}

	pub fn clear_content_index(&mut self) {
		for content in self.tile_content.iter_mut() {
			content.clear();
		}
	}
}

impl BaseMap for Map {
	fn is_opaque(&self, idx:usize) -> bool {
		self.tiles[idx] == TileType::Wall
	}

	fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
		let w = self.width as usize;
		let p1 = Point::new(idx1 % w, idx1 / w);
		let p2 = Point::new(idx2 % w, idx2 / w);
		rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
	}

	fn get_available_exits(&self, idx:usize) -> rltk::SmallVec<[(usize, f32); 10]> {
		let mut exits = rltk::SmallVec::new();
		let x = idx as i32 % self.width;
		let y = idx as i32 / self.width;
		let w = self.width as usize;

		// Cardinal directions
		if self.is_exit_valid(x-1, y) { exits.push((idx-1, 1.0)) };
		if self.is_exit_valid(x+1, y) { exits.push((idx+1, 1.0)) };
		if self.is_exit_valid(x, y-1) { exits.push((idx-w, 1.0)) };
		if self.is_exit_valid(x, y+1) { exits.push((idx+w, 1.0)) };

		// Diagonals
		if self.is_exit_valid(x-1, y-1) { exits.push(((idx-w)-1, 1.45)); }
		if self.is_exit_valid(x+1, y-1) { exits.push(((idx-w)+1, 1.45)); }
		if self.is_exit_valid(x-1, y+1) { exits.push(((idx+w)-1, 1.45)); }
		if self.is_exit_valid(x+1, y+1) { exits.push(((idx+w)+1, 1.45)); }

		exits
	}
}

impl Algorithm2D for Map {
	fn dimensions(&self) -> Point {
		Point::new(self.width, self.height)
	}
}

fn wall_glyph(map: &Map, x: i32, y: i32) -> rltk::FontCharType {
	if x < 1 || x > map.width - 2 || y < 1 || y > map.height - 2 as i32 { return 35; }
	let mut mask: u8 = 0;

	if is_revealed_and_wall(map, x, y - 1) { mask += 1; }
	if is_revealed_and_wall(map, x, y + 1) { mask += 2; }
	if is_revealed_and_wall(map, x - 1, y) { mask += 4; }
	if is_revealed_and_wall(map, x + 1, y) { mask += 8; }

	match mask {
		0 => { 9 }
		1 => { 186 }
		2 => { 186 }
		3 => { 186 }
		4 => { 205 }
		5 => { 188 }
		6 => { 187 }
		7 => { 185 }
		8 => { 205 }
		9 => { 200 }
		10 => { 201 }
		11 => { 204 }
		12 => { 205 }
		13 => { 202 }
		14 => { 203 }
		15 => { 206 }
		_ => { 35}
	}
}

fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
	let idx = map.xy_idx(x, y);
	map.tiles[idx] == TileType::Wall && map.revealed_tiles[idx]
}

pub fn draw_map(map: &Map, ctx : &mut Rltk) {
	let mut y = 0;
	let mut x = 0;
	for (idx,tile) in map.tiles.iter().enumerate() {
		// Render a tile depending upon the tile type

		if map.revealed_tiles[idx] {
			let glyph;
			let mut fg;
			match tile {
				TileType::Floor => {
					glyph = rltk::to_cp437('.');
					fg = RGB::from_f32(0.0, 0.5, 0.5);
				}
				TileType::Wall => {
					glyph = wall_glyph(&*map, x, y);
					fg = RGB::from_f32(0., 1.0, 0.);
				}
				TileType::DownStairs => {
					glyph = rltk::to_cp437('>');
					fg = RGB::from_f32(0.0, 1.0, 1.0);                }
			}
			if !map.visible_tiles[idx] { fg = fg.to_greyscale() }
			ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
		}

		// Move the coordinates
		x += 1;
		if x > MAPWIDTH as i32-1 {
			x = 0;
			y += 1;
		}
	}
}
