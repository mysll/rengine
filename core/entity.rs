use re_ops::def_entity;

#[def_entity]
struct Player {
    hp:i32,
    #[attr(save = true)]
    name: String,
    #[attr(replicated = true)]
    age: i32,
}
