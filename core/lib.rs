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
    use re_entity::factory::Factory;
    use re_ops::def_entity;

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
        let mut factory = Factory::new();
        let object = factory.create(TestPlayer::ClassName()).unwrap();
        {
            let player = object.borrow();
            let uid = player.entity_ref().uid();
            println!("{}", uid);
            let new_obj = factory.find(uid);
            println!("{:?}", new_obj);
        }
        factory.delete(object);
        println!("drop");
        factory.clear_deleted();

        let object = factory.create(TestPlayer::ClassName()).unwrap();
        factory.destroy(object);
        println!("after drop");
    }
}
