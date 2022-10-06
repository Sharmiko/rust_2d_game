use std::cell::RefCell;
use std::collections::BTreeMap;

use mint::Vector2;
use ggez::graphics::{self, *};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

use crate::utils::join_paths;
use crate::animation::MovingBackground;
use crate::resources::background;


pub struct Background {
    static_background: BTreeMap<String, RefCell<graphics::Image>>,
    moving_background: BTreeMap<String, RefCell<MovingBackground>>
}


impl Background {
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if _ctx.keyboard.is_key_pressed(KeyCode::D) || _ctx.keyboard.is_key_pressed(KeyCode::A) {
            let forward = if _ctx.keyboard.is_key_pressed(KeyCode::D) { true } else { false };
            for (_, value) in &self.moving_background {
                let mut moving = value.borrow_mut();
                if moving.forward != forward {
                    moving.step = 100f32 - moving.step;
                }
                moving.forward = forward;
                moving.update(_ctx).unwrap();
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, _ctx: &mut Context, canvas: &mut Canvas)  {

        let (w, h) = _ctx.gfx.drawable_size();
        for (_, value) in &self.static_background {
            let image = value.borrow();
            let scale_x = w / image.width() as f32;
            let scale_y = h / image.height() as f32;
            let params = graphics::DrawParam::default()
                .scale(Vector2 {
                     x: scale_x,
                     y: scale_y
                })
                .dest(Vector2 {
                    x: 0.,
                    y: 0.
                });
            image.draw(canvas, params);
        }

        for (_, value) in &self.moving_background {
            value.borrow_mut().draw(_ctx, canvas);
        }
    }
}


pub struct ParkBackground;

impl ParkBackground {
    pub fn new(_ctx: &mut Context) -> Background {
        let mut static_background = BTreeMap::new();
        static_background.insert(
            "background1".to_string(), 
            RefCell::new(graphics::Image::from_path(&_ctx.gfx, &join_paths(background::PARK_DAY_BACKGROUND, "1.png")).unwrap())
        );

        let mut moving_background = BTreeMap::new();
        moving_background.insert(
            "background2".to_string(), 
            RefCell::new(MovingBackground::new(_ctx, &join_paths(background::PARK_DAY_BACKGROUND, "2.png"), 0.5))
        );
        moving_background.insert(
            "background3".to_string(),
            RefCell::new(MovingBackground::new(_ctx, &join_paths(background::PARK_DAY_BACKGROUND, "3.png"), 1.5))
        );
        moving_background.insert(
            "background4".to_string(), 
            RefCell::new(MovingBackground::new(_ctx, &join_paths(background::PARK_DAY_BACKGROUND, "4.png"), 1.7))
        );
        moving_background.insert(
            "background5".to_string(), 
            RefCell::new(MovingBackground::new(_ctx, &join_paths(background::PARK_DAY_BACKGROUND, "5.png"), 2.))
        );
        
        Background {
            static_background: static_background,
            moving_background: moving_background
        }
    }
}