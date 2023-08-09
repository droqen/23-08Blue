use ambient_api::prelude::*;

use crate::a_nice_overhead_camera::components::{
    head_relpos,
    the_nice_camera,
    nice_yaw_pitch_tilting,
    nice_yaw_pitch_tilting_base,
};

pub fn setup() {
    spawn_query(the_nice_camera()).bind(|cameras|for (camera,_) in cameras{
        entity::add_components(camera, 
            Entity::new()
                .with(head_relpos(), vec3(0., 50., 25.))
                // .with(nice_yaw_pitch_tilting_base(), vec2())
                .with(nice_yaw_pitch_tilting(), vec2(0.25, -0.25))
        );
    });
}