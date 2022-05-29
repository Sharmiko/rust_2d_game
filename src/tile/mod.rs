use std::path::Path;
use std::cell::RefCell;
use std::collections::BTreeMap;

use glam::Vec2;
use ggez::graphics::{self, *};
use ggez::input::keyboard;
use ggez::event::KeyCode;
use ggez::{Context, GameResult};

use crate::utils::join_paths;
use crate::animation::MovingBackground;
use crate::consts::PARK_DAY_BACKGROUND_DIR;


pub struct Background {
    static_background: BTreeMap<String, RefCell<graphics::Image>>,
    moving_background: BTreeMap<String, RefCell<MovingBackground>>
}


impl Background {
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if keyboard::is_key_pressed(_ctx, KeyCode::D) {
            for (_, value) in &self.moving_background {
                value.borrow_mut().update(_ctx).unwrap();
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, _ctx: &mut Context)  {

        let (w, h) = graphics::size(_ctx);
        for (_, value) in &self.static_background {
            let image = value.borrow_mut();
            let scale_x = w / image.width() as f32;
            let scale_y = h / image.height() as f32;
            let params = graphics::DrawParam::default()
                .scale(Vec2::new(scale_x, scale_y))
                .dest(Vec2::new(0., 0.));
            image.draw(_ctx, params).unwrap();
        }

        for (_, value) in &self.moving_background {
            value.borrow_mut().draw(_ctx);
        }
    }
}


pub struct ParkBackground;

impl ParkBackground {
    pub fn new(_ctx: &mut Context) -> Background {
        let mut static_background = BTreeMap::new();
        static_background.insert("background1".to_string(), RefCell::new(graphics::Image::new(_ctx, &join_paths(PARK_DAY_BACKGROUND_DIR, "1.png")).unwrap()));

        let mut moving_background = BTreeMap::new();
        moving_background.insert("background2".to_string(), RefCell::new(MovingBackground::new(_ctx, &join_paths(PARK_DAY_BACKGROUND_DIR, "2.png"), 0.5)));
        moving_background.insert("background3".to_string(), RefCell::new(MovingBackground::new(_ctx, &join_paths(PARK_DAY_BACKGROUND_DIR, "3.png"), 1.5)));
        moving_background.insert("background4".to_string(), RefCell::new(MovingBackground::new(_ctx, &join_paths(PARK_DAY_BACKGROUND_DIR, "4.png"), 1.7)));
        moving_background.insert("background5".to_string(), RefCell::new(MovingBackground::new(_ctx, &join_paths(PARK_DAY_BACKGROUND_DIR, "5.png"), 2.)));
        Background {
            static_background: static_background,
            moving_background: moving_background
        }
    }
}