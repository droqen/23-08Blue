use ambient_api::prelude::*;

#[main]
pub fn main() {
    spawn_water_plane();

    spawn_temporary_landmark();
}


use ambient_api::core::{
    transform::{
        components::scale,
        concepts::make_transformable,
    },
    primitives::components::quad,
    rendering::components::{color, transparency_group},
};
fn spawn_water_plane() -> EntityId {
    Entity::new()
        .with_merge(make_transformable())
        .with(scale(), vec3(999., 999., 1.))
        .with(color(), vec4(0.5, 0.5, 0.9, 0.3))
        .with(transparency_group(), 1)
        .with(quad(), ())
        .spawn()
}

use ambient_api::core::primitives::components::cube;
fn spawn_temporary_landmark() -> EntityId {
    Entity::new()
        .with_merge(make_transformable())
        .with(scale(), vec3(1., 1., 10.))
        .with(cube(), ())
        .spawn()
}