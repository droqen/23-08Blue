use ambient_api::prelude::*;
use ambient_api::core::transform::components::{lookat_target, translation};
use crate::embers::cam::components::{head_pitch, head_yaw,head_targetent, head_target_relpos};

pub fn setup() {
    query((head_targetent(), head_pitch(), head_yaw())).each_frame(|heads|
        for (head,(targetent, pitch, yaw)) in heads {
            let relpos = entity::get_component(head, head_target_relpos()).unwrap_or(Vec3::ZERO);
            if let Some(targetent_pos) = entity::get_component(targetent, translation()) {
                entity::add_component(head, lookat_target(), targetent_pos + Quat::from_rotation_z(-yaw) * (Quat::from_rotation_x(pitch) * relpos));
            } else {
                println!("INVALID head_targetent() entity - has no translation() component!");
            }
        }
    );
}