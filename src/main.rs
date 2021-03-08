use rltk::{GameState, Rltk, RGB, Point};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod rect;
pub use rect::*;
mod player;
pub use player::*;
mod visibility_system;
use visibility_system::VisibilitySystem;
mod monster_ai_system;
use monster_ai_system::MonsterAI;
mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;


pub struct State {
	pub ecs: World,
	pub runstate: RunState
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }

impl GameState for State {
	fn tick(&mut self, ctx : &mut Rltk) {
		ctx.cls();

		match self.runstate {
			RunState::Running => {
				self.run_systems();
				self.runstate = RunState::Paused;
			}
			_ => {
				self.runstate = player_input(self, ctx);
			}
		}

		draw_map(&self.ecs, ctx);

		let positions = self.ecs.read_storage::<Position>();
		let renderables = self.ecs.read_storage::<Renderable>();
		let map = self.ecs.fetch::<Map>();

		for (pos, render) in (&positions, &renderables).join() {
			let idx = map.xy_idx(pos.x, pos.y);
			if map.visible_tiles[idx] {	ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph) };
		}
	}
}

impl State {
	fn run_systems(&mut self) {
		let mut vis = VisibilitySystem{};
		vis.run_now(&self.ecs);
		let mut mob = MonsterAI {};
		mob.run_now(&self.ecs);
		let mut mapindex = MapIndexingSystem {};
		mapindex.run_now(&self.ecs);

		self.ecs.maintain();
	}
}

fn main() -> rltk::BError {
	use rltk::RltkBuilder;
	let context = RltkBuilder::simple80x50()
		.with_title("Rogue")
		.with_fps_cap(24.0)
		.with_dimensions(160, 100)
		.build()?;
	let mut gs = State {
		ecs: World::new(),
		runstate: RunState::Running
	};
	gs.ecs.register::<Position>();
	gs.ecs.register::<Renderable>();
	gs.ecs.register::<Player>();
	gs.ecs.register::<Viewshed>();
	gs.ecs.register::<Monster>();
	gs.ecs.register::<Name>();
	gs.ecs.register::<BlocksTile>();

	let map: Map = Map::new_map_rooms_and_corridors();
	let (player_x, player_y) = map.rooms[0].center();
	let mut rng = rltk::RandomNumberGenerator::new();

	for (i,room) in map.rooms.iter().skip(1).enumerate() {
		let (x,y) = room.center();
		let name: String;
		let glyph: rltk::FontCharType;
		let roll = rng.roll_dice(1, 2);
		match roll {
			1 => { glyph = rltk::to_cp437('g'); name = "Gobiln".to_string(); }
			_ => { glyph = rltk::to_cp437('o'); name = "Ork".to_string(); }
		}

		gs.ecs.create_entity()
			.with(Position{ x, y })
			.with(Renderable{
				glyph: glyph,
				fg: RGB::named(rltk::RED),
				bg: RGB::named(rltk::BLACK)			
			})
			.with(Viewshed{visible_tiles: Vec::new(), range: 8, dirty: true})
			.with(Monster{})
			.with(Name {name: format!{"{} #{}", &name, i+1}})
			.with(BlocksTile{})
			.build();
	}

	gs.ecs.insert(map);

	gs.ecs.create_entity()
		.with(Position { x: player_x, y: player_y })
		.with(Renderable {
			glyph: rltk::to_cp437('@'),
			fg: RGB::named(rltk::YELLOW),
			bg: RGB::named(rltk::BLACK),
		})
		.with(Player{})
		.with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true})
		.with(Name{ name: "Player".to_string()})
		.with(BlocksTile{})
		.build();

	gs.ecs.insert(Point::new(player_x, player_y));


	rltk::main_loop(context, gs)
}