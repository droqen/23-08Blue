use ambient_api::prelude::*;

#[main]
pub fn main() {
    ease_components::setup();
    ease_autosets::setup();

    // default_camera::setup();
    // ease_demo_2_cancellation::setup();
    // ease_demo_3_cancellation_in_messages::setup();
    // ease_demo_4_inout_tween::setup();
}

mod ease_components {
    use crate::packages::this::components::*;
    use ambient_api::{core::transform::components::translation, prelude::*};
    pub fn setup() {
        generic_setup(
            ease_vec3_a(),
            ease_vec3_b(),
            ease_vec3_value(),
            |a, b, d| a.lerp(b, d),
        );
        generic_setup(
            ease_vec2_a(),
            ease_vec2_b(),
            ease_vec2_value(),
            |a, b, d| a.lerp(b, d),
        );
        generic_setup(ease_f32_a(), ease_f32_b(), ease_f32_value(), |a, b, d| {
            lerp(a, b, d)
        });
    }

    fn generic_setup<T: ambient_api::ecs::SupportedValue + 'static>(
        ca: Component<T>,
        cb: Component<T>,
        cout: Component<T>,
        clerp: impl Fn(T, T, f32) -> T + 'static,
    ) {
        query((ca, cb, ease_start_time(), ease_end_time())).each_frame(move |eases| {
            for (ease, (a, b, t1, t2)) in eases {
                let now = game_time().as_secs_f32()
                    + entity::get_component(ease, ease_time_offset()).unwrap_or(0.);
                if t2 <= t1 {
                    println!("Entity {ease:?} has ease_start_time after ease_end_time"); // should i use DURATION instead?
                    return;
                }
                let mut d = invlerp(t1, t2, now);

                if d < 0. {
                    d = 0.; // right?
                }

                if d > 1. {
                    if let Some(_loop_boomerang) =
                        entity::get_component(ease, ease_loop_boomerang())
                    {
                        d = d % 2.;
                        if d > 1. {
                            d = 2. - d;
                        }
                    } else if let Some(_loop_repeat) =
                        entity::get_component(ease, ease_loop_repeat())
                    {
                        d = d % 1.;
                    } else {
                        d = 1.;
                    }
                }

                if d > 0. && d < 1. {
                    if let Some(paramblend) = entity::get_component(ease, ease_filter_paramblend())
                    {
                        d = d * d / (2. * (d * d - d) + 1.); // https://stackoverflow.com/a/13462135
                    }
                }

                entity::add_component(ease, cout, clerp(a, b, d));
            }
        });
    }
    fn lerp(from: f32, to: f32, rel: f32) -> f32 {
        ((1. - rel) * from) + (rel * to)
    }
    fn invlerp(from: f32, to: f32, value: f32) -> f32 {
        (value - from) / (to - from)
    }
}

mod ease_autosets {
    use crate::packages::this::components::*;
    use ambient_api::{
        core::transform::components::{scale, translation},
        prelude::*,
    };

    pub fn setup() {
        generic_setup(ease_vec3_value(), ease_autoset_translation(), translation());
        generic_setup(ease_vec3_value(), ease_autoset_scale(), scale());
    }

    fn generic_setup<T: ambient_api::ecs::SupportedValue + 'static>(
        read_component: Component<T>,
        autoset_target_component: Component<EntityId>,
        write_component: Component<T>,
    ) {
        change_query((read_component, autoset_target_component))
            .track_change(read_component)
            .bind(move |eases| {
                for (ease, (read_value, autoset_target)) in eases {
                    entity::add_component(autoset_target, write_component, read_value);
                }
            });
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
