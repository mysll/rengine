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
