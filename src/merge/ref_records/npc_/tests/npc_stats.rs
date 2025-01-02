use super::{assert_eq, *};

mod complex;
mod debug;
mod log;

mod health_magicka {
    use super::{assert_eq, *};
    test_basic!(npc_, Npc, values_some_npc_data:data:stats);
}

mod skills_attributes {
    use super::{assert_eq, *};
    test_basic!(npc_, Npc, values_some_npc_data_skills_attributes:data:stats);
}
