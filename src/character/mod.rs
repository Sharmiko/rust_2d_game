use std::path::Path;

use ggez::{Context, GameResult};
use ggez::event::{EventHandler, KeyCode};
use ggez::input::keyboard;
use ggez::graphics::{self, *};

use glam::Vec2;

use crate::consts::{CHAR_WIDTH};


pub struct Animation {
    image: Image,
    current: i8,
    total: i8,
    width: f32
}


impl Animation {

    pub fn new(ctx: &mut Context, image_path: &str) -> Self {
        let image = graphics::Image::new(ctx, Path::new(image_path)).unwrap();
        let total = (image.width() / image.height()) as i8;
        Self {
            image: image,
            current: 0,
            total: total,
            width: 1. / total as f32,
        }
    }

    fn next_x(&mut self) -> f32 {
        if self.current == self.total {
            self.current = 0;
        }

        let x = 1. - ((self.total as f32 - self.current as f32) / self.total as f32);

        self.current += 1;

        return x;
    }
}


pub struct CharacterState {
    is_flipped: bool
}

impl CharacterState {
    pub fn default() -> Self {
        Self {
            is_flipped: false
        }
    }
}


pub struct Character {
    pub default: Animation,
    pub run: Animation,
    x: f32,
    y: f32,
    state: CharacterState
}

impl Character {

    pub fn param(&self, src_x: f32, src_y: f32, w: f32, h: f32) -> graphics::DrawParam{
        let mut params = graphics::DrawParam::default()
            .src(graphics::Rect {
                x: src_x,
                y: src_y,
                w: w,
                h: h
            })
            .dest(Vec2::new(self.x, self.y));

        if self.state.is_flipped {
            params = params.scale(Vec2::new(-1f32, 1f32));
        }

        return params;
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn run_right(&mut self, ctx: &mut Context) {
        let src_x = self.run.next_x();
        let params = self.param(src_x, 0f32, self.run.width, 1.);
        if self.state.is_flipped {
            self.state.is_flipped = false;
            self.x -= CHAR_WIDTH / 2.;
        }
        self.run.image.draw(ctx, params);
    }

    fn run_left(&mut self, ctx: &mut Context) {
        let src_x = self.run.next_x();
        let params = self.param(src_x, 0f32, self.run.width, 1.);
        if !self.state.is_flipped {
            self.state.is_flipped = true;
            self.x += CHAR_WIDTH / 2.;
        }
        self.run.image.draw(ctx, params);
    }

    fn idle(&mut self, ctx: &mut Context) {
        let src_x = self.default.next_x();
        let params = self.param(src_x, 0f32, self.default.width, 1.);
        self.default.image.draw(ctx, params);
    }

    pub fn draw(&mut self, ctx: &mut Context)  {
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.run_right(ctx);
        } else if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.run_left(ctx);
        } else {
            self.idle(ctx);
        }
    }
}


pub struct Punk;

impl Punk {
    pub fn new(_ctx: &mut Context) -> Character {
        Character {
            default: Animation::new(_ctx, "/Punk_idle.png"),
            run: Animation::new(_ctx, "/Punk_run.png"),
            x: 100.,
            y: 100.,
            state: CharacterState::default()
        }
    }
}