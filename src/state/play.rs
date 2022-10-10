use std::cell::RefCell;
use std::collections::HashMap;

use ggez::Context;
use ggez::graphics::{Canvas, InstanceArray, DrawParam, Rect};

use crate::state::{State, AllStates};
use crate::character::{Character};
use crate::base::LocationType;
use crate::quadtree::QuadTree;
use crate::character::chars::{CharacterFactory, CharacterType};
use crate::tile::{Background, ParkBackground};
use crate::tileset::Map;
use crate::enemy::Enemy;


pub struct PlayState {
    player: Character,
    enemy: Enemy,
    background: Background,
    map: Map,
    instance_arrs: HashMap<String, InstanceArray>
}

fn update_quadtree(tree: &mut QuadTree, location: LocationType) {
    match location {
        LocationType::Single(rect) => {
            tree.insert(rect.x, rect.y, rect);
        },
        LocationType::Multiple(locations) => {
            for loc in locations {
                tree.insert(loc.x, loc.y, loc);
            }
        }
    }
}

impl PlayState {
    pub fn new(ctx: &mut Context) ->  Self {

        let mut player = CharacterFactory::make(ctx, CharacterType::Punk, 100., 100.);

        let mut grid = Map::new("./levels/level_1.tmx");
        let mut arr: HashMap<String, InstanceArray> = HashMap::new();
        let mut locations: Vec<Rect> = Vec::new();

        grid.setup_instance_array(ctx, &mut arr, &mut locations);
        update_quadtree(&mut player.quadtree, LocationType::Multiple(locations));

        Self {
            player: player,
            enemy: Enemy::new(ctx, 1, 400., 100.),
            background: ParkBackground::new(ctx),
            map: grid,
            instance_arrs: arr
        }
    }
}


impl State for PlayState {

    fn enter(&self, _ctx: &mut Context, _current_state: &RefCell<AllStates>) {}

    fn exit(&self, _ctx: &mut Context, _current_state: &RefCell<AllStates>) {}

    fn draw(&mut self, _ctx: &mut Context, canvas: &mut Canvas, _current_state: &RefCell<AllStates>) {
        self.background.draw(_ctx, canvas);
        self.player.draw(_ctx, canvas);
        for (_, instance_arr) in &self.instance_arrs {
            canvas.draw(instance_arr, DrawParam::default());
        }
        self.enemy.draw(_ctx, canvas);
    }

    fn update(&mut self, _ctx: &mut Context, _current_state: &RefCell<AllStates>) {
        self.background.update(_ctx).unwrap();
        let player_layout = self.player.update(_ctx).unwrap();
        self.enemy.player_layout = player_layout;
        self.enemy.update(_ctx);
    }
}