use std::cell::RefCell;

use ggez::Context;

use crate::state::{State, AllStates};
use crate::character::{Character};
use crate::character::chars::{Punk, Biker, Cyborg};
use crate::tile::{Background, ParkBackground};


pub struct PlayState {
    player: Character,
    background: Background
}

impl PlayState {
    pub fn new(ctx: &mut Context) ->  Self {
        Self {
            player: Punk::new(ctx),
            background: ParkBackground::new(ctx)
        }
    }
}


impl State for PlayState {


    fn enter(&self, _ctx: &mut Context, _current_state: &RefCell<AllStates>) {}

    fn exit(&self, _ctx: &mut Context, _current_state: &RefCell<AllStates>) {}

    fn draw(&mut self, _ctx: &mut Context, _current_state: &RefCell<AllStates>) {
        self.background.draw(_ctx);
        self.player.draw(_ctx);
    }

    fn update(&mut self, _ctx: &mut Context, _current_state: &RefCell<AllStates>) {
        self.background.update(_ctx).unwrap();
        self.player.update(_ctx).unwrap();
    }
}