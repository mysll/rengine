use std::{
    any::Any,
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Debug,
    rc::Rc,
};

use tracing::{debug, warn};

use crate::{container::Container, factory::Factory, FactoryPtr, ObjectPtr, WeakObjectPtr};
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ClassType {
    #[default]
    None,
    Scene,
    Role,
    Npc,
    Item,
    Aide,
    Container,
}

pub trait Entity: Container {
    fn set_ptr(&mut self, self_ptr: ObjectPtr);
    fn set_factory(&mut self, factory: FactoryPtr);
    fn get_factory(&self) -> Option<Rc<RefCell<Factory>>>;
    fn destroy_children(&mut self);
    fn destroy_child(&mut self, child: ObjectPtr);
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

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct EntityInfo {
    pub class_name: &'static str,
    pub uid: u64,
    pub class_type: ClassType,
    pub deleted: bool,
    pub destroying: bool,
    pub attrs: Vec<&'static str>,
    pub index: HashMap<&'static str, u32>,
    pub saves_index: Vec<u32>,
    pub reps_index: Vec<u32>,
    pub saves: Vec<&'static str>,
    pub reps: Vec<&'static str>,
    pub saves_set: HashSet<u32>,
    pub reps_set: HashSet<u32>,
    pub dirty: bool,
    pub modify_attrs: Vec<u32>,
    pub children: Vec<Option<ObjectPtr>>,
    pub cap: usize,
    pub container_pos: usize,
    pub child_num: usize,
    pub parent: Option<WeakObjectPtr>,
    pub factory: Option<FactoryPtr>,
    // 方便获取自己的指针
    pub self_ptr: Option<WeakObjectPtr>,
}

impl Drop for EntityInfo {
    fn drop(&mut self) {
        debug!("drop entity {} uid {}", self.class_name, self.uid);
    }
}

impl EntityInfo {
    pub fn init(
        &mut self,
        class_name: &'static str,
        attrs: Vec<&'static str>,
        saves: Vec<&'static str>,
        reps: Vec<&'static str>,
    ) {
        self.class_name = class_name;
        self.attrs = attrs;
        self.saves = saves;
        self.reps = reps;
        self.attrs.iter().enumerate().for_each(|(i, attr)| {
            self.index.insert(attr, i as u32);
        });
        self.saves.iter().enumerate().for_each(|(_, &attr)| {
            let index = self.index[attr];
            self.saves_index.push(index);
            self.saves_set.insert(index);
        });
        self.reps.iter().enumerate().for_each(|(_, &attr)| {
            let index = self.index[attr];
            self.reps_index.push(index);
            self.reps_set.insert(index);
        });
    }
}

impl Entity for EntityInfo {
    fn set_ptr(&mut self, self_ptr: ObjectPtr) {
        self.self_ptr = Some(Rc::downgrade(&self_ptr));
    }
    fn set_factory(&mut self, factory: FactoryPtr) {
        self.factory = Some(factory)
    }

    fn get_factory(&self) -> Option<Rc<RefCell<Factory>>> {
        if let Some(weak_factory) = &self.factory {
            if let Some(strong_factory) = weak_factory.upgrade() {
                return Some(strong_factory);
            }
        }
        None
    }

    fn destroy_child(&mut self, child: ObjectPtr) {
        if child.borrow().entity_ref().destroying {
            warn!("object already destroying");
            return;
        }
        child.borrow_mut().entity_mut().destroying = true;
        let in_container = child.borrow().entity_ref().is_in_container();
        if in_container {
            self.remove_child(child.clone());
        }

        child.borrow_mut().entity_mut().destroy_children();

        if let Some(f) = self.get_factory() {
            f.borrow_mut().delete(child);
        }
    }

    fn destroy_children(&mut self) {
        let children = self.children.len();
        for i in 0..children {
            if let Some(child) = &self.children[i] {
                self.destroy_child(child.clone());
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
        &self.attrs
    }

    fn save_attrs<'a>(&'a self) -> &'a Vec<&str> {
        &self.saves
    }

    fn rep_attrs<'a>(&'a self) -> &'a Vec<&str> {
        &self.reps
    }

    fn save_attrs_index(&self) -> &Vec<u32> {
        &self.saves_index
    }

    fn rep_attrs_index(&self) -> &Vec<u32> {
        &self.reps_index
    }

    fn get_attr_count(&self) -> u32 {
        self.attrs.len() as u32
    }

    fn get_attr_index(&self, attr: &str) -> Option<u32> {
        match self.index.get(attr) {
            Some(&i) => Some(i),
            None => None,
        }
    }

    fn get_attr_name<'a>(&'a self, index: u32) -> Option<&'a str> {
        match self.attrs.get(index as usize) {
            Some(&attr) => Some(attr),
            None => None,
        }
    }

    fn change_attr(&mut self, index: u32, old: &dyn Any) {
        if self.saves_set.contains(&index) {
            self.dirty = true;
        }
        if self.reps_set.contains(&index) {
            self.modify_attrs.push(index);
        }
        debug!("old:{:?}\n", old);
    }
}

#[allow(dead_code)]
pub struct ObjectInitializer {
    pub name: &'static str,
    pub f: fn() -> ObjectPtr,
}

impl ObjectInitializer {
    pub const fn register_entity(name: &'static str, f: fn() -> ObjectPtr) -> Self {
        Self { name, f }
    }
}

inventory::collect!(ObjectInitializer);

#[derive(Debug)]
pub struct Registry {
    pub entity_vec: Vec<fn() -> ObjectPtr>,
    pub entity_index: HashMap<&'static str, usize>,
}

impl Registry {
    pub fn init() -> Self {
        let mut index_map: HashMap<&'static str, usize> = HashMap::new();
        let mut entity_vec = Vec::new();
        for initializer in inventory::iter::<ObjectInitializer> {
            if index_map.contains_key(initializer.name) {
                panic!("entity {} duplicate", initializer.name);
            }
            let entity_idx = entity_vec.len();
            entity_vec.push(initializer.f);
            index_map.insert(initializer.name, entity_idx);
        }
        Self {
            entity_vec: entity_vec,
            entity_index: index_map,
        }
    }

    pub fn get_class_index(&self, entity: &str) -> Option<usize> {
        if let Some(&idx) = self.entity_index.get(entity) {
            return Some(idx);
        }
        None
    }

    pub fn create_object_by_index(&self, idx: usize) -> Option<ObjectPtr> {
        if idx >= self.entity_vec.len() {
            return None;
        }
        let new_obj = self.entity_vec[idx]();
        new_obj.borrow_mut().entity_mut().set_ptr(new_obj.clone());
        Some(new_obj)
    }

    pub fn create_object(&self, entity: &str) -> Option<ObjectPtr> {
        match self.entity_index.get(entity) {
            Some(&idx) => self.create_object_by_index(idx),
            None => None,
        }
    }
}
