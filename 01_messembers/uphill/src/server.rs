use ambient_api::prelude::*;

mod rock;

#[main]
pub fn main() {
    there_is_a_rock::init();
    the_rock_rolls_downhill::init();
    there_are_trees::init();
}

mod there_is_a_rock {
    use crate::embers::uphill::components::{rock_progress, rock_true_velocity};
    use ambient_api::{
        core::{app::components::name, primitives::concepts::make_sphere},
        prelude::*,
    };

    pub fn init() {
        Entity::new()
            // .with_merge(make_sphere())
            .with(name(), "Sside Rock".to_string())
            .with(rock_progress(), 1.0)
            .with(rock_true_velocity(), 0.0)
            .spawn();
    }
}

mod the_rock_rolls_downhill {
    use crate::embers::uphill::components::{rock_progress, rock_true_velocity};
    use crate::rock;
    use ambient_api::prelude::*;

    pub fn init() {
        query((rock_progress(), rock_true_velocity())).each_frame(|rocks| {
            for (rock, (prog, vel)) in rocks {
                let mut vel2 = vel * 0.90 - rock::rock_progress_to_grade(prog) * delta_time();
                let mut prog2 = prog + vel2 * delta_time();
                if prog2 < 0.0 {
                    prog2 = 0.0;
                    if vel2 < 0.0 {
                        vel2 = vel2 * -0.35;
                    }
                }
                entity::set_component(rock, rock_progress(), prog2);
                entity::set_component(rock, rock_true_velocity(), vel2);
            }
        });
    }
}

mod there_are_trees {
    use std::f32::consts::PI;

    use ambient_api::{
        core::{
            primitives::components::{cube, quad},
            transform::components::{rotation, scale, translation},
        },
        prelude::*,
    };

    use crate::rock;

    pub fn init() {
        for _ in 0..100 {
            let prog = random::<f32>();
            let pos = rock::rock_progress_to_position(prog);
            let pos_back = rock::rock_progress_to_position(prog - 0.15);
            let pos_ahead = rock::rock_progress_to_position(prog + 0.15);
            let fwd_flattened = (pos_ahead - pos_back).truncate().normalize_or_zero();
            let right = Quat::from_rotation_z(PI * 0.5) * fwd_flattened.extend(0.);
            let mut side_of_road = 1.;
            if random::<f32>() < 0.5 {
                side_of_road = -1.;
            }
            let dist_from_road = 2. + 4. * random::<f32>();
            Entity::new()
                .with(quad(), ())
                .with(translation(), pos)
                .spawn();
            Entity::new()
                .with(cube(), ())
                .with(translation(), pos + right * side_of_road * dist_from_road)
                .with(rotation(), Quat::from_rotation_z(random::<f32>() * 3.1416))
                .with(scale(), vec3(0.1, 0.1, 2.0))
                .spawn();
        }
    }
}
