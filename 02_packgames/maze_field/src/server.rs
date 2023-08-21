use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        physics::components::cube_collider,
        primitives::components::{cube, quad},
        transform::{
            components::{lookat_target, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with(main_scene(), ())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with(quad(), ())
        .spawn();

    // spawn maze
    for x in -10..10 {
        for y in -10..10 {
            if random::<f32>() < 0.5 {
                Entity::new()
                    .with(cube(), ())
                    .with(cube_collider(), Vec3::splat(1.))
                    .with(translation(), ivec2(x, y).as_vec2().extend(0.5))
                    .spawn()
            }
        }
    }
}
