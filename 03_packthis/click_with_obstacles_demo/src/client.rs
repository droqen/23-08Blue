use ambient_api::prelude::*;

#[main]
pub fn main() {
    configure_clicky_camera::init();
    packages::clicks_auto::messages::ConfigureZPlane { z: 0.0 }.send_local_broadcast(false);
    decorate_sprite_as_sphere::init();
    scale_sprite_by_player_level::init();
    on_click_send_move_message::init();
    esprites_have_score_and_score_position::init();
    black_ground::init();
}

mod black_ground {
    use ambient_api::{
        core::{
            primitives::components::quad,
            rendering::components::color,
            transform::{components::scale, concepts::make_transformable},
        },
        prelude::*,
    };

    pub fn init() {
        Entity::new()
            .with_merge(make_transformable())
            .with(scale(), Vec3::splat(1000.0))
            .with(color(), vec4(0., 0., 0., 1.))
            .with(quad(), ())
            .spawn();
    }
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
            transform::{
                components::{scale, translation},
                concepts::make_transformable,
            },
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
                        .with(translation(), landpos.extend(0.))
                        .with(scale(), Vec3::splat(0.2)),
                );
            }
        });
        query(esprite_landpos()).each_frame(|esprites| {
            for (sprite, landpos) in esprites {
                entity::add_component(sprite, translation(), landpos.extend(0.));
            }
        });
    }
}

mod scale_sprite_by_player_level {
    use ambient_api::{core::transform::components::scale, prelude::*};

    use crate::packages::{easymover::components::esprite_mover, this::components::player_level};

    pub fn init() {
        query(esprite_mover()).each_frame(|sprites| {
            for (sprite, mover) in sprites {
                let desired_scale = match entity::get_component(mover, player_level()).unwrap_or(0)
                {
                    0 => 0.2,
                    1 => 0.5,
                    2 | _ => 1.0,
                };
                entity::mutate_component(sprite, scale(), move |scale| {
                    *scale = scale.lerp(Vec3::splat(desired_scale), 0.1)
                });
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
