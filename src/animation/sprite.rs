use std::path::Path;

use ggez::Context;
use ggez::graphics::{self, *};


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
        let image = graphics::Image::from_path(&ctx.gfx, Path::new(image_path)).unwrap();
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
