use std::rc::Rc;

use tracing::warn;

use crate::{
    game_object::GameObject,
    object::{ClassType, Object},
    ObjectPtr, WeakObjectPtr,
};

pub trait Container {
    fn capacity(&self) -> usize;
    fn set_capcity(&mut self, cap: usize) -> bool;
    fn child_count(&self) -> usize;
    fn set_container_pos(&mut self, pos: usize);
    fn is_in_container(&self) -> bool;
    fn get_container_pos(&self) -> usize;
    fn set_weak_parent(&mut self, parent: WeakObjectPtr);
    fn get_first_child(&self) -> (Option<ObjectPtr>, usize);
    fn get_next_child(&self, it: usize) -> (Option<ObjectPtr>, usize);
    fn get_child_id_list(&self, class_type: ClassType) -> Vec<u64>;
    fn create_child(&mut self, entity: &str, cap: usize, pos: usize) -> Option<ObjectPtr>;
    fn add_child(&mut self, child: ObjectPtr, pos: usize) -> bool;
    fn remove_child(&mut self, child: &ObjectPtr) -> bool;
    fn remove_child_by_index(&mut self, index: usize) -> bool;
    fn find_child_container_free_index(&self) -> Option<usize>;
}

impl Container for Object {
    fn capacity(&self) -> usize {
        self.cap
    }

    fn set_capcity(&mut self, cap: usize) -> bool {
        if self.cap == cap {
            return true;
        }

        if cap < self.cap {
            // only empty can shrink
            if self.children.len() > 0 {
                return false;
            }
            self.children.shrink_to(cap);
        } else {
            self.children.reserve(cap - self.children.len());
        }
        self.cap = cap;
        return true;
    }

    fn child_count(&self) -> usize {
        self.child_num
    }

    fn get_container_pos(&self) -> usize {
        self.container_pos
    }

    fn get_first_child(&self) -> (Option<ObjectPtr>, usize) {
        let num = self.children.len();
        for it in 0..num {
            if let Some(Some(obj)) = self.children.get(it) {
                return (Some(Rc::clone(obj)), it);
            }
        }
        (None, 0)
    }

    fn get_next_child(&self, it: usize) -> (Option<ObjectPtr>, usize) {
        let it = it + 1;
        let num = self.children.len();
        if it >= num {
            return (None, 0);
        }
        for it in it..num {
            if let Some(Some(obj)) = self.children.get(it) {
                return (Some(obj.clone()), it);
            }
        }
        (None, 0)
    }

    fn get_child_id_list(&self, class_type: ClassType) -> Vec<u64> {
        let mut result = Vec::with_capacity(self.child_num);
        for i in 0..self.children.len() {
            if let Some(obj) = self.children.get(i).unwrap() {
                match class_type {
                    ClassType::None => result.push(obj.as_ref().borrow().uid),
                    _ => {
                        if obj.as_ref().borrow().class_type == class_type {
                            result.push(obj.as_ref().borrow().uid);
                        }
                    }
                }
            }
        }
        result
    }

    fn create_child(&mut self, entity: &str, cap: usize, pos: usize) -> Option<ObjectPtr> {
        assert!(self.get_factory().is_some());
        if let Some(factory_ptr) = self.get_factory() {
            let mut factory = factory_ptr.borrow_mut();
            if let Some(new_object) = factory.create(entity, cap) {
                if !self.add_child(new_object.clone(), pos) {
                    factory.destroy(&new_object);
                    return None;
                }
                new_object.borrow_mut().set_factory(&factory_ptr);
                Object::created(&new_object);
                return Some(new_object.clone());
            }
        }

        None
    }

    fn add_child(&mut self, child: ObjectPtr, pos: usize) -> bool {
        if child.borrow().is_deleted() {
            warn!("object is delete");
            return false;
        }
        assert!(!child.borrow().is_in_container());
        if self.cap > 0 && self.child_num >= self.cap {
            return false;
        }
        let index: usize;
        let mut real_pos = pos;
        if pos > 0 && self.cap > 0 {
            if pos > self.cap {
                return false;
            }
            let old_size = self.children.len();
            if pos < old_size {
                if self.children[pos - 1].is_none() {
                    return false;
                }
            } else {
                self.children.resize(pos, None);
            }
            index = pos - 1;
        } else {
            index = match self.find_child_container_free_index() {
                Some(index) => index,
                None => {
                    return false;
                }
            };
            real_pos = index + 1;
        }
        if self.children.len() == index {
            self.children.push(Some(child.clone()));
        } else {
            self.children[index] = Some(child.clone());
        }
        self.child_num += 1;

        {
            let mut entity = child.borrow_mut();
            entity.set_container_pos(real_pos);
            if let Some(ptr) = &self.self_ptr {
                entity.set_weak_parent(ptr.clone());
            }
        }
        self.dirty = true;
        true
    }

    fn remove_child(&mut self, child: &ObjectPtr) -> bool {
        assert!(child.borrow().is_in_container());
        let index = child.borrow().get_container_pos() - 1;
        assert!(index < self.children.len());
        if index >= self.children.len() {
            return false;
        }
        if self.children[index].is_none() {
            return false;
        }

        match &self.children[index] {
            Some(obj) => {
                if obj.as_ptr() != child.as_ptr() {
                    return false;
                }
            }
            None => {
                return false;
            }
        }

        self.remove_child_by_index(index)
    }

    fn remove_child_by_index(&mut self, index: usize) -> bool {
        assert!(index < self.children.len());
        if index >= self.children.len() {
            return false;
        }
        let child = self.children[index].take();
        if child.is_none() {
            return false;
        }
        self.child_num -= 1;
        {
            let child = child.unwrap();
            let mut mut_child = child.borrow_mut();
            mut_child.set_container_pos(0);
        }
        self.dirty = true;
        true
    }

    fn find_child_container_free_index(&self) -> Option<usize> {
        let child_size = self.children.len();
        for i in 0..child_size {
            if self.children[i].is_none() {
                return Some(i);
            }
        }
        if self.cap == 0 || child_size < self.cap {
            return Some(child_size);
        }
        None
    }

    fn set_container_pos(&mut self, pos: usize) {
        self.container_pos = pos;
    }

    fn set_weak_parent(&mut self, parent: WeakObjectPtr) {
        self.parent = Some(parent);
    }

    fn is_in_container(&self) -> bool {
        self.container_pos > 0
    }
}
