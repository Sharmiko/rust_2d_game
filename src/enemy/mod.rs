use ggez::Context;
use ggez::graphics::{Canvas, Rect};
use crate::animation::{SpriteAnimation, HumanAnimation};
use crate::base::human::{BaseHuman, Animation};
use crate::base::CustomRect;
use crate::utils::join_paths;
use crate::resources::enemies;
use crate::consts::{
    CHAR_HEIGHT, WALK_SPEED
};
use crate::collisions::rect_collision;


pub mod fov;
pub use fov::FieldOfVision;


pub struct Enemy {
    pub entity: BaseHuman,
    pub fov: FieldOfVision,
    pub player_layout: Rect,
    walk_range_counter: f32,
    walk_right: bool
}

impl Enemy {

    const WALK_RANGE: f32 = 300.;

    pub fn new(_ctx: &mut Context, enemy_number: i8) -> Self {
        let mut entity = BaseHuman::default(_ctx);
        entity.insert_animation(
            Animation::Idle, 
            SpriteAnimation::new(_ctx, &join_paths(&enemies::DIR, &format!("{}/Idle.png", enemy_number)))
        );
        entity.insert_animation(
            Animation::Walk, 
            SpriteAnimation::new(_ctx, &join_paths(&enemies::DIR, &format!("{}/Walk.png", enemy_number)))
        );
        entity.insert_animation(
            Animation::Attack, 
            SpriteAnimation::new(_ctx, &join_paths(&enemies::DIR, &format!("{}/Attack.png", enemy_number)))
        );

        let (_, h) = _ctx.gfx.size();
        entity.layout.x += 400.;
        entity.layout.y = h - CHAR_HEIGHT * 1.5;

        let fov = FieldOfVision::new(200., 150., &entity.layout);

        Self {
            entity: entity,
            fov: fov,
            player_layout: Rect::new(0., 0., 0., 0.),
            walk_range_counter: 0.,
            walk_right: true
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas)  {
        self.entity.draw(ctx, canvas);
        CustomRect::from_rect(self.entity.dyn_layout()).draw(ctx, canvas);
        self.fov.layout.draw(ctx, canvas);
    }

    fn walk_right(&mut self, _ctx: &mut Context) {
        self.entity.update_current_anim(Animation::Walk);
        if self.entity.state.is_flipped {
            self.entity.state.is_flipped = false;
            self.entity.layout.x -= self.entity.layout.w / 4.;
        }
        self.entity.layout.x += WALK_SPEED;
    }

    fn walk_left(&mut self, _ctx: &mut Context) {
        self.entity.update_current_anim(Animation::Walk);
        if !self.entity.state.is_flipped {
            self.entity.state.is_flipped = true;
            self.entity.layout.x += self.entity.layout.w / 4.;
        }
        self.entity.layout.x -= WALK_SPEED;
    }


    pub fn update(&mut self, ctx: &mut Context) { 
        if let Some(_coll) = rect_collision(&self.fov.layout.fields, &self.player_layout) {
            println!("fov entered");
        }

        self.fov.update(&self.entity.layout);
        
        self.walk_range_counter += WALK_SPEED;
        if self.walk_range_counter >= Enemy::WALK_RANGE {
            self.walk_range_counter = 0.;
            self.walk_right = !self.walk_right
        }

        if self.walk_right {
            self.entity.layout.x += WALK_SPEED;
            self.walk_right(ctx);
        } else {
            self.entity.layout.x -= WALK_SPEED;
            self.walk_left(ctx);
        }
    }

}