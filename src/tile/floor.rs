use ggez::Context;
use ggez::graphics::{self, Image, Drawable};

use glam::Vec2;


pub struct Floor {
    left_corner: Image,
    middle: Image,
    right_corner: Image
}


impl Floor {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            left_corner: Image::new(ctx, "/tiles/park/tiles/Tile_01.png").unwrap(),
            middle: Image::new(ctx, "/tiles/park/tiles/Tile_02.png").unwrap(),
            right_corner: Image::new(ctx, "/tiles/park/tiles/Tile_02.png").unwrap()
        }
    }

    pub fn draw(&self, ctx: &mut Context)  {
        let (w, h) = graphics::size(ctx);
        let mut params = graphics::DrawParam::default()
            .dest(Vec2::new(0., h  - self.left_corner.height() as f32 * 2.));
        self.left_corner.draw(ctx, params).unwrap();

        let start_x = self.left_corner.width();
        for i in 1..(w  as u16 / self.left_corner.width()) - 1 {
            let params = graphics::DrawParam::default()
                .dest(Vec2::new((start_x * i) as f32 , h - self.middle.height() as f32 * 2.));
            self.middle.draw(ctx, params).unwrap();
        }

        params = graphics::DrawParam::default()
            .dest(Vec2::new(w - self.right_corner.width() as f32, h - self.right_corner.height() as f32 * 2.));
        self.right_corner.draw(ctx, params).unwrap();
    }   
}