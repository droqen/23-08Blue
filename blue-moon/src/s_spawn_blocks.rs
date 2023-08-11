use ambient_api::{
    core::transform::components::translation,
    prelude::*,
};

use crate::embers::ww_gen::components::{block_size, block_door_side, block_decor_cube, };

pub fn setup() {
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