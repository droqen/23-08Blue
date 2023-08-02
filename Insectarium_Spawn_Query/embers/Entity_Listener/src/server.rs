use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        primitives::components::{cube, quad},
        transform::{
            components::{lookat_target, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

#[main]
pub fn main() {

    spawn_quad();

    println!("LISTENER: Setting up spawn queries now...");

    spawn_query(quad()).bind(|quads|{
        for (_,_) in quads {
            println!("LISTENER: Quad found - I spawned this myself!");
        }
    });
    spawn_query(cube()).bind(|cubes|{
        for (_,_) in cubes {
            println!("LISTENER: Cube found - This was spawned in another ember!");
        }
    });

    println!("LISTENER: Done setting up spawn queries!");

    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 12.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    spawn_quad();

    run_async(async {
        loop {
            sleep(0.84).await;
            spawn_quad();
        }
    });
}


fn spawn_quad() {
    println!("LISTENER: Spawned a quad");
    Entity::new()
        .with_merge(make_transformable())
        .with(translation(), random::<Vec3>()*10. - Vec3::splat(5.))
        .with_default(quad())
        .spawn();
}