use ambient_api::{
    core::{
        transform::components::translation,
        physics::components::{physics_controlled, dynamic, sphere_collider, },
        app::components::name,
        // 
    },
    prelude::*,
};

use crate::embers::skatemouse::components::{is_skatemouse, mouse_cheese, mouse_fwd, mouse_pace};

pub fn setup() {
    let skatemouse = Entity::new() 
        .with(name(), "Mazemouse".to_string())
        .with(translation(), vec3(0., 0., 0.))
        .with(physics_controlled(), ())
        .with(dynamic(), true)
        .with(sphere_collider(), 1.0)
        .with(is_skatemouse(), ())
        .with(mouse_cheese(), vec3(0., 0., 0.))
        .with(mouse_fwd(), vec3(0., 1., 0.))
        .with(mouse_pace(), 0.)
        .spawn();
}