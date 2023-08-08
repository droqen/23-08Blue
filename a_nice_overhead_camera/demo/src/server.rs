use ambient_api::{
    core::{
        primitives::components::cube,
        transform::components::translation,
    },
    prelude::*,
};

#[main]
pub fn main() {
    Entity::new()
        .with(cube(), ())
        .with(translation(), vec3(0., 0., 0.))
        .spawn();
}
