
pub fn setup() -> Listener {
    crate::embers::skater::messages::NewSkater::subscribe(|_src,msg|{
        Entity::new()
            .with(skater_key(), msg.key)
            .with(translation(), msg.pos)
            .with(skater_target(), msg.pos)
            .with(skater_fwd(), vec3(0., 1., 0.))
            .with(skater_pace(), 0.)
            .spawn();
    })
}

use ambient_api::{
    core::transform::components::translation,
    prelude::*, message::Listener
};

use crate::embers::skater::components::{skater_key, skater_target, skater_fwd, skater_pace};