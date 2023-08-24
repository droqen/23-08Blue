use ambient_api::prelude::*;

#[main]
pub fn main() {
    default_camera::setup();
    // ease_demo::setup();
    ease_demo_2_cancellation::setup();
    ease_components::setup();
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
                        .with(ease_target_translation_of(), cube)
                        .spawn(),
                );
                // sleep(1.0).await;
                sleep(random::<f32>() * 2.0).await;
            }
        });
    }
}

mod ease_demo {
    use ambient_api::{
        core::{
            primitives::components::quad,
            transform::{components::translation, concepts::make_transformable},
        },
        prelude::*,
    };

    use crate::packages::this::components::*;

    pub fn setup() {
        let now = game_time().as_secs_f32();

        let pos_ease = Entity::new()
            .with(ease_vec3_a(), vec3(5., 5., 0.))
            .with(ease_vec3_b(), vec3(-5., -5., 0.))
            .with(ease_start_time(), now)
            .with(ease_end_time(), now + 10.0)
            .spawn();

        Entity::new()
            .with_merge(make_transformable())
            .with(quad(), ())
            .with(translation_easer(), pos_ease)
            .spawn();

        query(translation_easer()).each_frame(|ents| {
            for (ent, easer) in ents {
                if let Some(pos) = entity::get_component(easer, ease_vec3_value()) {
                    entity::add_component(ent, translation(), pos);
                }
            }
        });
    }
}

mod ease_components {
    use crate::packages::this::components::*;
    use ambient_api::{core::transform::components::translation, prelude::*};
    pub fn setup() {
        query((
            ease_vec3_a(),
            ease_vec3_b(),
            ease_start_time(),
            ease_end_time(),
        ))
        .each_frame(|eases| {
            for (ease, (a, b, t1, t2)) in eases {
                let now = game_time().as_secs_f32();
                if now >= t2 {
                    entity::add_component(ease, ease_vec3_value(), b);
                    entity::remove_components(
                        ease,
                        &[
                            &ease_vec3_a(),
                            &ease_vec3_b(),
                            &ease_start_time(),
                            &ease_end_time(),
                        ],
                    );
                } else {
                    entity::add_component(
                        ease,
                        ease_vec3_value(),
                        a.lerp(b, invlerp(t1, t2, now).clamp(0.0, 1.0)),
                    );
                }
            }
        });

        change_query((ease_target_translation_of(), ease_vec3_value()))
            .track_change(ease_vec3_value())
            .bind(|eases| {
                for (ease, (target_with_translation, vec3_value)) in eases {
                    entity::add_component(target_with_translation, translation(), vec3_value);
                }
            });
    }
    // fn lerp(from: f32, to: f32, rel: f32) -> f32 {
    //     ((1. - rel) * from) + (rel * to)
    // }
    fn invlerp(from: f32, to: f32, value: f32) -> f32 {
        (value - from) / (to - from)
    }
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
