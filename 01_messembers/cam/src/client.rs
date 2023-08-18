use ambient_api::prelude::*;

#[main]
pub fn main() {
    head_lookat::init();
    head_target::init();
    new_cam_message::init();
    query_cam_tilt::init();
    query_mouse_sets_tilt::init();
}

mod head_lookat {
    use crate::embers::cam::components::{head_pitch, head_relpos, head_yaw};
    use ambient_api::core::transform::components::{lookat_target, translation};
    use ambient_api::prelude::*;

    pub fn init() {
        query((lookat_target(), head_pitch(), head_yaw(), head_relpos())).each_frame(|heads| {
            for (head, (locus, pitch, yaw, relpos)) in heads {
                entity::add_component(
                    head,
                    translation(),
                    locus + Quat::from_rotation_z(yaw) * (Quat::from_rotation_x(pitch) * relpos),
                );
            }
        });
    }
}

mod head_target {
    use crate::embers::cam::components::{
        head_pitch, head_target_relpos, head_targetent, head_yaw,
    };
    use ambient_api::core::transform::components::{lookat_target, translation};
    use ambient_api::prelude::*;

    pub fn init() {
        query((head_targetent(), head_pitch(), head_yaw())).each_frame(|heads| {
            for (head, (targetent, pitch, yaw)) in heads {
                let relpos =
                    entity::get_component(head, head_target_relpos()).unwrap_or(Vec3::ZERO);
                if let Some(targetent_pos) = entity::get_component(targetent, translation()) {
                    entity::add_component(
                        head,
                        lookat_target(),
                        targetent_pos
                            + Quat::from_rotation_z(-yaw) * (Quat::from_rotation_x(pitch) * relpos),
                    );
                } else {
                    println!("INVALID head_targetent() entity - has no translation() component!");
                }
            }
        });
    }
}

mod new_cam_message {
    use crate::embers::cam::components::{cam_key, cam_tilt, head_pitch, head_relpos, head_yaw};
    use ambient_api::core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        transform::components::{lookat_target, translation},
    };
    use ambient_api::prelude::*;
    use std::f32::consts::PI;

    pub fn init() {
        crate::embers::cam::messages::NewCam::subscribe(|_src, msg| {
            println!("Message received NewCam");
            let camera = Entity::new()
                .with(cam_key(), msg.key)
                .with_merge(make_perspective_infinite_reverse_camera())
                .with(aspect_ratio_from_window(), entity::resources())
                .with(main_scene(), ())
                .with(lookat_target(), vec3(0., 0., 0.))
                .with(translation(), vec3(1., 1., 1.))
                .with(cam_tilt(), vec2(0.25, 0.00) * PI)
                .with(head_relpos(), vec3(0., 5., 5.))
                .spawn();
            crate::embers::babel::messages::RegisterCamera { entity: camera }
                .send_local_broadcast(false);
        });
    }
}

mod query_cam_tilt {
    use crate::embers::cam::components::{cam_tilt, head_pitch, head_yaw};
    use ambient_api::prelude::*;

    pub fn init() {
        // applying tilt, smoothed slightly
        query(cam_tilt()).each_frame(|cams| {
            for (cam, tilt) in cams {
                entity::mutate_component_with_default(cam, head_yaw(), tilt.x, |yaw| {
                    *yaw = lerp(*yaw, tilt.x, 0.35)
                });
                entity::mutate_component_with_default(cam, head_pitch(), tilt.y, |pitch| {
                    *pitch = lerp(*pitch, tilt.y, 0.35)
                });
            }
        });
    }

    fn lerp(from: f32, to: f32, rel: f32) -> f32 {
        ((1. - rel) * from) + (rel * to)
    }
}

mod query_mouse_sets_tilt {
    use crate::embers::cam::{
        components::{cam_key, cam_tilt},
        messages::MouseRay,
    };
    use ambient_api::core::app::components::window_physical_size;
    use ambient_api::prelude::*;
    use std::f32::consts::PI;

    pub fn init() {
        // input
        query(()).requires(cam_key()).each_frame(|cams| {
            let input = input::get();
            let input_tilt = get_mouse_uv(input.mouse_position) * 2. - 1.;
            for (cam, _) in cams {
                entity::add_component(
                    cam,
                    cam_tilt(),
                    PI * (input_tilt * vec2(0.25, 0.25) + vec2(0.25, 0.0)),
                );

                // broadcast mouse ray position
                {
                    let ray = get_mouse_camera_ray(input.mouse_position, cam);
                    MouseRay {
                        origin: ray.origin,
                        dir: ray.dir,
                    }
                    .send_local_broadcast(false);
                }
            }
        });
    }

    pub fn get_mouse_uv(mouse_position: Vec2) -> Vec2 {
        if let Some(window_size) =
            entity::get_component(entity::resources(), window_physical_size())
        {
            let mouse_position_uv = vec2(
                mouse_position.x / window_size.x as f32,
                mouse_position.y / window_size.y as f32,
            );
            let mouse_position_center_offset = vec2(
                (mouse_position_uv.x).clamp(0., 1.),
                (mouse_position_uv.y).clamp(0., 1.),
            );
            mouse_position_center_offset
        } else {
            vec2(0.5, 0.5)
        } // return middle of screen
    }

    pub fn get_mouse_camera_ray(mouse_position: Vec2, camera_ent: EntityId) -> Ray {
        let input = input::get();
        let lmb_click_position = input.mouse_position;
        camera::screen_position_to_world_ray(camera_ent, lmb_click_position)
    }
}
