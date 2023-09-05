use ambient_api::prelude::*;

#[main]
pub fn main() {
    create_obstacles::init();
    players_are_blockable_emovers::init();
    // players_have_player_score::init();
    handle_move_message::init();
    pickups::init();
}

mod create_obstacles {
    use crate::packages::{
        easymover::components::effect_spawn_emover_at,
        easymover_blockable::components::emover_blockable_radius,
    };
    use ambient_api::{
        core::{
            physics::components::{cube_collider, sphere_collider},
            primitives::{
                components::{cube, sphere},
                concepts::make_sphere,
            },
            transform::{
                components::{scale, translation},
                concepts::make_transformable,
            },
        },
        prelude::*,
    };
    pub fn init() {
        Entity::new()
            .with(cube(), ())
            .with(cube_collider(), vec3(1.0, 1.0, 1.0))
            .with_merge(make_transformable())
            .with(translation(), vec3(0., 0., 0.5))
            .spawn();
    }
}

mod players_are_blockable_emovers {
    use crate::packages::{
        easymover::components::effect_spawn_emover_at,
        easymover_blockable::components::emover_blockable_radius,
    };
    use ambient_api::{core::player::components::is_player, prelude::*};
    pub fn init() {
        spawn_query(()).requires(is_player()).bind(|plrs| {
            for (plr, _) in plrs {
                entity::add_components(
                    plr,
                    Entity::new()
                        .with(effect_spawn_emover_at(), random::<Vec2>() * 5. - 4.)
                        .with(emover_blockable_radius(), 0.25),
                );
            }
        });
    }
}

mod handle_move_message {
    use crate::packages::{
        easymover_blockable::components::effect_blockable_move, this::messages::MovePlayerBlockable,
    };
    use ambient_api::prelude::*;
    pub fn init() {
        MovePlayerBlockable::subscribe(|src, msg| {
            if let Some(plr) = src.client_entity_id() {
                entity::add_component(plr, effect_blockable_move(), msg.pos2);
            }
        });
    }
}

// mod players_have_player_score {
//     use crate::packages::player_score_display::components::player_score;
//     use ambient_api::{
//         core::{app::components::name, player::components::is_player},
//         prelude::*,
//     };
//     pub fn init() {
//         spawn_query(()).requires(is_player()).bind(|plrs| {
//             for (plr, _) in plrs {
//                 entity::add_component(plr, name(), "Score Haver".into());
//                 entity::add_component(plr, player_score(), 13);
//             }
//         });
//     }
// }

mod pickups {
    use crate::packages::{
        easymover::components::emover_landpos, player_score_display::components::player_score,
        this::components::is_pickup,
    };
    use ambient_api::{
        core::{
            primitives::components::cube,
            transform::{
                components::{rotation, scale, translation},
                concepts::make_transformable,
            },
        },
        prelude::*,
    };
    pub fn init() {
        // spawn pickups
        for x in -5..5 {
            for y in -5..5 {
                Entity::new()
                    .with_merge(make_transformable())
                    .with(translation(), vec3(x as f32, y as f32, 0.0))
                    .with(rotation(), Quat::from_rotation_z(0.8))
                    .with(scale(), Vec3::splat(0.15))
                    .with(is_pickup(), ())
                    .with(cube(), ())
                    .spawn();
            }
        }

        // let players pick em up:

        let find_pickups = query(translation()).requires(is_pickup()).build();
        query(emover_landpos()).each_frame(move |movers| {
            let pickups = find_pickups.evaluate();
            for (mover, moverpos2) in movers {
                for (pickup, pickpos3) in &pickups {
                    if moverpos2.distance(pickpos3.truncate()) < 0.25 {
                        entity::despawn(*pickup);
                        entity::mutate_component_with_default(mover, player_score(), 1, |score| {
                            *score = *score + 1
                        });
                    }
                }
            }
        });
    }
}
