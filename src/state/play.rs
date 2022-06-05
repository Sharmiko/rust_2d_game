use std::cell::RefCell;

use ggez::Context;

use crate::state::{State, AllStates};
use crate::character::{Character};
use crate::base::{LocationType, ObjectLocation};
use crate::quadtree::QuadTree;
use crate::character::chars::{Punk, Biker, Cyborg};
use crate::tile::{Background, ParkBackground, Floor};


pub struct PlayState {
    player: Character,
    background: Background,
    floor: Floor
}

fn update_quadtree(tree: &mut QuadTree, location: LocationType) {
    match location {
        LocationType::Single(rect) => {
            tree.insert(rect.x, rect.y, rect);
        },
        LocationType::Multiple(locations) => {
            for loc in locations {
                tree.insert(loc.x, loc.y, loc);
            }
        }
    }
}

impl PlayState {
    pub fn new(ctx: &mut Context) ->  Self {

        let mut player = Punk::new(ctx);
        let floor = Floor::new(ctx);

        update_quadtree(&mut player.quadtree, floor.get_location(ctx));

        Self {
            player: player,
            background: ParkBackground::new(ctx),
            floor: floor
        }
    }
}


impl State for PlayState {


    fn enter(&self, _ctx: &mut Context, _current_state: &RefCell<AllStates>) {}

    fn exit(&self, _ctx: &mut Context, _current_state: &RefCell<AllStates>) {}

    fn draw(&mut self, _ctx: &mut Context, _current_state: &RefCell<AllStates>) {
        self.background.draw(_ctx);
        self.floor.draw(_ctx);
        self.player.draw(_ctx);
    }

    fn update(&mut self, _ctx: &mut Context, _current_state: &RefCell<AllStates>) {
        self.background.update(_ctx).unwrap();
        self.player.update(_ctx).unwrap();
    }
}