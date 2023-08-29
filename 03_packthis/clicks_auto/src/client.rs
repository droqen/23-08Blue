use ambient_api::prelude::*;

#[main]
pub fn main() {
    config_setting_messages::init();
    turning_mouse_input_into_click_objects::init();
    click_camera_ray::init();
    spawning_and_despawning_click_start::init();
}

mod config_setting_messages {
    use crate::packages::this::{
        components::{clicks_config_camera, clicks_config_project_zplane},
        messages::{ConfigureCamera, ConfigureZPlane},
    };
    use ambient_api::prelude::*;

    pub fn init() {
        let cfg = Entity::new().spawn();
        {
            let cfg = cfg.clone();
            ConfigureCamera::subscribe(move |_src, msg| {
                entity::add_component(cfg, clicks_config_camera(), msg.camera);
            });
        }
        {
            let cfg = cfg.clone();
            ConfigureZPlane::subscribe(move |_src, msg| {
                entity::add_component(cfg, clicks_config_project_zplane(), msg.z);
            });
        }
    }
}

mod turning_mouse_input_into_click_objects {
    use crate::packages::this::components::{click_pos_screen, click_touch_id};
    use ambient_api::prelude::*;

    pub fn init() {
        let find_all_clicks = query(click_touch_id()).build();

        ambient_api::core::messages::Frame::subscribe(move |_| {
            let (delta, input) = input::get_delta();

            for button_pressed in delta.mouse_buttons {
                Entity::new()
                    .with(click_touch_id(), button_to_touch_id(&button_pressed))
                    .with(click_pos_screen(), input.mouse_position)
                    .spawn();
            }

            for (click, touch_id) in find_all_clicks.evaluate().iter() {
                let button = touch_id_to_button(*touch_id);

                if !input.mouse_buttons.contains(&button) {
                    entity::despawn(*click);
                    continue;
                }

                entity::set_component(*click, click_pos_screen(), input.mouse_position);

                if delta.mouse_buttons_released.contains(&button) {
                    entity::despawn(*click);
                }
            }
        });
    }

    fn button_to_touch_id(button: &MouseButton) -> u16 {
        match button {
            &MouseButton::Left => 0,
            &MouseButton::Middle => 1,
            &MouseButton::Right => 2,
            &MouseButton::Other(button_other) => 3 + button_other,
        }
    }

    fn touch_id_to_button(touch_id: u16) -> MouseButton {
        match touch_id {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            _ => MouseButton::Other(touch_id - 3),
        }
    }
}

mod click_camera_ray {
    use crate::packages::this::components::{
        click_pos_screen, click_touch_id, click_world_dir, click_world_origin,
        click_world_projected, clicks_config_camera, clicks_config_project_zplane,
    };
    use ambient_api::prelude::*;
    pub fn init() {
        let find_config_camera = query(clicks_config_camera()).build();
        let find_config_zplane = query(clicks_config_project_zplane()).build();
        let find_click_positions = query(click_pos_screen()).requires(click_touch_id()).build();
        ambient_api::core::messages::Frame::subscribe(move |_| {
            if let Some(camera_result) = find_config_camera.evaluate().first() {
                let camera: EntityId = camera_result.1;
                let zplane: Option<f32> = match find_config_zplane.evaluate().first() {
                    None => None,
                    Some(zplane_result) => Some(zplane_result.1),
                };
                for (click, screen_pos) in find_click_positions.evaluate().iter() {
                    let ray = get_mouse_camera_ray(*screen_pos, camera);
                    entity::add_component(*click, click_world_origin(), ray.origin);
                    entity::add_component(*click, click_world_dir(), ray.dir);
                    match zplane {
                        None => (),
                        Some(zplane) => entity::add_component(
                            *click,
                            click_world_projected(),
                            get_zplane_intersection(ray.origin, ray.dir, zplane),
                        ),
                    };
                }
            }
        });
    }

    pub fn get_mouse_camera_ray(mouse_position: Vec2, camera_ent: EntityId) -> Ray {
        let input = input::get();
        let lmb_click_position = input.mouse_position;
        camera::screen_position_to_world_ray(camera_ent, lmb_click_position)
    }
    fn get_zplane_intersection(origin: Vec3, dir: Vec3, z: f32) -> Vec3 {
        let dir_mult = (z - origin.z) / dir.z;
        origin + dir_mult * dir
    }
}

mod spawning_and_despawning_click_start {
    use crate::packages::this::components::{
        click_pos_screen, click_start, click_touch_id, click_world_dir, click_world_origin,
        click_world_projected, clicks_config_camera, clicks_config_project_zplane,
    };
    use ambient_api::prelude::*;

    pub fn init() {
        premade_spawn_query_spawn_start(click_pos_screen());
        premade_spawn_query_spawn_start(click_world_dir());
        premade_spawn_query_spawn_start(click_world_origin());
        premade_spawn_query_spawn_start(click_world_projected());

        despawn_query(click_start()).bind(|clicks| {
            for (click, start) in clicks {
                entity::despawn(start);
            }
        });
    }

    fn premade_spawn_query_spawn_start<T: ambient_api::ecs::SupportedValue + 'static>(
        component_to_pass_on: Component<T>,
    ) {
        spawn_query(component_to_pass_on)
            .requires(click_touch_id())
            .bind(move |clicks| {
                for (click, value) in clicks {
                    create_or_add_component_to_start(click, component_to_pass_on, value)
                }
            });
    }

    fn create_or_add_component_to_start<T: ambient_api::ecs::SupportedValue + 'static>(
        click: EntityId,
        component: Component<T>,
        value: T,
    ) {
        if let Some(start) = entity::get_component(click, click_start()) {
            entity::add_component(start, component, value);
        } else {
            let start = Entity::new().with(component, value).spawn();
            entity::add_component(click, click_start(), start);
        }
    }
}
