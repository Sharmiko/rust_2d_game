use std::cell::RefCell;

use ggez::Context;
use ggez::graphics::{self, Canvas};
use ggez::input::mouse::{self};

use crate::base::CustomRect;
use crate::state::{State, AllStates};


pub struct MenuState {
    rect: CustomRect
}

impl MenuState {
    pub fn new(_ctx: &mut Context) ->  Self {
        let (w, h) = _ctx.gfx.drawable_size();
        Self {
            rect: CustomRect::new(
                w / 2. - 300. / 2.,
                h / 2. - 60. / 2.,
                300.,
                60.
            )
        }
    }
}


impl State for MenuState {

    fn enter(&self, _ctx: &mut Context, _current_state: &RefCell<AllStates>) {}

    fn exit(&self, _ctx: &mut Context, _current_state: &RefCell<AllStates>) {}

    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, _current_state: &RefCell<AllStates>) {
        self.rect.draw(ctx, canvas);
        let draw_params = graphics::DrawParam::new()
            .dest(glam::Vec2::new(self.rect.fields.x, self.rect.fields.y));

        let text = graphics::Text::new("Play");
        canvas.draw(&text, draw_params.color(graphics::Color::RED));
    }

    fn update(&mut self, _ctx: &mut Context, current_state: &RefCell<AllStates>) {
        let point = mouse::position(_ctx);
        let (mouse_x, mouse_y) = (point.x, point.y);
        if mouse_x > self.rect.fields.x && 
           mouse_x < self.rect.fields.x + self.rect.fields.w &&
           mouse_y > self.rect.fields.y && 
           mouse_y < self.rect.fields.y + self.rect.fields.h
        {
            if mouse::button_pressed(_ctx, mouse::MouseButton::Left) {
                current_state.replace(AllStates::Play);
            }
        }
    }
}