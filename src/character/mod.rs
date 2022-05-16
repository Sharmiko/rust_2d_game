use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::consts::{CHAR_DIRECTORY, CHAR_WIDTH, RUN_SPEED};

use macroquad::input;
use macroquad::prelude::*;


type Action = RefCell<MultiStageAnimation>;


pub trait PerformAction {
    fn perform_action(&self, action: RefMut<'_, MultiStageAnimation>) -> (Texture2D, DrawTextureParams);
}


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

pub struct MultiStageAnimation {
    animation: Animation,
    performing: bool
}

impl MultiStageAnimation {
    async fn new(animation_path: &str) -> Action {
        RefCell::new(Self {
            animation: Animation::new(animation_path).await,
            performing: false
        })
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


struct CharacterState {
    is_flipped: bool
}

impl CharacterState {
    fn default() -> Self {
        Self {
            is_flipped: false
        }
    }
}


pub struct Character {
    name: String,
    health: i8,
    idle: Animation,
    run: Animation,
    jump: Action,
    double_jump: Action,
    attack: Action,
    kick: Action,
    location: Location,
    state: CharacterState
}


impl PerformAction for Character {
    fn perform_action(&self, mut action: RefMut<'_, MultiStageAnimation>) -> (Texture2D, DrawTextureParams) {
        if !action.performing {
            action.performing = true;
        } else {
            if action.animation.index as f32  + 1. == action.animation.texture.width() / CHAR_WIDTH {
                action.performing = false;
            }
        }

        let texture: Texture2D = action.animation.texture;
        let mut draw_params: DrawTextureParams = DrawTextureParams::default();
        draw_params.source = Some(action.animation.next());
        draw_params.flip_x = self.state.is_flipped;

        return (texture, draw_params);
    }
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

        if self.state.is_flipped {
            self.state.is_flipped = false;
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

        if !self.state.is_flipped {
            self.state.is_flipped = true;
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
        draw_params.flip_x = self.state.is_flipped;

        return (texture, draw_params);
    }

    fn perform_jump(&mut self) -> (Texture2D, DrawTextureParams) {
        self.perform_action(self.jump.borrow_mut())
    }

    fn perform_double_jump(&mut self) -> (Texture2D, DrawTextureParams) {
        self.jump.borrow_mut().performing = false;
        self.perform_action(self.double_jump.borrow_mut())
    }

    fn attack(&mut self) -> (Texture2D, DrawTextureParams) {
        self.perform_action(self.attack.borrow_mut())
    }

    fn kick(&mut self) -> (Texture2D, DrawTextureParams) {
        self.perform_action(self.kick.borrow_mut())
    }

    pub async fn update(&mut self) -> () {
        let current_texture: Texture2D;
        let draw_params: DrawTextureParams;

        if self.attack.borrow().performing || input::is_key_pressed(input::KeyCode::F) {
            (current_texture, draw_params) = self.attack();
        } else if self.kick.borrow().performing || input::is_key_pressed(input::KeyCode::V) {
            (current_texture, draw_params) = self.kick();
        } else if self.double_jump.borrow().performing {
            (current_texture, draw_params) = self.perform_double_jump();
        } else if self.jump.borrow().performing || input::is_key_pressed(input::KeyCode::Space) {
            if self.jump.borrow().performing && input::is_key_pressed(input::KeyCode::Space) {
                (current_texture, draw_params) = self.perform_double_jump();
            } else {
                (current_texture, draw_params) = self.perform_jump();
            }
        } else if input::is_key_down(input::KeyCode::A) {
            (current_texture, draw_params) = self.run_left();   
        } else if input::is_key_down(input::KeyCode::D) {
            (current_texture, draw_params) = self.run_right();
        } else {
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
            jump: MultiStageAnimation::new(&format!("{}/punk/Punk_jump.png", CHAR_DIRECTORY)).await,
            double_jump: MultiStageAnimation::new(&format!("{}/punk/Punk_doublejump.png", CHAR_DIRECTORY)).await,
            attack: MultiStageAnimation::new(&format!("{}/punk/Punk_attack1.png", CHAR_DIRECTORY)).await,
            kick: MultiStageAnimation::new(&format!("{}/punk/Punk_punch.png", CHAR_DIRECTORY)).await,
            location: Location::default(),
            state: CharacterState::default()
        }
    }
}