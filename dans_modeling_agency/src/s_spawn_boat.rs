use ambient_api::{
    core::{
        model::components::model_from_url,
        // 
    },
    prelude::*,
};

pub fn setup() {
    Entity::new()
        .with(model_from_url(), crate::embers::dans_modeling_agency::assets::url("MSH_Boat.glb"))
        .spawn();
}