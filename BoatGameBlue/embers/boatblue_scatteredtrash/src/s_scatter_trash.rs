
use ambient_api::core::rendering::components::color;
use ambient_api::prelude::*;

use ambient_api::core::{
    model::components::model_from_url,
    // transform::concepts::make_transformable,
    transform::components::translation,
    physics::components::{
        physics_controlled, dynamic, sphere_collider, linear_velocity, angular_velocity,
    },
};

pub fn spawn_random_sky_trash(min : f32, max : f32) -> EntityId {
    Entity::new()
        .with_merge(make_buoy())
        .with(color(), vec4(0.5, 0.5, 0.5, 1.0))
        .with(model_from_url(), crate::boatblue_scatteredtrash::assets::url("MSH_Prop_Trashbag.glb").unwrap())
        .with(physics_controlled(), ())
        .with(dynamic(), true)
        .with(sphere_collider(), 1.)
        .with(linear_velocity(), random::<Vec3>()*5.)
        .with(angular_velocity(), random::<Vec3>())
        .with(translation(), vec3(
            lerp(min, max, random::<f32>()),
            lerp(min, max, random::<f32>()),
            20.0,
        ))
        .with(matter_local_center(), vec3(
            lerp(-0.1, 0.1, random::<f32>()),
            lerp(-0.1, 0.1, random::<f32>()),
            -0.1,
        ))
        .spawn()
}

use crate::boatblue_matter::components::{
    matter_gravity, matter_local_center, buoy_max_force, buoy_max_drag, buoy_radius, buoy_water_level,
};

fn make_buoy() -> Entity {
    Entity::new()
        .with(matter_gravity(), 9.82)
        .with(matter_local_center(), vec3(0., 0., -0.2))
        .with(buoy_max_force(), 14.0)
        .with(buoy_max_drag(), 2.0)
        .with(buoy_radius(), 0.5)
        .with(buoy_water_level(), 0.)
}

fn lerp(a:f32,b:f32,t:f32) -> f32 { a*(1.-t)+b*t }
