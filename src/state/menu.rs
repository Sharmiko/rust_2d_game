use std::cell::RefCell;

use ggez::Context;
use ggez::graphics::{self};
use ggez::input::mouse::{self};

use crate::state::{State, AllStates};


pub struct MenuState {
    rect: graphics::Rect
}

impl MenuState {
    pub fn new(ctx: &mut Context) ->  Self {
        let (w, h) = graphics::size(ctx);
        Self {
            rect: graphics::Rect{
                x: w / 2. - 300. / 2.,
                y: h / 2. - 60. / 2.,
                w: 300.,
                h: 60.
            }
        }
    }
}


impl State for MenuState {

    fn enter(&self, ctx: &mut Context, current_state: &RefCell<String>) {}

    fn exit(&self, ctx: &mut Context, current_state: &RefCell<String>) {}

    fn draw(&mut self, ctx: &mut Context, current_state: &RefCell<String>) {
        let mesh = graphics::MeshBuilder::new()
            .rectangle(
                graphics::DrawMode::stroke(3.), 
                graphics::Rect {
                    x: 0.,
                    y: 0.,
                    w: self.rect.w,
                    h: self.rect.h
                },
                graphics::Color::BLUE
            ).unwrap().build(ctx).unwrap();
            

        let draw_params = graphics::DrawParam::new()
            .dest(glam::Vec2::new(self.rect.x, self.rect.y));

        let text = graphics::Text::new("Play");
        graphics::draw(ctx, &text, draw_params.color(graphics::Color::RED)).unwrap();
    
        graphics::draw(ctx, &mesh, draw_params).unwrap();

    }

    fn update(&mut self, ctx: &mut Context, current_state: &RefCell<String>) {
        let point = mouse::position(ctx);
        let (mouse_x, mouse_y) = (point.x, point.y);
        if mouse_x > self.rect.x && mouse_x < self.rect.x + self.rect.w &&
            mouse_y > self.rect.y && mouse_y < self.rect.y + self.rect.h
        {
            if mouse::button_pressed(ctx, mouse::MouseButton::Left) {
                current_state.replace(AllStates::Play.as_str());
            }
        }
    }
}