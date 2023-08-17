use ambient_api::prelude::*;
use crate::embers::cam::components::{cam_tilt, head_pitch, head_yaw, };

pub fn setup() {
    // applying tilt, smoothed slightly
    query(cam_tilt()).each_frame(|cams|{
        for (cam,tilt) in cams {
            entity::mutate_component_with_default(cam, head_yaw(), tilt.x, |yaw|*yaw=lerp(*yaw,tilt.x,0.35));
            entity::mutate_component_with_default(cam, head_pitch(), tilt.y, |pitch|*pitch=lerp(*pitch,tilt.y,0.35));
        }
    });
}

fn lerp(from : f32, to : f32, rel : f32) -> f32 { ((1. - rel) * from) + (rel * to) }