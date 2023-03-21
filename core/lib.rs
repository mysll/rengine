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

    use re_entity::entity::Registry;
    use re_ops::def_entity;

    #[def_entity]
    struct TestPlayer {
        #[attr(save)]
        name: String,
        #[attr(replicated)]
        age: i32,
        #[attr(save, replicated)]
        hp: i32,
        #[attr(save, replicated)]
        mp: i32,
    }

    #[test]
    fn test() {
        let r = Registry::init();
        let mut player = r.create::<TestPlayer>("TestPlayer").unwrap();
        let p = player.as_mut();
        p.set_name("hello".to_string());
        println!("{:?}", p.get_attrs());
        println!("{:?}", p.get_attr_by_name("name").unwrap());
        println!("{}", p.get_attr_index("name").unwrap());
        println!("{}", p.get_attr_name(1).unwrap());
        println!("{:?}", p.save_attrs());
        println!("{:?}", p.save_attrs_index());
        println!("{:?}", p.rep_attrs());
        println!("{:?}", p.rep_attrs_index());
        println!("{:?}", p.dirty());
        println!("{:?}", p.modify());
        p.clear_dirty();
        p.set_attr_by_index(2, &30i32);
        p.set_attr_by_name("hp", &100i32);
        p.set_mp(100);
        println!("{:?}", p.dirty());
        println!("{:?}", p.modify());
        println!("{:?}", p.get_modify());
        println!("{:?}", p.get_name());
        println!("{:?}", p.get_hp());
    }
}
