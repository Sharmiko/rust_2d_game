use std::path::Path;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

use ggez::{Context, GameResult};
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::graphics::{self, *};

use glam::Vec2;

use crate::consts::{CHAR_WIDTH, RUN_SPEED};


pub struct Animation {
    image: Image,
    current: i8,
    total: i8,
    width: f32,
    performing: bool
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
            performing: false
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


pub struct Location {
    x: f32,
    y: f32,
    src_x: f32,
    flip: f32
}

impl Location {
    pub fn default() -> Self {
        Self {
            x: 100.,
            y: 100.,
            src_x: 0.,
            flip: 1f32
        }
    }
}


pub struct Character {
    animations: HashMap<String, RefCell<Animation>>,
    location: Location,
    state: CharacterState,
    current: String
}

impl Character {

    pub fn perform_action(&self, mut action: RefMut<'_, Animation>) {
        if !action.performing {
            action.performing = true;
        } else if action.current + 1 == action.total {
            action.performing = false;
        }
    }

    pub fn param(&self, src_x: f32, src_y: f32, w: f32, h: f32) -> graphics::DrawParam{
        let mut params = graphics::DrawParam::default()
            .src(graphics::Rect {
                x: src_x,
                y: src_y,
                w: w,
                h: h
            })
            .dest(Vec2::new(self.location.x, self.location.y));

        if self.state.is_flipped {
            params = params.scale(Vec2::new(-1f32, 1f32));
        }

        return params;
    }

    fn run_right(&mut self, _ctx: &mut Context) {
        let mut current_anim = self.animations.get("run").unwrap().borrow_mut();
        self.location.src_x = current_anim.next_x();
        if self.state.is_flipped {
            self.state.is_flipped = false;
            self.location.x -= CHAR_WIDTH / 2.;
        }

        self.location.x += RUN_SPEED;
        let (width, _) = graphics::size(_ctx);
        self.location.x = if self.location.x + CHAR_WIDTH / 2. <= width {
            self.location.x
        } else {
            width - CHAR_WIDTH / 2.
        };

        self.current = "run".to_string();
    }

    fn run_left(&mut self, _ctx: &mut Context) {
        let mut current_anim = self.animations.get("run").unwrap().borrow_mut();
        self.location.src_x = current_anim.next_x();
        if !self.state.is_flipped {
            self.state.is_flipped = true;
            self.location.x += CHAR_WIDTH / 2.;
        }

        self.location.x -= RUN_SPEED;
        self.location.x = if self.location.x - CHAR_WIDTH / 2. >= 0. {
            self.location.x
        } else {
            CHAR_WIDTH / 2.
        };

        self.current = "run".to_string();
    }

    fn idle(&mut self, _ctx: &mut Context) {
        let mut idle_anim = self.animations.get("idle").unwrap().borrow_mut();
        self.location.src_x = idle_anim.next_x();
        self.current = "idle".to_string();
    }

    fn perform_jump(&mut self, _ctx: &mut Context) {
        self.current = "jump".to_string();
        let mut current_anim = self.animations.get("jump").unwrap().borrow_mut();
        self.location.src_x = current_anim.next_x();
        self.perform_action(current_anim);
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let jumping: bool = self.animations.get("jump").unwrap().borrow_mut().performing;
        if jumping || keyboard::is_key_pressed(_ctx, KeyCode::Space) {
            self.perform_jump(_ctx);
        } else if keyboard::is_key_pressed(_ctx, KeyCode::D) {
            self.run_right(_ctx);
        } else if keyboard::is_key_pressed(_ctx, KeyCode::A) {
            self.run_left(_ctx);
        } else {
            self.idle(_ctx);
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context)  {
        let current_anim = self.animations.get(&self.current).unwrap().borrow_mut();
        let params = self.param(self.location.src_x, 0f32, current_anim.width, self.location.flip);
        current_anim.image.draw(ctx, params).unwrap();
    }
}


pub struct Punk;

impl Punk {
    pub fn new(_ctx: &mut Context) -> Character {
        let mut animations = HashMap::new();
        animations.insert("idle".to_string(), RefCell::new(Animation::new(_ctx, "/Punk_idle.png")));
        animations.insert("run".to_string(), RefCell::new(Animation::new(_ctx, "/Punk_run.png")));
        animations.insert("jump".to_string(), RefCell::new(Animation::new(_ctx, "/Punk_jump.png")));
        Character {
            animations: animations,
            location: Location::default(),
            state: CharacterState::default(),
            current: "idle".to_string()
        }
    }
}