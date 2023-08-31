// Sets up all the stuff required for 'ease' to function.
// Is called on both client and server, and handled locally.

pub fn setup() {
    ease_components::setup();
    ease_autosets::setup();
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

    fn generic_setup<T: ambient_api::ecs::SupportedValue + 'static + std::cmp::PartialEq>(
        ca: Component<T>,
        cb: Component<T>,
        cout: Component<T>,
        clerp: impl Fn(T, T, f32) -> T + 'static,
    ) {
        query((ca, cb, ease_start_time(), ease_end_time())).each_frame(move |eases| {
            for (ease, (a, b, t1, t2)) in eases {
                let now = game_time().as_secs_f32()
                    + entity::get_component(ease, ease_time_offset()).unwrap_or(0.);
                if a == b {
                    entity::add_component(ease, cout, b);
                    return;
                }
                if t2 <= t1 {
                    println!("Entity {ease:?} has non-positive duration and a != b");
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
