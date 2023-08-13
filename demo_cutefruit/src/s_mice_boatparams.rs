use ambient_api::{
    prelude::*,
    core::transform::components::translation,
};

use crate::embers::skatemouse::components::is_skatemouse;
use crate::embers::skatemouse_demo::components::{autofloat_pwr, autofloat_z};
use crate::embers::buoy::components::buoy_local_center;

pub fn setup() {
    spawn_query(is_skatemouse()).bind(|mice|for(mouse,_)in mice{
        entity::add_components(
            mouse, 
            make_buoy()
                .with(buoy_local_center(), vec3(0., 0., -0.5))
                .with(buoy_max_force(), 9.81 * 5.)
                .with(buoy_radius(), 1.0)
        );
    });
    spawn_query(()).requires((autofloat_pwr(), autofloat_z())).bind(|mice|for(mouse,_)in mice{
        entity::remove_component(mouse, autofloat_pwr());
        entity::remove_component(mouse, autofloat_z());
    });
}

use crate::embers::buoy::components::{buoy_max_drag,buoy_max_force,buoy_radius,buoy_water_level,};
pub fn make_buoy() -> Entity {
    Entity::new()
        .with(buoy_max_drag(), 1.0)
        .with(buoy_max_force(), 9.81 * 2.)
        .with(buoy_radius(), 0.5)
        .with(buoy_water_level(), 0.)
}