use ggez::Context;

use crate::character::{Character, CharacterAnimation};
use crate::animation::SpriteAnimation;
use crate::utils::join_paths;
use crate::consts::{PUNK_DIR, BIKER_DIR, CYBORG_DIR};


pub struct Punk;

impl Punk {
    pub fn new(_ctx: &mut Context) -> Character {
        let mut punk = Character::default(_ctx);
        punk.insert_animation(CharacterAnimation::Idle, SpriteAnimation::new(_ctx, &join_paths(&PUNK_DIR, "Punk_idle.png")));
        punk.insert_animation(CharacterAnimation::Run, SpriteAnimation::new(_ctx, &join_paths(&PUNK_DIR, "Punk_run.png")));
        punk.insert_animation(CharacterAnimation::Jump,  SpriteAnimation::new(_ctx, &join_paths(&PUNK_DIR, "Punk_jump.png")));
        punk.insert_animation(CharacterAnimation::Attack, SpriteAnimation::new(_ctx, &join_paths(&PUNK_DIR, "Punk_attack1.png")));
        punk.insert_animation(CharacterAnimation::DoubleJump, SpriteAnimation::new(_ctx, &join_paths(&PUNK_DIR, "Punk_doublejump.png")));

        return punk;
    }
}


pub struct Biker;

impl Biker {
    pub fn new(_ctx: &mut Context) -> Character {
        let mut biker = Character::default(_ctx);
        biker.insert_animation(CharacterAnimation::Idle, SpriteAnimation::new(_ctx, &join_paths(&BIKER_DIR, "Biker_idle.png")));
        biker.insert_animation(CharacterAnimation::Run, SpriteAnimation::new(_ctx, &join_paths(&BIKER_DIR, "Biker_run.png")));
        biker.insert_animation(CharacterAnimation::Jump,  SpriteAnimation::new(_ctx, &join_paths(&BIKER_DIR, "Biker_jump.png")));
        biker.insert_animation(CharacterAnimation::Attack, SpriteAnimation::new(_ctx, &join_paths(&BIKER_DIR, "Biker_attack.png")));
        biker.insert_animation(CharacterAnimation::DoubleJump, SpriteAnimation::new(_ctx, &join_paths(&BIKER_DIR, "Biker_doublejump.png")));

        return biker;
    }
}


pub struct Cyborg;

impl Cyborg {
    pub fn new(_ctx: &mut Context) -> Character {
        let mut cyborg = Character::default(_ctx);
        cyborg.insert_animation(CharacterAnimation::Idle, SpriteAnimation::new(_ctx, &join_paths(&CYBORG_DIR, "Cyborg_idle.png")));
        cyborg.insert_animation(CharacterAnimation::Run, SpriteAnimation::new(_ctx, &join_paths(&CYBORG_DIR, "Cyborg_run.png")));
        cyborg.insert_animation(CharacterAnimation::Jump,  SpriteAnimation::new(_ctx, &join_paths(&CYBORG_DIR, "Cyborg_jump.png")));
        cyborg.insert_animation(CharacterAnimation::Attack, SpriteAnimation::new(_ctx, &join_paths(&CYBORG_DIR, "Cyborg_attack.png")));
        cyborg.insert_animation(CharacterAnimation::DoubleJump, SpriteAnimation::new(_ctx, &join_paths(&CYBORG_DIR, "Cyborg_doublejump.png")));

        return cyborg;
    }
}
