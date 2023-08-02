// use ambient_api::core::physics::components::visualize_collider;
use ambient_api::prelude::*;

#[main]
pub fn main() {

    spawn_water_plane();

    for _i in 0..3 {
        let example_boat = spawn_example_boat();
        entity::set_component(example_boat, translation(), vec3(-1.5, -1.5, 0.) + 3. * random::<Vec3>());
    }
    
    run_async(async {
        let get_boats = query((translation(), boat_steer())).build();
        loop {
            for (boat, (pos,_)) in get_boats.evaluate() {
                if pos.length() > 20. {
                    entity::set_component(boat, boat_steer(),-pos.truncate().normalize());
                } else {
                    entity::set_component(boat, boat_steer(),
                        (random::<Vec2>()-Vec2::splat(0.5)).normalize_or_zero()
                    );
                }
            }
            sleep(1.).await;
        }
    });
}

use ambient_api::core::{
    transform::{
        components::translation,
        concepts::make_transformable,
    },
    physics::components::{physics_controlled, dynamic, sphere_collider},
};

fn spawn_example_boat() -> EntityId {
    Entity::new()
        .with_merge(make_transformable())
        .with(physics_controlled(), ())
        .with(dynamic(), true)
        .with(sphere_collider(), 1.0) // how big?
        // .with(visualize_collider(), ())
        .with_merge(make_boat())
        .with_merge(make_buoy())
        .spawn()
}

use ambient_api::core::{
    transform::{
        components::scale,
        // concepts::make_transformable,
    },
    primitives::components::quad,
    rendering::components::{color, transparency_group},
};

fn spawn_water_plane() -> EntityId {
    Entity::new()
        .with_merge(make_transformable())
        .with(scale(), vec3(999., 999., 1.))
        .with(color(), vec4(0.5, 0.5, 0.9, 0.5))
        .with(transparency_group(), 1)
        .with(quad(), ())
        .spawn()
}

use boat::components::{
    boat_vel, boat_steer, boat_forward, boat_forward_rotvel,
};

fn make_boat() -> Entity {
    Entity::new()
        .with(boat_vel(), vec2(0., 0.))
        .with(boat_steer(), vec2(0., 0.))
        .with(boat_forward(), vec2(0., 1.))
        .with(boat_forward_rotvel(), 0.)
}

use matter::components::{
    matter_gravity, matter_local_center, buoy_max_force, buoy_max_drag, buoy_radius, buoy_water_level,
};

fn make_buoy() -> Entity {
    Entity::new()
        .with(matter_gravity(), 9.82)
        .with(matter_local_center(), vec3(0., 0., -2.))
        .with(buoy_max_force(), 15.0)
        .with(buoy_max_drag(), 4.0) 
        .with(buoy_radius(), 1.0)
        .with(buoy_water_level(), 0.)
}