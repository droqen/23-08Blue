use ambient_api::prelude::*;

mod usage;

#[main]
pub fn main() {
    usage::server_demo();
    handle_effect_goto_vec2::init();
}

mod handle_effect_goto_vec2 {
    use crate::packages::{
        easymover::{
            components::{emover_ease, emover_landgoal, emover_movespeed},
            messages::ConfigureRemoveDefaultLandgoalHandling,
        },
        this::components::effect_goto_vec2,
    };
    use ambient_api::prelude::*;
    pub fn init() {
        spawn_query(effect_goto_vec2()).bind(|movers| {
            for (mover, goal) in movers {
                if entity::has_components(mover, &[&emover_ease(), &emover_movespeed()]) {
                    entity::add_component(mover, emover_landgoal(), goal);
                } else {
                    println!("Used effect_goto_vec2 on invalid entity");
                }
                entity::remove_component(mover, effect_goto_vec2());
            }
        });
    }
}
