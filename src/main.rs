use std::env;
use std::path::{self};
use std::collections::HashMap;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, *};
use ggez::event::{self, EventHandler};
use ggez::timer::check_update_time;


mod consts;
mod state;
mod tile;
mod utils;
mod base;
mod collisions;
mod quadtree;
mod animation;
mod character;


use state::{StateMachine, State, MenuState, PlayState, AllStates};


fn main() {
    let mut cb = ContextBuilder::new("Rust 2d game", "Giorgi Sharmiashvili")
        .window_mode(ggez::conf::WindowMode::default()
            .resizable(true)
            .maximized(true));   

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }
    
    let (mut ctx, event_loop) = cb.build().expect("aieee, could not create ggez context!");

    graphics::set_default_filter(&mut ctx, graphics::FilterMode::Nearest);

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    state_machine: StateMachine,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let mut states = HashMap::new();

        let menu_state = MenuState::new(_ctx);
        let play_state = PlayState::new(_ctx);

        states.insert(AllStates::Menu, Box::new(menu_state) as Box<dyn State>);
        states.insert(AllStates::Play, Box::new(play_state) as Box<dyn State>);
        
        let state_machine = StateMachine::new(states, AllStates::Play);
        MyGame {
            state_machine: state_machine,
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 10;
        while check_update_time(_ctx, DESIRED_FPS) {
            let _dt = 1. / (DESIRED_FPS as f32);
            self.state_machine.update(_ctx);
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        graphics::clear(_ctx, Color::WHITE);
        self.state_machine.draw(_ctx);
        graphics::present(_ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(
            0.0,
            0.0,
            width as f32 * 1.0,
            height as f32 * 1.0,
        );
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}