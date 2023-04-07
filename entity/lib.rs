#![feature(trait_upcasting)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use factory::Factory;
use object::Object;

pub mod container;
pub mod entity;
pub mod factory;
pub mod object;
pub mod scene;

pub type ObjectPtr = Rc<RefCell<dyn Object>>;
pub type WeakObjectPtr = Weak<RefCell<dyn Object>>;
pub type FactoryPtr = Weak<RefCell<Factory>>;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
