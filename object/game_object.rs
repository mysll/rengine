use std::{any::Any, rc::Rc};

use tracing::{debug, warn};

use crate::{
    container::Container,
    object::{ClassType, Object},
    FactoryPtr, ObjectPtr,
};

pub trait GameObject: Container {
    fn set_ptr(&mut self, self_ptr: &ObjectPtr);
    fn set_factory(&mut self, factory: &FactoryPtr);
    fn get_factory(&self) -> Option<FactoryPtr>;
    fn set_parent(&mut self, parent: &ObjectPtr);
    fn get_parent(&self) -> Option<ObjectPtr>;
    fn destroy_children(&mut self);
    fn destroy_child(&mut self, child: &ObjectPtr);
    fn uid(&self) -> u64;
    fn set_uid(&mut self, uid: u64);
    fn get_class_type(&self) -> ClassType;
    fn is_deleted(&self) -> bool;
    fn delete(&mut self);
    fn dirty(&self) -> bool;
    fn clear_dirty(&mut self);
    fn modify(&self) -> bool;
    fn clear_modify(&mut self);
    fn get_modify<'a>(&'a self) -> &'a Vec<u32>;
    fn get_attrs<'a>(&'a self) -> &'a Vec<&str>;
    fn save_attrs<'a>(&'a self) -> &'a Vec<&str>;
    fn rep_attrs<'a>(&'a self) -> &'a Vec<&str>;
    fn save_attrs_index(&self) -> &Vec<u32>;
    fn rep_attrs_index(&self) -> &Vec<u32>;
    fn get_attr_count(&self) -> u32;
    fn get_attr_name<'a>(&'a self, index: u32) -> Option<&'a str>;
    fn get_attr_index(&self, attr: &str) -> Option<u32>;
    fn change_attr(&mut self, index: u32, old: &dyn Any);
}

impl GameObject for Object {
    fn set_ptr(&mut self, self_ptr: &ObjectPtr) {
        self.self_ptr = Some(Rc::downgrade(&self_ptr));
    }

    fn set_factory(&mut self, factory: &FactoryPtr) {
        self.factory = Some(Rc::downgrade(factory))
    }

    fn get_factory(&self) -> Option<FactoryPtr> {
        if let Some(weak_factory) = &self.factory {
            if let Some(strong_factory) = weak_factory.upgrade() {
                return Some(strong_factory);
            }
        }
        None
    }

    fn set_parent(&mut self, child: &ObjectPtr) {
        self.parent = Some(Rc::downgrade(child));
    }

    fn get_parent(&self) -> Option<ObjectPtr> {
        if let Some(parent) = &self.parent {
            if let Some(pobj) = parent.upgrade() {
                return Some(pobj);
            }
        }
        None
    }

    fn destroy_child(&mut self, child: &ObjectPtr) {
        if child.borrow().destroying {
            warn!("object already destroying");
            return;
        }
        child.borrow_mut().destroying = true;
        let in_container = child.borrow().is_in_container();
        if in_container {
            self.remove_child(child);
        }

        child.borrow_mut().destroy_children();

        if let Some(f) = self.get_factory() {
            f.borrow_mut().delete(&child);
        }
    }

    fn destroy_children(&mut self) {
        let children = self.children.len();
        for i in 0..children {
            if let Some(child) = &self.children[i] {
                self.destroy_child(&child.clone());
            }
        }
    }

    fn uid(&self) -> u64 {
        self.uid
    }

    fn set_uid(&mut self, uid: u64) {
        self.uid = uid;
    }

    fn get_class_type(&self) -> ClassType {
        self.class_type
    }

    fn is_deleted(&self) -> bool {
        self.deleted
    }

    fn delete(&mut self) {
        self.deleted = true;
    }

    fn dirty(&self) -> bool {
        self.dirty
    }

    fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    fn modify(&self) -> bool {
        self.modify_attrs.len() > 0
    }

    fn clear_modify(&mut self) {
        self.modify_attrs.clear();
    }

    fn get_modify<'a>(&'a self) -> &'a Vec<u32> {
        &self.modify_attrs
    }

    fn get_attrs<'a>(&'a self) -> &'a Vec<&str> {
        &self.model.attrs
    }

    fn save_attrs<'a>(&'a self) -> &'a Vec<&str> {
        &self.model.saves
    }

    fn rep_attrs<'a>(&'a self) -> &'a Vec<&str> {
        &self.model.reps
    }

    fn save_attrs_index(&self) -> &Vec<u32> {
        &self.model.saves_index
    }

    fn rep_attrs_index(&self) -> &Vec<u32> {
        &self.model.reps_index
    }

    fn get_attr_count(&self) -> u32 {
        self.model.attrs.len() as u32
    }

    fn get_attr_index(&self, attr: &str) -> Option<u32> {
        match self.model.index.get(attr) {
            Some(&i) => Some(i),
            None => None,
        }
    }

    fn get_attr_name<'a>(&'a self, index: u32) -> Option<&'a str> {
        match self.model.attrs.get(index as usize) {
            Some(&attr) => Some(attr),
            None => None,
        }
    }

    fn change_attr(&mut self, index: u32, old: &dyn Any) {
        if self.model.saves_set.contains(&index) {
            self.dirty = true;
        }
        if self.model.reps_set.contains(&index) {
            self.modify_attrs.push(index);
        }
        debug!("old:{:?}\n", old);
    }
}
