use ambient_api::prelude::*;

#[main]
pub fn main() {
    spawn_water_plane();
    for _ in 0..100 {
        spawn_water_grass();
    }
}


use ambient_api::core::{
    transform::{
        components::{translation, scale},
        // concepts::make_transformable,
    },
    primitives::components::quad,
    rendering::components::{color, transparency_group},
};
fn spawn_water_plane() -> EntityId {
    Entity::new()
        .with(translation(), Vec3::ZERO)
        .with(scale(), vec3(999., 999., 1.))
        .with(color(), vec4(0.5, 0.5, 0.9, 0.3))
        .with(transparency_group(), 1)
        .with(quad(), ())
        .spawn()
}

use ambient_api::core::primitives::components::cube;
fn spawn_water_grass() -> EntityId {
    Entity::new()
        .with(translation(), vec3(
            lerp(-50., 50., random::<f32>()),
            lerp(-50., 50., random::<f32>()),
            0.
        ))
        .with(scale(), vec3(0.1, 0.1, 0.5 + random::<f32>()*2.))
        .with(cube(), ())
        .spawn()
}

fn lerp(from : f32, to : f32, rel : f32) -> f32 { ((1. - rel) * from) + (rel * to) }