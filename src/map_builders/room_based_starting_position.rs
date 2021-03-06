use super::{BuilderMap, MetaMapBuilder, Position};
use rltk::RandomNumberGenerator as RNG;

pub struct RoomBasedStartingPosition {}

impl MetaMapBuilder for RoomBasedStartingPosition {
    fn build_map(&mut self, rng: &mut RNG, build_data: &mut BuilderMap) {
        self.build(rng, build_data);
    }
}

impl RoomBasedStartingPosition {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomBasedStartingPosition> {
        Box::new(RoomBasedStartingPosition {})
    }

    fn build(&mut self, _rng: &mut RNG, build_data: &mut BuilderMap) {
        if let Some(rooms) = &build_data.rooms {
            let start_pos = rooms[0].center();
            build_data.starting_position = Some(Position {
                x: start_pos.0,
                y: start_pos.1,
            });
        } else {
            panic!("Cannot set start point without rooms.");
        }
    }
}
