use ambient_api::prelude::*;
use crate::a_nice_overhead_camera::components::{the_nice_camera, nice_yaw_pitch_tilting, nice_yaw_pitch_tilting_base, head_relpos};
// use crate::x_head::make_fully_functional_head_camera;

use std::f32::consts::PI;

pub fn setup() {
    spawn_query(the_nice_camera()).bind(|cameras|for(camera, _)in cameras{
        // head_relpos: y = move back, z = move up
        entity::add_component(camera, head_relpos(), vec3(0., 0., 0.));
        
        // nice_yaw_pitch_tilting_base: x = base yaw, y = base pitch
        entity::add_component(camera, nice_yaw_pitch_tilting_base(), vec2(PI * 0.25, 0.));

        // nice_yaw_pitch_tilting: x = max yaw tilt, y = max pitch tilt (using mouse)
        entity::add_component(camera, nice_yaw_pitch_tilting(), vec2(0.25, 0.25));
    });
}