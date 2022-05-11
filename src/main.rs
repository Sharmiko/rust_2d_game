use std::{thread, time::Duration};

mod character;

use character::Character;

use macroquad::prelude::*;


#[macroquad::main("Texture")]
async fn main() {

    let mut my_char = Character::new().await;
    loop {
        my_char.update().await;
        thread::sleep(Duration::from_millis(100));
        next_frame().await
    }
}
