use rltk::{GameState, Rltk, Point};
use specs::prelude::*;
mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;
mod gui;
pub use gui::draw_ui;
mod gamelog;
pub use gamelog::GameLog;
mod spawner;
pub use spawner::{player, random_monster};

// Systems
mod visibility_system;
use visibility_system::VisibilitySystem;
mod monster_ai_system;
use monster_ai_system::MonsterAI;
mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;
mod melee_combat_system;
use melee_combat_system::MeleeCombatSystem;
mod damage_system;
use damage_system::DamageSystem;



#[derive(PartialEq, Copy, Clone)]
pub enum RunState { AwaitingInput, PreRun, PlayerTurn, MonsterTurn }

pub struct State {
	pub ecs: World
}

impl State {
	fn run_systems(&mut self) {
		let mut vis = VisibilitySystem{};
		vis.run_now(&self.ecs);
		let mut mob = MonsterAI{};
		mob.run_now(&self.ecs);
		let mut mapindex = MapIndexingSystem{};
		mapindex.run_now(&self.ecs);
		let mut melee = MeleeCombatSystem{};
		melee.run_now(&self.ecs);
		let mut damage = DamageSystem{};
		damage.run_now(&self.ecs);
		self.ecs.maintain();
	}
}

impl GameState for State {
	fn tick(&mut self, ctx : &mut Rltk) {
		ctx.cls();
		let mut newrunstate;
		{
			let runstate = self.ecs.fetch::<RunState>();
			newrunstate = *runstate;
		}

		match newrunstate {
			RunState::PreRun => {
				self.run_systems();
				newrunstate = RunState::AwaitingInput;
			}
			RunState::AwaitingInput => {
				newrunstate = player_input(self, ctx);
			}
			RunState::PlayerTurn => {
				self.run_systems();
				newrunstate = RunState::MonsterTurn;
			}
			RunState::MonsterTurn => {
				self.run_systems();
				newrunstate = RunState::AwaitingInput;
			}
		}

		{
			let mut runwriter = self.ecs.write_resource::<RunState>();
			*runwriter = newrunstate;
		}
		damage_system::delete_the_dead(&mut self.ecs);

		draw_map(&self.ecs, ctx);

		let positions = self.ecs.read_storage::<Position>();
		let renderables = self.ecs.read_storage::<Renderable>();
		let map = self.ecs.fetch::<Map>();

		for (pos, render) in (&positions, &renderables).join() {
			let idx = map.xy_idx(pos.x, pos.y);
			if map.visible_tiles[idx] { ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph) }
		}

		draw_ui(&self.ecs, ctx);
	}
}

fn main() -> rltk::BError {
	use rltk::RltkBuilder;
	let mut context = RltkBuilder::simple80x50()
		.with_title("Rogue")
		.with_fps_cap(24.0)
		.with_dimensions(160, 100)		
		.build()?;
	context.with_post_scanlines(true);
	let mut gs = State {
		ecs: World::new(),
	};
	gs.ecs.register::<Position>();
	gs.ecs.register::<Renderable>();
	gs.ecs.register::<Player>();
	gs.ecs.register::<Viewshed>();
	gs.ecs.register::<Monster>();
	gs.ecs.register::<Name>();
	gs.ecs.register::<BlocksTile>();
	gs.ecs.register::<CombatStats>();
	gs.ecs.register::<WantsToMelee>();
	gs.ecs.register::<SufferDamage>();

	let map : Map = Map::new_map_rooms_and_corridors();
	let (player_x, player_y) = map.rooms[0].center();

	let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

	gs.ecs.insert(rltk::RandomNumberGenerator::seeded(1));
	for room in map.rooms.iter().skip(1) {
		let (x,y) = room.center();
		spawner::random_monster(&mut gs.ecs, x, y);
	}

	gs.ecs.insert(map);
	gs.ecs.insert(Point::new(player_x, player_y));
	gs.ecs.insert(player_entity);
	gs.ecs.insert(RunState::PreRun);
	gs.ecs.insert(gamelog::GameLog{ entries: vec!["Welcome to Rogue.".to_string()]});


	rltk::main_loop(context, gs)
}
