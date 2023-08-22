use ambient_api::prelude::*;

#[main]
pub fn main() {
    visualize_runners::setup();
    screen_input::setup(camera_factory::spawn_camera());
}

const CLIENT_GAMETIME_OFFSET: f32 = 0.5;

mod runner_prediction;

mod visualize_runners {
    use crate::{
        packages::defender_ptt::components::{model_of_runner, runner_spawnpos, runner_spawntime},
        CLIENT_GAMETIME_OFFSET,
    };
    use ambient_api::{
        core::{primitives::components::cube, transform::components::translation},
        prelude::*,
    };
    pub fn setup() {
        spawn_query((runner_spawnpos(), runner_spawntime())).bind(|runners| {
            for (runner, (_, _)) in runners {
                if let Ok(pos) =
                    crate::runner_prediction::try_calc_runner_pos(runner, CLIENT_GAMETIME_OFFSET)
                {
                    Entity::new()
                        .with(cube(), ())
                        .with(translation(), pos)
                        .with(model_of_runner(), runner)
                        .spawn();
                } else {
                    println!("!!! runner_prediction error >:( during spawn, no pos found !!!");
                }
            }
        });
        query(model_of_runner()).each_frame(|models| {
            for (model, runner) in models {
                if let Ok(pos) =
                    crate::runner_prediction::try_calc_runner_pos(runner, CLIENT_GAMETIME_OFFSET)
                {
                    entity::set_component(model, translation(), pos);
                } else if !entity::exists(runner) {
                    entity::despawn(model); // deleted model for deleted runner
                } else {
                    println!("!!! runner_prediction error >:( during eachframe, no pos found !!!");
                }
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
            .with(translation(), vec3(15.0, 15.0, 10.0))
            .with(lookat_target(), vec3(0., 0., 0.))
            .spawn()
    }
}

mod screen_input {
    use crate::packages::defender_ptt::messages::Shoot;
    use ambient_api::prelude::*;

    pub fn setup(camera: EntityId) {
        ambient_api::core::messages::Frame::subscribe(move |_| {
            let (delta, input) = input::get_delta();
            if delta.mouse_buttons.contains(&MouseButton::Left) {
                let click_ray: Ray =
                    camera::screen_position_to_world_ray(camera, input.mouse_position);
                let click_point = get_zplane_intersection(click_ray.origin, click_ray.dir, 0.0);
                Shoot {
                    point: click_point.truncate(),
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
