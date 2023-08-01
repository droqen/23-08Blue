use ambient_api::prelude::*;
use ambient_api::core::transform::components::{translation, lookat_target};
use boatblue_selfie_camera::components::{selfie_ground_distance, selfie_ground_height, selfie_focus_ent, selfie_focus_offset, selfie_pitch, selfie_yaw};

#[main]
pub fn main() {
    query((selfie_ground_distance(), selfie_ground_height(), selfie_focus_ent())).each_frame(|selfiesticks|{
        for (stick,(dist, height, target_ent)) in selfiesticks {
            let target_pos : Vec3 = entity::get_component(
                target_ent,
                translation()
            ).expect("Selfie stick focus ent does not have a 'translation' component")
            + entity::get_component(stick, selfie_focus_offset()).unwrap_or(Vec3::ZERO);
            let pitch = entity::get_component(stick, selfie_pitch()).unwrap_or(0.);
            let yaw = entity::get_component(stick, selfie_yaw()).unwrap_or(0.);

            entity::add_component(stick, translation(), target_pos + Quat::from_rotation_z(-yaw) * (Quat::from_rotation_x(pitch) * vec3(0., dist, height)));
            entity::add_component(stick, lookat_target(), target_pos);
        }
    });
}