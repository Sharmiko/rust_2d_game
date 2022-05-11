use std::{thread, time::Duration};

mod character;

use character::character::Character;


use macroquad::prelude::*;

#[macroquad::main("Texture")]
async fn main() {


    let mut count = 0;
    let my_char = Character::new();
    let mut idle_anim = my_char.idle.iter().cycle();
    loop {
        let my_char: Texture2D = load_texture(idle_anim.next().unwrap()).await.unwrap();
        count = count + 1;
        if count > 3 {
            count = 0;
        }
        clear_background(LIGHTGRAY);
        draw_texture(
            my_char,
            screen_width() / 2. - my_char.width() / 2.,
            screen_height() / 2. - my_char.height() / 2.,
            WHITE
        );
        thread::sleep(Duration::from_millis(300));
        next_frame().await
    }
}
