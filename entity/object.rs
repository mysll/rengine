use crate::entity::Entity;
use crate::{entity::EntityInfo, ObjectPtr};
use std::any::Any;
use std::fmt::Debug;

pub trait Object: Debug + Any {
    fn entity_ref<'a>(&'a self) -> &'a EntityInfo;
    fn entity_mut<'a>(&'a mut self) -> &'a mut EntityInfo;
    fn get_attr_by_name<'a>(&'a self, attr: &str) -> Option<&'a dyn Any>;
    fn set_attr_by_name(&mut self, attr: &str, val: &dyn Any) -> bool;
    fn get_attr_by_index<'a>(&'a self, index: u32) -> Option<&'a dyn Any>;
    fn set_attr_by_index(&mut self, index: u32, val: &dyn Any) -> bool;
}

pub struct GameObject {}

impl GameObject {
    pub fn destroy_self(this: ObjectPtr) {
        assert!(this.borrow().entity_ref().parent.is_some());
        let parent = match &this.borrow().entity_ref().parent {
            Some(parent) => parent.upgrade(),
            None => {
                return;
            }
        };
        if let Some(parent) = parent {
            parent.borrow_mut().entity_mut().destroy_child(this);
        }
    }
}
