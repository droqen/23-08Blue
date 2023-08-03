use std::f32::consts::PI;

use ambient_api::prelude::*;
use ambient_api::core::{
    transform::components::translation,
    physics::components::linear_velocity,
};

use boatblue_boat::components::{boat_vel, boat_steer, boat_forward, boat_forward_rotvel, };
use boatblue_boat::components::{boat_stat_speed, boat_stat_accel, };
use boatblue_matter::components::buoy_submerged;

#[main]
pub fn main() {
    
    query((translation(), boat_steer(), boat_vel(), boat_forward(), boat_forward_rotvel())).each_frame(|gliders|{
        for (glider, (_pos, steer_vec, _vel, fwd, rotvel)) in gliders {

            // let sub = entity::get_component(glider, buoy_submerged())

            let control = match entity::get_component(glider, buoy_submerged()) {
                None => 0.0,
                Some(sub) => invlerp(0.01, 0.4, sub).clamp(0., 1.)
            }
                * entity::get_component(glider, boat_stat_accel()).unwrap_or(1.00);

            let accellin = 10.0 * delta_time() * control;
            let accellerp = 0.; // 0.01 * control;
            let desired_landvel : Vec2 = steer_vec
                * 20.
                * entity::get_component(glider, boat_stat_speed()).unwrap_or(1.00);

            // entity::set_component(glider, glider_forward(), desired_landvel.extend(1.0));
            
            entity::mutate_component(glider, linear_velocity(), move |linvel|{
                let to_desired_landvel : Vec2 = desired_landvel - linvel.truncate();
                if to_desired_landvel.length_squared() < accellin * accellin {
                    *linvel = (
                        desired_landvel
                    ).extend(linvel.z);
                } else {
                    *linvel = (
                        linvel.xy()
                        + to_desired_landvel.clamp_length_max(accellin)
                        + to_desired_landvel * accellerp
                    ).extend(linvel.z);
                }

                entity::set_component(glider, boat_vel(), linvel.xy());
            });

            if steer_vec.length_squared() > 0.01 {
                let angle_to_fwd = fwd.angle_between(steer_vec);
                // let mut angle_to_fwd = fwd.angle_between(steer_vec);
                // println!("A{angle_to_fwd}");
                // if angle_to_fwd > PI * 0.8 { angle_to_fwd = -PI + angle_to_fwd; }
                // if angle_to_fwd < -PI * 0.8 { angle_to_fwd = PI + angle_to_fwd; }
                // println!("B{angle_to_fwd}");
                let desired_rotvel = (angle_to_fwd * 5.0).clamp(-PI, PI);
                let to_desired_rotvel = desired_rotvel - rotvel;
                entity::mutate_component(glider, boat_forward_rotvel(), |rotvel|*rotvel += to_desired_rotvel*0.3);
            } else {
                entity::mutate_component(glider, boat_forward_rotvel(), |rotvel|*rotvel *= 0.7); // friction i suppose?
            }
        }
    });

    query(boat_forward_rotvel()).each_frame(|gliders|{
        for(glider,fwd_rotvel) in gliders {
            entity::mutate_component(glider, boat_forward(), |forward|*forward=forward.rotate(Vec2::from_angle(fwd_rotvel*delta_time())));
        }
    });

}

// fn lerp(from : f32, to : f32, rel : f32) -> f32 { ((1. - rel) * from) + (rel * to) }
fn invlerp(from : f32, to : f32, value : f32) -> f32 { (value - from) / (to - from) }