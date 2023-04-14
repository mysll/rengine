use std::ops::DerefMut;

use tracing::debug;

use crate::{
    container::Container, game_model::Model, game_object::GameObject, GameModelPtr, ObjectPtr,
    WeakFactoryPtr, WeakObjectPtr,
};

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

#[derive(Debug)]
pub struct Object {
    pub uid: u64,
    pub class_type: ClassType,
    pub deleted: bool,
    pub destroying: bool,
    pub dirty: bool,
    pub modify_attrs: Vec<u32>,
    pub children: Vec<Option<ObjectPtr>>,
    pub cap: usize,
    pub container_pos: usize,
    pub child_num: usize,
    pub parent: Option<WeakObjectPtr>,
    pub factory: Option<WeakFactoryPtr>,
    // 方便获取自己的指针
    pub self_ptr: Option<WeakObjectPtr>,
    pub game_model: GameModelPtr,
    pub model: Model,
}

impl Drop for Object {
    fn drop(&mut self) {
        debug!("drop entity {} uid {}", self.model.class_name, self.uid);
    }
}

impl Object {
    pub fn new(game_model: GameModelPtr) -> Self {
        let model = game_model.borrow().get_model();
        Self {
            uid: 0,
            class_type: ClassType::None,
            deleted: false,
            destroying: false,
            dirty: false,
            modify_attrs: Vec::new(),
            children: Vec::new(),
            cap: 0,
            container_pos: 0,
            child_num: 0,
            parent: None,
            factory: None,
            self_ptr: None,
            game_model: game_model,
            model: model,
        }
    }

    pub fn new_with_cap(game_model: GameModelPtr, cap: usize) -> Self {
        assert!(cap > 0);
        let model = game_model.borrow().get_model();
        Self {
            uid: 0,
            class_type: ClassType::None,
            deleted: false,
            destroying: false,
            dirty: false,
            modify_attrs: Vec::new(),
            children: Vec::with_capacity(cap),
            cap: cap,
            container_pos: 0,
            child_num: 0,
            parent: None,
            factory: None,
            self_ptr: None,
            game_model: game_model,
            model: model,
        }
    }

    pub fn create(parent: &ObjectPtr, entity: &str, cap: usize, pos: usize) -> Option<ObjectPtr> {
        parent.borrow_mut().create_child(entity, cap, pos)
    }

    pub fn created(object: &ObjectPtr) {
        let obj_ptr = object.borrow_mut().deref_mut() as *mut Object;
        object.borrow().game_model.borrow_mut().set_gameobj(obj_ptr);
    }

    pub fn object_map<F, U>(this: &ObjectPtr, f: F) -> U
    where
        F: FnOnce(&Object) -> U,
    {
        f(&*this.borrow())
    }

    pub fn object_map_mut<F, U>(this: &ObjectPtr, f: F) -> U
    where
        F: FnOnce(&mut Object) -> U,
    {
        f(&mut *this.borrow_mut())
    }

    pub fn model_map<T, F, U>(this: &ObjectPtr, f: F) -> U
    where
        T: 'static,
        F: FnOnce(&T) -> U,
    {
        if let Some(gm) = this
            .borrow()
            .game_model
            .borrow()
            .get_any()
            .downcast_ref::<T>()
        {
            return f(gm);
        }
        panic!("parse failed")
    }

    pub fn model_map_mut<T, F, U>(this: &ObjectPtr, f: F) -> U
    where
        T: 'static,
        F: FnOnce(&mut T) -> U,
    {
        if let Some(gm) = this
            .borrow()
            .game_model
            .borrow_mut()
            .get_mut_any()
            .downcast_mut::<T>()
        {
            return f(gm);
        }
        panic!("parse failed")
    }

    pub fn destroy_object(parent: &ObjectPtr, target: &ObjectPtr) {
        assert!(target.borrow().parent.is_some());
        if Self::check_parent(target, parent) {
            parent.borrow_mut().destroy_child(target);
        }
    }

    pub fn destroy_self(this: &ObjectPtr) {
        assert!(this.borrow().parent.is_some());
        let parent = this.borrow().get_parent();
        if let Some(parent) = parent {
            parent.borrow_mut().destroy_child(this);
        }
    }

    pub fn check_parent(child: &ObjectPtr, parent: &ObjectPtr) -> bool {
        if let Some(parent_ptr) = child.borrow().get_parent() {
            return parent_ptr.as_ptr() == parent.as_ptr();
        }
        return false;
    }
}
