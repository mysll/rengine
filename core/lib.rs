mod connection;
mod entity;
mod package;

pub const MAX_LEN: usize = 64 * 1024;

pub mod core;
pub mod macros;
pub mod options;
pub mod runtime;
pub mod shutdown;
pub mod tcp_server;
pub mod tokio_util;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use re_entity::entity::Entity;
    use re_entity::{entity::Registry, factory::Factory};
    use re_ops::def_entity;

    #[def_entity]
    struct TestScene {}

    #[def_entity]
    struct TestPlayer {
        hp: i32,
        #[attr(save, replicated)]
        name: String,
        #[attr(replicated)]
        age: i32,
    }

    impl Drop for TestPlayer {
        fn drop(&mut self) {
            println!("Dropping!");
        }
    }

    #[test]
    fn test() {
        let registry = Registry::init();
        let scene = registry.create_object(TestScene::ClassName()).unwrap();
        let factory = Rc::new(RefCell::new(Factory::new(registry, scene.clone())));
        scene.borrow_mut().entity_mut().set_factory(factory.clone());
        factory.borrow_mut().init();
        let object = factory
            .borrow_mut()
            .create(TestPlayer::ClassName())
            .unwrap();

        {
            let player = object.borrow();
            let uid = player.entity_ref().uid();
            println!("{}", uid);
            let new_obj = factory.borrow_mut().find(uid);
            println!("{:?}", new_obj);
        }
        factory.borrow_mut().delete(object);
        println!("drop");
        factory.borrow_mut().clear_deleted();

        let object = factory
            .borrow_mut()
            .create(TestPlayer::ClassName())
            .unwrap();
        factory.borrow_mut().destroy(object);
        println!("after drop");
    }
}
