use std::cell::RefCell;

use ggez::Context;
use ggez::graphics::{self};
use ggez::input::mouse::{self};

use crate::state::{State};
use crate::character::{Character, Punk};


pub struct PlayState {
    player: Character
}

impl PlayState {
    pub fn new(ctx: &mut Context) ->  Self {
        Self {
            player: Punk::new(ctx)
        }
    }
}


impl State for PlayState {


    fn enter(&self, ctx: &mut Context, current_state: &RefCell<String>) {}

    fn exit(&self, ctx: &mut Context, current_state: &RefCell<String>) {}

    fn draw(&mut self, ctx: &mut Context, current_state: &RefCell<String>) {
        self.player.draw(ctx);
    }

    fn update(&mut self, ctx: &mut Context, current_state: &RefCell<String>) {
        self.player.update(ctx);
    }
}