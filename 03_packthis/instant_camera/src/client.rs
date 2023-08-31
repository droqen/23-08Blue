use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        transform::components::{lookat_target, translation},
    },
    prelude::*,
};

use packages::this::components::is_instant_camera;

#[main]
pub fn main() {
    Entity::new()
        .with(is_instant_camera(), ())
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with(main_scene(), ())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();
}
