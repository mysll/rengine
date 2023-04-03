#![feature(trait_upcasting)]

use std::{cell::RefCell, rc::Rc};

use entity::Object;
use factory::Factory;

pub mod container;
pub mod entity;
pub mod factory;

pub type ObjectPtr = Rc<RefCell<dyn Object>>;
pub type FactoryPtr = Rc<RefCell<Factory>>;
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
