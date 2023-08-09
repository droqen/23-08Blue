use ambient_api::{
    prelude::*,
    core::primitives::components::quad
};

pub fn setup() {
    Entity::new()
        .with(quad(), ())
        .spawn();
}