use ambient_api::prelude::*;

mod easing_system;

#[main]
pub fn main() {
    easing_system::setup();

    // default_camera::setup();
    // ease_demo_2_cancellation::setup();
    // ease_demo_3_cancellation_in_messages::setup();
    // ease_demo_4_inout_tween::setup();
}

mod default_camera {
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

    use crate::packages::this::components::*;

    pub fn setup() {
        Entity::new()
            .with_merge(make_perspective_infinite_reverse_camera())
            .with(aspect_ratio_from_window(), EntityId::resources())
            .with(main_scene(), ())
            .with(translation(), Vec3::ONE * 5.)
            .with(lookat_target(), vec3(0., 0., 0.))
            .spawn();
    }
}

mod ease_demo_4_inout_tween {
    use crate::packages::this::components::*;
    use ambient_api::{
        core::{
            app::components::name, primitives::components::cube, transform::components::translation,
        },
        prelude::*,
    };
    pub fn setup() {
        let cube = Entity::new()
            .with(name(), "Cube".into())
            .with(translation(), Vec3::ZERO)
            .with(cube(), ())
            .spawn();
        let ease = Entity::new()
            .with(ease_loop_boomerang(), true)
            .with(ease_filter_paramblend(), 2.0)
            .with(ease_vec3_a(), vec3(-5., 0., 0.))
            .with(ease_vec3_b(), vec3(5., -5., 0.))
            .with(ease_start_time(), 0.0)
            .with(ease_end_time(), 2.0)
            .with(ease_autoset_translation(), cube)
            .spawn();
    }
}

mod ease_demo_3_cancellation_in_messages {
    use ambient_api::{
        core::{
            app::components::name, primitives::components::cube, transform::components::translation,
        },
        prelude::*,
    };

    use crate::packages::this::components::*;

    pub fn setup() {
        let cube = Entity::new()
            .with(name(), "Cube".into())
            .with(translation(), Vec3::ZERO)
            .with(cube(), ())
            .spawn();

        let mut pos_ease: Option<EntityId> = None;

        ambient_api::core::messages::Frame::subscribe(move |_| {
            if random::<f32>() < 0.10 {
                let pos = entity::get_component(cube, translation()).unwrap();
                let pos2 = (random::<Vec2>() * 10. - 5.).extend(0.0);
                let dist = pos2.distance(pos);
                let speed = 5.0;
                // if let Some(pos_ease) = pos_ease {
                //     entity::despawn(pos_ease);
                // }
                let now = game_time().as_secs_f32();
                pos_ease = Some(
                    Entity::new()
                        .with(name(), format!("Ease#{}", random::<u8>()))
                        .with(ease_vec3_a(), pos)
                        .with(ease_vec3_b(), pos2)
                        .with(ease_start_time(), now)
                        .with(ease_end_time(), now + dist / speed)
                        .with(ease_autoset_translation(), cube)
                        .spawn(),
                );
            }
        });
    }
}

mod ease_demo_2_cancellation {
    use ambient_api::{
        core::{
            app::components::name, primitives::components::cube, transform::components::translation,
        },
        prelude::*,
    };

    use crate::packages::this::components::*;

    pub fn setup() {
        let cube = Entity::new()
            .with(name(), "Cube".into())
            .with(translation(), Vec3::ZERO)
            .with(cube(), ())
            .spawn();

        run_async(async move {
            let mut pos_ease: Option<EntityId> = None;
            loop {
                let pos = entity::get_component(cube, translation()).unwrap();
                let pos2 = (random::<Vec2>() * 10. - 5.).extend(0.0);
                let dist = pos2.distance(pos);
                let speed = 5.0;
                if let Some(pos_ease) = pos_ease {
                    entity::despawn(pos_ease);
                }
                let now = game_time().as_secs_f32();
                pos_ease = Some(
                    Entity::new()
                        .with(name(), format!("Ease#{}", random::<u8>()))
                        .with(ease_vec3_a(), pos)
                        .with(ease_vec3_b(), pos2)
                        .with(ease_start_time(), now)
                        .with(ease_end_time(), now + dist / speed)
                        .with(ease_autoset_translation(), cube)
                        .spawn(),
                );
                // sleep(1.0).await;
                sleep(random::<f32>() * 2.0).await;
            }
        });
    }
}
