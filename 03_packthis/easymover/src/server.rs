use ambient_api::prelude::*;

mod both;
mod usage;

#[main]
pub fn main() {
    easymover_create_ease::init();
    easymover_alter_ease_on_goal_change::init();
    usage::server_demo();
    both::init_all();
}

mod easymover_create_ease {
    use crate::packages::{ease::components::*, this::components::*};
    use ambient_api::prelude::*;
    pub fn init() {
        spawn_query(effect_spawn_emover_at()).bind(|emovers| {
            for (mover, pos) in emovers {
                let ease = crate::vec2_point_to_point::make(pos + vec2(0.1, 0.0), pos, 1.0).spawn();
                entity::add_component(mover, emover_ease(), ease);
                entity::remove_component(mover, effect_spawn_emover_at());
            }
        });
    }
}

mod easymover_alter_ease_on_goal_change {

    const EASYMOVER_SPEED: f32 = 1.0;

    use crate::{
        packages::{ease::components::*, this::components::*},
        vec2_point_to_point,
    };
    use ambient_api::prelude::*;
    pub fn init() {
        change_query((emover_ease(), emover_landgoal(), emover_movespeed()))
            .track_change(emover_landgoal())
            .bind(|emovers| {
                for (mover, (ease, goal, speed)) in emovers {
                    crate::vec2_point_to_point::effect_ease_goto(ease, goal, speed);
                }
            });
        spawn_query(effect_spawn_emover_at()).bind(|emovers| {
            for (mover, pos) in emovers {
                let ease = crate::vec2_point_to_point::make_still(pos).spawn();
                entity::add_component(mover, emover_ease(), ease);
                entity::add_component(mover, emover_landgoal(), pos);
                entity::add_component_if_required(mover, emover_movespeed(), 1.0);
                entity::remove_component(mover, effect_spawn_emover_at());
            }
        });
    }
}

mod vec2_point_to_point {
    use crate::packages::ease::components::{
        ease_end_time, ease_start_time, ease_vec2_a, ease_vec2_b, ease_vec2_value,
    };
    use ambient_api::{core::app::components::name, prelude::*};
    pub fn make_still(ab: Vec2) -> Entity {
        Entity::new()
            .with(name(), "Ease Vec2".into())
            .with(ease_vec2_a(), ab)
            .with(ease_vec2_b(), ab)
            .with(ease_start_time(), 0.0)
            .with(ease_end_time(), 0.0)
    }
    pub fn make(a: Vec2, b: Vec2, speed: f32) -> Entity {
        let start = game_time().as_secs_f32();
        let dur = a.distance(b) / speed;
        Entity::new()
            .with(name(), "Ease Vec2".into())
            .with(ease_vec2_a(), a)
            .with(ease_vec2_b(), b)
            .with(ease_start_time(), start)
            .with(ease_end_time(), start + dur.max(0.01))
    }
    pub fn effect_ease_goto(ease: EntityId, goto_target: Vec2, speed: f32) {
        let here = match entity::get_component(ease, ease_vec2_value()) {
            None => Vec2::ZERO,
            Some(pos) => pos,
        };
        entity::add_components(ease, make(here, goto_target, speed));
    }
}
