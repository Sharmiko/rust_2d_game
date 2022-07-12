use std::path::Path;
use std::cell::RefMut;

use ggez::{Context, GameResult};
use ggez::graphics::{self, *};

use crate::base::human::Animation;


pub struct SpriteAnimation {
    pub image: Image,
    pub performing: bool,
    pub src_x: f32,
    pub image_idx: i8,
    pub image_count: i8,
    pub image_width: f32
}


impl SpriteAnimation {

    pub fn new(ctx: &mut Context, image_path: &str) -> Self {
        let image = graphics::Image::from_path(ctx, Path::new(image_path), true).unwrap();
        let image_count = (image.width() / image.height()) as i8;
        Self {
            image: image,
            image_idx: 0,
            src_x: 0.,
            image_count: image_count,
            image_width: 1. / image_count as f32,
            performing: false
        }
    }

    pub fn next_x(&mut self) -> f32 {
        if self.image_idx == self.image_count {
            self.image_idx = 0;
        }

        let x = 1. - ((self.image_count as f32 - self.image_idx as f32) / self.image_count as f32);

        self.image_idx += 1;

        return x;
    }
}


pub struct MovingBackground {
    image: graphics::Image,
    pub step: f32,
    step_size: f32,
    pub forward: bool
}

impl MovingBackground {

    pub fn new(_ctx: &mut Context, image_path: &str, step_size: f32) -> Self {
        Self {
            image: graphics::Image::from_path(_ctx, Path::new(image_path), true).unwrap(),
            step: 0.,
            step_size: step_size,
            forward: true 
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.step as i8 > 100 {
            self.step = 0.;
        }

        self.step += self.step_size;

        Ok(())
    }

    pub fn draw(&mut self, _ctx: &mut Context, canvas: &mut Canvas) {
        if self.forward {
            self.draw_forward(_ctx, canvas);
        } else {
            self.draw_backward(_ctx, canvas);
        }
    }


    pub fn draw_forward(&mut self, _ctx: &mut Context, canvas: &mut Canvas)  {

        let step = self.step as f32 / 100.;

        let (w, h) = _ctx.gfx.drawable_size();
        let mut scale_x  = (w * (1. - step as f32)) / (self.image.width() as f32 * (1. - step as f32));
        let mut scale_y  = h / self.image.height() as f32;
        let src_w = 1. - step as f32;
        let params = graphics::DrawParam::default()
            .scale([scale_x * src_w, scale_y])
            .src(graphics::Rect {
                x: step as f32,
                y: 0f32,
                w: src_w,
                h: 1f32
            })
            .dest([0., 0.]);
        canvas.draw(&self.image, params);

        scale_x  = (w * step) / (self.image.width() as f32 * step);
        scale_y  = h / self.image.height() as f32;
        let params = graphics::DrawParam::default()
            .scale([scale_x * step, scale_y])
            .src(graphics::Rect {
                x: 0.,
                y: 0f32,
                w: step,
                h: 1f32
            })
            .dest([self.image.width() as f32 * scale_x * (1. - step as f32), 0.]);
        canvas.draw(&self.image, params);
    }

    pub fn draw_backward(&mut self, _ctx: &mut Context, canvas: &mut Canvas)  {

        let step = self.step as f32 / 100.;
        let (w, h) = _ctx.gfx.drawable_size();
        let mut scale_x  = (w * (1. - step as f32)) / (self.image.width() as f32 * (1. - step as f32));
        let mut scale_y  = h / self.image.height() as f32;
        let src_w = 1. - step as f32;
        let params = graphics::DrawParam::default()
            .scale([scale_x * src_w, scale_y])
            .src(graphics::Rect {
                x: 0.,
                y: 0f32,
                w: src_w,
                h: 1f32
            })
            .dest([self.image.width() as f32 * scale_x * step, 0.]);
        canvas.draw(&self.image, params);

        scale_x  = (w * step) / (self.image.width() as f32 * step);
        scale_y  = h / self.image.height() as f32;
        let params = graphics::DrawParam::default()
            .scale([scale_x * step, scale_y])
            .src(graphics::Rect {
                x: 1f32 - step,
                y: 0f32,
                w: step,
                h: 1f32
            })
            .dest([0., 0.]);
        canvas.draw(&self.image, params);
    }
}


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
