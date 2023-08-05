use ambient_api::{
    core::{
        app::components::{main_scene, name},
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

use crate::saa_physics_momentum_err::components::driven_power;

pub fn setup() {

    query(driven_power()).each_frame(|driven_bodies|{
        for (body,power) in driven_bodies {
            // (!!!) at any nonzero value, this cancels out gravity
            // (!!!) when moving into collision, this causes weird behaviour
            entity::mutate_component(body, linear_velocity(), |linvel|linvel.x -= power * delta_time());
        }
    });

    // spawn three cubes.

    for i in -1..1+1 {
        let cube = Entity::new()
            .with(physics_controlled(), ())
            .with(dynamic(), true)
            .with(translation(), vec3(0., -3. * i as f32, 4.))
            .with(cube(), ())
            .with(cube_collider(), vec3(1., 1., 1.))
            .with(linear_velocity(), vec3(0., 0., 0.))
            .spawn();
        match i {
            1 => {
                // front cube: driven_power of 9.81, at ground level. -- shows hitting wall
                entity::add_component(cube, name(), "Glitchy collision with wall,\nx-accel = 9.81".to_string());
                entity::add_component(cube, driven_power(), 9.81);
                entity::mutate_component(cube, translation(), |pos|pos.z = 1.);
            },
            0 => {
                // middle cube: driven_power of 0.10. -- shows how it cancels out gravity
                entity::add_component(cube, name(), "Cancels out/slows gravity,\nx-accel = 0.10".to_string());
                entity::add_component(cube, driven_power(), 0.10);
            },
            -1|_ => {
                // back cube: normal behaviour
                entity::add_component(cube, name(), "Normal gravity,\nx-accel = 0.00".to_string());
                entity::add_component(cube, driven_power(), 0.00);
            },
        }
    }

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

}
