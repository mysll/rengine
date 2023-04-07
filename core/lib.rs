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

    use std::rc::Rc;

    use re_entity::container::Container;
    use re_entity::entity::GameEntity;
    use re_entity::entity::Registry;
    use re_entity::object::GameObject;
    use re_entity::scene::GameScene;
    use re_ops::def_entity;
    use time::macros::format_description;
    use tracing::info;
    use tracing_subscriber::fmt::time::LocalTime;
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::FmtSubscriber;

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

    #[test]
    fn test() {
        let subscriber = FmtSubscriber::builder()
            .with_env_filter(EnvFilter::new("debug"))
            .with_timer(LocalTime::new(format_description!(
                "[hour]:[minute]:[second].[subsecond digits:3]"
            )))
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        let registry = Rc::new(Registry::init());
        let scene = GameScene::new(TestScene::ClassName(), registry.clone()).unwrap();
        {
            let object = scene
                .scene_object
                .borrow_mut()
                .entity_mut()
                .create_child(TestPlayer::ClassName(), 1)
                .unwrap();

            let player = object.borrow();
            let uid = player.entity_ref().uid();
            info!("{}", uid);
            let new_obj = scene.factory.borrow_mut().find(uid);
            info!("{:?}", new_obj);
        }

        let obj2 = scene
            .scene_object
            .borrow_mut()
            .entity_mut()
            .create_child(TestPlayer::ClassName(), 2)
            .unwrap();
        {
            let go = GameObject::new(obj2.clone());
            let entity = go.get_entity();
            println!("{}", entity.class_name);
        }
        GameObject::destroy_self(obj2);
        scene.clear_all();
        let GameScene {
            scene_object,
            factory,
        } = scene;

        drop(scene_object);
        drop(factory);
    }
}
