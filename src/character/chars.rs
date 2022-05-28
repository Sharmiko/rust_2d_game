use std::cell::RefCell;
use std::collections::HashMap;

use ggez::Context;

use crate::character::{Character, Location, CharacterState, CharacterAnimation};
use crate::animation::SpriteAnimation;


pub struct Punk;

impl Punk {
    pub fn new(_ctx: &mut Context) -> Character {
        let mut animations = HashMap::new();
        animations.insert(CharacterAnimation::Idle, RefCell::new(SpriteAnimation::new(_ctx, "/Punk_idle.png")));
        animations.insert(CharacterAnimation::Run, RefCell::new(SpriteAnimation::new(_ctx, "/Punk_run.png")));
        animations.insert(CharacterAnimation::Jump,  RefCell::new(SpriteAnimation::new(_ctx, "/Punk_jump.png")));
        animations.insert(CharacterAnimation::Attack, RefCell::new(SpriteAnimation::new(_ctx, "/Punk_attack1.png")));
        animations.insert(CharacterAnimation::DoubleJump, RefCell::new(SpriteAnimation::new(_ctx, "/Punk_doublejump.png")));

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
        animations.insert(CharacterAnimation::Idle, RefCell::new(SpriteAnimation::new(_ctx, "/Biker_idle.png")));
        animations.insert(CharacterAnimation::Run, RefCell::new(SpriteAnimation::new(_ctx, "/Biker_run.png")));
        animations.insert(CharacterAnimation::Jump,  RefCell::new(SpriteAnimation::new(_ctx, "/Biker_jump.png")));
        animations.insert(CharacterAnimation::Attack, RefCell::new(SpriteAnimation::new(_ctx, "/Biker_attack1.png")));
        animations.insert(CharacterAnimation::DoubleJump, RefCell::new(SpriteAnimation::new(_ctx, "/Biker_doublejump.png")));

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
        animations.insert(CharacterAnimation::Idle, RefCell::new(SpriteAnimation::new(_ctx, "/Cyborg_idle.png")));
        animations.insert(CharacterAnimation::Run, RefCell::new(SpriteAnimation::new(_ctx, "/Cyborg_run.png")));
        animations.insert(CharacterAnimation::Jump,  RefCell::new(SpriteAnimation::new(_ctx, "/Cyborg_jump.png")));
        animations.insert(CharacterAnimation::Attack, RefCell::new(SpriteAnimation::new(_ctx, "/Cyborg_attack1.png")));
        animations.insert(CharacterAnimation::DoubleJump, RefCell::new(SpriteAnimation::new(_ctx, "/Cyborg_doublejump.png")));

        Character {
            animations: animations,
            location: Location::default(_ctx),
            state: CharacterState::default(),
            current: CharacterAnimation::Idle
        }
    }
}
