use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, *};
use ggez::event::{self, EventHandler};
use ggez::timer::check_update_time;


mod consts;
mod character;

use character::{Character, Punk};


fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Rust 2d game", "Giorgi Sharmiashvili")
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    char: Character
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
            char: Punk::new(_ctx)
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 10;
        while check_update_time(_ctx, DESIRED_FPS) {
            let _dt = 1. / (DESIRED_FPS as f32);
            self.char.update(_ctx).unwrap();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        self.char.draw(ctx);
        graphics::present(ctx)
    }
}