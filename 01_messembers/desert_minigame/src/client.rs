use ambient_api::prelude::*;

#[main]
pub async fn main() {
    pilgrim::init();
    sandstorm_particles::init();
    sleep(1.).await;
    embers::cam::messages::NewCam { key: 0 }.send_local_broadcast(false);
}

mod pilgrim {
    use ambient_api::{
        core::{
            primitives::components::cube,
            transform::components::{scale, translation},
        },
        prelude::*,
    };
    pub fn init() {
        Entity::new()
            .with(cube(), ())
            .with(translation(), Vec3::ZERO)
            .with(scale(), vec3(0.25, 0.25, 1.00))
            .spawn();
    }
}

mod sandstorm_particles {
    use crate::embers::desert_minigame::components::sand_reset;
    use ambient_api::{
        core::{
            primitives::components::cube,
            transform::components::{rotation, scale, translation},
        },
        prelude::*,
    };
    pub fn init() {
        for _ in 0..100 {
            spawn_sand();
        }
        query((translation(), sand_reset())).each_frame(|grains| {
            for (sand, (pos, reset)) in grains {
                if reset <= 0.0 {
                    entity::set_component(sand, translation(), random_sand_startpos());
                    entity::set_component(sand, sand_reset(), random::<f32>());
                } else {
                    entity::set_component(sand, translation(), pos + random::<Vec3>());
                    entity::set_component(sand, sand_reset(), reset - delta_time());
                }
            }
        });
    }
    fn spawn_sand() {
        Entity::new()
            .with(cube(), ())
            .with(translation(), random_sand_startpos())
            .with(rotation(), random::<Quat>())
            .with(scale(), vec3(0.10, 0.10, 0.10))
            .with(sand_reset(), random::<f32>())
            .spawn();
    }
    fn random_sand_startpos() -> Vec3 {
        vec3(
            random::<f32>() * -20.,
            random::<f32>() * -20.,
            random::<f32>() * 10.,
        )
    }
}
