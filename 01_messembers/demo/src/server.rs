use ambient_api::prelude::*;
#[main]
pub async fn main() {
    sleep(1.).await; // necessary to allow the cam ember to set itself up before this message is sent (so it's not missed!)
    run_async(demo_king_of_ephyra::init());
}

mod demo_king_of_ephyra {
    use ambient_api::{
        core::{
            primitives::{components::quad, concepts::make_sphere},
            transform::concepts::make_transformable,
        },
        prelude::*,
    };
    pub async fn init() {
        make_sphere().spawn();
    }
}

mod demo_shouter {
    use ambient_api::{
        core::{primitives::components::quad, transform::concepts::make_transformable},
        prelude::*,
    };
    pub async fn init() {
        Entity::new()
            .with_merge(make_transformable())
            .with(quad(), ())
            .spawn();

        sleep(1.).await; // necessary to allow embers to set themselves up before init messages are sent (so they're not missed!)

        run_async(async {
            loop {
                crate::embers::shouter::messages::NewShout {
                    key: 0,
                    pos: vec3(r(-2., 2.), r(-2., 2.), r(-1., 1.)),
                    txtmsg: "Hello".to_string(),
                }
                .send_local_broadcast(true);
                sleep(random::<f32>() * 0.5).await;
            }
        });
    }
    fn r(lowest: f32, highest: f32) -> f32 {
        return lowest + (highest - lowest) * random::<f32>();
    }
}
