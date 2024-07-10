use std::cell::{RefCell};
use std::rc::Rc;
use poise::Command;
use crate::{Data, Error};
use crate::utils::traits::Cmd;

pub struct CmdBuilder{
    commands: Rc<RefCell<Vec<Command<Data, Error>>>>,
}

impl CmdBuilder{
    pub fn new() -> Self{
        let commands = Rc::new(RefCell::new(Vec::new()));
        Self{
            commands
        }
    }
    pub fn add_cmd<T: Cmd>(self) -> Self{
        self.commands.borrow_mut().extend(T::commands());
        self
    }
    pub fn build(self) -> Vec<Command<Data, Error>> {
        Rc::try_unwrap(self.commands).unwrap().into_inner()
    }
}
