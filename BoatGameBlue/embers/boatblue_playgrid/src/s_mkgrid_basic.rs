use std::f32::consts::PI;

use ambient_api::{
    prelude::*,
    core::{
        transform::components::{translation, rotation, scale},
        physics::components::{
            cube_collider,
            visualize_collider,
        },
        primitives::components::quad, rendering::components::color,
    }, entity::mutate_component
};
use crate::boatblue_playgrid::components::{
    grid_cell, grid_is_shack, grid_shack_door_dir,
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

    spawn_query(grid_cell()).requires(grid_is_shack()).bind(|cells|{
        for (id, _cell) in cells {
            // todo- check cell?
            if random::<f32>() < 0.25 {
                entity::add_component(id, grid_shack_door_dir(), random::<u8>()%2);
            }
        }
    });

}

pub fn setup_shack_colliders() {
    spawn_query(translation()).requires(grid_is_shack()).bind(|shack_cells|{
        for (id, pos) in shack_cells {
            let radius = 4.5; // whatever
            let height = 10.0; // whatever
            if let Some(door_side) = entity::get_component(id, grid_shack_door_dir()) {
                spawn_garage_colliders(id, pos, door_side, radius, height, 0.5, true);
            } else {
                entity::add_component(id, cube_collider(), vec3(radius, radius, height * 2.)); // should this be a child ent?
            }
        }
    });

    // wiggle to make visible
    query(()).requires((translation(), cube_collider())).each_frame(|cubes|
        for (cube, _) in cubes {
            entity::mutate_component(cube, translation(), |pos|pos.z -= 0.00000001);
        }
    );
}

fn spawn_garage_colliders(building_parent : EntityId, base_pos : Vec3, door_side : u8, radius : f32, height : f32, wall_thickness : f32, visualize_walls : bool ) {
    for side in 0..4 {
        if side == door_side {
            continue;
        }
        let wall = Entity::new()
            .with(translation(), base_pos + (side_to_dir(side).unwrap() * (radius - wall_thickness) * 0.5 + vec3(0., 0., height*0.5)) )
            .with(rotation(), side_to_rotation(side).unwrap())
            .with(cube_collider(), vec3(wall_thickness, 5., height))
            .spawn();
        entity::add_child(building_parent, wall);
        if visualize_walls {
            entity::add_component(wall, visualize_collider(), ());
        }
    }
}

pub fn setup_shack_models() {
    spawn_query(translation()).requires(grid_is_shack()).bind(|shack_cells|{
        for (id, pos) in shack_cells {
            let door_side = entity::get_component(id, grid_shack_door_dir());
            let radius = 2.20 + random::<f32>() * 0.05;
            let height = 2.5 + random::<f32>() * 5.0;
            for side in 0..4 {
                if door_side.is_some()&&door_side.unwrap()==side
                    {entity::add_child(id, spawn_shack_hole_wall(pos, side, radius, height));}
                else
                    {entity::add_child(id, spawn_shack_wall(pos, side, radius, height));}
            }
            entity::add_child(id, spawn_shack_roof(pos, radius, height));
        }
    });
}

fn spawn_shack_hole_wall(pos : Vec3, side : u8, radius : f32, height : f32) -> EntityId {
    Entity::new()
        .with(quad(), ())
        .with(translation(), pos + side_to_dir(side).expect("Invalid 'side' value") * radius + vec3(0., 0., height * 0.5))
        .with(rotation(), side_to_rotation(side).expect("Invalid 'side' value") * Quat::from_rotation_y(PI * 0.5))
        .with(scale(), vec3(height, radius * 2., 1.))
        .with(color(), vec4(0., 0., 0., 1.)) // entry wall is black
        .spawn()
}

fn spawn_shack_wall(pos : Vec3, side : u8, radius : f32, height : f32) -> EntityId {
    Entity::new()
        .with(quad(), ())
        .with(translation(), pos + side_to_dir(side).expect("Invalid 'side' value") * radius + vec3(0., 0., height * 0.5))
        .with(rotation(), side_to_rotation(side).expect("Invalid 'side' value") * Quat::from_rotation_y(PI * 0.5))
        .with(scale(), vec3(height, radius * 2., 1.))
        .spawn()
}

fn spawn_shack_roof(pos : Vec3, radius : f32, height : f32) -> EntityId {
    Entity::new()
        .with(quad(), ())
        .with(translation(), pos + vec3(0., 0., height))
        // .with(rotation(), side_to_rotation(side).expect("Invalid 'side' value") * Quat::from_rotation_y(PI * 0.5))
        .with(scale(), vec3(radius * 2., radius * 2., 1.))
        .spawn()
}

fn side_to_dir(side : u8) -> Option<Vec3> {
    match side {
        0 => { Some(vec3(1., 0., 0.)) }
        1 => { Some(vec3(0., 1., 0.)) }
        2 => { Some(vec3(-1., 0., 0.)) }
        3 => { Some(vec3(0., -1., 0.)) }
        _ => { None }
    }
}

fn side_to_rotation(side : u8) -> Option<Quat> {
    match side {
        0|1|2|3 => { Some(Quat::from_rotation_z(side as f32 * PI * 0.5)) }
        _ => { None }
    }
}

fn cell_to_translation(c : IVec2) -> Vec3 {
    (c.as_vec2() * 5.0).extend(0.0)
}