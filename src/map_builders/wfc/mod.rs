use super::{BuilderMap, Map, MetaMapBuilder, TileType};
use rltk::RandomNumberGenerator as Rng;
mod constraints;
use constraints::*;
mod common;
use common::*;
mod solver;
use solver::Solver;

pub struct WFCBuilder {}

impl MetaMapBuilder for WFCBuilder {
    fn build_map(&mut self, rng: &mut Rng, build_data: &mut BuilderMap) {
        self.build(rng, build_data);
    }
}

impl WFCBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<WFCBuilder> {
        Box::new(WFCBuilder {})
    }

    fn build(&mut self, rng: &mut Rng, build_data: &mut BuilderMap) {
        const CHUNK_SIZE: i32 = 8;
        build_data.take_snapshot();

        let patterns = build_patterns(&build_data.map, CHUNK_SIZE, true, true);
        let constraints = patterns_to_constraints(patterns, CHUNK_SIZE);
        self.render_tile_gallery(&constraints, CHUNK_SIZE, build_data);

        build_data.map = Map::new(build_data.map.depth, build_data.width, build_data.height);
        loop {
            let mut solver = Solver::new(constraints.clone(), CHUNK_SIZE, &build_data.map);
            while !solver.iteration(&mut build_data.map, rng) {
                build_data.take_snapshot();
            }
            build_data.take_snapshot();
            if solver.possible {
                break;
            } // If it has hit an impossible condition, try again
        }
        build_data.spawn_list.clear();
    }

    fn render_tile_gallery(
        &mut self,
        constraints: &[MapChunk],
        chunk_size: i32,
        build_data: &mut BuilderMap,
    ) {
        build_data.map = Map::new(0, build_data.width, build_data.height);
        let mut counter = 0;
        let mut x = 1;
        let mut y = 1;
        while counter < constraints.len() {
            render_pattern_to_map(&mut build_data.map, &constraints[counter], chunk_size, x, y);

            x += chunk_size + 1;
            if x + chunk_size > build_data.map.width {
                // Move to the next row
                x = 1;
                y += chunk_size + 1;

                if y + chunk_size > build_data.map.height {
                    // Move to the next page
                    build_data.take_snapshot();
                    build_data.map = Map::new(0, build_data.width, build_data.height);

                    x = 1;
                    y = 1;
                }
            }

            counter += 1;
        }
        build_data.take_snapshot();
    }
}
