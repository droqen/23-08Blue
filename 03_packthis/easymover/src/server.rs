use ambient_api::prelude::*;

mod both;
mod usage;

#[main]
pub fn main() {
    easymover_spawning_effect::init();
    easymover_handle_landgoal_change::init();
    usage::server_demo();
    both::init_all();
}

mod easymover_spawning_effect {
    use crate::{packages::this::components::*, vec2_landgoal_ease};
    use ambient_api::prelude::*;
    pub fn init() {
        spawn_query(effect_spawn_emover_at()).bind(|emovers| {
            for (mover, pos) in emovers {
                entity::add_component(mover, emover_landpos(), pos);
                entity::add_component_if_required(mover, emover_movespeed(), 1.0);
                entity::remove_component(mover, effect_spawn_emover_at());
                vec2_landgoal_ease::effect_stop_moving(mover);
            }
        });
    }
}

mod easymover_handle_landgoal_change {

    use crate::packages::this::components::*;
    use ambient_api::prelude::*;
    pub fn init() {
        change_query((emover_landpos(), emover_landgoal(), emover_movespeed()))
            .track_change(emover_landgoal())
            .bind(|emovers| {
                for (mover, (_landpos, goal, speed)) in emovers {
                    crate::vec2_landgoal_ease::effect_goto_target(mover, goal);
                }
            });
        despawn_query(emover_landgoal())
            .requires(emover_landpos())
            .bind(|emovers| {
                for (mover, _) in emovers {
                    crate::vec2_landgoal_ease::effect_stop_moving(mover);
                }
            });
    }
}

mod vec2_landgoal_ease {

    const DEFAULT_MOVESPEED: f32 = 1.0;

    use std::thread::current;

    use crate::packages::{
        ease::components::{
            ease_end_time, ease_start_time, ease_vec2_a, ease_vec2_b, ease_vec2_value,
        },
        this::components::{
            ease_propagate_landpos_to, emover_landpos, emover_movespeed, emover_posdriver_ease,
        },
    };
    use ambient_api::{core::app::components::name, prelude::*};

    pub fn effect_stop_moving(mover: EntityId) {
        let pos = entity::get_component(mover, emover_landpos()).unwrap();
        entity::set_components(
            get_ease(mover),
            Entity::new()
                // .with(ease_vec2_a(), pos)
                .with(ease_vec2_b(), pos)
                .with(ease_start_time(), 0.)
                .with(ease_end_time(), 0.),
        );
    }
    pub fn effect_goto_target(mover: EntityId, b: Vec2) {
        let speed = entity::get_component(mover, emover_movespeed()).unwrap_or(DEFAULT_MOVESPEED);
        let a = entity::get_component(mover, emover_landpos()).unwrap();
        let now = game_time().as_secs_f32();
        let dur = a.distance(b) / speed;
        entity::set_components(
            get_ease(mover),
            Entity::new()
                .with(ease_vec2_a(), a)
                .with(ease_vec2_b(), b)
                .with(ease_start_time(), now)
                .with(ease_end_time(), now + dur)
                .with(ease_propagate_landpos_to(), mover),
        );
    }
    fn get_ease(mover: EntityId) -> EntityId {
        if let Some(ease) = entity::get_component(mover, emover_posdriver_ease()) {
            ease
        } else {
            let starting_pos = entity::get_component(mover, emover_landpos()).unwrap();
            let ease = Entity::new()
                .with(name(), "Ease Vec2".into())
                .with(ease_vec2_a(), starting_pos)
                .with(ease_vec2_b(), starting_pos)
                .with(ease_start_time(), 0.)
                .with(ease_end_time(), 0.)
                .with(ease_propagate_landpos_to(), mover)
                .spawn();
            entity::add_component(mover, emover_posdriver_ease(), ease);
            ease
        }
    }
}
