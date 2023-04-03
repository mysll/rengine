use std::{
    any::Any,
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Debug,
    rc::Rc,
};

use crate::{container::Container, factory::Factory, ObjectPtr, FactoryPtr};
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

pub trait Object: Debug + Any {
    fn entity_ref<'a>(&'a self) -> &'a dyn Entity;
    fn entity_mut<'a>(&'a mut self) -> &'a mut dyn Entity;
    fn get_attr_by_name<'a>(&'a self, attr: &str) -> Option<&'a dyn Any>;
    fn set_attr_by_name(&mut self, attr: &str, val: &dyn Any) -> bool;
    fn get_attr_by_index<'a>(&'a self, index: u32) -> Option<&'a dyn Any>;
    fn set_attr_by_index(&mut self, index: u32, val: &dyn Any) -> bool;
}

pub trait Entity: Container {
    fn set_factory(&mut self, factory: FactoryPtr);
    fn get_factory(&self) -> Option<FactoryPtr>;
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
    pub uid: u64,
    pub class_type: ClassType,
    pub delete: bool,
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
    pub childs: Vec<Option<ObjectPtr>>,
    pub cap: usize,
    pub container_pos: usize,
    pub child_num: usize,
    pub parent: Option<ObjectPtr>,
    pub factory: Option<Rc<RefCell<Factory>>>,
}

impl Drop for EntityInfo {
    fn drop(&mut self) {
        println!("drop entity");
    }
}
impl EntityInfo {
    pub fn init(
        &mut self,
        attrs: Vec<&'static str>,
        saves: Vec<&'static str>,
        reps: Vec<&'static str>,
    ) {
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
    fn set_factory(&mut self, factory: FactoryPtr) {
        self.factory = Some(factory)
    }
    fn get_factory(&self) -> Option<FactoryPtr> {
        match &self.factory {
            Some(f) => Some(f.clone()),
            None => None,
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
        self.delete
    }
    fn delete(&mut self) {
        self.delete = true;
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
        print!("old:{:?}\n", old);
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
    pub entity_map: HashMap<&'static str, fn() -> ObjectPtr>,
}

impl Registry {
    pub fn init() -> Self {
        let mut map: HashMap<&'static str, fn() -> ObjectPtr> = HashMap::new();
        for initializer in inventory::iter::<ObjectInitializer> {
            if map.contains_key(initializer.name) {
                panic!("entity {} duplicate", initializer.name);
            }
            map.insert(initializer.name, initializer.f);
        }
        Self { entity_map: map }
    }

    pub fn create_object(&self, entity: &str) -> Option<ObjectPtr> {
        match self.entity_map.get(entity) {
            Some(&f) => Some(f()),
            None => None,
        }
    }
}
