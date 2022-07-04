use std::env;
use std::path::{self};
use std::collections::HashMap;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics;
use ggez::event::{self, EventHandler};


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

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    state_machine: StateMachine,
    screen_cords: graphics::Rect
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
            screen_cords: graphics::Rect {
                x: 0.,
                y: 0.,
                w: _ctx.gfx.drawable_size().0,
                h: _ctx.gfx.drawable_size().1,
            }
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 10;
        while _ctx.time.check_update_time(DESIRED_FPS) {
            let _dt = 1. / (DESIRED_FPS as f32);
            self.state_machine.update(_ctx);
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(_ctx, graphics::Color::WHITE);
        canvas.set_screen_coordinates(self.screen_cords);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());
        
        self.state_machine.draw(_ctx, &mut canvas);
        canvas.finish(_ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) -> GameResult {
        self.screen_cords = graphics::Rect::new(
            0.0,
            0.0,
            width as f32 * 1.0,
            height as f32 * 1.0,
        );

        Ok(())
    }
}