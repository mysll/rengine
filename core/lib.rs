mod connection;
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

    use re_object::{
        game_object::GameObject, game_scene::GameScene, object::Object, registry::Registry,
    };
    use re_ops::def_entity;
    use time::macros::format_description;
    use tracing_subscriber::{fmt::time::LocalTime, EnvFilter, FmtSubscriber};

    #[def_entity]
    struct TestScene {
        #[attr()]
        name: &'static str,
    }

    #[def_entity]
    struct TestPlayer {
        hp: i32,
        #[attr(save, replicated)]
        name: String,
        #[attr(replicated)]
        age: i32,
    }

    #[def_entity]
    struct TestBox {
        #[attr(save, replicated)]
        name: String,
    }

    #[def_entity]
    struct TestItem {
        #[attr(save, replicated)]
        name: String,
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
        Object::model_map(&scene.scene_object, |scene: &TestScene| {
            println!("{:?}", scene.__go.0);
        });
        Object::model_map_mut(&scene.scene_object, |scene: &mut TestScene| {
            scene.set_name("test");
            println!("{:?}", scene.name);
        });
        {
            let player = scene.create_in_scene(TestPlayer::ClassName(), 0).unwrap();
            println!("{:?}", player);
            let item_box = Object::create(&player, TestBox::ClassName(), 1, 0).unwrap();
            Object::create(&item_box, TestItem::ClassName(), 0, 0);
            Object::create(&item_box, TestItem::ClassName(), 0, 0);
        }

        scene.clear_all();
        let GameScene {
            scene_object,
            factory,
        } = scene;

        drop(scene_object);
        drop(factory);
    }
}
