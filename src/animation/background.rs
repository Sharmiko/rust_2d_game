use std::path::Path;

use ggez::{Context, GameResult};
use ggez::graphics::{self, *};


pub struct MovingBackground {
    pub step: f32,
    pub forward: bool,
    image: Image,
    step_size: f32
}

impl MovingBackground {

    pub fn new(_ctx: &mut Context, image_path: &str, step_size: f32) -> Self {
        Self {
            image: graphics::Image::from_path(&_ctx.gfx, Path::new(image_path)).unwrap(),
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
        self.progress_background(_ctx, canvas, self.forward);
    }

    pub fn progress_background(&mut self, _ctx: &mut Context, canvas: &mut Canvas, is_forward: bool) {
        let step = self.step as f32 / 100.;

        let (w, h) = _ctx.gfx.drawable_size();
        let mut scale_x  = (w * (1. - step as f32)) / (self.image.width() as f32 * (1. - step as f32));
        let scale_y  = h / self.image.height() as f32;

        let src_w = 1. - step as f32;
        let params = graphics::DrawParam::default()
            .scale([scale_x, scale_y])
            .src(graphics::Rect {
                x: step as f32 * (if is_forward { 1.0 } else { 0.0 }),
                y: 0f32,
                w: src_w,
                h: 1f32
            })
            .dest([
                self.image.width() as f32 * scale_x * step * (if is_forward {0.0} else { 1.0 }),
                0.
            ]);
        canvas.draw(&self.image, params);

        scale_x  = (w * step) / (self.image.width() as f32 * step);
        let params = graphics::DrawParam::default()
            .scale([scale_x, scale_y])
            .src(graphics::Rect {
                x: (1f32 - step) * (if is_forward { 0.0 } else { 1.0 }),
                y: 0f32,
                w: step,
                h: 1f32
            })
            .dest([
                self.image.width() as f32 * scale_x * (1. - step as f32) * (if is_forward { 1.0 } else { 0.0 }), 
                0.
            ]);
        canvas.draw(&self.image, params);
    }
}