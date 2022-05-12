use std::{thread, time::Duration};

mod consts;
mod character;

use character::{Punk};

use macroquad::prelude::*;


#[macroquad::main("Texture")]
async fn main() {

    let mut my_char = Punk::new().await;
    loop {
        my_char.update().await;
        thread::sleep(Duration::from_millis(100));
        next_frame().await
    }
}
