#![feature(trait_upcasting)]

use std::{cell::RefCell, rc::Rc};

use entity::Object;

pub mod container;
pub mod entity;
pub mod factory;

pub type ObjectPtr = Rc<RefCell<dyn Object>>;
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
