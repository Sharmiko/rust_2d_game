use std::fs;
use std::collections::HashMap;

use serde_derive::Deserialize;
use ggez::Context;
use ggez::graphics::{Image, DrawParam, InstanceArray, Rect};

use crate::consts::LEVELS_DIR;


#[derive(Deserialize, Debug)]
pub struct Tile {
    id: String,
    image: ImagePath
}


#[derive(Deserialize, Debug)]
pub struct ImagePath {
    source: String
}


#[derive(Deserialize, Debug)]
pub struct Tiles {
    pub tile: Vec<Tile>
}

impl Tiles {
    pub fn new(path: &str) -> HashMap<String, String> {
        let xml_string = fs::read_to_string(path).unwrap();
        let tileset: Tiles = serde_xml_rs::from_str(xml_string.as_str()).unwrap();

        let mut image_mapping = HashMap::new();
        for row in &tileset.tile {
            image_mapping.insert(
                row.id.to_string(),
                row.image.source.to_string()
            );
        }

        return image_mapping;
    }
}

#[derive(Deserialize, Debug)]
pub struct Tileset {
    pub source: String
}

#[derive(Deserialize, Debug)]
pub struct Layer {
    pub data: String,
    pub width: i8,
    pub height: i8
}


#[derive(Deserialize, Debug)]
pub struct Map {
    pub tileset: Tileset,
    pub layer: Layer
}

impl Map {
    pub fn new(path: &str) -> Map {
        let xml_string = fs::read_to_string(path).unwrap();
        let map: Map = serde_xml_rs::from_str(xml_string.as_str()).unwrap();
        return map;
    }

    pub fn setup_instance_array(&mut self, ctx: &mut Context, instance_arrs: &mut HashMap<String, InstanceArray>, locations: &mut Vec<Rect>) {
        
        let tiles: HashMap<String, String> = Tiles::new(
            &format!("{}/{}", LEVELS_DIR, self.tileset.source.as_str())
        );
        
        let (mut x, mut y) = ctx.gfx.size();
        y -= 64f32;
        x -= 32f32;

        let mut lines: Vec<&str> = Vec::new();
        for line in self.layer.data.split("\n") {
            lines.push(line);
        }
        let lines: Vec<&str> = lines.into_iter().rev().collect();

        for line in lines {
            for id in line.chars().rev().collect::<String>().split(",") {
                if id == " " || id == "0" || id == "" {
                    x -= 32f32;
                    continue;
                }

                let parsed_str = id.trim().parse::<i32>();
                if parsed_str.is_err() {
                    continue;
                }

                let image_path = tiles.get(&(parsed_str.unwrap() - 1i32).to_string());
                if image_path.is_none() {
                    continue;
                }

                let image_path = image_path.unwrap().trim_start_matches("../resources");

                let image = Image::from_path(&ctx.gfx, image_path).unwrap();
                let image_width = image.width() as f32;
                // TODO - add support for other types of objects
                
                if !instance_arrs.contains_key(image_path) {
                    instance_arrs.insert(image_path.to_string(), InstanceArray::new(&ctx.gfx, image, 1));
                }

                instance_arrs.get_mut(image_path).unwrap().push(DrawParam::default().dest([x, y]));
                locations.push(Rect {
                    x: x,
                    y: y,
                    w: 32f32,
                    h: 32f32
                });
                x -= 32f32;
            }
            (x, _) = ctx.gfx.size();
            y -= 32f32;
        }
    }

}
