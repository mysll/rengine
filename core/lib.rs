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

    use std::collections::HashMap;

    use re_ops::def_entity;

    #[def_entity]
    struct Player {
        #[attr(save = true)]
        name: String,
        #[attr(replicated = true)]
        age: i32,
    }

    #[test]
    fn test() {
        let mut p = Player::new();
        p.set_name("hello".to_string());
        println!("{:?}", p.get_attrs());
        println!("{:?}", p.get_attr_by_name("name").unwrap());
        println!("{}", p.get_attr_index("name").unwrap());
        println!("{}", p.get_attr_name(1).unwrap());
        println!("{:?}", p.save_attrs());
        println!("{:?}", p.save_attrs_index());
        println!("{:?}", p.rep_attrs());
        println!("{:?}", p.rep_attrs_index());
    }

    #[test]
    fn test2() {
        let v: Vec<&str> = vec!["hello", "world"];
        let mut v1: HashMap<&str, u32> = HashMap::new();
        v.iter().enumerate().for_each(|(i, e)| {
            v1.insert(e, i as u32);
        });

        let mut v2: Vec<u32> = Vec::new();

        v.iter().enumerate().for_each(|(_, attr)| v2.push(v1[attr]));
        println!("{:?}", v2);
    }
}
