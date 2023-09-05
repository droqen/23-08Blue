use ambient_api::prelude::*;

mod usage;

#[main]
pub fn main() {
    usage::server_demo();
    handle_effect_blockable_move::init();
}

mod super_cheap_shapecast {
    use ambient_api::prelude::*;

    const CIRCLECAST_ZHEIGHT: f32 = 0.1;

    const INFINITE_NOCOLLISION: f32 = 99999.;

    pub fn circlecast_to_collision(here2: Vec2, there2: Vec2, radius: f32) -> Option<Vec2> {
        let dist: f32 = here2.distance(there2);

        let dir2: Vec2 = (there2 - here2).normalize_or_zero();
        let dir: Vec3 = dir2.extend(0.0);
        let shift_fwd: Vec3 = dir * radius.min(dist);
        let shift_sideways = dir2.perp().normalize_or_zero().extend(0.0) * radius;
        let shift_radius: Vec3 = dir * radius;
        let here: Vec3 = here2.extend(CIRCLECAST_ZHEIGHT);
        let there: Vec3 = there2.extend(CIRCLECAST_ZHEIGHT);

        let closest_hit_dist: Option<f32> = vec![
            (here + shift_fwd, there + shift_radius - here - shift_fwd),
            (here + shift_fwd + shift_sideways, there - here - shift_fwd),
            (here + shift_fwd - shift_sideways, there - here - shift_fwd),
        ]
        .iter()
        .map(move |ray| {
            let hit = physics::raycast_first(ray.0, ray.1);
            match hit {
                None => INFINITE_NOCOLLISION,
                Some(hit) => {
                    (hit.distance * (ray.0.distance(ray.1)) - radius - shift_fwd.length()).max(0.)
                }
            }
        })
        .min_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"));

        match closest_hit_dist {
            None | Some(INFINITE_NOCOLLISION) => None,
            Some(0.) => Some(here2),
            Some(closest_hit_dist) => Some(here2 + dir2 * dist.min(closest_hit_dist)),
        }
    }
}

mod handle_effect_blockable_move {
    use crate::{
        packages::{
            easymover::components::{
                emover_landgoal, emover_landpos, emover_movespeed, emover_posdriver_ease,
            },
            this::components::{effect_blockable_move, emover_blockable_radius},
        },
        super_cheap_shapecast,
    };
    use ambient_api::prelude::*;

    const RAYCAST_ZHEIGHT: f32 = 0.1;

    pub fn init() {
        spawn_query((emover_landpos(), effect_blockable_move()))
            // .requires((emover_posdriver_ease(), emover_movespeed()))
            .bind(|effected_movers| {
                for (mover, (landpos, goal)) in effected_movers {
                    match super_cheap_shapecast::circlecast_to_collision(
                        landpos,
                        goal,
                        entity::get_component(mover, emover_blockable_radius()).unwrap_or(0.50),
                    ) {
                        None => {
                            // move to goal; no collisions found
                            entity::add_component(mover, emover_landgoal(), goal);
                        }
                        Some(modified_goal) => {
                            if modified_goal == landpos {
                                // no movement: you're walking straight into a wall
                                entity::remove_component(mover, emover_landgoal())
                            } else {
                                // move to collision
                                entity::add_component(mover, emover_landgoal(), modified_goal);
                            }
                        }
                    }

                    entity::remove_component(mover, effect_blockable_move());
                }
            });
    }
}
