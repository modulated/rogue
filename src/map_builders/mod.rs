use super::{
    spawner::{spawn_entity, spawn_region},
    Map, Position, Rect, TileType, SHOW_MAPGEN_VISUALIZER,
};
use rltk::RandomNumberGenerator as Rng;
mod simple_map;
#[allow(unused_imports)]
use simple_map::SimpleMapBuilder;
mod common;
use common::*;
use specs::prelude::*;
mod bsp_dungeon;
#[allow(unused_imports)]
use bsp_dungeon::BspDungeonBuilder;
mod bsp_interior;
#[allow(unused_imports)]
use bsp_interior::BspInteriorBuilder;
mod cellular_automata;
#[allow(unused_imports)]
use cellular_automata::CellularAutomataBuilder;
mod drunkards_walk;
#[allow(unused_imports)]
use drunkards_walk::DrunkardsWalkBuilder;
mod maze;
#[allow(unused_imports)]
use maze::MazeBuilder;
mod dla;
#[allow(unused_imports)]
use dla::DLABuilder;
mod voronoi;
#[allow(unused_imports)]
use voronoi::VoronoiCellBuilder;
mod wfc;
#[allow(unused_imports)]
use wfc::WFCBuilder;
mod prefab_builder;
#[allow(unused_imports)]
use prefab_builder::PrefabBuilder;
mod room_based_spawner;
#[allow(unused_imports)]
use room_based_spawner::RoomBasedSpawner;
mod room_based_stairs;
#[allow(unused_imports)]
use room_based_stairs::RoomBasedStairs;
mod room_based_starting_position;
#[allow(unused_imports)]
use room_based_starting_position::RoomBasedStartingPosition;
mod area_starting_points;
#[allow(unused_imports)]
use area_starting_points::{AreaStartingPosition, XStart, YStart};
mod cull_unreachable;
#[allow(unused_imports)]
use cull_unreachable::CullUnreachable;
mod voronoi_spawning;
#[allow(unused_imports)]
use voronoi_spawning::VoronoiSpawning;
mod distant_exit;
#[allow(unused_imports)]
use distant_exit::DistantExit;
mod room_eroder;
#[allow(unused_imports)]
use room_eroder::RoomEroder;
mod room_rounder;
#[allow(unused_imports)]
use room_rounder::RoomRounder;
mod room_corridor_dogleg;
#[allow(unused_imports)]
use room_corridor_dogleg::DoglegCorridors;
mod room_corridor_bsp;
#[allow(unused_imports)]
use room_corridor_bsp::BspCorridors;
mod room_sorter;
#[allow(unused_imports)]
use room_sorter::{RoomSort, RoomSorter};
#[allow(unused_imports)]
mod room_draw;
use room_draw::RoomDrawer;
mod room_corridor_nearest;
#[allow(unused_imports)]
use room_corridor_nearest::NearestCorridors;
mod room_corridor_straight;
#[allow(unused_imports)]
use room_corridor_straight::StraightCorridors;
mod room_corridor_spawner;
#[allow(unused_imports)]
use room_corridor_spawner::CorridorSpawner;
mod door_builder;
#[allow(unused_imports)]
use door_builder::DoorBuilder;
mod edge_wall_builder;
#[allow(unused_imports)]
use edge_wall_builder::EdgeWallBuilder;
mod town;
#[allow(unused_imports)]
use town::*;

pub struct BuilderMap {
    pub spawn_list: Vec<(usize, String)>,
    pub map: Map,
    pub starting_position: Option<Position>,
    pub rooms: Option<Vec<Rect>>,
    pub corridors: Option<Vec<Vec<usize>>>,
    pub history: Vec<Map>,
    pub width: i32,
    pub height: i32,
}

impl BuilderMap {
    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            for v in snapshot.revealed_tiles.iter_mut() {
                *v = true;
            }
            self.history.push(snapshot);
        }
    }
}

pub struct BuilderChain {
    starter: Option<Box<dyn InitialMapBuilder>>,
    builders: Vec<Box<dyn MetaMapBuilder>>,
    pub build_data: BuilderMap,
}

impl BuilderChain {
    pub fn new(new_depth: i32, width: i32, height: i32) -> BuilderChain {
        BuilderChain {
            starter: None,
            builders: Vec::new(),
            build_data: BuilderMap {
                spawn_list: Vec::new(),
                map: Map::new(new_depth, width, height),
                starting_position: None,
                rooms: None,
                corridors: None,
                history: Vec::new(),
                width,
                height,
            },
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialMapBuilder>) {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("There can only be one starting builder."),
        }
    }

    pub fn with(&mut self, metabuilder: Box<dyn MetaMapBuilder>) {
        self.builders.push(metabuilder);
    }

    pub fn build_map(&mut self, rng: &mut Rng) {
        match &mut self.starter {
            None => panic!("Cannot build without an initial starting builder."),
            Some(starter) => {
                starter.build_map(rng, &mut self.build_data);
            }
        }

        for metabuilder in self.builders.iter_mut() {
            metabuilder.build_map(rng, &mut self.build_data);
        }
    }

    pub fn spawn_entities(&mut self, ecs: &mut World) {
        if self.build_data.spawn_list.len() == 0 {
            panic!("Cannot spawn without spawn list.");
        }

        for entity in self.build_data.spawn_list.iter() {
            spawn_entity(ecs, &(&entity.0, &entity.1));
        }
    }
}

pub trait InitialMapBuilder {
    fn build_map(&mut self, rng: &mut Rng, build_data: &mut BuilderMap);
}

pub trait MetaMapBuilder {
    fn build_map(&mut self, rng: &mut Rng, build_data: &mut BuilderMap);
}

pub fn level_builder(new_depth: i32, rng: &mut Rng, width: i32, height: i32) -> BuilderChain {
    println!("Depth: {}", new_depth);
    match new_depth {
        1 => town_builder(new_depth, rng, width, height),
        _ => random_builder(new_depth, rng, width, height),
    }
}

#[allow(unused_variables)]
pub fn random_builder(new_depth: i32, rng: &mut Rng, width: i32, height: i32) -> BuilderChain {
    let mut builder = BuilderChain::new(new_depth, width, height);
    let type_roll = rng.roll_dice(1, 2);
    match type_roll {
        1 => random_room_builder(rng, &mut builder),
        _ => random_shape_builder(rng, &mut builder),
    }

    if rng.roll_dice(1, 4) == 1 {
        builder.with(WFCBuilder::new());
        // builder.with(EdgeWallBuilder::new());
        let (start_x, start_y) = random_start_position(rng);
        builder.with(AreaStartingPosition::new(start_x, start_y));

        builder.with(VoronoiSpawning::new());
        builder.with(DistantExit::new());
    }

    if rng.roll_dice(1, 20) == 1 {
        builder.with(PrefabBuilder::sectional(
            prefab_builder::prefab_sections::UNDERGROUND_FORT,
        ));
    }

    builder.with(DoorBuilder::new());
    builder.with(PrefabBuilder::vaults());

    builder
}

#[allow(dead_code)]
fn random_start_position(rng: &mut Rng) -> (XStart, YStart) {
    let x;
    let xroll = rng.roll_dice(1, 3);
    match xroll {
        1 => x = XStart::Left,
        2 => x = XStart::Center,
        _ => x = XStart::Right,
    }

    let y;
    let yroll = rng.roll_dice(1, 3);
    match yroll {
        1 => y = YStart::Bottom,
        2 => y = YStart::Center,
        _ => y = YStart::Top,
    }

    (x, y)
}

#[allow(dead_code)]
fn random_room_builder(rng: &mut Rng, builder: &mut BuilderChain) {
    let build_roll = rng.roll_dice(1, 3);
    match build_roll {
        1 => builder.start_with(SimpleMapBuilder::new()),
        2 => builder.start_with(BspDungeonBuilder::new()),
        _ => builder.start_with(BspInteriorBuilder::new()),
    }

    // BSP Interior still makes holes in the walls
    if build_roll != 3 {
        // Sort by one of the 5 available algorithms
        let sort_roll = rng.roll_dice(1, 5);
        match sort_roll {
            1 => builder.with(RoomSorter::new(RoomSort::LeftMost)),
            2 => builder.with(RoomSorter::new(RoomSort::RightMost)),
            3 => builder.with(RoomSorter::new(RoomSort::TopMost)),
            4 => builder.with(RoomSorter::new(RoomSort::BottomMost)),
            _ => builder.with(RoomSorter::new(RoomSort::Central)),
        }

        builder.with(RoomDrawer::new());

        let corridor_roll = rng.roll_dice(1, 4);
        match corridor_roll {
            1 => builder.with(DoglegCorridors::new()),
            2 => builder.with(NearestCorridors::new()),
            3 => builder.with(StraightCorridors::new()),
            _ => builder.with(BspCorridors::new()),
        }

        let cspawn_roll = rng.roll_dice(1, 2);
        if cspawn_roll == 1 {
            builder.with(CorridorSpawner::new());
        }

        let modifier_roll = rng.roll_dice(1, 6);
        match modifier_roll {
            1 => builder.with(RoomEroder::new()),
            2 => builder.with(RoomRounder::new()),
            _ => {}
        }
    }

    let start_roll = rng.roll_dice(1, 2);
    match start_roll {
        1 => builder.with(RoomBasedStartingPosition::new()),
        _ => {
            let (start_x, start_y) = random_start_position(rng);
            builder.with(AreaStartingPosition::new(start_x, start_y));
        }
    }

    let exit_roll = rng.roll_dice(1, 2);
    match exit_roll {
        1 => builder.with(RoomBasedStairs::new()),
        _ => builder.with(DistantExit::new()),
    }

    let spawn_roll = rng.roll_dice(1, 2);
    match spawn_roll {
        1 => builder.with(RoomBasedSpawner::new()),
        _ => builder.with(VoronoiSpawning::new()),
    }
}

#[allow(dead_code)]
fn random_shape_builder(rng: &mut Rng, builder: &mut BuilderChain) {
    let builder_roll = rng.roll_dice(1, 16);
    match builder_roll {
        1 => builder.start_with(CellularAutomataBuilder::new()),
        2 => builder.start_with(DrunkardsWalkBuilder::open_area()),
        3 => builder.start_with(DrunkardsWalkBuilder::open_halls()),
        4 => builder.start_with(DrunkardsWalkBuilder::winding_passages()),
        5 => builder.start_with(DrunkardsWalkBuilder::fat_passages()),
        6 => builder.start_with(DrunkardsWalkBuilder::fearful_symmetry()),
        7 => builder.start_with(MazeBuilder::new()),
        8 => builder.start_with(DLABuilder::walk_inwards()),
        9 => builder.start_with(DLABuilder::walk_outwards()),
        10 => builder.start_with(DLABuilder::central_attractor()),
        11 => builder.start_with(DLABuilder::insectoid()),
        12 => builder.start_with(VoronoiCellBuilder::pythagoras()),
        13 => builder.start_with(VoronoiCellBuilder::manhattan()),
        _ => builder.start_with(PrefabBuilder::constant(
            prefab_builder::prefab_level::WFC_POPULATED,
        )),
    }

    // Set the start to the center and cull
    builder.with(AreaStartingPosition::new(XStart::Center, YStart::Center));
    builder.with(CullUnreachable::new());

    // Now set the start to a random starting area
    let (start_x, start_y) = random_start_position(rng);
    builder.with(AreaStartingPosition::new(start_x, start_y));

    // Setup an exit and spawn mobs
    builder.with(VoronoiSpawning::new());
    builder.with(DistantExit::new());
}
