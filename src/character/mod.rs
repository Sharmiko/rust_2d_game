use std::path::Path;
use std::cell::{RefCell, RefMut};

use ggez::{Context, GameResult};
use ggez::event::{EventHandler, KeyCode};
use ggez::input::keyboard;
use ggez::graphics::{self, *};

use glam::Vec2;

use crate::consts::{CHAR_WIDTH, RUN_SPEED};


type Action = RefCell<MultiStageAnimation>;


pub struct MultiStageAnimation {
    animation: Animation,
    performing: bool
}

impl MultiStageAnimation {
    pub fn new(ctx: &mut Context, image_path: &str) -> Action {
        RefCell::new(Self {
            animation: Animation::new(ctx, image_path),
            performing: false
        })
    }
} 


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
    pub jump: Action,
    x: f32,
    y: f32,
    state: CharacterState
}

impl Character {

    pub fn perform_action(&self, ctx: &mut Context, mut action: RefMut<'_, MultiStageAnimation>) {
        if !action.performing {
            action.performing = true;
        } else if action.animation.current + 1 == action.animation.total {
            action.performing = false;
        }

        let src_x = action.animation.next_x();
        let params = self.param(src_x, 0f32, action.animation.width, 1.);
        action.animation.image.draw(ctx, params);
    }

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

        self.x += RUN_SPEED;
        let (width, _) = graphics::size(ctx);
        self.x = if self.x + CHAR_WIDTH / 2. <= width {
            self.x
        } else {
            width - CHAR_WIDTH / 2.
        };

        self.run.image.draw(ctx, params);
    }

    fn run_left(&mut self, ctx: &mut Context) {
        let src_x = self.run.next_x();
        let params = self.param(src_x, 0f32, self.run.width, 1.);
        if !self.state.is_flipped {
            self.state.is_flipped = true;
            self.x += CHAR_WIDTH / 2.;
        }

        self.x -= RUN_SPEED;
        self.x = if self.x - CHAR_WIDTH / 2. >= 0. {
            self.x
        } else {
            CHAR_WIDTH / 2.
        };

        self.run.image.draw(ctx, params);
    }

    fn idle(&mut self, ctx: &mut Context) {
        let src_x = self.default.next_x();
        let params = self.param(src_x, 0f32, self.default.width, 1.);
        self.default.image.draw(ctx, params);
    }

    fn perform_jump(&mut self, ctx: &mut Context) {
        self.perform_action(ctx, self.jump.borrow_mut());
    }

    pub fn draw(&mut self, ctx: &mut Context)  {
        if self.jump.borrow_mut().performing || keyboard::is_key_pressed(ctx, KeyCode::Space) {
            self.perform_jump(ctx);
        } else if keyboard::is_key_pressed(ctx, KeyCode::D) {
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
            jump: MultiStageAnimation::new(_ctx, "/Punk_jump.png"),
            x: 100.,
            y: 100.,
            state: CharacterState::default()
        }
    }
}