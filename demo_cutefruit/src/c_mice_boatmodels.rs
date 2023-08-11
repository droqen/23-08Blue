use ambient_api::{
    prelude::*,
    core::{
        model::components::model_from_url,
        // 
    },
};

use crate::embers::skatemouse::components::{mouse_spr_fwd, is_mouse_spr, };

pub fn setup() {
    spawn_query(()).requires(is_mouse_spr()).bind(|msprs|for(mouse_spr,_) in msprs{
        println!("Changing mouse spr");
        entity::add_component(mouse_spr, model_from_url(), crate::embers::demo_cutefruit::assets::url("MSH_Boat.glb"));
        entity::add_component(mouse_spr, mouse_spr_fwd(), vec3(0., 1., 0.));
    });
}