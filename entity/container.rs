use std::rc::Rc;

use crate::entity::{ClassType, EntityInfo, Object};

pub trait Container {
    fn capacity(&self) -> usize;
    fn set_capcity(&mut self, cap: usize) -> bool;
    fn child_count(&self) -> usize;
    fn get_index_in_container(&self) -> usize;
    fn get_parent(&self) -> Option<Rc<dyn Object>>;
    fn get_first_child(&self) -> (Option<Rc<dyn Object>>, usize);
    fn get_next_child(&self, it: usize) -> (Option<Rc<dyn Object>>, usize);
    fn get_child_id_list(&self, class_type: ClassType) -> Vec<u64>;
}

impl Container for EntityInfo {
    fn capacity(&self) -> usize {
        self.cap
    }

    fn set_capcity(&mut self, cap: usize) -> bool {
        if self.cap == cap {
            return true;
        }

        if cap < self.cap {
            // only empty can shrink
            if self.childs.len() > 0 {
                return false;
            }
            self.childs.shrink_to(cap);
        } else {
            self.childs.reserve(cap - self.childs.len());
        }
        self.cap = cap;
        return true;
    }

    fn child_count(&self) -> usize {
        self.child_num
    }

    fn get_index_in_container(&self) -> usize {
        self.container_pos
    }
    fn get_parent(&self) -> Option<Rc<dyn Object>> {
        if let Some(parent) = &self.parent {
            Some(Rc::clone(parent))
        } else {
            None
        }
    }
    fn get_first_child(&self) -> (Option<Rc<dyn Object>>, usize) {
        let num = self.childs.len();
        for it in 0..num {
            if let Some(Some(obj)) = self.childs.get(it) {
                return (Some(Rc::clone(obj)), it);
            }
        }
        (None, 0)
    }
    fn get_next_child(&self, it: usize) -> (Option<Rc<dyn Object>>, usize) {
        let it = it + 1;
        let num = self.childs.len();
        if it >= num {
            return (None, 0);
        }
        for it in it..num {
            if let Some(Some(obj)) = self.childs.get(it) {
                return (Some(Rc::clone(obj)), it);
            }
        }
        (None, 0)
    }
    fn get_child_id_list(&self, class_type: ClassType) -> Vec<u64> {
        let mut result = Vec::with_capacity(self.child_num);
        for i in 0..self.childs.len() {
            if let Some(obj) = self.childs.get(i).unwrap() {
                match class_type {
                    ClassType::None => result.push(obj.entity_ref().uid()),
                    _ => {
                        if obj.entity_ref().get_class_type() == class_type {
                            result.push(obj.entity_ref().uid());
                        }
                    }
                }
            }
        }
        result
    }
}
