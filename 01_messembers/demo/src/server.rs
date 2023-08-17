use ambient_api::{
    core::{
        primitives::components::quad,
        transform::{
            // components::{lookat_target, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

#[main]
pub async fn main() {
    Entity::new()
        .with_merge(make_transformable())
        .with(quad(), ())
        .spawn();

    sleep(1.).await; // necessary to allow embers to set themselves up before init messages are sent (so they're not missed!)

    run_async(async{loop{
        embers::shouter::messages::NewShout{key:0, pos:vec3(r(-2., 2.), r(-2., 2.), r(-1., 1.)), txtmsg:"Hello".to_string()}.send_local_broadcast(true);
        sleep(random::<f32>()*0.5).await;
    }});
}

fn r(lowest:f32, highest:f32)->f32 {
    return lowest + (highest-lowest)*random::<f32>();
}