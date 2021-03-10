use rltk:: {RGB, Rltk, Point, VirtualKeyCode };
use specs::prelude::*;
use super::{CombatStats, Player, GameLog, Map, Name, Position, State, InBackpack, Viewshed, RunState};

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult { Cancel, NoResponse, Selected }

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection { NewGame, LoadGame, Quit }

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult { NoSeleciton{ selected: MainMenuSelection }, Selected{ selected: MainMenuSelection } }

pub fn show_inventory(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
	let player_entity = gs.ecs.fetch::<Entity>();
	let names = gs.ecs.read_storage::<Name>();
	let backpack = gs.ecs.read_storage::<InBackpack>();
	let entities = gs.ecs.entities();

	let inventory = (&backpack, &names).join().filter(|item| item.0.owner == *player_entity);
	let count = inventory.count();

	let mut y = (25 - (count / 2)) as i32;
	ctx.draw_box(15, y-2, 31, (count+3) as i32, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
	ctx.print_color(18, y-2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Inventory");
	ctx.print_color(18, y+count as i32+1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "ESCAPE to cancel");

	let mut equippable: Vec<Entity> = Vec::new();
	let mut j = 0;
	for (entity, _pack, name) in (&entities, &backpack, &names).join().filter(|item| item.1.owner == *player_entity ) {
		ctx.set(17, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437('('));
		ctx.set(18, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 97+j as rltk::FontCharType);
		ctx.set(19, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437(')'));

		ctx.print(21, y, &name.name.to_string());
		equippable.push(entity);
		y += 1;
		j += 1;
	}

	match ctx.key {
		None => (ItemMenuResult::NoResponse, None),
		Some(key) => {
			match key {
				VirtualKeyCode::Escape => { (ItemMenuResult::Cancel, None) }
				_ => {
					let selection = rltk::letter_to_option(key);
					if selection > -1 && selection < count as i32 {
						return (ItemMenuResult::Selected, Some(equippable[selection as usize]));
					}
					(ItemMenuResult::NoResponse, None)
				}
			}
		}
	}
}

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
	ctx.draw_box(0, 43, 79, 6, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

	let combat_stats = ecs.read_storage::<CombatStats>();
	let players = ecs.read_storage::<Player>();
	for (_player, stats) in (&players, &combat_stats).join() {
		let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
		ctx.print_color(12, 43, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &health);

		ctx.draw_bar_horizontal(28, 43, 51, stats.hp, stats.max_hp, RGB::named(rltk::RED), RGB::named(rltk::BLACK));

		let log = ecs.fetch::<GameLog>();

		let mut y = 44;
		for s in log.entries.iter().rev() {
			if y < 49 { ctx.print(2, y, s); }
			y += 1;
		}
	}

	let mouse_pos = ctx.mouse_pos();
	ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));

	draw_tooltips(&ecs, ctx);
}

pub fn main_menu(gs: &mut State, ctx: &mut Rltk) -> MainMenuResult {
	let runstate = gs.ecs.fetch::<RunState>();

	ctx.print_color_centered(15, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Rogue");
	
	if let RunState::MainMenu{ menu_selection: selection } = *runstate {
		if selection == MainMenuSelection::NewGame {
            ctx.print_color_centered(24, RGB::named(rltk::MAGENTA), RGB::named(rltk::BLACK), "Begin New Game");
        } else {
            ctx.print_color_centered(24, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Begin New Game");
        }

        if selection == MainMenuSelection::LoadGame {
            ctx.print_color_centered(25, RGB::named(rltk::MAGENTA), RGB::named(rltk::BLACK), "Load Game");
        } else {
            ctx.print_color_centered(25, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Load Game");
        }

        if selection == MainMenuSelection::Quit {
            ctx.print_color_centered(26, RGB::named(rltk::MAGENTA), RGB::named(rltk::BLACK), "Quit");
        } else {
            ctx.print_color_centered(26, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Quit");
        }

		match ctx.key {
			None => return MainMenuResult::NoSeleciton{ selected: selection },
			Some(key) => {
				match key {
					VirtualKeyCode::Escape => { return MainMenuResult::NoSeleciton{ selected: MainMenuSelection::Quit }}
					VirtualKeyCode::Up => {
						let newselection;
						match selection {
							MainMenuSelection::NewGame => newselection = MainMenuSelection::Quit,
                            MainMenuSelection::LoadGame => newselection = MainMenuSelection::NewGame,
                            MainMenuSelection::Quit => newselection = MainMenuSelection::LoadGame                        
						}
						return MainMenuResult::NoSeleciton{ selected: newselection };
					}
					VirtualKeyCode::Down => {
						let newselection;
						match selection {
							MainMenuSelection::NewGame => newselection = MainMenuSelection::LoadGame,
                            MainMenuSelection::LoadGame => newselection = MainMenuSelection::Quit,
                            MainMenuSelection::Quit => newselection = MainMenuSelection::NewGame                        
						}
						return MainMenuResult::NoSeleciton{ selected: newselection };
					}
					VirtualKeyCode::Return => return MainMenuResult::Selected{ selected: selection },
					_ => return MainMenuResult::NoSeleciton{ selected: selection }
				}
			}
		}
	}

	MainMenuResult::NoSeleciton { selected: MainMenuSelection::NewGame }
}

pub fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
	let map = ecs.fetch::<Map>();
	let names = ecs.read_storage::<Name>();
	let positions = ecs.read_storage::<Position>();

	let mouse_pos = ctx.mouse_pos();
	if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height { return; }

	let mut tooltip: Vec<String> = Vec::new();
	for (name, position) in (&names, &positions).join() {
		let idx = map.xy_idx(position.x, position.y);
		if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
			tooltip.push(name.name.to_string());
		}
	}

	if !tooltip.is_empty() {
		let mut width: i32 = 0;
		for s in tooltip.iter() {
			if width < s.len() as i32 { width = s.len() as i32; }
		}
		width += 3;

		if mouse_pos.0 > 40 {
			let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
			let left_x = mouse_pos.0 - width;
			let mut y = mouse_pos.1;
			for s in tooltip.iter() {
				ctx.print_color(left_x, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), s);
				let padding = (width - s.len() as i32) - 1;
				for i in 0..padding {
					ctx.print_color(arrow_pos.x - i, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &" ".to_string());
				}
				y += 1;
			}
			ctx.print_color(arrow_pos.x, arrow_pos.y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &"->".to_string());
		} else {
			let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
			let left_x = mouse_pos.0 +3;
			let mut y = mouse_pos.1;
			for s in tooltip.iter() {
				ctx.print_color(left_x + 1, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), s);
				let padding = (width - s.len() as i32)-1;
				for i in 0..padding {
					ctx.print_color(arrow_pos.x + 1 + i, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &" ".to_string());
				}
				y += 1;
			}
			ctx.print_color(arrow_pos.x, arrow_pos.y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &"<-".to_string());
		}
	}
}

pub fn drop_item_menu(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
	let player_entity = gs.ecs.fetch::<Entity>();
	let names = gs.ecs.read_storage::<Name>();
	let backpack = gs.ecs.read_storage::<InBackpack>();
	let entities = gs.ecs.entities();

	let inventory = (&backpack, &names).join().filter(|item| item.0.owner == *player_entity);
	let count = inventory.count();

	let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(15, y-2, 31, (count+3) as i32, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    ctx.print_color(18, y-2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Drop Which Item?");
    ctx.print_color(18, y+count as i32+1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "ESCAPE to cancel");
    
	let mut equippable : Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, _pack, name) in (&entities, &backpack, &names).join().filter(|item| item.1.owner == *player_entity ) {
        ctx.set(17, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437('('));
        ctx.set(18, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 97+j as rltk::FontCharType);
        ctx.set(19, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437(')'));

        ctx.print(21, y, &name.name.to_string());
        equippable.push(entity);
        y += 1;
        j += 1;
    }

	match ctx.key {
		None => (ItemMenuResult::NoResponse, None),
		Some(key) => {
			match key {
				VirtualKeyCode::Escape => { (ItemMenuResult::Cancel, None)}
				_ => {
					let selection = rltk::letter_to_option(key);
					if selection > -1 && selection < count as i32 {
						return (ItemMenuResult::Selected, Some(equippable[selection as usize]));
					}
					(ItemMenuResult::NoResponse, None)
				}
			}
		}
	}
}

pub fn ranged_target(gs: &mut State, ctx: &mut Rltk, range: i32) -> (ItemMenuResult, Option<Point>) {
	let player_entity = gs.ecs.fetch::<Entity>();
	let player_pos = gs.ecs.fetch::<Point>();
	let viewsheds = gs.ecs.read_storage::<Viewshed>();

	ctx.print_color(5, 0, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Select Target:");

	// Highlight available target tiles
	let mut available_cells = Vec::new();
	let visible = viewsheds.get(*player_entity);
	if let Some(visible) = visible {
		for idx in visible.visible_tiles.iter() {
			let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_pos, *idx);
			if distance <= range as f32 {
				ctx.set_bg(idx.x, idx.y, RGB::named(rltk::BLUE));
				available_cells.push(idx);
			}	
		}
	} else {
		return (ItemMenuResult::Cancel, None);
	}

	// Draw mouse cursor
	let mouse_pos = ctx.mouse_pos();
	let mut valid_target = false;
	for idx in available_cells.iter() {
		if idx.x == mouse_pos.0 && idx.y == mouse_pos.1 {
			valid_target = true;
		}		
	}
	if valid_target {
		if ctx.left_click {
			return (ItemMenuResult::Selected, Some(Point::new(mouse_pos.0, mouse_pos.1)));
		}
	} else {
		ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::RED));
		if ctx.left_click {
			return (ItemMenuResult::Cancel, None);
		}
	}

	(ItemMenuResult::NoResponse, None)
}