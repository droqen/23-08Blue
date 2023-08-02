use ambient_api::prelude::*;
use ambient_api::core::app::components::{name, main_scene};
use ambient_api::core::transform::components::translation;
use ambient_api::core::camera::components::aspect_ratio_from_window;
use ambient_api::core::camera::concepts::make_perspective_infinite_reverse_camera;
use boatblue_selfie_camera::components::{selfie_ground_distance, selfie_ground_height, selfie_focus_ent, selfie_focus_offset, selfie_pitch, selfie_yaw};
use boatblue_selfie_camera_demo::components::pitch_wobbler_time;

#[main]
pub fn main() {
    let entity_zero = Entity::new().with( translation(), vec3(0., 0., 0.) ).spawn();

    Entity::new()
        .with_merge( make_perspective_infinite_reverse_camera() )
        .with( name(), "Example Selfie Camera".to_string() )
        .with( main_scene(), () ) // is this needed?
        .with( aspect_ratio_from_window(), entity::resources() )
        
        .with( selfie_ground_distance(), 8.00 )
        .with( selfie_ground_height(), 3.50 )
        .with( selfie_focus_ent(), entity_zero )

        .with( selfie_focus_offset(), vec3(0., 0., 0.) ) // optional
        .with( selfie_pitch(), 0. ) // optional
        .with( selfie_yaw(), 0. ) // optional

        .with( pitch_wobbler_time(), 0. )

        .spawn();

    query(selfie_focus_ent()).requires(selfie_yaw()).each_frame(|selfie_sticks|{
        for (selfie, _) in selfie_sticks {
            entity::mutate_component(selfie, selfie_yaw(), |yaw|*yaw+=delta_time());
        }
    });

    query(pitch_wobbler_time()).requires(selfie_pitch()).each_frame(|wobblers|{
        for (selfie, time) in wobblers {
            entity::mutate_component(selfie, pitch_wobbler_time(), |t|*t+=delta_time()*2.61);
            entity::set_component(selfie, selfie_pitch(), time.sin() * 0.25);
        }
    });

}
