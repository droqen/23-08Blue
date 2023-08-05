pub fn setup() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), vec3(5., 5., 15.))
        .with(rotation(), Quat::from_rotation_x(PI))
        // .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();
}

use std::f32::consts::PI;

use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        primitives::components::quad,
        transform::{
            components::{lookat_target, translation, rotation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};