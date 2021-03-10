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
pub use gui::*;
mod gamelog;
pub use gamelog::GameLog;
mod spawner;
pub use spawner::*;

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
mod inventory_system;
pub use inventory_system::{InventorySystem, ItemUseSystem, ItemDropSystem};



#[derive(PartialEq, Copy, Clone)]
pub enum RunState { 
	AwaitingInput, 
	PreRun, 
	PlayerTurn, 
	MonsterTurn, 
	ShowInventory, 
	ShowDropItem, 
	ShowTargeting { range: i32, item: Entity },
	MainMenu { menu_selection: MainMenuSelection },
	SaveGame
}

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
		let mut inventory = InventorySystem{};
		inventory.run_now(&self.ecs);
		let mut item = ItemUseSystem{};
		item.run_now(&self.ecs);
		let mut drop_items = ItemDropSystem{};
		drop_items.run_now(&self.ecs);
		
		self.ecs.maintain();
	}
}

impl GameState for State {
	fn tick(&mut self, ctx : &mut Rltk) {
		let mut newrunstate;
		{
			let runstate = self.ecs.fetch::<RunState>();
			newrunstate = *runstate;
		}

		ctx.cls();

		match newrunstate {
			RunState::MainMenu{..} => {}
			_ => {
				draw_map(&self.ecs, ctx);
		
				{
					let positions = self.ecs.read_storage::<Position>();
					let renderables = self.ecs.read_storage::<Renderable>();
					let map = self.ecs.fetch::<Map>();
					
					let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
					data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
			
					for (pos, render) in data.iter() {
						let idx = map.xy_idx(pos.x, pos.y);
						if map.visible_tiles[idx] { ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph) }
					}
			
					draw_ui(&self.ecs, ctx);
				}		
			}
		}


		match newrunstate {
			RunState::MainMenu{..} => {
				let result = main_menu(self, ctx);
				match result {
					MainMenuResult::NoSeleciton{selected} => newrunstate = RunState::MainMenu{ menu_selection: selected },
					MainMenuResult::Selected{ selected } => {
						match selected {
							MainMenuSelection::NewGame => newrunstate = RunState::PreRun,
							MainMenuSelection::LoadGame => newrunstate = RunState::PreRun,
							MainMenuSelection::Quit => { std::process::exit(0); }
						}
					}
				}
			}
			RunState::PreRun => {
				self.run_systems();
				self.ecs.maintain();
				newrunstate = RunState::AwaitingInput;
			}
			RunState::AwaitingInput => {
				newrunstate = player_input(self, ctx);
			}
			RunState::PlayerTurn => {
				self.run_systems();
				self.ecs.maintain();
				newrunstate = RunState::MonsterTurn;
			}
			RunState::MonsterTurn => {
				self.run_systems();
				self.ecs.maintain();
				newrunstate = RunState::AwaitingInput;
			}
			RunState::ShowInventory => {
				let result = show_inventory(self, ctx);
				match result.0 {
					ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
					ItemMenuResult::NoResponse => {}
					ItemMenuResult::Selected => {
						let item_entity = result.1.unwrap();
						let is_ranged = self.ecs.read_storage::<Ranged>();
						let is_item_ranged = is_ranged.get(item_entity);
						if let Some(is_item_ranged) = is_item_ranged {
							newrunstate = RunState::ShowTargeting{ range: is_item_ranged.range, item: item_entity };
						} else {
							let mut intent = self.ecs.write_storage::<WantsToUseItem>();
							intent.insert(*self.ecs.fetch::<Entity>(), WantsToUseItem {item: item_entity, target: None}).expect("Unable to insert intent to use item.");
							newrunstate = RunState::PlayerTurn;
						}
					}

				}
			}
			RunState::ShowDropItem => {
				let result = drop_item_menu(self, ctx);
				match result.0 {
					ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
					ItemMenuResult::NoResponse => {}
					ItemMenuResult::Selected => {
						let item_entity = result.1.unwrap();
						let mut intent = self.ecs.write_storage::<WantsToDropItem>();
						intent.insert(*self.ecs.fetch::<Entity>(), WantsToDropItem{ item:item_entity}).expect("Unable to drop item");
						newrunstate = RunState::PlayerTurn;
					}
				}
			}

			RunState::ShowTargeting{range,item} => {
				let result = ranged_target(self, ctx, range);
				match result.0 {
					ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
					ItemMenuResult::NoResponse => {}
					ItemMenuResult::Selected => {
						let mut intent = self.ecs.write_storage::<WantsToUseItem>();
						intent.insert(*self.ecs.fetch::<Entity>(), WantsToUseItem{ item, target: result.1}).expect("Unable to insert target intent.");
						newrunstate = RunState::PlayerTurn;
					}
				}				
			}
		}

		{
			let mut runwriter = self.ecs.write_resource::<RunState>();
			*runwriter = newrunstate;
		}
		damage_system::delete_the_dead(&mut self.ecs);
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

	components::register(&mut gs.ecs);

	let map : Map = Map::new_map_rooms_and_corridors();
	let (player_x, player_y) = map.rooms[0].center();

	let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

	gs.ecs.insert(rltk::RandomNumberGenerator::new());
	for room in map.rooms.iter().skip(1) {		
		spawner::spawn_room(&mut gs.ecs, room);
	}

	gs.ecs.insert(map);
	gs.ecs.insert(Point::new(player_x, player_y));
	gs.ecs.insert(player_entity);
	gs.ecs.insert(RunState::MainMenu{menu_selection: MainMenuSelection::NewGame});
	gs.ecs.insert(gamelog::GameLog{ entries: vec!["Welcome to Rogue.".to_string()]});


	rltk::main_loop(context, gs)
}
