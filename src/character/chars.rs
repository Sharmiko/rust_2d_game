use ggez::Context;

use crate::character::Character;
use crate::base::human::Animation;
use crate::animation::SpriteAnimation;
use crate::utils::join_paths;
use crate::resources::chars;


pub struct Punk;

impl Punk {
    pub fn new(_ctx: &mut Context) -> Character {
        let mut punk = Character::default(_ctx);
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
    pub fn new(_ctx: &mut Context) -> Character {
        let mut biker = Character::default(_ctx);
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
    pub fn new(_ctx: &mut Context) -> Character {
        let mut cyborg = Character::default(_ctx);
        cyborg.entity.insert_animation(Animation::Idle, SpriteAnimation::new(_ctx, &join_paths(&chars::CYBORG, "Cyborg_idle.png")));
        cyborg.entity.insert_animation(Animation::Run, SpriteAnimation::new(_ctx, &join_paths(&chars::CYBORG, "Cyborg_run.png")));
        cyborg.entity.insert_animation(Animation::Jump,  SpriteAnimation::new(_ctx, &join_paths(&chars::CYBORG, "Cyborg_jump.png")));
        cyborg.entity.insert_animation(Animation::Attack, SpriteAnimation::new(_ctx, &join_paths(&chars::CYBORG, "Cyborg_attack.png")));
        cyborg.entity.insert_animation(Animation::DoubleJump, SpriteAnimation::new(_ctx, &join_paths(&chars::CYBORG, "Cyborg_doublejump.png")));

        return cyborg;
    }
}
