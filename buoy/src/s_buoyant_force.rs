use ambient_api::{
    prelude::*,
    core::{
        transform::components::{translation, rotation},
        physics::components::{linear_velocity, angular_velocity},
    },
};

use crate::buoy::components::{buoy_local_center,buoy_max_drag,buoy_max_force,buoy_radius,buoy_submerged,buoy_submerged_center,buoy_water_level};

pub fn setup() {
    query((
        translation(),
        rotation(),
        buoy_submerged(),
        buoy_submerged_center(),
        buoy_max_force(),
        buoy_max_drag(),
        linear_velocity(),
    )).each_frame(|buoys|{
        for(floaty_ent,(
            pos,
            rot,
            submerged,
            submerged_center,
            b_max_force,
            b_max_friction,
            linvel,
        )) in buoys {
            if submerged > 0.0 {
                let b_force = vec3(0.,0.,1.) * b_max_force * submerged;
                let b_friction = b_max_friction * submerged;
                let b_friction_linvel_force = linvel * (-1.) * b_friction;
                match entity::get_component(floaty_ent, buoy_local_center()) {
                    Some(m_center) => add_force_at_position(
                        floaty_ent, pos + rot * m_center, b_force + b_friction_linvel_force, submerged_center),
                    None => add_force_at_position(
                        floaty_ent, pos, b_force + b_friction_linvel_force, submerged_center),
                };
                // let b_friction_angvel_force = angvel * (-1.) * b_friction;
                entity::mutate_component(floaty_ent, angular_velocity(), |angvel|*angvel *= 1.0 - b_friction); //?
            }
        }
    });
}

pub fn add_force_at_position(ent : EntityId, ent_mass_center_pos : Vec3, force : Vec3, force_point : Vec3) {
    physics::add_force_at_position(ent, force, force_point - ent_mass_center_pos);
    // entity::mutate_component(ent, linear_velocity(), |linvel|*linvel += force);
    // entity::mutate_component(ent, angular_velocity(), |angvel|*angvel += (force_point - ent_mass_center_pos).cross(force));
}