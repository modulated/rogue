use super::{apply_horizontal_tunnel, apply_vertical_tunnel, BuilderMap, MetaMapBuilder, Rect};
use rltk::RandomNumberGenerator as Rng;

pub struct DoglegCorridors {}

impl MetaMapBuilder for DoglegCorridors {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut Rng, build_data: &mut BuilderMap) {
        self.corridors(rng, build_data);
    }
}

impl DoglegCorridors {
    #[allow(dead_code)]
    pub fn new() -> Box<DoglegCorridors> {
        Box::new(DoglegCorridors {})
    }

    fn corridors(&mut self, rng: &mut Rng, build_data: &mut BuilderMap) {
        let rooms: Vec<Rect>;
        if let Some(rooms_buidler) = &build_data.rooms {
            rooms = rooms_buidler.clone();
        } else {
            panic!("Cannot place corridors without room structures.");
        }

        let mut corridors: Vec<Vec<usize>> = Vec::new();
        for (i, room) in rooms.iter().enumerate() {
            if i > 0 {
                let (new_x, new_y) = room.center();
                let (prev_x, prev_y) = rooms[i as usize - 1].center();
                if rng.range(0, 2) == 1 {
                    let mut c1 =
                        apply_horizontal_tunnel(&mut build_data.map, prev_x, new_x, prev_y);
                    let mut c2 = apply_vertical_tunnel(&mut build_data.map, prev_y, new_y, new_x);
                    c1.append(&mut c2);
                    corridors.push(c1);
                } else {
                    let mut c1 = apply_vertical_tunnel(&mut build_data.map, prev_y, new_y, prev_x);
                    let mut c2 = apply_horizontal_tunnel(&mut build_data.map, prev_x, new_x, new_y);
                    c1.append(&mut c2);
                    corridors.push(c1);
                }
                build_data.take_snapshot();
            }
        }
        build_data.corridors = Some(corridors);
    }
}
