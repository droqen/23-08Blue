use ambient_api::prelude::*;

#[main]
pub fn main() {
    // spawn_mouse_tilt_camera();
}

// use ambient_api::core::{
//     app::components::{main_scene, name},
//     camera::components::aspect_ratio_from_window,
//     camera::concepts::make_perspective_infinite_reverse_camera,
//     transform::components::{lookat_target, translation},
// };

// fn spawn_mouse_tilt_camera() {
//     Entity::new()
//         .with_merge(make_perspective_infinite_reverse_camera())
//         .with(aspect_ratio_from_window(), EntityId::resources())
//         .with(main_scene(), ())
//         .with(name(), "Mouse Tilt Camera".to_string())
//         .with(translation(), Vec3::ONE * 5.)
//         .with(lookat_target(), vec3(0., 0., 0.))
//         .spawn();
// }