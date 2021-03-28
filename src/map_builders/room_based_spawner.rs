use super::{MetaMapBuilder, BuilderMap, super::spawner::spawn_room};
use rltk::RandomNumberGenerator as RNG;

pub struct RoomBasedSpawner {}

impl MetaMapBuilder for RoomBasedSpawner {
    fn build_map(&mut self, rng: &mut RNG, build_data: &mut BuilderMap) {
        self.build(rng, build_data);
    }
}

impl RoomBasedSpawner {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomBasedSpawner> {
        Box::new(RoomBasedSpawner{})
    }

    fn build(&mut self, rng: &mut RNG, build_data: &mut BuilderMap) {
        if let Some(rooms) = &build_data.rooms {
            for room in rooms.iter().skip(1) {
                spawn_room(&build_data.map, rng, room, build_data.map.depth, &mut build_data.spawn_list);
            }
        } else {
            panic!("Cannot spawn rooms without creating any.");
        }
    }
}