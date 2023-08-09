use ambient_api::{
    core::{
        player::components::{player, user_id},
        transform::components::translation, primitives::components::cube,
        physics::components::{physics_controlled, dynamic, sphere_collider, }, ecs::components::children,
        // 
    },
    prelude::*,
};

use crate::skatemouse::components::{is_skatemouse, mouse_cheese};
use crate::demo::messages::SetMouseCheese;

pub fn setup() {
    spawn_query((player(), user_id())).bind(|plrs|for(plr,(_,uid)) in plrs{
        let skatemouse = Entity::new() 
            .with(user_id(), uid)
            .with(translation(), vec3(0., 0., 0.))
            .with(physics_controlled(), ())
            .with(dynamic(), true)
            .with(sphere_collider(), 1.0)
            .with(is_skatemouse(), ())
            .with(mouse_cheese(), vec3(0., 0., 0.))
            .spawn();
        entity::add_child(plr, skatemouse);
    });
    SetMouseCheese::subscribe(|src,msg|{
        if let Some(plrent) = src.client_entity_id() {
            if let Some(plrchildren) = entity::get_component(plrent, children()) {
                for child in plrchildren {
                    if entity::has_component(child, mouse_cheese()) {
                        entity::set_component(child, mouse_cheese(), msg.cheese);
                    }
                }
            }
        }
    });
}

fn get_zplane_intersection(origin:Vec3, dir:Vec3, z:f32) -> Vec3 {
    let dir_mult = (z-origin.z)/dir.z;
    origin+dir_mult*dir
}