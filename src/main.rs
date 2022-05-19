use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, *};
use ggez::event::{self, EventHandler};
use ggez::timer::sleep;
use glam::*;

use std::path::Path;
use std::time::Duration;

mod consts;
mod character;

use character::{Character, Punk};


fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
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
        let path = Path::new("/Punk_idle.png");
        let image = graphics::Image::new(_ctx, path).unwrap();
        MyGame {
            char: Punk::new(_ctx)
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        sleep(Duration::from_millis(150));
        graphics::clear(ctx, Color::WHITE);
        self.char.draw(ctx);
        graphics::present(ctx)
    }
}