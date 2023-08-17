use ambient_api::prelude::*;
use ambient_api::core::transform::components::{lookat_target, translation};
use crate::embers::cam::components::{head_pitch, head_yaw, head_relpos};

pub fn setup() {
    query((lookat_target(), head_pitch(), head_yaw(), head_relpos())).each_frame(|heads|
        for (head,(locus, pitch, yaw, relpos)) in heads {
            entity::add_component(head, translation(), locus + Quat::from_rotation_z(yaw) * (Quat::from_rotation_x(pitch) * relpos));
        }
    );
}