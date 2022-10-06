use std::cell::RefMut;

use ggez::Context;

use crate::animation::SpriteAnimation;
use crate::base::human::Animation;

pub trait HumanAnimation {
    fn perform_action(&self, mut animation: RefMut<'_, SpriteAnimation>) {
        if !animation.performing {
            animation.performing = true;
        } else if animation.image_idx == animation.image_count {
            animation.performing = false;
            animation.image_idx = 0;
        }
    }

    fn update_current_anim(&mut self, animation_state: Animation);
    fn run_right(&mut self, _ctx: &mut Context);
    fn run_left(&mut self, _ctx: &mut Context);
    fn idle(&mut self, _ctx: &mut Context);
    fn perform_jump(&mut self, _ctx: &mut Context);
    fn perform_attack(&mut self, _ctx: &mut Context);
}
