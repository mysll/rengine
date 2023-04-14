use std::{cell::RefCell, rc::Rc};

use crate::{
    container::Container, factory::Factory, game_object::GameObject, object::Object,
    registry::Registry, FactoryPtr, ObjectPtr,
};

pub struct GameScene {
    pub scene_object: ObjectPtr,
    pub factory: FactoryPtr,
}

impl GameScene {
    pub fn new(scene_class: &str, registry: Rc<Registry>) -> Option<Self> {
        let scene_model = registry.create_object(scene_class);
        if scene_model.is_none() {
            panic!("error");
        }
        let scene = Rc::new(RefCell::new(Object::new(scene_model.unwrap())));
        let factory = Rc::new(RefCell::new(Factory::new(registry, scene.clone())));

        scene.borrow_mut().set_factory(&factory);
        Object::created(&scene);
        factory.borrow_mut().init();

        Some(Self {
            scene_object: scene,
            factory,
        })
    }

    pub fn clear_all(&self) {
        Object::object_map_mut(&self.scene_object, |scene| {
            scene.destroy_children();
        });
        self.factory.borrow_mut().clear_deleted();
    }

    pub fn create_in_scene(&self, entity: &str, cap: usize) -> Option<ObjectPtr> {
        self.scene_object.borrow_mut().create_child(entity, cap, 0)
    }
}
