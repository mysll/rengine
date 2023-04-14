#![feature(trait_upcasting)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use factory::Factory;
use game_model::GameModel;
use object::Object;

pub mod container;
pub mod factory;
pub mod game_model;
pub mod game_object;
pub mod object;
pub mod registry;
pub mod game_scene;

pub type ObjectPtr = Rc<RefCell<Object>>;
pub type WeakObjectPtr = Weak<RefCell<Object>>;
pub type FactoryPtr = Rc<RefCell<Factory>>;
pub type WeakFactoryPtr = Weak<RefCell<Factory>>;
pub type GameModelPtr = Rc<RefCell<dyn GameModel>>;
pub type WeakGameModelPtr = Weak<RefCell<dyn GameModel>>;

#[derive(Debug)]
pub struct MutObjectPtr(pub *mut Object);

impl Default for MutObjectPtr {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}