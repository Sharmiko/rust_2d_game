use std::cell::RefCell;
use std::collections::HashMap;

use ggez::Context;
use ggez::graphics::{self, *};

use crate::animation::{SpriteAnimation, HumanAnimation};
use crate::consts::{
    CHAR_WIDTH, RUN_SPEED, JUMP_SPEED_DY, 
    JUMP_SPEED_DX, CHAR_SCALE_FACTOR
};


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Animation {
    Idle,
    Run,
    Jump,
    DoubleJump,
    Attack
}


pub struct State {
    pub is_flipped: bool,
    pub jumping_right: bool,
    pub jumping_left: bool,
    pub falling: bool
}

impl State {
    pub fn default() -> Self {
        Self {
            is_flipped: false,
            jumping_right: false,
            jumping_left: false,
            falling: true
        }
    }
}


pub struct BaseHuman {
    pub animations: HashMap<Animation, RefCell<SpriteAnimation>>,
    pub layout: graphics::Rect,
    pub state: State,
    pub current: Animation
}

impl HumanAnimation for BaseHuman {

    fn update_current_anim(&mut self, animation_state: Animation) {
        self.current = animation_state;
        let mut anim = self.animations.get(&self.current).unwrap().borrow_mut();
        anim.src_x = anim.next_x();
    }

    fn run_right(&mut self, _ctx: &mut Context) {
        self.update_current_anim(Animation::Run);
        if self.state.is_flipped {
            self.state.is_flipped = false;
            self.layout.x -= self.layout.w / 4.;
        }
        self.layout.x += RUN_SPEED;
    }
    fn run_left(&mut self, _ctx: &mut Context) {
        self.update_current_anim(Animation::Run);
        if !self.state.is_flipped {
            self.state.is_flipped = true;
            self.layout.x += self.layout.w / 4.;
        }
        self.layout.x -= RUN_SPEED;
    }
    fn idle(&mut self, _ctx: &mut Context) {    
        self.update_current_anim(Animation::Idle);
    }
    fn perform_jump(&mut self, _ctx: &mut Context) {
        self.update_current_anim(Animation::Jump);
        let anim = self.animations.get(&self.current).unwrap().borrow_mut();
        if anim.image_idx * 2 <= anim.image_count {
            self.layout.y -= JUMP_SPEED_DY;
        } else {
            self.layout.y += JUMP_SPEED_DY;
        }

        if self.state.jumping_right {
            self.layout.x += JUMP_SPEED_DX;
        } else if self.state.jumping_left {
            self.layout.x += JUMP_SPEED_DX;
        }

        self.perform_action(anim);
    }

    fn perform_attack(&mut self, _ctx: &mut Context) {
        self.update_current_anim(Animation::Attack);
        self.perform_action(self.animations.get(&self.current).unwrap().borrow_mut());
    }
}


impl BaseHuman {
    pub fn default(_ctx: &mut Context) -> Self {
        let (w, h) = _ctx.gfx.drawable_size();
        Self {
            animations: HashMap::new(),
            layout: Rect {
                x: 100.,
                y: 100.,
                w: CHAR_WIDTH,
                h: CHAR_WIDTH,
            },
            state: State::default(),
            current: Animation::Idle,
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
            .dest([self.layout.x, self.layout.y]);
            
        if self.state.is_flipped {
            params = params.scale([-CHAR_SCALE_FACTOR * w, CHAR_SCALE_FACTOR]);
        } else {
            params = params.scale([CHAR_SCALE_FACTOR * w, CHAR_SCALE_FACTOR]);
        }

        return params;
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas)  {
        let current_anim = self.animations.get(&self.current).unwrap().borrow_mut();
        let params = self.param(current_anim.src_x, current_anim.image_width);
        canvas.draw(&current_anim.image, params);
    }
}