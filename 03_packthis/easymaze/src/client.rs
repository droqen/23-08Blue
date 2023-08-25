use ambient_api::prelude::*;

#[main]
pub fn main() {
    let camera = camera_factory::spawn_camera();
    screen_input::setup(camera);
}

mod camera_factory {
    use ambient_api::{
        core::{
            app::components::main_scene,
            camera::{
                components::aspect_ratio_from_window,
                concepts::make_perspective_infinite_reverse_camera,
            },
            transform::components::{lookat_target, translation},
        },
        prelude::*,
    };

    pub fn spawn_camera() -> EntityId {
        Entity::new()
            .with_merge(make_perspective_infinite_reverse_camera())
            .with(aspect_ratio_from_window(), EntityId::resources())
            .with(main_scene(), ())
            .with(translation(), vec3(2., 0.2, 10.))
            .with(lookat_target(), vec3(0., 0., 0.))
            .spawn()
    }
}

// mod camera_slowlerp {
//     use crate::packages::this::components::{c_zoom, c_zoom_slowlerp, c_zoom_slowlerpvel};
//     use ambient_api::{core::transform::components::translation, prelude::*};
//     pub fn setup(camera: EntityId) {
//         ambient_api::core::messages::Frame::subscribe(move |_| {
//             let (delta, input) = input::get_delta();
//             if delta.mouse_buttons.contains(&MouseButton::Left) {
//                 entity::add_component(
//                     camera,
//                     c_zoom_slowlerp(),
//                     1.0 * 0.5
//                         + 0.5
//                             * entity::get_component(camera, c_zoom())
//                                 .unwrap_or(crate::STARTING_CAMERA_ZOOM),
//                 );
//             }
//         });
//         query((c_zoom(), c_zoom_slowlerp())).each_frame(|cams| {
//             for (cam, (zoom, zoomgoal)) in cams {
//                 entity::set_component(cam, c_zoom(), zoom * 0.9 + 0.1 * zoomgoal);
//                 entity::set_component(cam, translation(), crate::ZOOMED_OUT_CAMERA_POSITION * zoom);
//             }
//         });
//     }
// }

mod screen_input {
    use crate::packages::this::messages::PlayerMove;
    use ambient_api::prelude::*;

    pub fn setup(camera: EntityId) {
        ambient_api::core::messages::Frame::subscribe(move |_| {
            let (delta, input) = input::get_delta();
            if delta.mouse_buttons.contains(&MouseButton::Left) {
                let click_ray: Ray =
                    camera::screen_position_to_world_ray(camera, input.mouse_position);
                let click_point = get_zplane_intersection(click_ray.origin, click_ray.dir, 0.0);
                PlayerMove {
                    target: click_point.truncate(),
                }
                .send_server_reliable(); // go!
                println!("PlayerMove: {}", click_point.truncate());
            }
        });
    }

    fn get_zplane_intersection(origin: Vec3, dir: Vec3, z: f32) -> Vec3 {
        let dir_mult = (z - origin.z) / dir.z;
        origin + dir_mult * dir
    }
}
