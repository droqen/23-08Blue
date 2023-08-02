use ambient_api::prelude::*;

use ambient_api::core::app::components::name;
use ambient_api::core::transform::components::{translation, rotation};

use boatblue_boat::components::{
    boat_forward,
    //boat_forward_rotvel,
    image_of_boat,
};

#[main]
pub fn main() {
    spawn_query((translation(), boat_forward())).bind(|boats|{
        for(boat,(pos, fwd)) in boats {
            let boat_image = Entity::new()
                .with(name(), "Boat Image".to_string())
                .with(translation(), pos)
                .with(rotation(), fwd_to_quat(fwd))
                .with(image_of_boat(), boat)
                .spawn();
            entity::add_child(boat, boat_image);
        }
    });

    query(image_of_boat()).requires((translation(),rotation())).each_frame(|images|{
        for(image,boat) in images {
            // let rotvel = entity::get_component(boat, boat_forward_rotvel()).unwrap_or(0.);
            // let rotvel_tilt = Quat::from_rotation_y(rotvel) * 0.02;
            if let Some(fwd) = entity::get_component(boat, boat_forward()) {
                entity::mutate_component(image, rotation(), |rot|*rot=fwd_to_quat(fwd));
            }

            if let Some(pos) = entity::get_component(boat, translation()) {
                entity::set_component(image, translation(), pos);
            }
        }
    });
}

fn fwd_to_quat(fwd:Vec2) -> Quat {
    if let Some(fwd) = fwd.try_normalize() {
        Quat::from_rotation_arc_2d(vec2(0., 1.), fwd)
    } else {
        Quat::IDENTITY
    }
}

// fn rot_to_quat(rot:f32) -> Quat {
//     return Quat::from_rotation_z(rot);
// }