use crate::entity::GameEntity;
use crate::{entity::Entity, ObjectPtr};
use std::any::Any;
use std::fmt::Debug;
use std::ops::DerefMut;

pub trait Object: Debug + Any {
    fn entity_ref<'a>(&'a self) -> &'a Entity;
    fn entity_mut<'a>(&'a mut self) -> &'a mut Entity;
    fn get_attr_by_name<'a>(&'a self, attr: &str) -> Option<&'a dyn Any>;
    fn set_attr_by_name(&mut self, attr: &str, val: &dyn Any) -> bool;
    fn get_attr_by_index<'a>(&'a self, index: u32) -> Option<&'a dyn Any>;
    fn set_attr_by_index(&mut self, index: u32, val: &dyn Any) -> bool;
}

#[allow(unused)]
pub struct GameObject {
    object: ObjectPtr,
    raw_ptr: *mut dyn Object,
}

impl GameObject {
    pub fn new(ptr: ObjectPtr) -> Self {
        let obj_ptr = ptr.borrow_mut().deref_mut() as *mut dyn Object;
        Self {
            object: ptr,
            raw_ptr: obj_ptr,
        }
    }

    pub fn get_entity<'a>(&'a self) -> &'a Entity {
        unsafe { (*self.raw_ptr).entity_ref() }
    }

    pub fn get_entity_mut<'a>(&'a mut self) -> &'a mut Entity {
        unsafe { (*self.raw_ptr).entity_mut() }
    }

    pub fn destroy_object(parent: ObjectPtr, target: ObjectPtr) {
        assert!(target.borrow().entity_ref().parent.is_some());
        if Self::check_parent(target.clone(), parent.clone()) {
            parent.borrow_mut().entity_mut().destroy_child(target);
        }
    }

    pub fn destroy_self(this: ObjectPtr) {
        assert!(this.borrow().entity_ref().parent.is_some());
        let parent = this.borrow().entity_ref().get_parent();
        if let Some(parent) = parent {
            parent.borrow_mut().entity_mut().destroy_child(this);
        }
    }

    pub fn check_parent(child: ObjectPtr, parent: ObjectPtr) -> bool {
        if let Some(parent_ptr) = child.borrow().entity_ref().get_parent() {
            return parent_ptr.as_ptr() == parent.as_ptr();
        }
        return false;
    }
}
