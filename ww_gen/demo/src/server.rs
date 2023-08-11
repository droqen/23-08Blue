use ambient_api::{
    core::{
        app::components::main_scene,
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

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 30.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    // Entity::new()
    //     .with_merge(make_transformable())
    //     .with_default(quad())
    //     .spawn();

    // println!("Hello, Ambient!");

    spawn_blocks();
}


use embers::ww_gen::components::{block_size, block_door_side, block_decor_cube, };

pub fn spawn_blocks() {
    Entity::new()
        .with(translation(), vec3(0., 0., 0.))
        .with(block_size(), vec3(6.0, 6.0, 10.0))
        .with(block_decor_cube(), ())
        .spawn();

    Entity::new()
        .with(translation(), vec3(6.0, -6.0, 0.))
        .with(block_size(), vec3(6.0, 6.0, 10.0))
        .with(block_door_side(), 0)
        .with(block_decor_cube(), ())
        .spawn();

    Entity::new()
        .with(translation(), vec3(12.0, -12.0, 0.))
        .with(block_size(), vec3(6.0, 6.0, 10.0))
        .with(block_door_side(), 1)
        .with(block_decor_cube(), ())
        .spawn();
}