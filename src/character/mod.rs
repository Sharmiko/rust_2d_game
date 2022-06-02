use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

use ggez::{Context, GameResult};
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::graphics::{self, *};

use glam::Vec2;

use crate::consts::{CHAR_WIDTH, RUN_SPEED, JUMP_SPEED_DY, JUMP_SPEED_DX};

use crate::animation::SpriteAnimation;


pub mod chars;


#[derive(Debug, PartialEq, Eq, Hash)]
enum CharacterAnimation {
    Idle,
    Run,
    Jump,
    DoubleJump,
    Attack
}


pub struct CharacterState {
    is_flipped: bool,
    jumping_right: bool,
    jumping_left: bool
}

impl CharacterState {
    pub fn default() -> Self {
        Self {
            is_flipped: false,
            jumping_right: false,
            jumping_left: false
        }
    }
}


pub struct Location {
    x: f32,
    y: f32,
    src_x: f32
}

impl Location {
    pub fn default(_ctx: &mut Context) -> Self {
        let (_, h) = graphics::size(_ctx);
        Self {
            x: 100.,
            y: h - CHAR_WIDTH - 64.,
            src_x: 0.
        }
    }
}


pub struct Character {
    animations: HashMap<CharacterAnimation, RefCell<SpriteAnimation>>,
    location: Location,
    state: CharacterState,
    current: CharacterAnimation
}

impl Character {

    pub fn perform_action(&self, mut animation: RefMut<'_, SpriteAnimation>) {
        if !animation.performing {
            animation.performing = true;
        } else if animation.image_idx == animation.image_count {
            animation.performing = false;
            animation.image_idx = 0;
        }
    }

    pub fn param(&self, src_x: f32, w: f32) -> graphics::DrawParam{
        let mut params = graphics::DrawParam::default()
            .src(graphics::Rect {
                x: src_x,
                y: 0f32,
                w: w,
                h: 1f32
            })
            .dest(Vec2::new(self.location.x, self.location.y));
            

        let mut scale_x = 3f32;

        if self.state.is_flipped {
            scale_x *= -1.;
        }

        params = params.scale(Vec2::new(scale_x, 3f32));

        return params;
    }

    fn run_right(&mut self, _ctx: &mut Context) {
        self.current = CharacterAnimation::Run;
        let mut current_anim = self.animations.get(&self.current).unwrap().borrow_mut();
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
    }

    fn run_left(&mut self, _ctx: &mut Context) {
        self.current = CharacterAnimation::Run;
        let mut current_anim = self.animations.get(&self.current).unwrap().borrow_mut();
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
    }

    fn idle(&mut self, _ctx: &mut Context) {
        self.current = CharacterAnimation::Idle;
        let mut idle_anim = self.animations.get(&self.current).unwrap().borrow_mut();
        self.location.src_x = idle_anim.next_x();
    }

    fn perform_jump(&mut self, _ctx: &mut Context) {
        self.current = CharacterAnimation::Jump;
        let mut current_anim = self.animations.get(&self.current).unwrap().borrow_mut();
        if current_anim.image_idx * 2 < current_anim.image_count {
            self.location.y -= JUMP_SPEED_DY;
        } else {
            self.location.y += JUMP_SPEED_DY;
        }

        if self.state.jumping_right {
            self.location.x += JUMP_SPEED_DX;
        } else if self.state.jumping_left {
            self.location.x -= JUMP_SPEED_DX;
        }

        self.location.src_x = current_anim.next_x();
        self.perform_action(current_anim);
    }

    fn perform_attack(&mut self, _ctx: &mut Context) {
        self.current = CharacterAnimation::Attack;
        let mut current_anim = self.animations.get(&self.current).unwrap().borrow_mut();
        self.location.src_x = current_anim.next_x();
        self.perform_action(current_anim);
    }

    fn perform_double_jump(&mut self, _ctx: &mut Context) {
        self.current = CharacterAnimation::DoubleJump;
        let mut current_anim = self.animations.get(&self.current).unwrap().borrow_mut();
        self.location.src_x = current_anim.next_x();
        self.perform_action(current_anim);
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let jumping: bool = self.animations.get(&CharacterAnimation::Jump).unwrap().borrow().performing;
        let attacking: bool = self.animations.get(&CharacterAnimation::Attack).unwrap().borrow().performing;

        if !jumping {
            self.state.jumping_left = false;
            self.state.jumping_right = false;
        }

        if jumping || keyboard::is_key_pressed(_ctx, KeyCode::Space) {
            if keyboard::is_key_pressed(_ctx, KeyCode::D) && keyboard::is_key_pressed(_ctx, KeyCode::Space) {
                self.state.jumping_right = true;
                self.state.is_flipped = false;
            } else if keyboard::is_key_pressed(_ctx, KeyCode::A) && keyboard::is_key_pressed(_ctx, KeyCode::Space){
                self.state.is_flipped = true;
                self.state.jumping_left = true;
            }
            self.perform_jump(_ctx);
        } else if attacking || keyboard::is_key_pressed(_ctx, KeyCode::F) { 
            self.perform_attack(_ctx);
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
        let params = self.param(self.location.src_x, current_anim.image_width);
        current_anim.image.draw(ctx, params).unwrap();
    }
}
