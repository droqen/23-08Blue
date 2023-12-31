use ambient_api::prelude::*;
use ambient_api::core::transform::components::translation;

#[main]
pub fn main() {
    let entity_zero = Entity::new().with( translation(), vec3(0., 0., 0.) ).spawn();
    setup_tilt_input_broadcast();
    spawn_mouse_tilt_camera(entity_zero);
}


// use boatblue_selfie_camera_mouse_tilt::messages::PinCameraTilt; // used below

fn setup_tilt_input_broadcast() {
    ambient_api::core::messages::Frame::subscribe(move |_|{
        let mouse_uv: Vec2 = get_mouse_uv();
        let uv_message = PinCameraTilt{ tilt: vec2(mouse_uv.x*2.-1., mouse_uv.y*2.-1.) };
        // uv_message.send_server_unreliable();
        uv_message.send_local_broadcast(true);
    });
}


use ambient_api::core::app::components::name;
use boatblue_selfie_camera::components::{selfie_ground_distance, selfie_ground_height, selfie_focus_ent, selfie_focus_offset, selfie_pitch, selfie_yaw};
use std::f32::consts::PI;
use boatblue_selfie_camera_mouse_tilt::messages::{PinCameraTilt, RefocusCamera};

fn spawn_mouse_tilt_camera(target_ent : EntityId) {
    let my_camera = Entity::new()
        .with_merge(make_basic_camera())
        .with( name(), "Example Mouse Tilt Camera".to_string() )

        .with( selfie_ground_distance(), 14.00 )
        .with( selfie_ground_height(), 18.00 )
        .with( selfie_focus_ent(), target_ent )

        .with( selfie_focus_offset(), vec3(0., 0., 0.) ) // optional
        .with( selfie_pitch(), 0. ) // optional
        .with( selfie_yaw(), PI * 0.25 ) // optional

        .spawn();
        
    subscribe_camera_tilt(my_camera.clone());
    subscribe_camera_refocus(my_camera.clone());
}
fn subscribe_camera_tilt(my_camera : EntityId) {
    PinCameraTilt::subscribe(move |_src, msg|{
        entity::set_component(my_camera, selfie_yaw(), PI * (0.25 - 0.05 * msg.tilt.x));
        // entity::set_component(my_camera, selfie_focus_offset(), Vec2::splat(msg.tilt.y).extend(0.));
        entity::set_component(my_camera, selfie_pitch(), PI * (0.00 + 0.05 * msg.tilt.y));
    });
}
fn subscribe_camera_refocus(my_camera : EntityId) {
    RefocusCamera::subscribe(move |_src, msg|{
        entity::set_component(my_camera, selfie_focus_ent(), msg.focus_ent);
    });
}


use ambient_api::core::app::components::main_scene;
use ambient_api::core::camera::components::aspect_ratio_from_window;
use ambient_api::core::camera::concepts::make_perspective_infinite_reverse_camera;

fn make_basic_camera() -> Entity {
    Entity::new()
        .with_merge( make_perspective_infinite_reverse_camera() )
        .with( main_scene(), () ) // is this needed?
        .with( aspect_ratio_from_window(), entity::resources() )
}


use ambient_api::core::app::components::window_physical_size;

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