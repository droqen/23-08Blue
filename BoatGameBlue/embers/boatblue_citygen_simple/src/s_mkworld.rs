
fn lerp(a:f32,b:f32,t:f32) -> f32 { a*(1.-t)+b*t }

pub fn just_do_it() {

    // let parent_query: GeneralQuery<Component<()>> = query(is_mkworld_parent()).build();

    gen_water();

    // crate::messages::GenerateWorld::subscribe(move |_src,_msg|{
    // let parent = spawn_parent(parent_query);
    
    for cell in gen_cells_azoa() {
        let _building = spawn_building_at(cell.as_vec2().extend(0.0) * 5.);
        // entity::add_child(parent, building);
    }
    // });
}


pub fn gen_cells_azoa() -> Vec<IVec2> {
    let mut cells = Vec::<IVec2>::new();
    for x in -10..10 {
        for y in -10..10 {
            if (
                (x % 2 == 0 || random::<f32>()<0.1)
                &&
                (y % 2 == 0 || random::<f32>()<0.1)
            ) && random::<f32>()<0.9 {
                cells.push(ivec2(x,y));
            }
        }
    }
    return cells;
}

pub fn spawn_pillar_at(
    base_pos : Vec3,
    width : f32,
    height : f32,
    depth : f32,
) -> EntityId {
    return Entity::new()
        .with(name(), "Pillar".to_string())
        .with_merge(make_transformable())
        .with(cube(),())
        .with(cube_collider(), vec3(1.,1.,1.))
        .with(translation(), base_pos + vec3(0.,0.,height * 0.5 - depth * 0.5))
        .with(scale(), vec3(width,width,height + depth))
        .spawn();
}

pub fn spawn_building_at(base_pos : Vec3) -> EntityId {

    let asset_index = 4; // random::<u8>()%5;
    let asset_string = vec![
        "MSH_Building_5x5x5_001", "MSH_Building_5x5x5_002",
        "MSH_Building_5x5x10_001", "MSH_Building_5x5x10_002",
        "building_mousehole"][asset_index as usize];
    let asset_height = match asset_index {
        0 | 1 => { 0. },
        2 | 3 => { 5. },
        4 | _ => { 0. },
    };

    let is_pickup = random::<bool>();

    if is_pickup { spawn_pickup(base_pos); }
    else { spawn_dropoff(base_pos); }

    let base_rot = Quat::from_rotation_z(PI*0.5*(random::<u8>()%4) as f32);

    let building_stack_base = Entity::new()
        .with(name(), "Building Stack".to_string())
        .with_merge(make_transformable())
        .with(model_from_url(), asset::url(format!("boatblue_citygen_simple/assets/{asset_string}.glb")).unwrap())
        // .with(cube(),())
        // .with(cube_collider(), vec3(5.,5.,10.))
        .with(translation(), base_pos + vec3(0., 0., asset_height))
        .with(rotation(), base_rot)
        // .with(scale(), vec3(width,width,height + depth))
        .spawn();

    spawn_garage_colliders(building_stack_base, base_pos, base_rot, 5. * (1. - 0.69) / 2., false);

    let darkness_colour = match is_pickup {
        true => { vec4(0., 0., 0., 0.5) } // black for pickup zone
        false => { vec4(1., 1., 0., 0.5) } // yellow for dropoff zone
    };

    for depth in 0..3 {
        let darkness_plane = Entity::new()
            .with(quad(), ())
            .with(translation(), base_pos
                + vec3(0., 0., 2.25)
                + base_rot * vec3(2.25 - 0.10 * depth as f32, 0., 0.))
            .with(rotation(), base_rot * Quat::from_rotation_y(PI * 0.5))
            .with(scale(), vec3(5., 3., 1.))
            .with(color(), darkness_colour) // black
            .spawn();
        entity::add_child(building_stack_base, darkness_plane);
        if depth < 2 {
            entity::add_component(darkness_plane, transparency_group(), 6);
        }
    }

    if random::<f32>() < 0.2 {
        spawn_building_ontop(building_stack_base, base_pos + vec3(0., 0., 5.));
    }

    return building_stack_base;
}

pub fn spawn_building_ontop(beneath_ent : EntityId, base_pos : Vec3) {
    let asset_index = random::<u8>()%4;
    let asset_string = vec![
        "MSH_Building_5x5x5_001", "MSH_Building_5x5x5_002",
        "MSH_Building_5x5x10_001", "MSH_Building_5x5x10_002",
    ][asset_index as usize];
    let asset_height = match asset_index {
        0 | 1 => { 0. },
        2 | 3 => { 5. },
        _ => { 0. },
    };
    
    let base_rot = Quat::from_rotation_z(PI*0.5*(random::<u8>()%4) as f32);

    // visual only: no collider needed
    let building_ontop = Entity::new()
        .with(name(), "Building On Top".to_string())
        .with(model_from_url(), asset::url(format!("boatblue_citygen_simple/assets/{asset_string}.glb")).unwrap())
        .with(translation(), base_pos + vec3(0., 0., asset_height))
        .with(rotation(), base_rot)
        .spawn();
    
    let building_separator = Entity::new()
        .with(name(), "Building Sep".to_string())
        .with(model_from_url(), asset::url("boatblue_citygen_simple/assets/MSH_Building_Seperator.glb").unwrap())
        .with(translation(), base_pos + vec3(0., 0., 0.))
        .with(rotation(), base_rot)
        .spawn();

    entity::add_child(beneath_ent, building_ontop);
    entity::add_child(beneath_ent, building_separator);

    if random::<f32>() < 0.1 {
        spawn_building_ontop(building_ontop, base_pos + vec3(0., 0., 5. + asset_height));
    }
}

fn spawn_garage_colliders(building_parent : EntityId, base_pos : Vec3, base_rot : Quat, wall_thickness : f32, visualize_walls : bool ) {
    for wall_local_rot_index in 0..3 {
        let wall_local_rot = Quat::from_rotation_z(PI*0.5*wall_local_rot_index as f32);
        let wall = Entity::new()
            .with(translation(), base_pos + base_rot * wall_local_rot * vec3(0., 2.50 - wall_thickness/2., 2.50))
            .with(rotation(), base_rot * wall_local_rot)
            .with(cube_collider(), vec3(5., wall_thickness, 5.))
            .spawn();
        entity::add_child(building_parent, wall);
        if visualize_walls {
            entity::add_component(wall, visualize_collider(), ());
        }
    }
}

// pub fn spawn_parent(parent_query : GeneralQuery<Component<()>>)->EntityId {
//     // despawn all old parents (& children)
//     for(parent,_)in parent_query.evaluate(){entity::despawn_recursive(parent);}

//     Entity::new()
//         .with(name(), "Mkworld Parent".to_string())
//         .with(is_mkworld_parent(), ())
//         .spawn()
// }

fn gen_water() {
    
    // for i in 0..10+1 {
    //     let depth = i as f32 * 0.1;
    //     Entity::new()
    //         .with(name(), "Water layer".to_string())
    //         .with_merge(make_transformable())
    //         .with(quad(), ())
    //         .with(scale(), vec3(999., 999., 1.0))
    //         .with(transparency_group(), 1)
    //         .with(color(), vec4(
    //             lerp(0.1, 0.0, depth), 
    //             lerp(0.2, 0.0, depth), 
    //             lerp(0.7, 0.0, depth), 
    //             lerp(0.1, 1.0, depth),
    //         ))
    //         .with(translation(), vec3(0., 0., lerp(0.5, -0.5, depth)))
    //         .spawn();
    // }
    Entity::new()
        .with(name(), "Water (1)".to_string())
        .with_merge(make_transformable())
        .with(scale(), vec3(999., 999., 1.0))
        .with(transparency_group(), 1)
        .with(color(), vec4(0.4, 0.6, 0.9, 0.8))
        .with(quad(), ())
        .with(translation(), vec3(0., 0., 0.5))
        .spawn();

    // for i in 0..10 {
    //     Entity::new()
    //     .with(name(), "Underwater Darkness Layer".to_string())
    //     .with_merge(make_transformable())
    //     .with(scale(), vec3(999., 999., 1.0))
    //     .with(transparency_group(), 2)
    //     .with(color(), vec4(0., 0., 0., 0.1 * i as f32))
    //     .with(quad(), ())
    //     .with(translation(), vec3(0., 0., -0.5 - 0.5 * i as f32))
    //     .spawn();
    // }

        Entity::new()
            .with(name(), "Water (2)".to_string())
            .with_merge(make_transformable())
            .with(scale(), vec3(999., 999., 1.0))
            .with(transparency_group(), 1)
            .with(color(), vec4(0., 0., 0., 0.6))
            .with(quad(), ())
            .with(translation(), vec3(0., 0., -4.))
            .spawn();
        
        Entity::new()
            .with(name(), "Water (3)".to_string())
            .with_merge(make_transformable())
            .with(scale(), vec3(999., 999., 1.0))
            .with(transparency_group(), 1)
            .with(color(), vec4(0., 0., 0., 0.8))
            .with(quad(), ())
            .with(translation(), vec3(0., 0., -7.))
            .spawn();
        
            Entity::new()
                .with(name(), "Water (4)".to_string())
                .with_merge(make_transformable())
                .with(scale(), vec3(999., 999., 1.0))
                .with(transparency_group(), 1)
                .with(color(), vec4(0., 0., 0., 0.9))
                .with(quad(), ())
                .with(translation(), vec3(0., 0., -9.))
                .spawn();
}


fn make_prox(pos : Vec3, radius : f32) -> Entity {
    Entity::new()
        // .with_merge(make_transformable())
        // .with_merge(make_sphere())
        .with(name(), "Prox Sphere".to_string())
        .with(translation(), pos)
        // .with(scale(), Vec3::splat(radius/4.))
        // .with(color(), vec3(1., 1., 0.).extend(0.25))
        // .with(transparency_group(), 3)
        .with(proximity_trigger(), radius)
}

fn spawn_pickup(pos : Vec3) -> EntityId {
    make_prox(pos, 2.0)
        .with(prox_is_pickup(), ())
        .spawn()
}
fn spawn_dropoff(pos : Vec3) -> EntityId {
    make_prox(pos, 2.0)
        .with(prox_is_dropoff(), ())
        .spawn()
}

use std::f32::consts::PI;

use ambient_api::prelude::*;
use ambient_api::core::{
    app::components::name,
    transform::components::{translation, rotation, scale},
    transform::concepts::make_transformable,
    model::components::model_from_url,
    physics::components::{cube_collider,visualize_collider},
    primitives::components::{cube, quad},
    rendering::components::{color, transparency_group},
};

// use crate::components::is_mkworld_parent;
use crate::boatblue_citygen_simple::components::{ proximity_trigger, prox_is_dropoff, prox_is_pickup, }; // see ambient.toml