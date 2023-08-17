use std::f32::consts::PI;

use ambient_api::{prelude::*, core::transform::components::{translation, scale}};

use crate::embers::shouter::components::{
    shout_key,shout_prog,shout_base_pos,shout_text_center,
};

pub fn setup() {
    query((shout_base_pos(), shout_prog())).each_frame(|shouts|for(shout,(base_pos,prog))in shouts{
        let prog2 = prog + delta_time() * 0.5;
        if prog2 >= 1. {
            entity::despawn(shout);
        } else {
            let center = entity::get_component(shout, shout_text_center()).unwrap_or(Vec3::ZERO);
            entity::set_component(shout, shout_prog(), prog2);
            let desired_scale = Vec3::splat(0.03 + 0.07 * (prog2*PI).sin());
            entity::add_component(shout, scale(), desired_scale);
            entity::add_component(shout, translation(), base_pos + vec3(0., (prog2*PI).sin(), 0.) - desired_scale * center);
        }
    });
}