use specs::prelude::*;
use super::{Viewshed, Monster};
use rltk::{field_of_view, Point, console};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
	type SystemData = (
		ReadStorage<'a, Viewshed>,
		ReadExpect<'a, Point>,
		ReadStorage<'a, Monster>
	);

	fn run(&mut self, data: Self::SystemData) {
		let (viewshed, point, monster) = data;

		for (viewshed, _monster) in (&viewshed, &monster).join() {
			if viewshed.visible_tiles.contains(&*point) {
				console::log("Monster shouts insults");
			}
			
		}
	}
}