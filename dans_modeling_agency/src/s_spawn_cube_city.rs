use std::f32::consts::PI;

use ambient_api::{
    core::{
        primitives::components::cube,
        transform::components::{translation, rotation, scale},
        // 
    },
    prelude::*,
};

pub fn setup() {
    Entity::new()
        .with(cube(), ())
        .with(rotation(), Quat::from_rotation_z(PI * 0.25))
        .with(scale(), vec3(1.0, 1.0, 1.0))
        .spawn();

    Entity::new()
        .with(cube(), ())
        .with(translation(), vec3(2.0, -2.0, 0.0))
        .with(scale(), vec3(0.5, 0.5, 0.5))
        .spawn();
}