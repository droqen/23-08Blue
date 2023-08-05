use std::f32::consts::PI;

use ambient_api::{
    prelude::*,
    core::{
        transform::components::{translation, rotation, scale},
        physics::components::{
            cube_collider,
            visualize_collider,
        },
        primitives::components::quad, rendering::components::color, model::components::model_from_url,
    }
};
use crate::boatblue_playgrid::components::{
    grid_cell, grid_is_shack, grid_shack_door_dir,
    grow_chunk_pos, grow_chunk_height_left,
};

pub fn setup_grid_entities() {

    for x in -10..1 { for y in -10..1 {
        let mut score = 0;
        if random::<f32>()<0.35 { score += 1; }
        if x % 2 == 0 { score += 1; }
        if y % 2 == 0 { score += 1; }
        if score >= 2 {
            Entity::new()
                .with(grid_cell(), ivec2(x, y))
                .with(grid_is_shack(), ())
                // .with(visualize_collider(), ())
                .spawn();
        }
    }}

    spawn_query(grid_cell()).bind(|cells|{
        for (id,cell) in cells {
            entity::add_component(id, translation(), cell_to_translation(cell));
        }
    });

    spawn_query((translation(), grid_cell())).requires(grid_is_shack()).bind(|cells|{
        for (id, (pos, _cell)) in cells {

            entity::add_component(id, grow_chunk_pos(), pos);
            entity::add_component(id, grow_chunk_height_left(), 6.0 + 6.0 * random::<f32>());

            // todo- check cell? is it free?
            if random::<f32>() < 0.25 {
                entity::add_component(id, grid_shack_door_dir(), random::<u8>()%2); // door shack
            } else if random::<f32>() < 0.25 {
                entity::set_component(id, grow_chunk_height_left(), 2.0 + 4.0 * random::<f32>()); // extra short lil building
            }

        }
    });

}

pub fn setup_shack_colliders() {
    spawn_query(translation()).requires(grid_is_shack()).bind(|shack_cells|{
        for (id, pos) in shack_cells {
            let height = 10.0; // whatever
            if let Some(door_side) = entity::get_component(id, grid_shack_door_dir()) {
                spawn_garage_colliders(id, pos, door_side, GRID_RADIUS, height, 0.5, false);
            } else {
                entity::add_component(id, cube_collider(), vec3(GRID_SIZE, GRID_SIZE, height * 2.)); // should this be a child ent?
                // entity::add_component(id, visualize_collider(), ()); // should this be a child ent?
            }
        }
    });

    // // wiggle to make visible
    // query(()).requires((translation(), cube_collider())).each_frame(|cubes|
    //     for (cube, _) in cubes {
    //         entity::mutate_component(cube, translation(), |pos|pos.z -= 0.01 );
    //     }
    // );
}

fn spawn_garage_colliders(building_parent : EntityId, base_pos : Vec3, door_side : u8, radius : f32, height : f32, wall_thickness : f32, visualize_walls : bool ) {
    for side in 0..4 {
        if side == door_side%4 {
            continue;
        }
        let wall = Entity::new()
            .with(translation(), base_pos + (side_to_dir(side) * (radius - wall_thickness * 0.5) + vec3(0., 0., height*0.5)) )
            .with(rotation(), side_to_rotation(side))
            .with(cube_collider(), vec3(GRID_SIZE, wall_thickness, height))
            .spawn();
        entity::add_child(building_parent, wall);
        if visualize_walls {
            entity::add_component(wall, visualize_collider(), ());
        }
    }
}

pub fn setup_shack_models() {
    spawn_query((grow_chunk_pos(), grow_chunk_height_left())).bind(|growroots|
        for (root, (pos, height_left)) in growroots {
            let mut possible_chunks : Vec<ShackChunk> = vec![];
            if pos.z < 1. && entity::has_component(root, grid_shack_door_dir()) {
                possible_chunks.push(ShackChunk::TallWithHole); // bottom floor: must
            } else {
                if height_left >= 6.00 { possible_chunks.push(ShackChunk::Tall); }
                if height_left >= 1.85 { possible_chunks.push(ShackChunk::Short); }
            }
            if possible_chunks.is_empty() {
                spawn_chunk(root, ShackChunk::Rooftop, pos);
                possible_chunks.push(ShackChunk::Rooftop);
            } else {
                let chunk_type = possible_chunks[random::<usize>()%possible_chunks.len()];
                let chunk_height = match chunk_type {
                    ShackChunk::Rooftop => 0.,
                    ShackChunk::Separator => 0.,
                    ShackChunk::Short => 1.85,
                    ShackChunk::Tall => 6.00,
                    ShackChunk::TallWithHole => 6.00,
                };

                // if pos.z >= 1. && chunk_height > 1. {
                //     spawn_chunk(root, ShackChunk::Separator, pos);
                // }

                let chunk_ent = spawn_chunk(root, chunk_type, pos);
                
                entity::add_component(chunk_ent.clone(), grow_chunk_pos(), pos + vec3(0., 0., chunk_height));
                entity::add_component(chunk_ent.clone(), grow_chunk_height_left(), height_left - chunk_height);

            }

            entity::remove_component(root, grow_chunk_pos());
            entity::remove_component(root, grow_chunk_height_left());
        }
    );
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum ShackChunk {
    Short, Tall, TallWithHole, Rooftop, Separator,
}

const GRID_SIZE : f32 = 6.00;
const GRID_RADIUS : f32 = GRID_SIZE * 0.5;
// const GRID_RAD_HALF : f32 = GRID_SIZE * 0.25;
// const GRID_RAD_QUARTER : f32 = GRID_SIZE * 0.125;

fn spawn_chunk(base_ent : EntityId, chunktype : ShackChunk, pos : Vec3) -> EntityId {
    let chunk_ent : EntityId = Entity::new().spawn();
    entity::add_child(base_ent, chunk_ent);
    match chunktype {
        ShackChunk::Short => for side in 0..3 { spawn_wall(chunk_ent, ShackWall::Short, pos, side); },
        ShackChunk::Tall => for side in 0..3 { spawn_wall(chunk_ent, ShackWall::Tall, pos, side); },
        ShackChunk::TallWithHole => {
            let door_side = entity::get_component(base_ent, grid_shack_door_dir());
            for side in 0..3 {
                spawn_wall(
                    chunk_ent, 
                    match Some(side)==door_side{true=>ShackWall::TallWithHole, false=>ShackWall::Tall},
                    pos, side
                );
            }
        },
        ShackChunk::Rooftop => spawn_roof(chunk_ent, pos),
        ShackChunk::Separator => spawn_separator(chunk_ent, pos),
    }
    return chunk_ent;
}

enum ShackWall {
    Short, Tall, TallWithHole,
}

fn spawn_wall(chunk_ent : EntityId, walltype : ShackWall, chunk_pos : Vec3, wall_side : u8) {
    let wall_model = Entity::new()
        .with(translation(), chunk_pos + side_to_dir(wall_side) * GRID_RADIUS)
        .with(rotation(), side_to_rotation(wall_side))
        .spawn();

    match walltype {
        ShackWall::Short => entity::add_component(wall_model, model_from_url(), asset::url("boatblue_playgrid/assets/MSH_Building_Wall_Sml_001.glb").unwrap()),
        ShackWall::Tall => entity::add_component(wall_model, model_from_url(), asset::url("boatblue_playgrid/assets/MSH_Building_Wall_Lrg_001.glb").unwrap()),
        ShackWall::TallWithHole => {
            entity::add_component(wall_model, model_from_url(), asset::url("boatblue_playgrid/assets/MSH_Building_Wall_Lrg_Door_001.glb").unwrap());
            let darkness_plane = Entity::new()
                .with(quad(), ())
                .with(translation(), chunk_pos + side_to_dir(wall_side) * (GRID_RADIUS - 0.2))
                .with(rotation(), side_to_rotation(wall_side+2) * Quat::from_rotation_x(PI*0.5) )
                .with(scale(), vec3(4., 10., 1.))
                .with(color(), vec4(0., 0., 0., 1.))
                .spawn();
            entity::add_child(chunk_ent, darkness_plane)
        },
    }

    match walltype {
        ShackWall::TallWithHole => entity::add_component(chunk_ent, color(), vec4(0., 0., 0., 0.)),
        _ => {},
    }

    entity::add_child(chunk_ent, wall_model);
}

fn spawn_separator(grident : EntityId, chunk_pos : Vec3) {
    entity::add_child(
        grident, 
        Entity::new()
        .with(model_from_url(), asset::url("boatblue_playgrid/assets/MSH_Building_Seperator_001.glb").unwrap())
            .with(translation(), chunk_pos)
            .spawn()
    );
}

fn spawn_roof(grident : EntityId, chunk_pos : Vec3) {
    entity::add_child(
        grident, 
        Entity::new()
        .with(model_from_url(), asset::url("boatblue_playgrid/assets/MSH_Building_Roof_001.glb").unwrap())
            .with(translation(), chunk_pos)
            .spawn()
    );
}

// fn spawn_shack_hole_wall(pos : Vec3, side : u8, radius : f32, height : f32) -> EntityId {
//     Entity::new()
//         .with(quad(), ())
//         .with(translation(), pos + side_to_dir(side) * radius + vec3(0., 0., height * 0.5))
//         .with(rotation(), side_to_rotation(side) * Quat::from_rotation_y(PI * 0.5))
//         .with(scale(), vec3(height, radius * 2., 1.))
//         .with(color(), vec4(0., 0., 0., 1.)) // entry wall is black
//         .spawn()
// }

// fn spawn_shack_wall(pos : Vec3, side : u8, radius : f32, height : f32) -> EntityId {
//     Entity::new()
//         .with(quad(), ())
//         .with(translation(), pos + side_to_dir(side) * radius + vec3(0., 0., height * 0.5))
//         .with(rotation(), side_to_rotation(side) * Quat::from_rotation_y(PI * 0.5))
//         .with(scale(), vec3(height, radius * 2., 1.))
//         .spawn()
// }

// fn spawn_shack_roof(pos : Vec3, radius : f32, height : f32) -> EntityId {
//     Entity::new()
//         .with(quad(), ())
//         .with(translation(), pos + vec3(0., 0., height))
//         // .with(rotation(), side_to_rotation(side) * Quat::from_rotation_y(PI * 0.5))
//         .with(scale(), vec3(radius * 2., radius * 2., 1.))
//         .spawn()
// }

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

fn cell_to_translation(c : IVec2) -> Vec3 {
    (c.as_vec2() * GRID_SIZE).extend(0.0)
}