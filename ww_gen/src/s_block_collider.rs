use std::f32::consts::PI;

use ambient_api::{
    core::{
        primitives::components::cube,
        transform::components::{translation, rotation, scale},
        physics::components::cube_collider,
    },
    prelude::*,
};
use crate::ww_gen::components::{block_size, block_door_side, block_decor_cube};

pub fn setup() {
    let collider_height : f32 = 10.; // fixed.
    let wall_thickness : f32 = 0.2;
    spawn_query((translation(), block_size())).bind(move |blocks|for (block, (pos, size)) in blocks {
        let radius : f32 = size.x.max(size.y) * 0.5;
        let _vis_height : f32 = size.z; // variable.
        let visualize : bool = entity::has_component(block, block_decor_cube());
        if let Some(mousehole_side) = entity::get_component(block, block_door_side()) {
            spawn_garage_colliders(block, pos, radius, collider_height, mousehole_side, wall_thickness, visualize);
        } else {
            spawn_solid_colliders(block, pos, radius, collider_height, visualize);
        }
    });
}

fn spawn_garage_colliders(building_parent : EntityId, base_pos : Vec3, radius : f32, height : f32, door_side : u8, wall_thickness : f32, visualize : bool ) {
    for side in 0..4 {
        if side == door_side%4 {
            continue;
        }
        let wall = Entity::new()
            .with(translation(), base_pos + (side_to_dir(side) * (radius - wall_thickness * 0.5) + vec3(0., 0., height*0.5)) )
            .with(rotation(), side_to_rotation(side))
            .with(scale(), vec3(radius+radius, wall_thickness, height))
            .with(cube_collider(), vec3(1.,1.,1.))
            .spawn();
        entity::add_child(building_parent, wall);
        if visualize {
            entity::add_component(wall, cube(), ());
        }
    }
}

fn spawn_solid_colliders(building_parent : EntityId, base_pos : Vec3, radius : f32, height : f32, visualize : bool ) {
    let solid = Entity::new()
        .with(translation(), base_pos + vec3(0., 0., height*0.5))
        .with(scale(), vec3(radius+radius, radius+radius, height))
        .with(cube_collider(), vec3(1.,1.,1.))
        .spawn();
    entity::add_child(building_parent, solid);
    if visualize {
        entity::add_component(solid, cube(), ());
    }
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