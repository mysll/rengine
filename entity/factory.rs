use std::collections::VecDeque;

use crate::{entity::Registry, ObjectPtr};

pub struct Factory {
    registry: Registry,
    objects: Vec<Option<ObjectPtr>>,
    free_list: VecDeque<usize>,
    deletes: VecDeque<ObjectPtr>,
    used_size: usize,
    serial: usize,
}

impl Factory {
    pub fn new() -> Self {
        let mut s = Self {
            registry: Registry::init(),
            objects: Vec::with_capacity(16),
            free_list: VecDeque::with_capacity(16),
            deletes: VecDeque::new(),
            used_size: 0,
            serial: 0,
        };
        s.objects.resize(16, None);
        s
    }

    pub fn create(&mut self, ent: &str) -> Option<ObjectPtr> {
        let new_obj = self.registry.create_object(ent);
        if new_obj.is_none() {
            return None;
        }
        let index;
        if self.free_list.len() == 0 {
            if self.used_size == self.objects.len() {
                if self.used_size > 0x1000000 {
                    panic!("too many objects created, abort!");
                }
                // double size
                self.objects.resize(self.used_size * 2, None);
            }
            index = self.used_size;
            self.used_size += 1;
        } else {
            index = self.free_list.pop_back().unwrap();
        }
        if self.serial >= 0x7FFFFFFF {
            self.serial = 1;
        } else {
            self.serial += 1;
        }
        let mut id = (self.serial << 32) as u64;
        id += index as u64;
        let new_obj = new_obj.unwrap();
        {
            let mut obj = new_obj.borrow_mut();
            obj.entity_mut().set_uid(id);
        }
        let ret = new_obj.clone();
        self.objects[index] = Some(new_obj);
        Some(ret)
    }

    /// 立即销毁一个对象
    /// 从工厂移除，立即drop
    pub fn destroy(&mut self, obj_ptr: ObjectPtr) {
        let id: u64;
        {
            let object = obj_ptr.as_ref().borrow();
            let entity = object.entity_ref();
            if entity.is_deleted() {
                return;
            }
            id = entity.uid();
        }

        let index = (id & 0x7FFFFFFF) as usize;
        if index > self.objects.len() {
            panic!("object id error");
        }

        match &self.objects[index] {
            Some(rcobj) => {
                if rcobj.as_ref().borrow().entity_ref().uid() != id {
                    panic!("object not match");
                }
            }
            None => {
                panic!("object is null");
            }
        }
        obj_ptr.borrow_mut().entity_mut().delete();
        self.objects[index] = None;
        self.free_list.push_back(index);
    }

    /// 设置删除标志
    /// 从工厂移除，延迟drop
    pub fn delete(&mut self, obj_ptr: ObjectPtr) {
        {
            let id: u64;
            {
                let object = obj_ptr.as_ref().borrow();
                let entity = object.entity_ref();
                if entity.is_deleted() {
                    return;
                }
                id = entity.uid();
            }

            let index = (id & 0x7FFFFFFF) as usize;
            if index > self.objects.len() {
                panic!("object id error");
            }

            match &self.objects[index] {
                Some(rcobj) => {
                    if rcobj.as_ref().borrow().entity_ref().uid() != id {
                        panic!("object not match");
                    }
                }
                None => {
                    panic!("object is null");
                }
            }
            obj_ptr.borrow_mut().entity_mut().delete();
            self.objects[index] = None;
            self.free_list.push_back(index);
        }
        self.deletes.push_back(obj_ptr);
    }

    /// 立即销毁标志为删除的对象
    pub fn clear_deleted(&mut self) {
        if self.deletes.len() == 0 {
            return;
        }

        while self.deletes.len() > 0 {
            self.deletes.pop_front();
        }
    }

    pub fn find(&self, uid: u64) -> Option<ObjectPtr> {
        let index = (uid & 0x7FFFFFFF) as usize;
        if index > self.objects.len() {
            return None;
        }

        match &self.objects[index] {
            Some(rcobj) => {
                if rcobj.borrow().entity_ref().uid() == uid {
                    let obj = rcobj.clone();
                    Some(obj)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
