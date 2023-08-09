use ambient_api::{
    core::{
        primitives::components::quad,
        transform::components::scale,
        rendering::components::{color, transparency_group}
        // 
    },
    prelude::*,
};

pub fn setup() {
    Entity::new()
        .with(quad(), ())
        .with(scale(), vec3(999., 999., 1.))
        .with(color(), vec4(1., 1., 1., 0.25))
        .with(transparency_group(), 1)
        .spawn();
}