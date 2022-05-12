use std::cmp;

use crate::consts::{CHAR_DIRECTORY, CHAR_WIDTH, RUN_SPEED};

use macroquad::input;
use macroquad::prelude::*;
use std::{thread, time::Duration};
use macroquad::experimental::coroutines::wait_seconds;

pub struct Animation {
    texture: Texture2D,
    index: i8
}


impl Animation {

    async fn new(animation_path: &str) -> Self {
        Self {
            texture: load_texture(animation_path).await.unwrap(),
            index: 0
        }
    }

    fn one_cycle(&mut self) -> Vec<Rect> {
        let mut res: Vec<Rect> = vec![];
        let mut index = 0;
        while index as f32 != self.texture.width() / CHAR_WIDTH {
            let rect: Rect = Rect {
                x: index as f32 * CHAR_WIDTH,
                y: 0.,
                w: CHAR_WIDTH,
                h: CHAR_WIDTH
            };
            res.push(rect);
            index += 1;
        }

        return res;
    }

    fn next(&mut self) -> Rect {
        if self.index as f32 == self.texture.width() / CHAR_WIDTH {
            self.index = 0;
        }
        let rect: Rect = Rect {
            x: self.index as f32 * CHAR_WIDTH,
            y: 0.,
            w: CHAR_WIDTH,
            h: CHAR_WIDTH
        };

        self.index += 1;

        return rect;
    }
}


struct Location {
    x: f32,
    y: f32
}

impl Location {
    fn default() -> Self {
        Self {
            x: screen_width() / 2. - CHAR_WIDTH / 2.,
            y: screen_height() / 2. - CHAR_WIDTH / 2.
        }
    }
}


pub struct Character {
    name: String,
    health: i8,
    idle: Animation,
    run: Animation,
    jump: Animation,
    location: Location,
    is_fliped: bool,
    jumping: bool
}


impl Character {

    fn draw_char(&mut self, texture: Texture2D, draw_params: DrawTextureParams) {
        clear_background(LIGHTGRAY);

        let x: f32 = if self.location.x + CHAR_WIDTH / 2. <= screen_width() {
            self.location.x
        } else {
            screen_width() - CHAR_WIDTH / 2.
        };

        let y: f32 = if self.location.y <= screen_height() {
            self.location.y
        } else {
            screen_height()
        };

        draw_texture_ex(
            texture,
            x,
            y,
            WHITE,
            draw_params
        );
    }

    fn run_right(&mut self) -> (Texture2D, DrawTextureParams) {
        let texture: Texture2D = self.run.texture;
        let mut draw_params: DrawTextureParams = DrawTextureParams::default();
        draw_params.source = Some(self.run.next());

        if self.is_fliped {
            self.is_fliped = false;
            self.location.x += CHAR_WIDTH / 2.;
        }

        self.location.x += RUN_SPEED;
        self.location.x = if self.location.x + CHAR_WIDTH / 2. <= screen_width() {
            self.location.x
        } else {
            screen_width() - CHAR_WIDTH / 2.
        };

        return (texture, draw_params);
    }

    fn run_left(&mut self) -> (Texture2D, DrawTextureParams) {
        let texture: Texture2D = self.run.texture;
        let mut draw_params: DrawTextureParams = DrawTextureParams::default();
        draw_params.flip_x = true;
        draw_params.source = Some(self.run.next());

        if !self.is_fliped {
            self.is_fliped = true;
            self.location.x -= CHAR_WIDTH / 2.;
        }

        self.location.x -= RUN_SPEED;
        self.location.x = if self.location.x + CHAR_WIDTH / 2. >= 0. {
            self.location.x
        } else {
            - CHAR_WIDTH / 2.
        };

        return (texture, draw_params);
    }

    fn idle(&mut self) -> (Texture2D, DrawTextureParams) {
        let texture: Texture2D = self.idle.texture;
        let mut draw_params: DrawTextureParams = DrawTextureParams::default();
        draw_params.source = Some(self.idle.next());
        draw_params.flip_x = self.is_fliped;

        return (texture, draw_params);
    }

    fn perform_jump(&mut self) -> (Texture2D, DrawTextureParams) {
        if !self.jumping {
            self.jumping = true;
        } else {
            if self.jump.index as f32  + 1. == self.jump.texture.width() / CHAR_WIDTH {
                self.jumping = false;
            }
        }
        let texture: Texture2D = self.jump.texture;
        let mut draw_params: DrawTextureParams = DrawTextureParams::default();
        draw_params.source = Some(self.jump.next());
        draw_params.flip_x = self.is_fliped;

        return (texture, draw_params);
    }


    pub async fn update(&mut self) -> () {
        let current_texture: Texture2D;
        let draw_params: DrawTextureParams;

        if self.jumping || input::is_key_pressed(input::KeyCode::Space) {
            (current_texture, draw_params) = self.perform_jump();
        } else if input::is_key_down(input::KeyCode::A) {
            (current_texture, draw_params) = self.run_left();   
        } else if input::is_key_down(input::KeyCode::D) {
            (current_texture, draw_params) = self.run_right();
        }  else {
            (current_texture, draw_params) = self.idle();
        }

        self.draw_char(current_texture, draw_params);
    }
}
        

pub struct Punk;

impl Punk {
    pub async fn new() -> Character {
        Character {
            name: "Punk".to_string(),
            health: 10,
            idle: Animation::new(&format!("{}/punk/Punk_idle.png", CHAR_DIRECTORY)).await,
            run: Animation::new(&format!("{}/punk/Punk_run.png", CHAR_DIRECTORY)).await,
            jump: Animation::new(&format!("{}/punk/Punk_jump.png", CHAR_DIRECTORY)).await,
            location: Location::default(),
            is_fliped: false,
            jumping: false
        }
    }
}