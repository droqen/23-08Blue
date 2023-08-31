use ambient_api::prelude::*;

#[main]
pub async fn main() {
    let find_camera = query(())
        .requires(packages::instant_camera::components::is_instant_camera())
        .build();
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

    packages::clicks_auto::messages::ConfigureCamera {
        camera: camera.unwrap(),
    }
    .send_local_broadcast(false);

    packages::clicks_auto::messages::ConfigureZPlane { z: 0.0 }.send_local_broadcast(false);

    spawn_query((
        packages::clicks_auto::components::click_touch_id(),
        packages::clicks_auto::components::click_world_projected(),
    ))
    .bind(move |clicks| {
        for (click, (button, pos)) in clicks {
            // button == 0 means LMB
            if button == 0 {
                packages::this::messages::MovePlayer {
                    pos2: pos.truncate(),
                }
                .send_server_reliable();
            }
        }
    });

    client_decorate_sprite_as_cube::init();
}

mod client_decorate_sprite_as_cube {
    use crate::packages::easymover::components::*;
    use ambient_api::{
        core::{
            primitives::components::cube,
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
                        .with(cube(), ())
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
