use ambient_api::prelude::*;
use ambient_api::core::app::components::window_physical_size;

#[main]
pub fn main() {
    ambient_api::core::messages::Frame::subscribe(move |_|{
        let mouse_uv: Vec2 = get_mouse_uv();
        // let mouse_down: bool = crate::c_pinutils::get_pin_mouse_down();
        // if mouse_down {
        //     if let Some(my_cam) = crate::c_findmycam::try_find_my_cam(findmycam_query) {
        //         let mouse_ray: Ray = crate::c_pinutils::get_pin_mouse_ray(my_cam);
        //         messages::MouseDown{ mouse_ray_origin: mouse_ray.origin, mouse_ray_dir: mouse_ray.dir }.send_server_unreliable();
        //     } else {
        //         println!("No my_cam found!");
        //     }
        // }
        let uv_message = pin_mouseuv::messages::PinMouseUv{ u: mouse_uv.x, v: mouse_uv.y };
        uv_message.send_server_unreliable();
        uv_message.send_local_broadcast(false);
    });
}

use ambient_api::prelude::*;

fn get_mouse_uv() -> Vec2 {
    let input = input::get();
    if let Some(window_size) = entity::get_component(entity::resources(), window_physical_size()) {
        let mouse_position_uv = vec2(
            input.mouse_position.x / window_size.x as f32,
            input.mouse_position.y / window_size.y as f32,
        );
        let mouse_position_center_offset = vec2(
            (mouse_position_uv.x).clamp(0.,1.),
            (mouse_position_uv.y).clamp(0.,1.),
        );
        mouse_position_center_offset
    } else { vec2(0.5, 0.5) } // return middle of screen
}

// pub fn get_pin_mouse_down() -> bool {
//     let input = input::get();
//     return input.mouse_buttons.contains(&MouseButton::Left);
// }

// pub fn get_pin_mouse_ray(camera_ent : EntityId) -> Ray {
//     let input = input::get();
//     let lmb_click_position = input.mouse_position;
//     camera::screen_position_to_world_ray(camera_ent, lmb_click_position)
//     // Ray{origin:vec3(0., 0., 0.), dir:vec3(1., 0., 0.)}
// }