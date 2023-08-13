
use ambient_api::{
    core::{
        ecs::components::parent,
        primitives::components::cube,
    },
    prelude::*,
};
use crate::embers::ww_gen::components::block_decor_cube;

pub fn setup() {
    // remove all child cubes from block decors. // DOES NOT WORK CLIENTSIDE
    spawn_query(parent()).requires(cube()).bind(|childcubes|for(childcube,parent)in childcubes{
        if entity::has_component(parent, block_decor_cube()) { entity::remove_component(childcube, cube()); }
    });
}