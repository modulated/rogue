use rltk::{GameState, Rltk, Point};
use saveload_system::delete_save;
use specs::prelude::*;
use specs::saveload::{SimpleMarkerAllocator};
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
mod random_table;
pub use random_table::RandomTable;
mod saveload_system;
pub use saveload_system::{save_game};
pub mod map_builders;

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
pub use inventory_system::{InventorySystem, ItemUseSystem, ItemDropSystem, ItemRemoveSystem};
mod trigger_system;
pub use trigger_system::TriggerSystem;


#[derive(PartialEq, Copy, Clone)]
pub enum RunState { 
	AwaitingInput, 
	PreRun, 
	PlayerTurn, 
	MonsterTurn, 
	ShowInventory, 
	ShowDropItem, 
	ShowRemoveItem,
	ShowTargeting { range: i32, item: Entity },
	MainMenu { menu_selection: MainMenuSelection },
	SaveGame,
	NextLevel,
	GameOver,
	MagicMapReveal{ row: i32 }
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
		let mut trigger_system = TriggerSystem{};
		trigger_system.run_now(&self.ecs);
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
		let mut remove_items = ItemRemoveSystem{};
		remove_items.run_now(&self.ecs);
		
		self.ecs.maintain();
	}

	fn entities_to_remove_on_level_change(&mut self) -> Vec<Entity> {
		let entities = self.ecs.entities();
		let player = self.ecs.read_storage::<Player>();
		let backpack = self.ecs.read_storage::<InBackpack>();
		let player_entity = self.ecs.fetch::<Entity>();
		let equipped = self.ecs.read_storage::<Equipped>();

		let mut to_delete: Vec<Entity> = Vec::new();
		for  entity in entities.join() {
			let mut should_delete = true;

			let p = player.get(entity);
			if let Some(_p) = p {
				should_delete = false;
			}
			let bp = backpack.get(entity);
			if let Some(bp) = bp {
				if bp.owner ==*player_entity {
					should_delete = false;
				}
			}
			let eq = equipped.get(entity);
			if let Some(eq) = eq {
				if eq.owner == *player_entity {
					should_delete = false;
				}
			}

			if should_delete {
				to_delete.push(entity);
			}
		}

		to_delete
	}

	fn goto_next_level(&mut self) {
		let to_delete = self.entities_to_remove_on_level_change();
		for target in to_delete {
			self.ecs.delete_entity(target).expect("Unable to delete entity.");
		}

		let worldmap;
		let current_depth;
		{
			let mut worldmap_resource = self.ecs.write_resource::<Map>();
			current_depth = worldmap_resource.depth;
			*worldmap_resource = map_builders::build_random_map(current_depth + 1);
			worldmap = worldmap_resource.clone();
		}

		for room in worldmap.rooms.iter().skip(1) {
			spawner::spawn_room(&mut self.ecs, room, current_depth + 1);
		}

		let (player_x, player_y) = worldmap.rooms[0].center();
		let mut player_position = self.ecs.write_resource::<Point>();
		*player_position = Point::new(player_x, player_y);
		let mut position_components = self.ecs.write_storage::<Position>();
		let player_entity = self.ecs.fetch::<Entity>();
		let player_pos_comp = position_components.get_mut(*player_entity);
		if let Some(player_pos_comp) = player_pos_comp {
			player_pos_comp.x = player_x;
			player_pos_comp.y = player_y;
		}

		let mut viewshed_components = self.ecs.write_storage::<Viewshed>();
		let vs = viewshed_components.get_mut(*player_entity);
		if let Some(vs) = vs {
			vs.dirty = true;
		}

		let mut gamelog = self.ecs.fetch_mut::<GameLog>();
		gamelog.entries.push("You descend to the next level, and take a moment to rest.".to_string());
		let mut player_health_store = self.ecs.write_storage::<CombatStats>();
		let player_health = player_health_store.get_mut(*player_entity);
		if let Some(player_health) = player_health {
			player_health.hp = i32::max(player_health.hp, player_health.max_hp);
		}
	}

	fn game_over_cleanup(&mut self) {
		let mut to_delete = Vec::new();
		for e in self.ecs.entities().join() {
			to_delete.push(e);
		}
		for del in to_delete.iter() {
			self.ecs.delete_entity(*del).expect("Game over cleanup delete failed.");
		}

		let worldmap;
		{
			let mut worldmap_resource = self.ecs.write_resource::<Map>();
			*worldmap_resource = map_builders::build_random_map(1);
			worldmap = worldmap_resource.clone();
		}

		for room in worldmap.rooms.iter().skip(1) {
			spawner::spawn_room(&mut self.ecs, room, 1);
		}

		let (player_x, player_y) = worldmap.rooms[0].center();
		let player_entity = spawner::player(&mut self.ecs, player_x, player_y);
		let mut player_position = self.ecs.write_resource::<Point>();
		*player_position = Point::new(player_x, player_y);
		let mut position_components = self.ecs.write_storage::<Position>();
		let mut player_entity_writer = self.ecs.write_resource::<Entity>();
		*player_entity_writer = player_entity;
		let player_pos_comp = position_components.get_mut(player_entity);
		if let Some(player_pos_comp) = player_pos_comp {
			player_pos_comp.x = player_x;
			player_pos_comp.y = player_y;
		}
	
		// Mark the player's visibility as dirty
		let mut viewshed_components = self.ecs.write_storage::<Viewshed>();
		let vs = viewshed_components.get_mut(player_entity);
		if let Some(vs) = vs {
			vs.dirty = true;
		}

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
					let hidden = self.ecs.read_storage::<Hidden>();
					let map = self.ecs.fetch::<Map>();
					
					let mut data = (&positions, &renderables, !&hidden).join().collect::<Vec<_>>();
					data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
			
					for (pos, render, _hidden) in data.iter() {
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
							MainMenuSelection::LoadGame => {
								saveload_system::load_game(&mut self.ecs);
								newrunstate = RunState::AwaitingInput;
								delete_save();
							}
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
				match *self.ecs.fetch::<RunState>() {
					RunState::MagicMapReveal{..} => newrunstate = RunState::MagicMapReveal{row:0},
					_ => newrunstate = RunState::MonsterTurn
				}				
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

			RunState::ShowRemoveItem => {
				let result = remove_item_menu(self, ctx);
				match result.0 {
					ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
					ItemMenuResult::NoResponse => {}
					ItemMenuResult::Selected => {
						let item_entity = result.1.unwrap();
						let mut intent = self.ecs.write_storage::<WantsToRemoveItem>();
						intent.insert(*self.ecs.fetch::<Entity>(), WantsToRemoveItem{ item: item_entity }).expect("Unable to insert intent to remove item.");
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

			RunState::SaveGame => {
				save_game(&mut self.ecs);
				
				newrunstate = RunState::MainMenu{ menu_selection: MainMenuSelection::LoadGame };
			}

			RunState::NextLevel => {
				self.goto_next_level();
				newrunstate = RunState::PreRun;
			}

			RunState::GameOver => {
				let result = game_over(ctx);
				match result {
					GameOverResult::NoSeleciton => {}
					GameOverResult::QuitToMenu => {
						self.game_over_cleanup();
						newrunstate = RunState::MainMenu { menu_selection: MainMenuSelection::NewGame };
					}
				}
			}

			RunState::MagicMapReveal{row} => {
				let mut map = self.ecs.fetch_mut::<Map>();
				for x in 0..MAPWIDTH {
					let idx = map.xy_idx(x as i32, row);
					map.revealed_tiles[idx] = true;
				}
				if row as usize == MAPHEIGHT - 1 {
					newrunstate = RunState::MonsterTurn;
				} else {
					newrunstate = RunState::MagicMapReveal{ row: row + 1 };
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
	gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

	let map : Map = map_builders::build_random_map(1);
	let (player_x, player_y) = map.rooms[0].center();

	let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

	gs.ecs.insert(rltk::RandomNumberGenerator::new());
	for room in map.rooms.iter().skip(1) {		
		spawner::spawn_room(&mut gs.ecs, room, 1);
	}

	gs.ecs.insert(map);
	gs.ecs.insert(Point::new(player_x, player_y));
	gs.ecs.insert(player_entity);
	gs.ecs.insert(RunState::MainMenu{menu_selection: MainMenuSelection::NewGame});
	gs.ecs.insert(gamelog::GameLog{ entries: vec!["Welcome to Rogue.".to_string()]});


	rltk::main_loop(context, gs)
}
