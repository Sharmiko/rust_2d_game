use std::cell::RefCell;

use ggez::{Context, GameResult};
use ggez::graphics::Canvas;
use ggez::input::keyboard::KeyCode;

use crate::base::CustomRect;
use crate::quadtree::QuadTree;
use crate::animation::{SpriteAnimation, HumanAnimation};
use crate::collisions::{rect_collision, SideCollided};
use crate::base::human::{BaseHuman, Animation};

pub mod chars;


pub struct Character {
    pub entity: BaseHuman,
    pub quadtree: QuadTree
}

impl Character {

    pub fn default(_ctx: &mut Context) -> Self {
        let (w, h) = _ctx.gfx.drawable_size();
        Self {
            entity: BaseHuman::default(_ctx),
            quadtree: QuadTree::new(0., 0., w, h)
        }
    }

    pub fn insert_animation(&mut self, animation: Animation, sprite: SpriteAnimation) {
        self.entity.animations.insert(animation, RefCell::new(sprite));
    }


    fn run_right(&mut self, _ctx: &mut Context) {
        self.entity.run_right(_ctx);
        let (width, _) = _ctx.gfx.size();
        self.entity.layout.x = if self.entity.layout.x + self.entity.layout.w / 2. <= width {
            self.entity.layout.x
        } else {
            width - self.entity.layout.w
        };
    }

    fn run_left(&mut self, _ctx: &mut Context) {
        self.entity.run_left(_ctx);
        self.entity.layout.x = if self.entity.layout.x - self.entity.layout.w / 2. >= 0. {
            self.entity.layout.x
        } else {
            self.entity.layout.w / 2.
        };
    }

    fn idle(&mut self, _ctx: &mut Context) {
        self.entity.idle(_ctx);
    }

    fn perform_jump(&mut self, _ctx: &mut Context) {
        self.entity.perform_jump(_ctx);
    }

    fn perform_attack(&mut self, _ctx: &mut Context) {
        self.entity.perform_attack(_ctx);
    }

    // fn perform_double_jump(&mut self, _ctx: &mut Context) {
    //     self.current = CharacterAnimation::DoubleJump;
    //     let mut current_anim = self.animations.get(&self.current).unwrap().borrow_mut();
    //     current_anim.src_x = current_anim.next_x();
    //     self.perform_action(current_anim);
    // }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()>{
        if !self.entity.state.falling {
            self._update(_ctx).unwrap();
        } else {
            self.entity.layout.y += 50.;
        }

        let mut char_rect = CustomRect::new(
            self.entity.layout.x,
            self.entity.layout.y,
            self.entity.layout.w,
            self.entity.layout.h
        );
        if self.entity.state.is_flipped {
            char_rect.fields.x -= self.entity.layout.w / 4.;
        }
        let data = self.quadtree.search(self.entity.layout.x, self.entity.layout.y);
        if data.is_some() {
            for loc in data.unwrap() {
                let side = rect_collision(&char_rect.fields, loc);
                if side.is_some() {
                    match side.unwrap() {
                        SideCollided::Top => {
                            if self.entity.state.falling {
                                self.entity.layout.y = loc.y - self.entity.layout.h;
                            }
                            self.entity.state.falling = false;
                        }
                        _ => ()
                    }
                }
            }
        }

        Ok(())
    }

    pub fn _update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let jumping: bool = self.entity.animations.get(&Animation::Jump).unwrap().borrow().performing;
        let attacking: bool = self.entity.animations.get(&Animation::Attack).unwrap().borrow().performing;
        if !jumping {
            self.entity.state.jumping_left = false;
            self.entity.state.jumping_right = false;
        }
        
        if jumping || _ctx.keyboard.is_key_pressed(KeyCode::Space) {
            if _ctx.keyboard.is_key_pressed(KeyCode::D) {
                self.entity.state.jumping_right = true;
                self.entity.state.is_flipped = false;
            } else if _ctx.keyboard.is_key_pressed(KeyCode::A) {
                self.entity.state.is_flipped = true;
                self.entity.state.jumping_left = true;
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
        self.entity.draw(ctx, canvas);

        // TODO - remove later
        // START
        // let mut char_rect = CustomRect::new(
        //     self.layout.x,
        //     self.layout.y,
        //     self.layout.w,
        //     self.layout.h
        // );
        // if self.entity.state.is_flipped {
        //     char_rect.fields.x -= self.layout.w / 4.;
        // }
        // char_rect.draw(ctx, canvas);

        //self.quadtree.draw_boundries(ctx, canvas, graphics::Color::BLUE);
        
        // let data = self.quadtree.search(self.layout.x, self.layout.y);
        // if data.is_some() {
        //     for loc in data.unwrap() {
        //         let mut rect = CustomRect::from_rect(*loc);
        //         rect.draw(ctx, canvas);
        //     }
        // }
        // END
    }
}
