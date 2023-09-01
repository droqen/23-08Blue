use ambient_api::prelude::*;

mod usage;

#[main]
pub fn main() {
    usage::server_demo();
    handle_effect_goto_vec2::init();
}

mod handle_effect_goto_vec2 {
    use crate::packages::{
        easymover::components::{
            emover_landgoal, emover_landpos, emover_movespeed, emover_posdriver_ease,
        },
        this::components::{effect_goto_vec2, emover_blockable_radius},
    };
    use ambient_api::prelude::*;

    const RAYCAST_ZHEIGHT: f32 = 0.1;

    pub fn init() {
        spawn_query((emover_landpos(), effect_goto_vec2()))
            // .requires((emover_posdriver_ease(), emover_movespeed()))
            .bind(|movers| {
                for (mover, (landpos, goal)) in movers {
                    let to_goal: Vec2 = goal - landpos;
                    if let Some(dir_to_goal) = to_goal.try_normalize() {
                        let dist_to_goal: f32 = to_goal.length();
                        let mover_radius: f32 =
                            entity::get_component(mover, emover_blockable_radius())
                                .unwrap_or_default(); // 0
                        if let Some(hit) = physics::raycast_first(
                            landpos.extend(RAYCAST_ZHEIGHT),
                            to_goal.extend(RAYCAST_ZHEIGHT) * (dist_to_goal + mover_radius),
                        ) {
                            // collision
                            if hit.distance > mover_radius {
                                entity::add_component(
                                    mover,
                                    emover_landgoal(),
                                    landpos * dist_to_goal.min(hit.distance - mover_radius),
                                );
                            } else {
                                // stop movin - have no goal
                                // todo: 'bump' animation ?
                                entity::remove_component(mover, emover_landgoal());
                            }
                        } else {
                            // no collision - just go on ahead
                            entity::add_component(mover, emover_landgoal(), goal);
                        }
                    } else {
                        // stop movin - have no goal
                        entity::remove_component(mover, emover_landgoal());
                    }
                    entity::remove_component(mover, effect_goto_vec2());
                }
            });
    }
}
