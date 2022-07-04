use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

use ggez::{Context, GameResult};
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard;
use ggez::graphics::{self, *};

use glam::Vec2;

use crate::base::CustomRect;
use crate::quadtree::QuadTree;
use crate::animation::SpriteAnimation;
use crate::collisions::{rect_collision, SideCollided};
use crate::consts::{
    CHAR_WIDTH, RUN_SPEED, JUMP_SPEED_DY, 
    JUMP_SPEED_DX, CHAR_SCALE_FACTOR
};



pub mod chars;


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CharacterAnimation {
    Idle,
    Run,
    Jump,
    DoubleJump,
    Attack
}


pub struct CharacterState {
    is_flipped: bool,
    jumping_right: bool,
    jumping_left: bool,
    falling: bool
}

impl CharacterState {
    pub fn default() -> Self {
        Self {
            is_flipped: false,
            jumping_right: false,
            jumping_left: false,
            falling: true
        }
    }
}


pub struct Layout {
    x: f32,
    y: f32,
    w: f32,
    h: f32
}

impl Layout {
    pub fn default(_ctx: &mut Context) -> Self {
        let (_, h) = _ctx.gfx.drawable_size();
        Self {
            x: 100.,
            y: 100.,
            w: CHAR_WIDTH,
            h: CHAR_WIDTH
        }
    }
}


pub struct Character {
    animations: HashMap<CharacterAnimation, RefCell<SpriteAnimation>>,
    layout: Layout,
    state: CharacterState,
    current: CharacterAnimation,
    pub quadtree: QuadTree
}

impl Character {

    pub fn default(_ctx: &mut Context) -> Self {
        let (w, h) = _ctx.gfx.drawable_size();
        Self {
            animations: HashMap::new(),
            layout: Layout::default(_ctx),
            state: CharacterState::default(),
            current: CharacterAnimation::Idle,
            quadtree: QuadTree::new(0., 0., w, h)
        }
    }

    pub fn insert_animation(&mut self, animation: CharacterAnimation, sprite: SpriteAnimation) {
        self.animations.insert(animation, RefCell::new(sprite));
    }


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
            .dest(Vec2::new(self.layout.x, self.layout.y));
            //.image_scale(false);
            
        if self.state.is_flipped {
            params = params.scale([-CHAR_SCALE_FACTOR * w, CHAR_SCALE_FACTOR]);
        } else {
            params = params.scale([CHAR_SCALE_FACTOR * w, CHAR_SCALE_FACTOR]);
        }

        return params;
    }

    fn run_right(&mut self, _ctx: &mut Context) {
        self.current = CharacterAnimation::Run;
        let mut current_anim = self.animations.get(&self.current).unwrap().borrow_mut();
        current_anim.src_x = current_anim.next_x();
        if self.state.is_flipped {
            self.state.is_flipped = false;
            self.layout.x -= self.layout.w / 4.;
        }

        self.layout.x += RUN_SPEED;
        let (width, _) = _ctx.gfx.size();
        self.layout.x = if self.layout.x + self.layout.w / 2. <= width {
            self.layout.x
        } else {
            width - self.layout.w
        };
    }

    fn run_left(&mut self, _ctx: &mut Context) {
        self.current = CharacterAnimation::Run;
        let mut current_anim = self.animations.get(&self.current).unwrap().borrow_mut();
        current_anim.src_x = current_anim.next_x();
        if !self.state.is_flipped {
            self.state.is_flipped = true;
            self.layout.x += self.layout.w / 4.;
        }

        self.layout.x -= RUN_SPEED;
        self.layout.x = if self.layout.x - self.layout.w / 2. >= 0. {
            self.layout.x
        } else {
            self.layout.w / 2.
        };
    }

    fn idle(&mut self, _ctx: &mut Context) {
        self.current = CharacterAnimation::Idle;
        let mut idle_anim = self.animations.get(&self.current).unwrap().borrow_mut();
        idle_anim.src_x = idle_anim.next_x();
    }

    fn perform_jump(&mut self, _ctx: &mut Context) {
        self.current = CharacterAnimation::Jump;
        let mut current_anim = self.animations.get(&self.current).unwrap().borrow_mut();
        if current_anim.image_idx * 2 < current_anim.image_count {
            self.layout.y -= JUMP_SPEED_DY;
        } else {
            self.layout.y += JUMP_SPEED_DY;
        }

        if self.state.jumping_right {
            self.layout.x += JUMP_SPEED_DX;
        } else if self.state.jumping_left {
            self.layout.x -= JUMP_SPEED_DX;
        }

        current_anim.src_x = current_anim.next_x();
        self.perform_action(current_anim);
    }

    fn perform_attack(&mut self, _ctx: &mut Context) {
        self.current = CharacterAnimation::Attack;
        let mut current_anim = self.animations.get(&self.current).unwrap().borrow_mut();
        current_anim.src_x = current_anim.next_x();
        self.perform_action(current_anim);
    }

    // fn perform_double_jump(&mut self, _ctx: &mut Context) {
    //     self.current = CharacterAnimation::DoubleJump;
    //     let mut current_anim = self.animations.get(&self.current).unwrap().borrow_mut();
    //     current_anim.src_x = current_anim.next_x();
    //     self.perform_action(current_anim);
    // }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()>{
        if !self.state.falling {
            self._update(_ctx).unwrap();
        } else {
            self.layout.y += 50.;
        }

        let mut char_rect = CustomRect::new(
            self.layout.x,
            self.layout.y,
            self.layout.w,
            self.layout.h
        );
        if self.state.is_flipped {
            char_rect.fields.x -= self.layout.w / 4.;
        }
        let data = self.quadtree.search(self.layout.x, self.layout.y);
        if data.is_some() {
            for loc in data.unwrap() {
                let side = rect_collision(&char_rect.fields, loc);
                if side.is_some() {
                    match side.unwrap() {
                        SideCollided::Top => {
                            if self.state.falling {
                                self.layout.y = loc.y - self.layout.h;
                            }
                            self.state.falling = false;
                        }
                        _ => ()
                    }
                }
            }
        }


        Ok(())
    }

    pub fn _update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let jumping: bool = self.animations.get(&CharacterAnimation::Jump).unwrap().borrow().performing;
        let attacking: bool = self.animations.get(&CharacterAnimation::Attack).unwrap().borrow().performing;

        if !jumping {
            self.state.jumping_left = false;
            self.state.jumping_right = false;
        }

        if jumping || _ctx.keyboard.is_key_pressed(KeyCode::Space) {
            if _ctx.keyboard.is_key_pressed(KeyCode::D) && _ctx.keyboard.is_key_pressed(KeyCode::Space) {
                self.state.jumping_right = true;
                self.state.is_flipped = false;
            } else if _ctx.keyboard.is_key_pressed(KeyCode::A) && _ctx.keyboard.is_key_pressed(KeyCode::Space){
                self.state.is_flipped = true;
                self.state.jumping_left = true;
            }
            self.perform_jump(_ctx);
        } else if attacking || _ctx.keyboard.is_key_pressed(KeyCode::F) { 
            self.perform_attack(_ctx);
        } else if _ctx.keyboard.is_key_pressed(KeyCode::D) {
            self.run_right(_ctx);
        } else if _ctx.keyboard.is_key_pressed(KeyCode::A) {
            self.run_left(_ctx);
        } else {
            self.idle(_ctx);
        }
        Ok(())
    }


    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas)  {
        let current_anim = self.animations.get(&self.current).unwrap().borrow_mut();
        let params = self.param(current_anim.src_x, current_anim.image_width);
        canvas.draw(&current_anim.image, params);

        // TODO - remove later
        // START
        let mut char_rect = CustomRect::new(
            self.layout.x,
            self.layout.y,
            self.layout.w,
            self.layout.h
        );
        if self.state.is_flipped {
            char_rect.fields.x -= self.layout.w / 4.;
        }
        char_rect.draw(ctx, canvas);

        self.quadtree.draw_boundries(ctx, canvas, graphics::Color::BLUE);
        
        let data = self.quadtree.search(self.layout.x, self.layout.y);
        if data.is_some() {
            for loc in data.unwrap() {
                let mut rect = CustomRect::from_rect(*loc);
                rect.draw(ctx, canvas);
            }
        }
        // END
    }
}
