use std::{cell::RefCell, rc::Rc};

use crate::{entity::{Registry, Entity}, factory::Factory, ObjectPtr};

pub struct GameScene {
    pub scene_object: ObjectPtr,
    pub factory: Rc<RefCell<Factory>>,
}

impl GameScene {
    pub fn new(scene_class: &str, registry: Rc<Registry>) -> Option<Self> {
        if let Some(scene) = registry.create_object(scene_class) {
            let factory = Rc::new(RefCell::new(Factory::new(registry, scene.clone())));
            scene
                .borrow_mut()
                .entity_mut()
                .set_factory(Rc::downgrade(&factory));
            factory.borrow_mut().init();
            return Some(Self {
                scene_object: scene,
                factory,
            });
        }
        None
    }

    pub fn clear_all(&self) {
        let mut scene = self.scene_object.borrow_mut();
        scene.entity_mut().destroy_children();
        self.factory.borrow_mut().clear_deleted();
    }
}
