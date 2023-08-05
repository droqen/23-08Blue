use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        primitives::components::{quad, cube},
        transform::{
            components::{lookat_target, translation, rotation, scale},
            concepts::make_transformable,
        },
        physics::components::{cube_collider, visualize_collider, physics_controlled, dynamic, linear_velocity}, rendering::components::color,
    },
    prelude::*,
};

use saa_physics_momentum_err::components::is_driven;

#[main]
pub fn main() {

    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 10.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    Entity::new()
        .with(physics_controlled(), ())
        .with(dynamic(), true)
        .with(translation(), vec3(0., -2., 4.))
        .with(cube(), ())
        .with(cube_collider(), vec3(1., 1., 1.))
        .with(linear_velocity(), vec3(0., 0., 0.))
        .spawn();

    Entity::new()
        .with(physics_controlled(), ())
        .with(dynamic(), true)
        .with(translation(), vec3(0., 2., 4.))
        .with(cube(), ())
        .with(cube_collider(), vec3(1., 1., 1.))
        .with(linear_velocity(), vec3(0., 0., 0.))
        .with(is_driven(), ())
        .spawn();

    // floor
    Entity::new()
        .with(translation(), vec3(0., 0., 0.))
        .with(cube(), ())
        .with(cube_collider(), vec3(1., 1., 1.))
        .with(scale(), vec3(10., 10., 1.)) // static
        .with(color(), vec4(1., 0.5, 0.5, 1.))
        .spawn();

    // wall
    Entity::new()
        .with(translation(), vec3(-5., 0., 2.50))
        .with(cube(), ())
        .with(cube_collider(), vec3(1., 1., 1.))
        .with(scale(), vec3(1., 10., 5.)) // static
        .with(color(), vec4(1., 0.5, 0.5, 1.))
        .spawn();

    query(()).requires(is_driven()).each_frame(|driven_bodies|{
        for (body,_) in driven_bodies {
            entity::mutate_component(body, linear_velocity(), |linvel|linvel.x -= 0.10 * delta_time());
        }
    });

}
