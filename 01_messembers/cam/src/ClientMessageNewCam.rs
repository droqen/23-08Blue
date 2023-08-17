use ambient_api::prelude::*;
use ambient_api::core::{
    app::components::main_scene,
    camera::{
        components::aspect_ratio_from_window,
        concepts::make_perspective_infinite_reverse_camera,
    },
    transform::components::{lookat_target, translation},  
};
use crate::embers::cam::components::{cam_key, cam_tilt, head_pitch, head_yaw, head_relpos,};
use std::f32::consts::PI;

pub fn setup() {
    println!("Setup message NewCam");
    crate::embers::cam::messages::NewCam::subscribe(|_src,msg|{
        println!("Message received NewCam");
        Entity::new()
            .with(cam_key(), msg.key)
            .with_merge(make_perspective_infinite_reverse_camera())
            .with(aspect_ratio_from_window(), entity::resources())
            .with(main_scene(), ())
            .with(lookat_target(), vec3(0., 0., 0.))
            .with(cam_tilt(), vec2(0.25, 0.00) * PI)
            .with(head_relpos(), vec3(0., 5., 5.))
            .spawn();
    });
}