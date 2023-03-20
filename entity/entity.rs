use std::any::Any;

pub trait Object {
    fn get_attrs<'a>(&'a self) -> &'a Vec<&str>;
    fn save_attrs<'a>(&'a self) -> &'a Vec<&str>;
    fn rep_attrs<'a>(&'a self) -> &'a Vec<&str>;
    fn save_attrs_index(&self) -> &Vec<u32>;
    fn rep_attrs_index(&self) -> &Vec<u32>;
    fn get_attr_count(&self) -> u32;
    fn get_attr_by_name<'a>(&'a self, attr: &str) -> Option<&'a dyn Any>;
    fn set_attr_by_name(&mut self, attr: &str, val: &dyn Any) -> bool;
    fn get_attr_name<'a>(&'a self, index: u32) -> Option<&'a str>;
    fn get_attr_index(&self, attr: &str) -> Option<u32>;
    fn get_attr_by_index<'a>(&'a self, index: u32) -> Option<&'a dyn Any>;
    fn set_attr_by_index(&mut self, index: u32, val: &dyn Any) -> bool;
}

#[allow(dead_code)]
#[derive(Default)]
pub struct EntityInfo {
    pub attrs: Vec<&'static str>,
    pub index: std::collections::HashMap<&'static str, u32>,
    pub saves_index: Vec<u32>,
    pub reps_index: Vec<u32>,
    pub saves: Vec<&'static str>,
    pub reps: Vec<&'static str>,
    pub dirty: bool,
    pub modify_attrs: Vec<u32>,
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
        self.saves
            .iter()
            .enumerate()
            .for_each(|(_, attr)| self.saves_index.push(self.index[attr]));
        self.reps
            .iter()
            .enumerate()
            .for_each(|(_, attr)| self.reps_index.push(self.index[attr]));
    }

    pub fn attr_count(&self) -> u32 {
        self.attrs.len() as u32
    }
    pub fn get_attr_index(&self, attr: &str) -> Option<u32> {
        match self.index.get(attr) {
            Some(&i) => Some(i),
            None => None,
        }
    }
    pub fn get_attr_name<'a>(&'a self, index: u32) -> Option<&'a str> {
        match self.attrs.get(index as usize) {
            Some(&attr) => Some(attr),
            None => None,
        }
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
    pub fn modify(&self) -> bool {
        self.modify_attrs.len() > 0
    }
    pub fn get_modify<'a>(&'a self) -> &'a Vec<u32> {
        &self.modify_attrs
    }
    pub fn clear_modify(&mut self) {
        self.modify_attrs.clear();
    }
}
