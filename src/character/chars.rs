use ggez::Context;

use crate::character::Character;
use crate::base::human::Animation;
use crate::animation::SpriteAnimation;
use crate::utils::join_paths;
use crate::resources::chars;


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CharacterType {
    Punk,
    Biker,
    Cyborg
}


pub struct Punk;

impl Punk {
    pub fn new(_ctx: &mut Context, x: f32, y: f32) -> Character {
        let mut punk = Character::default(_ctx, x, y);
        punk.entity.insert_animation(Animation::Idle, SpriteAnimation::new(_ctx, &join_paths(&chars::PUNK, "Punk_idle.png")));
        punk.entity.insert_animation(Animation::Run, SpriteAnimation::new(_ctx, &join_paths(&chars::PUNK, "Punk_run.png")));
        punk.entity.insert_animation(Animation::Jump,  SpriteAnimation::new(_ctx, &join_paths(&chars::PUNK, "Punk_jump.png")));
        punk.entity.insert_animation(Animation::Attack, SpriteAnimation::new(_ctx, &join_paths(&chars::PUNK, "Punk_attack1.png")));
        punk.entity.insert_animation(Animation::DoubleJump, SpriteAnimation::new(_ctx, &join_paths(&chars::PUNK, "Punk_doublejump.png")));

        return punk;
    }
}


pub struct Biker;

impl Biker {
    pub fn new(_ctx: &mut Context, x: f32, y: f32) -> Character {
        let mut biker = Character::default(_ctx, x, y);
        biker.entity.insert_animation(Animation::Idle, SpriteAnimation::new(_ctx, &join_paths(&chars::BIKER, "Biker_idle.png")));
        biker.entity.insert_animation(Animation::Run, SpriteAnimation::new(_ctx, &join_paths(&chars::BIKER, "Biker_run.png")));
        biker.entity.insert_animation(Animation::Jump,  SpriteAnimation::new(_ctx, &join_paths(&chars::BIKER, "Biker_jump.png")));
        biker.entity.insert_animation(Animation::Attack, SpriteAnimation::new(_ctx, &join_paths(&chars::BIKER, "Biker_attack.png")));
        biker.entity.insert_animation(Animation::DoubleJump, SpriteAnimation::new(_ctx, &join_paths(&chars::BIKER, "Biker_doublejump.png")));

        return biker;
    }
}


pub struct Cyborg;

impl Cyborg {
    pub fn new(_ctx: &mut Context, x: f32, y: f32) -> Character {
        let mut cyborg = Character::default(_ctx, x, y);
        cyborg.entity.insert_animation(Animation::Idle, SpriteAnimation::new(_ctx, &join_paths(&chars::CYBORG, "Cyborg_idle.png")));
        cyborg.entity.insert_animation(Animation::Run, SpriteAnimation::new(_ctx, &join_paths(&chars::CYBORG, "Cyborg_run.png")));
        cyborg.entity.insert_animation(Animation::Jump,  SpriteAnimation::new(_ctx, &join_paths(&chars::CYBORG, "Cyborg_jump.png")));
        cyborg.entity.insert_animation(Animation::Attack, SpriteAnimation::new(_ctx, &join_paths(&chars::CYBORG, "Cyborg_attack.png")));
        cyborg.entity.insert_animation(Animation::DoubleJump, SpriteAnimation::new(_ctx, &join_paths(&chars::CYBORG, "Cyborg_doublejump.png")));

        return cyborg;
    }
}


pub struct CharacterFactory;

impl CharacterFactory {
    pub fn make(ctx: &mut Context, character_type: CharacterType, x: f32, y: f32) -> Character{
        match character_type {
            CharacterType::Biker => Biker::new(ctx, x, y),
            CharacterType::Punk => Punk::new(ctx, x, y),
            CharacterType::Cyborg => Cyborg::new(ctx, x, y)
        }
    }
}
