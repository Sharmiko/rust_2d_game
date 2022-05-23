use std::collections::HashMap;
use std::cell::RefCell;

use ggez::Context;


pub mod menu;
pub use menu::MenuState;

pub mod play;
pub use play::PlayState;


pub enum AllStates {
    Menu,
    Play
}

impl AllStates {
    pub fn as_str(&self) -> String {
        match self {
            AllStates::Menu => "menu".to_string(),
            AllStates::Play => "play".to_string()
        }
    }
}


pub trait State {
    fn enter(&self, ctx: &mut Context, current_state: &RefCell<String>);
    fn exit(&self, ctx: &mut Context, current_state: &RefCell<String>);
    fn draw(&mut self, ctx: &mut Context, current_state: &RefCell<String>);
    fn update(&mut self, ctx: &mut Context, current_state: &RefCell<String>);
}


pub type StateFunctions = HashMap<String, Box<dyn State>>;


pub struct StateMachine {
    states: RefCell<StateFunctions>,
    current: RefCell<String>
}


impl StateMachine {

    pub fn new(states: StateFunctions, current: String) -> Self {
        Self {
            states: RefCell::new(states),
            current: RefCell::new(current)
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let mut borrowed = self.states.borrow_mut();
        let mut current = borrowed.get_mut(&*self.current.borrow()).unwrap();
        current.update(ctx, &self.current);
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let mut borrowed = self.states.borrow_mut();
        let mut current = borrowed.get_mut(&*self.current.borrow()).unwrap();
        current.draw(ctx, &self.current)
    }
}
