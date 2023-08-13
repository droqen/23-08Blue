use std::f32::consts::PI;

use ambient_api::prelude::*;
use ambient_api::core::{
    app::components::{name, window_physical_size},
    transform::components::{translation, lookat_target},
};
use crate::embers::a_nice_overhead_camera::{
    components::{the_nice_camera, nice_input_tilt, nice_yaw_pitch_tilting, nice_yaw_pitch_tilting_base, },
    components::{head_pitch, head_yaw, },
    messages::MouseRay
};
use crate::x_head::make_fully_functional_head_camera;

pub fn setup() {
    let _nicecam = make_fully_functional_head_camera()
        .with(the_nice_camera(), ())
        .with(name(), "The Nice Camera".to_string())
        .with(nice_input_tilt(), vec2(0., 0.))
        .with(nice_yaw_pitch_tilting_base(), vec2(PI * 0.25, 0.00))
        .with(nice_yaw_pitch_tilting(), vec2(0.25, 0.25))
        .spawn();

    // input
    query(()).requires((the_nice_camera(), lookat_target())).each_frame(|cams|{
        let input = input::get();
        let input_tilt = get_mouse_uv(input.mouse_position)*2.-1.;
        for (cam,_) in cams {
            entity::add_component(cam, nice_input_tilt(), input_tilt);

            // broadcast mouse ray position
            {
                let ray = get_mouse_camera_ray(input.mouse_position, cam);
                MouseRay{origin:ray.origin, dir:ray.dir}.send_local_broadcast(false);
            }
        }
    });

    // applying tilt
    query((nice_input_tilt(), nice_yaw_pitch_tilting(), nice_yaw_pitch_tilting_base())).each_frame(|cams|{
        for (cam,(input_tilt,yaw_pitch_tilt,yaw_pitch_base)) in cams {
            entity::add_component(cam, head_yaw(), yaw_pitch_base.x + yaw_pitch_tilt.x * -input_tilt.x);
            entity::add_component(cam, head_pitch(), yaw_pitch_base.y + yaw_pitch_tilt.y * -input_tilt.y);
        }
    });
}

pub fn get_mouse_uv(mouse_position : Vec2) -> Vec2 {
    if let Some(window_size) = entity::get_component(entity::resources(), window_physical_size()) {
        let mouse_position_uv = vec2(
            mouse_position.x / window_size.x as f32,
            mouse_position.y / window_size.y as f32,
        );
        let mouse_position_center_offset = vec2(
            (mouse_position_uv.x).clamp(0.,1.),
            (mouse_position_uv.y).clamp(0.,1.),
        );
        mouse_position_center_offset
    } else { vec2(0.5, 0.5) } // return middle of screen
}

pub fn get_mouse_camera_ray(mouse_position : Vec2, camera_ent : EntityId) -> Ray {
    let input = input::get();
    let lmb_click_position = input.mouse_position;
    camera::screen_position_to_world_ray(camera_ent, lmb_click_position)
}