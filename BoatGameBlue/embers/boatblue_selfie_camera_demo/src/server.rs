use ambient_api::prelude::*;

use ambient_api::core::primitives::components::quad;
use ambient_api::core::transform::concepts::make_transformable;

#[main]
pub fn main() {
    // Entity::new()
    //     .with_merge(make_perspective_infinite_reverse_camera())
    //     .with(aspect_ratio_from_window(), EntityId::resources())
    //     .with_default(main_scene())
    //     .with(translation(), Vec3::ONE * 5.)
    //     .with(lookat_target(), vec3(0., 0., 0.))
    //     .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .spawn();

    println!("Hello, Default!");
}
