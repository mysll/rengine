use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use crate::object::Object;

pub trait GameModel: Debug {
    fn get_model(&self) -> Model;
    fn set_gameobj(&mut self, go: *mut Object);
    fn get_attr_by_name<'a>(&'a self, attr: &str) -> Option<&'a dyn Any>;
    fn set_attr_by_name(&mut self, attr: &str, val: &dyn Any) -> bool;
    fn get_attr_by_index<'a>(&'a self, index: u32) -> Option<&'a dyn Any>;
    fn set_attr_by_index(&mut self, index: u32, val: &dyn Any) -> bool;
    fn get_any<'a>(&'a self) -> &'a dyn Any;
    fn get_mut_any<'a>(&'a mut self) -> &'a mut dyn Any;
}

#[derive(Default, Debug, Clone)]
pub struct Model {
    pub class_name: &'static str,
    pub attrs: Vec<&'static str>,
    pub index: HashMap<&'static str, u32>,
    pub saves_index: Vec<u32>,
    pub reps_index: Vec<u32>,
    pub saves: Vec<&'static str>,
    pub reps: Vec<&'static str>,
    pub saves_set: HashSet<u32>,
    pub reps_set: HashSet<u32>,
}

impl Model {
    pub fn new(
        class_name: &'static str,
        attrs: Vec<&'static str>,
        saves: Vec<&'static str>,
        reps: Vec<&'static str>,
    ) -> Self {
        let mut index = HashMap::new();
        attrs.iter().enumerate().for_each(|(i, &attr)| {
            index.insert(attr, i as u32);
        });
        let mut saves_index = Vec::new();
        let mut reps_index = Vec::new();
        let mut saves_set = HashSet::new();
        let mut reps_set = HashSet::new();
        saves.iter().enumerate().for_each(|(_, &attr)| {
            let idx = index[attr];
            saves_index.push(idx);
            saves_set.insert(idx);
        });
        reps.iter().enumerate().for_each(|(_, &attr)| {
            let idx = index[attr];
            reps_index.push(idx);
            reps_set.insert(idx);
        });

        Self {
            class_name,
            attrs: attrs,
            index: index,
            saves_index,
            reps_index,
            saves,
            reps,
            saves_set,
            reps_set,
        }
    }
}
