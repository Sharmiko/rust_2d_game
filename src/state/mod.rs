use std::collections::HashMap;
use std::cell::RefCell;

use ggez::Context;


pub mod menu;
pub use menu::MenuState;

pub mod play;
pub use play::PlayState;


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AllStates {
    Menu,
    Play
}


pub trait State {
    fn enter(&self, ctx: &mut Context, current_state: &RefCell<AllStates>);
    fn exit(&self, ctx: &mut Context, current_state: &RefCell<AllStates>);
    fn draw(&mut self, ctx: &mut Context, current_state: &RefCell<AllStates>);
    fn update(&mut self, ctx: &mut Context, current_state: &RefCell<AllStates>);
}


pub type StateFunctions = HashMap<AllStates, Box<dyn State>>;


pub struct StateMachine {
    states: RefCell<StateFunctions>,
    current: RefCell<AllStates>
}


impl StateMachine {

    pub fn new(states: StateFunctions, current: AllStates) -> Self {
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
