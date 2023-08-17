use ambient_api::prelude::*;
use ambient_api::core::app::components::window_physical_size;
use crate::embers::cam::components::{cam_key, cam_tilt, };
use std::f32::consts::PI;

pub fn setup() {

    // input
    query(()).requires(cam_key()).each_frame(|cams|{
        let input = input::get();
        let input_tilt = get_mouse_uv(input.mouse_position)*2.-1.;
        for (cam,_) in cams {
            entity::add_component(cam, cam_tilt(),
                PI *
                (input_tilt * vec2(0.1, 0.1) + vec2(0.25, 0.0))
            );

            // // broadcast mouse ray position
            // {
            //     let ray = get_mouse_camera_ray(input.mouse_position, cam);
            //     MouseRay{origin:ray.origin, dir:ray.dir}.send_local_broadcast(false);
            // }
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

// pub fn get_mouse_camera_ray(mouse_position : Vec2, camera_ent : EntityId) -> Ray {
//     let input = input::get();
//     let lmb_click_position = input.mouse_position;
//     camera::screen_position_to_world_ray(camera_ent, lmb_click_position)
// }