
use std::f32::consts::PI;

use ambient_api::{
    core::{
        ecs::components::parent,
        transform::components::{translation, rotation},
        model::components::model_from_url,
    },
    prelude::*,
};
use crate::embers::ww_gen::components::{block_size, block_door_side, block_decor_dan1};
use crate::embers::dans::assets;
pub fn setup() {
    spawn_query((translation(), block_size())).requires(block_decor_dan1()).bind(|blocks|for(block,(pos,size))in blocks{
        // let radius : f32 = size.x.max(size.y) * 0.5;
        let _vis_height : f32 = size.z; // variable.
        if let Some(mousehole_side) = entity::get_component(block, block_door_side()) {
            spawn_garage_model(block, pos, mousehole_side);
        } else {
            spawn_solid_model(block, pos);
        }
    });
}

fn spawn_garage_model(building_parent : EntityId, base_pos : Vec3, door_side : u8) {
    let model = Entity::new()
        .with(translation(), base_pos)
        .with(rotation(), side_to_rotation(door_side))
        .with(model_from_url(), assets::url(format!("Msh_Building_Hole_{:0>3}.glb", random::<u8>() % 5 + 1).as_mut_str()))
        .spawn();
    entity::add_child(building_parent, model);
}

fn spawn_solid_model(building_parent : EntityId, base_pos : Vec3) {
    let model = Entity::new()
        .with(translation(), base_pos)
        .with(rotation(), side_to_rotation(random::<u8>() % 4))
        .with(model_from_url(), assets::url(format!("Msh_Building_{:0>3}.glb", random::<u8>() % 15 + 1).as_mut_str()))
        .spawn();
    entity::add_child(building_parent, model);
}

fn side_to_dir(side : u8) -> Vec3 {
    match side%4 {
        0 => { vec3(1., 0., 0.) }
        1 => { vec3(0., 1., 0.) }
        2 => { vec3(-1., 0., 0.) }
        3 => { vec3(0., -1., 0.) }
        _ => { panic!() }
    }
}

fn side_to_rotation(side : u8) -> Quat {
    Quat::from_rotation_z((side%4 + 3) as f32 * PI * 0.5)
}