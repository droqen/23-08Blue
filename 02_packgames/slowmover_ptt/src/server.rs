use ambient_api::prelude::*;

#[main]
pub fn main() {
    move_message::setup();
    cube_maze_spawning::setup();
    floor_spawning::setup();
    player_features::setup();
}

mod player_features {
    use crate::packages::slowmover_ptt::concepts::make_slowmover;
    use ambient_api::{
        core::player::components::{is_player, user_id},
        prelude::*,
    };
    pub fn setup() {
        spawn_query((is_player(), user_id())).bind(|players| {
            for (player, (_, uid)) in players {
                entity::add_components(player, make_slowmover());
            }
        });
    }
}

mod floor_spawning {
    use ambient_api::{
        core::{
            primitives::components::quad,
            rendering::components::color,
            transform::{components::scale, concepts::make_transformable},
        },
        prelude::*,
    };
    pub fn setup() {
        Entity::new()
            .with_merge(make_transformable())
            .with(quad(), ())
            .with(color(), vec4(0.5, 0.5, 0.5, 1.0))
            .with(scale(), Vec3::splat(20.0))
            .spawn();
    }
}

mod mover_prediction;

mod move_message {
    use crate::{
        mover_prediction,
        packages::{
            slowmover_ptt::components::{mover_pos_end, mover_pos_start, mover_time_start},
            slowmover_ptt::messages::PlayerMove,
        },
    };
    use ambient_api::prelude::*;
    pub fn setup() {
        println!("PlayerMove setup");
        PlayerMove::subscribe(|src, msg| {
            if let Some(player_mover) = src.client_entity_id() {
                let start = mover_prediction::get_mover_pos2(player_mover, 0.0);
                let target = msg.target;

                // use raycast to determine where the actual end-of-movement is
                let end = target;

                println!("PlayerMove received");

                entity::add_components(
                    player_mover,
                    Entity::new()
                        .with(mover_pos_start(), start)
                        .with(mover_pos_end(), end)
                        .with(mover_time_start(), game_time().as_secs_f32()),
                );
            } else {
                println!("PlayerMove received, but no player found");
                // error - no player found there
            }
        });
    }
}

mod cube_maze_spawning {
    use ambient_api::{
        core::{
            physics::components::cube_collider, primitives::components::cube,
            transform::components::translation,
        },
        prelude::*,
    };
    pub fn setup() {
        // spawn maze
        for x in -10..10 {
            for y in -10..10 {
                if random::<f32>() < 0.5 && ((x as i32).abs() + (y as i32).abs() > 2) {
                    Entity::new()
                        .with(cube(), ())
                        .with(cube_collider(), Vec3::splat(1.))
                        .with(translation(), ivec2(x, y).as_vec2().extend(0.5))
                        .spawn();
                }
            }
        }
    }
}
