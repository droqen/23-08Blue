use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        primitives::components::quad,
        transform::{
            components::{lookat_target, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

#[main]
pub fn main() {
    continuously_spawn_runners::setup();
    runners_reach_end::setup();
    player_shoots_runners::setup();
}

mod runner_prediction;

mod player_shoots_runners {
    use ambient_api::prelude::*;

    use crate::runner_prediction;

    pub fn setup() {
        let find_runners = query(())
            .requires(crate::packages::defender_ptt::components::runner_spawnpos())
            .build();
        crate::packages::defender_ptt::messages::Shoot::subscribe(move |_src, msg| {
            for (runner, _) in find_runners.evaluate() {
                if let Ok(pos) = runner_prediction::try_calc_runner_pos(runner, 0.0) {
                    if pos.distance(msg.point.extend(0.)) < 1. {
                        entity::despawn(runner);
                    }
                }
            }
        });
    }
}

mod continuously_spawn_runners {
    use crate::packages::defender_ptt::components::{runner_spawnpos, runner_spawntime};
    use ambient_api::prelude::*;
    pub fn setup() {
        run_async(async {
            loop {
                sleep(0.25).await;
                let lr = random::<f32>() * 10. - 5.;
                Entity::new()
                    .with(runner_spawnpos(), vec2(lr - 15., -lr - 15.))
                    .with(runner_spawntime(), game_time().as_secs_f32())
                    .spawn();
            }
        });
    }
}

mod runners_reach_end {
    use crate::packages::defender_ptt::components::{runner_spawnpos, runner_spawntime};
    use ambient_api::prelude::*;
    pub fn setup() {
        query(runner_spawntime()).each_frame(|runners| {
            for (runner, spawntime) in runners {
                if crate::runner_prediction::try_calc_is_runner_done(runner, 0.0).unwrap_or(false) {
                    entity::despawn(runner); // runner dead
                }
            }
        });
    }
}
