use ambient_api::prelude::*;
use ambient_api::core::transform::components::translation;
use crate::embers::buoy::components::{buoy_submerged, buoy_submerged_center, buoy_radius, buoy_water_level};

pub fn setup() {
    query((translation(), buoy_radius(), buoy_water_level())).each_frame(|buoys|{
        for (buoy,(pos,b_radius,b_water_level)) in buoys {
            let submerged = calc_submerged_percentage(pos.z, b_radius, b_water_level);
            if submerged > 0. {
                let subcenter = calc_submerged_center_point(pos, b_water_level, b_radius, submerged).unwrap();
                entity::add_component(buoy, buoy_submerged(),
                    submerged);
                entity::add_component(buoy, buoy_submerged_center(),
                    subcenter);
                let vel = physics::get_velocity_at_position(buoy, subcenter);
                physics::add_force_at_position(buoy, -vel * submerged * 0.1, subcenter);
            } else {
                entity::remove_component(buoy, buoy_submerged());
                entity::remove_component(buoy, buoy_submerged_center());
            }
        }
    });
}

// currently assumes object is basically a cube or cylinder.
// todo - could assume object is sphere?
// todo - could allow for any arbitrary shape?
// also todo - use a flexible water plane rather than a rigid water level? not sure
pub fn calc_submerged_percentage(z : f32, radius : f32, water_level : f32) -> f32 {
    if z < water_level-radius { return 1.00; }
    if z > water_level+radius { return 0.00; }
    if radius.abs() < 0.0001 { return 0.50; }
    let minusone_to_one = (water_level - z) / radius;
    let zero_to_one = minusone_to_one * 0.5 + 0.5;
    return zero_to_one;
}

pub fn calc_submerged_center_point(pos : Vec3, water_level : f32, radius : f32, submerged_percentage : f32) -> Option<Vec3> {
    if submerged_percentage <= 0.0 { return None; }
    let diameter = radius * 2.;
    return Some(vec3(pos.x, pos.y, water_level - diameter * submerged_percentage));
}