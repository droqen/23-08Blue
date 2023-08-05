use ambient_api::{prelude::*, core::{primitives::concepts::make_sphere, transform::components::translation}};

use crate::sab_physics_pathfinding::components::*;

pub fn setup() {
    Entity::new()
        .with_merge(make_sphere())
        .with(translation(), vec3(4., 0., 0.))
        .with(roller_steer(), vec2(0., 0.))
        .with(roller_targetcell(), ivec2(4, 8))
        .spawn();
}