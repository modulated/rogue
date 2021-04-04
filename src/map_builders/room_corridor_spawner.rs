use super::{spawn_region, BuilderMap, MetaMapBuilder};
use rltk::RandomNumberGenerator as Rng;

pub struct CorridorSpawner {}

impl MetaMapBuilder for CorridorSpawner {
    fn build_map(&mut self, rng: &mut Rng, build_data: &mut BuilderMap) {
        self.build(rng, build_data);
    }
}

impl CorridorSpawner {
    #[allow(dead_code)]
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    fn build(&mut self, rng: &mut Rng, build_data: &mut BuilderMap) {
        if let Some(corridors) = &build_data.corridors {
            for c in corridors.iter() {
                let depth = build_data.map.depth;
                spawn_region(&build_data.map, rng, &c, depth, &mut build_data.spawn_list);
            }
        } else {
            panic!("Corridor spawning requires corridors.");
        }
    }
}
