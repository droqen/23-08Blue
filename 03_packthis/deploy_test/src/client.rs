use ambient_api::{
    core::{primitives::concepts::make_sphere, transform::components::translation},
    prelude::*,
};

#[main]
pub fn main() {
    println!("CLIEENNTTT!@!!");
    Entity::new()
        .with_merge(make_sphere())
        .with(translation(), vec3(0., 0., 2.5))
        .spawn();
}
