use ambient_api::prelude::*;
mod mover_prediction;

#[main]
pub fn main() {
    screen_input::setup(camera_factory::spawn_camera());
    predictive_model_movement::setup();
}

mod predictive_model_movement {
    use ambient_api::{
        core::{
            primitives::{components::cube, concepts::make_capsule},
            transform::components::{scale, translation},
        },
        prelude::*,
    };

    use crate::{
        mover_prediction,
        packages::this::components::{
            c_model_velocity, c_mover_model, mover_pos_end, mover_pos_start, mover_time_start,
        },
    };
    pub fn setup() {
        spawn_query(mover_pos_end()).bind(|movers| {
            for (mover, pos) in movers {
                Entity::new()
                    // .with(cube(), ())
                    .with_merge(make_capsule())
                    .with(scale(), vec3(0.5, 0.5, 0.5))
                    .with(translation(), pos.extend(0.5))
                    .with(c_mover_model(), mover)
                    .with(c_model_velocity(), Vec2::ZERO)
                    .spawn();
            }
        });

        // VERSION WITHOUT VELOCITY
        // query((translation(), c_mover_model())).each_frame(|movers| {
        //     for (model, (pos, mover)) in movers {
        //         let target_pos = mover_prediction::get_mover_pos3(mover, 0.5);
        //         entity::set_component(model, translation(), pos.lerp(target_pos, 0.25));
        //     }
        // });

        // VERSION WITH VELOCITY
        query((translation(), c_model_velocity(), c_mover_model())).each_frame(|movers| {
            for (model, (pos, vel, mover)) in movers {
                let target_pos = mover_prediction::get_mover_pos3(mover, 0.5);
                let to_target_pos = target_pos - pos;
                let vel2 = (vel * 0.90).lerp(to_target_pos.truncate(), 0.01);
                entity::set_component(model, translation(), (pos.truncate() + vel2).extend(0.5));
                entity::set_component(model, c_model_velocity(), vel2);
            }
        });
    }
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
            .with(translation(), vec3(1.0, 7.0, 20.0))
            .with(lookat_target(), vec3(0., 0., 0.))
            .spawn()
    }
}

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
            }
        });
    }

    fn get_zplane_intersection(origin: Vec3, dir: Vec3, z: f32) -> Vec3 {
        let dir_mult = (z - origin.z) / dir.z;
        origin + dir_mult * dir
    }
}
