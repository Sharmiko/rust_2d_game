use macroquad::input;
use macroquad::prelude::*;


pub struct Animation {
    texture: Texture2D,
    index: i8
}


impl Animation {
    fn next(&mut self) -> Rect {
        if self.index as f32 == self.texture.width() / 144. {
            self.index = 0;
        }
        let rect: Rect = Rect {
            x: self.index as f32 * 144.,
            y: 0.,
            w: 144.,
            h: 144.
        };

        self.index += 1;

        return rect;
    }
}


pub struct Character {
    name: String,
    health: i8,
    idle: Animation,
    run: Animation
}


impl Character {

    pub async fn new() -> Self {
        Self {
            name: "Punk".to_string(),
            health: 10,
            idle: Animation {
                texture: load_texture("assets/chars/punk/Punk_idle.png").await.unwrap(),
                index: 0
            },
            run: Animation {
                texture: load_texture("assets/chars/punk/Punk_run.png").await.unwrap(),
                index: 0
            }
        }
    }


    pub async fn update(&mut self) -> () {
        let current_texture: Texture2D;
        let mut draw_params: DrawTextureParams = DrawTextureParams::default();
        let mut offset: f32 = 0.;

        if input::is_key_down(input::KeyCode::A) {
            current_texture = self.run.texture;
            draw_params.flip_x = true;
            draw_params.source = Some(self.run.next());
            offset = 144. / 2.;
        } else if input::is_key_down(input::KeyCode::D) {
            current_texture = self.run.texture;
            draw_params.source = Some(self.run.next());
        } else {
            current_texture = self.idle.texture;
            draw_params.source = Some(self.idle.next());
        }

        clear_background(LIGHTGRAY);
        println!("{} {} {}", current_texture.width(), current_texture.height(), current_texture.width() / 144.);
        draw_texture_ex(
            current_texture,
            screen_width() / 2. - 144. / 2. - offset,
            screen_height() / 2. - 144. / 2.,
            WHITE,
            draw_params
        );
    }
}
        