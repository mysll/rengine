use std::collections::HashMap;

use crate::GameModelPtr;

#[allow(dead_code)]
pub struct ObjectInitializer {
    pub name: &'static str,
    pub f: fn() -> GameModelPtr,
}

impl ObjectInitializer {
    pub const fn register_entity(name: &'static str, f: fn() -> GameModelPtr) -> Self {
        Self { name, f }
    }
}

inventory::collect!(ObjectInitializer);

#[derive(Debug)]
pub struct Registry {
    pub entity_vec: Vec<fn() -> GameModelPtr>,
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

    pub fn create_object_by_index(&self, idx: usize) -> Option<GameModelPtr> {
        if idx >= self.entity_vec.len() {
            return None;
        }
        let new_obj = self.entity_vec[idx]();
        Some(new_obj)
    }

    pub fn create_object(&self, entity: &str) -> Option<GameModelPtr> {
        match self.entity_index.get(entity) {
            Some(&idx) => self.create_object_by_index(idx),
            None => None,
        }
    }
}
