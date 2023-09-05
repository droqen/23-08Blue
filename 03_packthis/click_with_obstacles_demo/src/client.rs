use ambient_api::prelude::*;

#[main]
pub fn main() {
    configure_clicky_camera::init();
    packages::clicks_auto::messages::ConfigureZPlane { z: 0.0 }.send_local_broadcast(false);
    decorate_sprite_as_sphere::init();
    on_click_send_move_message::init();
    esprites_have_score_and_score_position::init();
}

mod configure_clicky_camera {
    use crate::packages::{
        clicks_auto::messages::ConfigureCamera, instant_camera::components::is_instant_camera,
    };
    use ambient_api::prelude::*;

    pub fn init() {
        let find_camera = query(()).requires(is_instant_camera()).build();
        run_async(async move {
            let mut camera: Option<EntityId> = None;

            for _ in 0..100 {
                println!("Waiting for camera to spawn...");
                sleep(0.05).await; // wait until camera is spawned;
                if let Some(try_camera) = find_camera.evaluate().first() {
                    camera = Some(try_camera.0);
                    break;
                }
            }

            if camera.is_none() {
                panic!("Timed out waiting for camera to spawn");
            }

            println!("I have a camera: {:?}", camera.unwrap());

            ConfigureCamera {
                camera: camera.unwrap(),
            }
            .send_local_broadcast(false);
        });
    }
}

mod decorate_sprite_as_sphere {
    use crate::packages::easymover::components::*;
    use ambient_api::{
        core::{
            primitives::concepts::make_sphere,
            transform::{components::translation, concepts::make_transformable},
        },
        prelude::*,
    };

    pub fn init() {
        spawn_query(esprite_landpos()).bind(|esprites| {
            for (sprite, landpos) in esprites {
                entity::add_components(
                    sprite,
                    make_transformable()
                        .with_merge(make_sphere())
                        .with(translation(), landpos.extend(0.5)),
                );
            }
        });
        query(esprite_landpos()).each_frame(|esprites| {
            for (sprite, landpos) in esprites {
                entity::add_component(sprite, translation(), landpos.extend(0.5));
            }
        });
    }
}

mod on_click_send_move_message {
    use crate::packages::{
        clicks_auto::components::{click_touch_id, click_world_projected},
        this::messages::MovePlayerBlockable,
    };
    use ambient_api::prelude::*;
    pub fn init() {
        spawn_query((click_touch_id(), click_world_projected())).bind(move |clicks| {
            for (click, (button, pos)) in clicks {
                // button == 0 means LMB
                if button == 0 {
                    MovePlayerBlockable {
                        pos2: pos.truncate(),
                    }
                    .send_server_reliable();
                }
            }
        });
    }
}

mod esprites_have_score_and_score_position {
    use crate::packages::{
        easymover::components::esprite_mover,
        player_score_display::components::{player_score, player_score_position},
    };
    use ambient_api::{core::transform::components::translation, prelude::*};
    pub fn init() {
        query((translation(), esprite_mover())).each_frame(|sprites| {
            for (sprite, (pos, mover)) in sprites {
                if let Some(mover_score) = entity::get_component(mover, player_score()) {
                    entity::add_component(sprite, player_score(), mover_score);
                    entity::add_component(sprite, player_score_position(), pos);
                } else {
                    println!("Parent mover does not have player_score component (it is expected)");
                }
            }
        });
    }
}
