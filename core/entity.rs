use re_ops::def_entity;

#[def_entity]
struct Player {
    hp:i32,
    #[attr(save, replicated)]
    name: String,
    #[attr(replicated)]
    age: i32,
}
