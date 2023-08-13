
use ambient_api::{
    core::{
        transform::components::translation,
        physics::components::linear_velocity,
        // 
    },
    prelude::*,
};

use crate::embers::skatemouse::components::is_skatemouse;
use crate::embers::demo::components::{autofloat_z, autofloat_pwr};
pub fn setup() {
    query((is_skatemouse(), translation(), linear_velocity(), autofloat_z(), autofloat_pwr())).each_frame(|mice|for(mouse,(_,pos,vel,floatz,floatpow))in mice{
        physics::add_force( mouse, vec3( 0., 0., floatpow * (floatz-pos.z-vel.z) ) );
    });
}
