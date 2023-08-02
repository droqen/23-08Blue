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
    
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 12.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    println!("SPAWNER: Ready to spawn cubes every 0.9 seconds or so!");

    run_async(async {
        loop {
            spawn_cube();
            sleep(0.9).await;
        }
    });
}

fn spawn_cube() {
    println!("SPAWNER: Spawned a cube");
    Entity::new()
        .with_merge(make_transformable())
        .with(translation(), random::<Vec3>()*10. - Vec3::splat(5.))
        .with_default(cube())
        .spawn();
}
