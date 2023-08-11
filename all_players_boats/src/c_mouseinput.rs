use ambient_api::{
    core::{
        physics::components::plane_collider, transform::components::translation, primitives::components::cube,
        // 
    },
    prelude::*,
};

use crate::embers::a_nice_overhead_camera::messages::MouseRay;

pub fn setup() {
    let mouse_cube = Entity::new()
        .with(translation(), vec3(0., 0., 0.))
        .with(cube(), ())
        .spawn();
    MouseRay::subscribe(move |_src,msg|{
        if msg.dir.z >= 0. { println!("Mouse is pointing towards sky. This is illegal. origin: {:?}, dir: {:?}", msg.origin, msg.dir); return; }
        let point = get_zplane_intersection(msg.origin, msg.dir, 0.);
        entity::add_component(mouse_cube, translation(), point);
    });
}

fn get_zplane_intersection(origin:Vec3, dir:Vec3, z:f32) -> Vec3 {
    let dir_mult = (z-origin.z)/dir.z;
    origin+dir_mult*dir
}