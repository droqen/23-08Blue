use ambient_api::prelude::*;
use crate::embers::buoy::components::{buoy_max_drag,buoy_max_force,buoy_radius,buoy_water_level};

pub fn make_buoy() -> Entity {
    Entity::new()
        .with(buoy_max_drag(), 1.0)
        .with(buoy_max_force(), 9.81 * 2.)
        .with(buoy_radius(), 0.5)
        .with(buoy_water_level(), 0.)
}