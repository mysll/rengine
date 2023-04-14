use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use tracing::{debug, warn};

use crate::{game_object::GameObject, object::Object, registry::Registry, ObjectPtr};

#[derive(Debug)]
pub struct Factory {
    registry: Rc<Registry>,
    objects: Vec<Option<ObjectPtr>>,
    free_list: VecDeque<usize>,
    deletes: VecDeque<ObjectPtr>,
    used_size: usize,
    serial: usize,
    owner: ObjectPtr,
}

impl Drop for Factory {
    fn drop(&mut self) {
        self.clear_deleted();
        debug!("factory destroyed");
    }
}

impl Factory {
    pub fn new(registry: Rc<Registry>, owner: ObjectPtr) -> Self {
        let mut s = Self {
            registry: registry,
            objects: Vec::with_capacity(16),
            free_list: VecDeque::with_capacity(16),
            deletes: VecDeque::new(),
            used_size: 1, // ignore 0
            serial: 0,
            owner: owner,
        };
        s.objects.resize(16, None);
        s
    }

    pub fn get_owner(&self) -> ObjectPtr {
        self.owner.clone()
    }

    pub fn init(&mut self) {
        self.objects[0] = Some(self.owner.clone());
        self.owner.borrow_mut().set_uid(1 << 32);
    }

    pub fn create(&mut self, ent: &str, cap: usize) -> Option<ObjectPtr> {
        let new_data = self.registry.create_object(ent);
        if new_data.is_none() {
            return None;
        }
        let new_obj;
        if cap == 0 {
            new_obj = Rc::new(RefCell::new(Object::new(new_data.unwrap())));
        } else {
            new_obj = Rc::new(RefCell::new(Object::new_with_cap(new_data.unwrap(), cap)));
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

        Object::object_map_mut(&new_obj, |obj| {
            obj.set_ptr(&new_obj);
            obj.set_uid(id);
        });
        let ret = new_obj.clone();
        self.objects[index] = Some(new_obj);
        Some(ret)
    }

    /// 立即销毁一个对象
    /// 从工厂移除，立即drop
    pub fn destroy(&mut self, obj_ptr: &ObjectPtr) {
        let id: u64;
        {
            let entity = obj_ptr.borrow();
            if entity.is_deleted() {
                warn!("already deleted");
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
                if rcobj.as_ptr() != obj_ptr.as_ptr() {
                    panic!("object not match");
                }
            }
            None => {
                panic!("object is null");
            }
        }
        obj_ptr.borrow_mut().delete();
        self.objects[index] = None;
        self.free_list.push_back(index);
    }

    /// 设置删除标志
    /// 从工厂移除，延迟drop
    pub fn delete(&mut self, obj_ptr: &ObjectPtr) {
        {
            let id: u64;
            {
                let entity = obj_ptr.borrow();
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
                    if rcobj.as_ptr() != obj_ptr.as_ptr() {
                        panic!("object not match");
                    }
                }
                None => {
                    panic!("object is null");
                }
            }
            obj_ptr.borrow_mut().delete();
            self.objects[index] = None;
            self.free_list.push_back(index);
        }
        self.deletes.push_back(obj_ptr.clone());
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

    /// 查找对象
    pub fn find(&self, uid: u64) -> Option<ObjectPtr> {
        let index = (uid & 0x7FFFFFFF) as usize;
        if index > self.objects.len() {
            return None;
        }

        match &self.objects[index] {
            Some(rcobj) => {
                if rcobj.borrow().uid == uid {
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
