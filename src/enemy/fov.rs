use ggez::graphics::Rect;

use crate::base::CustomRect;
use crate::consts::{CHAR_HEIGHT, CHAR_WIDTH};



pub struct FieldOfVision {
    pub layout: CustomRect,
    horizontal: f32,
    vertical: f32
}


impl FieldOfVision {

    pub fn new(horizontal: f32, vertical: f32, entity_layout: &Rect) -> Self {
        let rect = CustomRect::new(
            entity_layout.x - horizontal, 
            entity_layout.y  - CHAR_HEIGHT / 2. - vertical / 2., 
            horizontal * 2. + CHAR_WIDTH / 2.,
            vertical * 2.
        );

        Self {
            horizontal: horizontal,
            vertical: vertical,
            layout: rect
        }
    }

    pub fn update(&mut self, entity_layout: &Rect, is_flipped: bool) {
        self.layout = CustomRect::new(
            entity_layout.x - self.horizontal - (if is_flipped { CHAR_WIDTH / 2. } else { 0. }),
            entity_layout.y - CHAR_HEIGHT / 2. - self.vertical / 2.,
            self.horizontal * 2. + CHAR_WIDTH / 2.,
            self.vertical * 2.
        );
    }
}
