
use ambient_api::{
    core::{
        transform::components::translation,
        physics::components::linear_velocity,
        // 
    },
    prelude::*,
};

use crate::skatemouse::components::is_skatemouse;
pub fn setup() {
    physics::set_gravity(Vec3::ZERO);
    query((is_skatemouse(), translation(), linear_velocity())).each_frame(|mice|for(mouse,(_,pos,vel))in mice{
        physics::add_force(mouse, vec3(0., 0., -pos.z-vel.z));
    });
}
