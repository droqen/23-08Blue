use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        player::components::{is_player, user_id},
        primitives::components::quad,
        transform::{
            components::{lookat_target, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

use packages::slowmover_ptt::{
    components::{mover_pos_end, mover_pos_start, mover_time_start},
    concepts::make_slowmover,
};

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with(main_scene(), ())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with(quad(), ())
        .spawn();

    spawn_query((is_player(), user_id())).bind(|players| {
        for (player, (_, uid)) in players {
            entity::add_components(player, make_slowmover());
        }
    });

    println!("Hello, Ambient!");
    move_message::setup();
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
