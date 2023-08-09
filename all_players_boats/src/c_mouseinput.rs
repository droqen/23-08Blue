use ambient_api::{
    core::{
        physics::components::plane_collider,
        // 
    },
    prelude::*,
};

pub fn setup() {
    Entity::new()
        .with(plane_collider(), ())
        .spawn();
}