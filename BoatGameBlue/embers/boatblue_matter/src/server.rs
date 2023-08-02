use ambient_api::prelude::*;
use ambient_api::core::physics::components::{linear_velocity, angular_velocity};
use ambient_api::core::transform::components::{translation, rotation};
use boatblue_matter::components::{
    // is_matter,
    matter_gravity, matter_local_center,
    // matter_is_buoyant,
    buoy_radius, buoy_water_level, buoy_max_force, buoy_max_drag,
    buoy_submerged, buoy_submerged_center,
};

#[main]
pub fn main() {
    query((translation(), buoy_radius(), buoy_water_level())).each_frame(|buoys|{
        for (buoy,(pos,b_radius,b_water_level)) in buoys {
            let submerged = get_submerged_percentage(pos.z, b_radius, b_water_level);
            if submerged > 0. {
                entity::add_component(buoy, buoy_submerged(),
                    submerged);
                entity::add_component(buoy, buoy_submerged_center(),
                    get_submerged_center_point(pos, b_water_level, b_radius, submerged).unwrap());
            } else {
                entity::remove_component(buoy, buoy_submerged());
                entity::remove_component(buoy, buoy_submerged_center());
            }
        }
    });

    query(
        matter_gravity()
    ).each_frame(|buoys|{
        for (floaty_ent,gravity) in buoys {
            entity::mutate_component(floaty_ent, linear_velocity(), |linvel|*linvel += vec3(0., 0., -gravity * delta_time()));
        }
    });

    query((
        translation(),
        rotation(),
        matter_local_center(),
        buoy_submerged(),
        buoy_submerged_center(),
        buoy_max_force(),
        buoy_max_drag(),
        linear_velocity(),
    )).each_frame(|buoys|{
        for(floaty_ent,(
            pos,
            rot,
            m_center,
            submerged,
            submerged_center,
            b_max_force,
            b_max_friction,
            linvel,
        )) in buoys {

            if submerged > 0.0 {
                let b_force = vec3(0.,0.,1.) * b_max_force * submerged * delta_time();
                let b_friction = b_max_friction * submerged * delta_time();
                let b_friction_linvel_force = linvel * (-1.) * b_friction;
                add_force_at_position(floaty_ent, pos + rot*m_center, b_force + b_friction_linvel_force, submerged_center);
                // let b_friction_angvel_force = angvel * (-1.) * b_friction;
                entity::mutate_component(floaty_ent, angular_velocity(), |angvel|*angvel *= 1.0 - b_friction);
            }

        }
    });
}

fn add_force_at_position(ent : EntityId, ent_mass_center_pos : Vec3, force : Vec3, force_point : Vec3) {
    entity::mutate_component(ent, linear_velocity(), |linvel|*linvel += force);
    entity::mutate_component(ent, angular_velocity(), |angvel|*angvel += (force_point - ent_mass_center_pos).cross(force));
}


// currently assumes object is basically a cube or cylinder.
// todo - could assume object is sphere?
// todo - could allow for any arbitrary shape?
// also todo - use a flexible water plane rather than a rigid water level? not sure
fn get_submerged_percentage(z : f32, radius : f32, water_level : f32) -> f32 {
    if z < water_level-radius { return 1.00; }
    if z > water_level+radius { return 0.00; }
    if radius.abs() < 0.0001 { return 0.50; }
    let minusone_to_one = (water_level - z) / radius;
    let zero_to_one = minusone_to_one * 0.5 + 0.5;
    return zero_to_one;
}


fn get_submerged_center_point(pos : Vec3, water_level : f32, radius : f32, submerged_percentage : f32) -> Option<Vec3> {
    if submerged_percentage <= 0.0 { return None; }
    let diameter = radius * 2.;
    return Some(vec3(pos.x, pos.y, water_level - diameter * submerged_percentage));
}