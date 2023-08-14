use ambient_api::prelude::*;

mod s_buoyant_force;
mod s_submerged;
mod s_make;

#[main]
pub fn main() {
    s_buoyant_force::setup();
    s_submerged::setup();
}

pub fn make_buoy() -> Entity // todo- this could/should be a concept? this is easier though...
    { s_make::make_buoy() }

pub fn add_force_at_position(ent : EntityId, ent_mass_center_offset : Vec3, force : Vec3, force_point : Vec3)
    { s_buoyant_force::add_force_at_position(ent, ent_mass_center_offset, force, force_point) }

pub fn calc_submerged_percentage(z : f32, radius : f32, water_level : f32) -> f32
    { s_submerged::calc_submerged_percentage(z, radius, water_level) }

pub fn calc_submerged_center_point(pos : Vec3, water_level : f32, radius : f32, submerged_percentage : f32) -> Option<Vec3>
    { s_submerged::calc_submerged_center_point(pos, water_level, radius, submerged_percentage) }