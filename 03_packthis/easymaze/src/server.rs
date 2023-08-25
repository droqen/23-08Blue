use ambient_api::{
    core::{primitives::components::quad, transform::concepts::make_transformable},
    prelude::*,
};
use packages::ease::components::*;

#[main]
pub fn main() {
    let quad = Entity::new()
        .with_merge(make_transformable())
        .with(quad(), ())
        .spawn();

    Entity::new()
        .with(ease_vec3_a(), vec3(5., -5., 0.))
        .with(ease_vec3_b(), vec3(5., -5., 0.))
        .with(ease_start_time(), 0.0)
        .with(ease_end_time(), 2.0)
        .with(ease_filter_paramblend(), 2.0)
        .with(ease_loop_boomerang(), true)
        .with(ease_autoset_translation(), quad)
        .spawn();
}
