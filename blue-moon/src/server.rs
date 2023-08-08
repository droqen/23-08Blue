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

use ww_gen::components::block_size;
use ww_gen::components::block_door_side;
use blue_moon::components::test_component;

mod s_test;

#[main]
pub fn main() {

    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    Entity::new()
        .with(translation(), vec3(0., 0., 0.))
        .with(block_size(), vec3(6.0, 6.0, 10.0))
        .spawn();

    Entity::new()
        .with(translation(), vec3(6.0, -6.0, 0.))
        .with(block_size(), vec3(6.0, 6.0, 10.0))
        .with(block_door_side(), 0)
        .with(test_component(), ())
        .spawn();

    println!("Hello, Ambient!");

    s_test::setup();
}
