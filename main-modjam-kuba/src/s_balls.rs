
use std::f32::consts::PI;

use ambient_api::{
    prelude::*,
    components::core::{
        physics::{physics_controlled, dynamic, sphere_collider, mass, linear_velocity}, transform::{translation, scale}, rendering::color,
        // primitives::sphere,
    },
    concepts::{make_sphere, make_transformable}
};

use crate::components::charge;

pub fn spawn_planets(n: usize) -> Vec<EntityId> {
    (0..n).into_iter().map(|_| {
        let planet_radius = random::<f32>() * random::<f32>() * 2. + 0.3;
        let start_pos = vec3(rt(), rt(), rt(),);
        Entity::new()
            .with_merge(make_transformable())
            .with(physics_controlled(), ())
            .with(dynamic(), true)
            .with(sphere_collider(), 0.5)
            .with(mass(), planet_radius * planet_radius * planet_radius)
            .with(charge(), bitflip())
            .with(translation(), start_pos)
            .with(scale(), Vec3::splat(planet_radius))
            // .with(linear_velocity(), Quat::from_rotation_z(0.5 * PI) * start_pos * start_pos_coefficient)
            .with(color(), random::<Vec3>().extend(1.))
            .with_merge(make_sphere())
            .spawn()
    }).collect()
}

pub fn setup() {
    physics::set_gravity(Vec3::ZERO);

    let mut frame_count = 0;
    let mut planets = spawn_planets(10);
    ambient_api::messages::Frame::subscribe(move |_| {
        for i in 0..planets.len()-1 {
            for j in i+1..planets.len() {
                let chargei = entity::get_component(planets[i], charge()).unwrap();
                let chargej = entity::get_component(planets[j], charge()).unwrap();
                let posi = entity::get_component(planets[i], translation()).unwrap();
                let posj = entity::get_component(planets[j], translation()).unwrap();
                let diri = (posj-posi).normalize_or_zero();
                if diri == Vec3::ZERO {
                    // ok
                } else {
                    let massi = entity::get_component(planets[i], mass()).unwrap();
                    let massj = entity::get_component(planets[j], mass()).unwrap();
                    let distancesq = (posi-posj).length_squared();
                    let gravity_const = 2.0;
                    let forcei = diri * massi * massj * gravity_const / distancesq * chargei * chargej * -1.;
                    let forcej = -forcei;

                    physics::add_force(planets[i], forcei);
                    physics::add_force(planets[j], forcej);
                }
            }
        }

        frame_count += 1;

        // respawn planets every 10s
        if frame_count % (60 * 10) == 0 {
            for planet in planets.iter() {
                entity::despawn(*planet);
            }
            planets.clear();
            planets = spawn_planets(10);
        }
    });
}

fn r() -> f32 {
    1. * (random::<f32>() - random::<f32>())
}
fn rt() -> f32 {
    5. * (random::<f32>() - random::<f32>())
}
fn bitflip() -> f32 {
    if random::<bool>() { return 1.; } else { -1. }
}