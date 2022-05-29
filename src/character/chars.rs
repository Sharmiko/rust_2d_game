use std::cell::RefCell;
use std::collections::HashMap;

use ggez::Context;

use crate::character::{Character, Location, CharacterState, CharacterAnimation};
use crate::animation::SpriteAnimation;
use crate::utils::join_paths;
use crate::consts::{PUNK_DIR, BIKER_DIR, CYBORG_DIR};


pub struct Punk;

impl Punk {
    pub fn new(_ctx: &mut Context) -> Character {
        let mut animations = HashMap::new();
        animations.insert(CharacterAnimation::Idle, RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&PUNK_DIR, "Punk_idle.png"))));
        animations.insert(CharacterAnimation::Run, RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&PUNK_DIR, "Punk_run.png"))));
        animations.insert(CharacterAnimation::Jump,  RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&PUNK_DIR, "Punk_jump.png"))));
        animations.insert(CharacterAnimation::Attack, RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&PUNK_DIR, "Punk_attack1.png"))));
        animations.insert(CharacterAnimation::DoubleJump, RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&PUNK_DIR, "Punk_doublejump.png"))));

        Character {
            animations: animations,
            location: Location::default(_ctx),
            state: CharacterState::default(),
            current: CharacterAnimation::Idle
        }
    }
}


pub struct Biker;

impl Biker {
    pub fn new(_ctx: &mut Context) -> Character {
        let mut animations = HashMap::new();
        animations.insert(CharacterAnimation::Idle, RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&BIKER_DIR, "Biker_idle.png"))));
        animations.insert(CharacterAnimation::Run, RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&BIKER_DIR, "Biker_run.png"))));
        animations.insert(CharacterAnimation::Jump,  RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&BIKER_DIR, "Biker_jump.png"))));
        animations.insert(CharacterAnimation::Attack, RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&BIKER_DIR, "Biker_attack.png"))));
        animations.insert(CharacterAnimation::DoubleJump, RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&BIKER_DIR, "Biker_doublejump.png"))));

        Character {
            animations: animations,
            location: Location::default(_ctx),
            state: CharacterState::default(),
            current: CharacterAnimation::Idle
        }
    }
}


pub struct Cyborg;

impl Cyborg {
    pub fn new(_ctx: &mut Context) -> Character {
        let mut animations = HashMap::new();
        animations.insert(CharacterAnimation::Idle, RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&CYBORG_DIR, "Cyborg_idle.png"))));
        animations.insert(CharacterAnimation::Run, RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&CYBORG_DIR, "Cyborg_run.png"))));
        animations.insert(CharacterAnimation::Jump,  RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&CYBORG_DIR, "Cyborg_jump.png"))));
        animations.insert(CharacterAnimation::Attack, RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&CYBORG_DIR, "Cyborg_attack.png"))));
        animations.insert(CharacterAnimation::DoubleJump, RefCell::new(SpriteAnimation::new(_ctx, &join_paths(&CYBORG_DIR, "Cyborg_doublejump.png"))));

        Character {
            animations: animations,
            location: Location::default(_ctx),
            state: CharacterState::default(),
            current: CharacterAnimation::Idle
        }
    }
}
