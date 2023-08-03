use ambient_api::{
    core::{
        app::components::{
            main_scene,
            name,
        },
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        primitives::components::quad,
        transform::{
            components::{lookat_target, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

// use flume;

#[main]
pub async fn main() {
    run_async(async move{
        let (quad_ent, cam_ent) = find_first_pair_async(
            spawn_query(quad()),
            |_|{true},
            spawn_query(lookat_target()),
            |cam_id|{entity::has_component(cam_id, main_scene())}
        ).await;

        println!("Found cam '{}' and quad '{}'!",
            entity::get_component(cam_ent, name()).unwrap_or_else(||"{noname}".to_string()),
            entity::get_component(quad_ent, name()).unwrap_or_else(||"{noname}".to_string()),
        );
    });

    // let (quad_tx, quad_rx) = flume::bounded::<EntityId>(1);
    // let (cam_tx, cam_rx) = flume::bounded::<EntityId>(1);

    // let see_quad = spawn_query(quad()).bind(move |quads| for (id,_) in quads {
    //     quad_tx.send(id).unwrap(); // unwrap?
    //     break;
    // });

    // let see_cam = spawn_query(lookat_target()).bind(move |cams| for (id,_) in cams {
    //     if entity::has_component(id, main_scene()) {
    //         cam_tx.send(id).unwrap();
    //         break;
    //     }
    // });

    // run_async(async move{
    //     let quad_ent = quad_rx.recv_async().await.unwrap();
    //     see_quad.stop();
    //     let cam_ent = cam_rx.recv_async().await.unwrap();
    //     see_cam.stop();

    //     println!("Found cam '{}' and quad '{}'!",
    //         entity::get_component(cam_ent, name()).unwrap_or_else(||"{noname}".to_string()),
    //         entity::get_component(quad_ent, name()).unwrap_or_else(||"{noname}".to_string()),
    //     );
    // });
    

    Entity::new()
        .with_merge(make_transformable())
        .with(name(), "Quad 1".to_string())
        .with_default(quad())
        .spawn();

    sleep(1.).await;

    Entity::new()
        .with_merge(make_transformable())
        .with(name(), "Quad 2".to_string())
        .with_default(quad())
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with(name(), "Quad 3".to_string())
        .with_default(quad())
        .spawn();

    sleep(3.).await;

    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(name(), "Cam".to_string())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    println!("Hello, Ambient!");
}

use ambient_api::ecs::{
        ComponentsTuple,
        EventQuery,
};
use flume;

async fn find_first_pair_async<T: ComponentsTuple + Copy, U: ComponentsTuple + Copy> (
        q1 : EventQuery<T>, condforq1 : impl Fn(EntityId) -> bool + 'static,
        q2 : EventQuery<U>, condforq2 : impl Fn(EntityId) -> bool + 'static
    ) -> (EntityId, EntityId) {
        let (tx1, rx1) = flume::bounded::<EntityId>(1);
        let (tx2, rx2) = flume::bounded::<EntityId>(1);
    
        let see1 = q1.bind(move |ents| for (id,_) in ents {
            if condforq1(id) {
                tx1.send(id).unwrap(); // unwrap?
                break;
            }
        });
    
        let see2 = q2.bind(move |ents| for (id,_) in ents {
            if condforq2(id) {
                tx2.send(id).unwrap();
                break;
            }
        });
    
        let ent1 = rx1.recv_async().await.unwrap();
        see1.stop();
        let ent2 = rx2.recv_async().await.unwrap();
        see2.stop();

        (ent1, ent2)
}