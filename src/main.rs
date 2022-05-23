use std::collections::HashMap;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, *};
use ggez::event::{self, EventHandler};
use ggez::timer::check_update_time;


mod consts;
mod state;
mod character;


use state::{StateMachine, State, MenuState, PlayState, AllStates};


fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Rust 2d game", "Giorgi Sharmiashvili")
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    state_machine: StateMachine,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let mut states = HashMap::new();

        let menu_state = MenuState::new(ctx);
        let play_state = PlayState::new(ctx);

        states.insert(AllStates::Menu.as_str(), Box::new(menu_state) as Box<dyn State>);
        states.insert(AllStates::Play.as_str(), Box::new(play_state) as Box<dyn State>);
        
        let state_machine = StateMachine::new(states, AllStates::Menu.as_str());
        MyGame {
            state_machine: state_machine,
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 10;
        while check_update_time(ctx, DESIRED_FPS) {
            let _dt = 1. / (DESIRED_FPS as f32);
            self.state_machine.update(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        self.state_machine.draw(ctx);
        graphics::present(ctx)
    }
}