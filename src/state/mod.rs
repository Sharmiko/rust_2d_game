use std::collections::HashMap;
use std::cell::RefCell;

use ggez::Context;
use ggez::graphics::Canvas;


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
    fn enter(&self, _ctx: &mut Context, _current_state: &RefCell<AllStates>);
    fn exit(&self, _ctx: &mut Context, _current_state: &RefCell<AllStates>);
    fn draw(&mut self, _ctx: &mut Context, canvas: &mut Canvas, _current_state: &RefCell<AllStates>);
    fn update(&mut self, _ctx: &mut Context, _current_state: &RefCell<AllStates>);
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

    pub fn update(&mut self, _ctx: &mut Context) {
        let mut borrowed = self.states.borrow_mut();
        let current = borrowed.get_mut(&*self.current.borrow()).unwrap();
        current.update(_ctx, &self.current);
    }

    pub fn draw(&mut self, _ctx: &mut Context, canvas: &mut Canvas) {
        let mut borrowed = self.states.borrow_mut();
        let current = borrowed.get_mut(&*self.current.borrow()).unwrap();
        current.draw(_ctx, canvas, &self.current)
    }
}
