use ambient_api::{
    core::{
        primitives::components::quad,
        rendering::components::{color, transparency_group},
        transform::{
            components::{translation, scale},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_transformable())
        .with(quad(), ())
        .with(scale(), vec3(999., 999., 1.))
        .with(color(), vec4(0.5, 0.5, 1.0, 0.5))
        .with(transparency_group(), 1)
        .spawn();
    
    for pos in vec![
        vec3(0.0, 0.0, 5.0),
        vec3(0.5, 0.0, 0.0),
        vec3(0.0, 0.4, 2.0),
    ] {
        Entity::new()
            .with_merge(make_transformable())
            .with_merge(make_physical_unit_cube())
            .with_merge(make_buoy())
            .with(color(), random::<Vec3>().extend(1.0))
            .with(translation(), pos)
            .spawn();
    }
}

use matter::components::{
    matter_gravity, matter_local_center, buoy_max_force, buoy_max_drag, buoy_radius, buoy_water_level,
};

fn make_buoy() -> Entity {
    Entity::new()
        .with(matter_gravity(), 9.82)
        .with(matter_local_center(), Vec3::splat(-0.25))
        .with(buoy_max_force(), 20.0)
        .with(buoy_max_drag(), 1.0) 
        .with(buoy_radius(), 0.5)
        .with(buoy_water_level(), 0.)
}

use ambient_api::core::{
    primitives::components::cube,
    physics::components::{cube_collider, physics_controlled, dynamic},
};

fn make_physical_unit_cube() -> Entity {
    Entity::new()
        .with_merge(make_transformable())
        .with(cube(), ())
        .with(cube_collider(), Vec3::splat(1.))
        .with(physics_controlled(), ())
        .with(dynamic(), true)
}