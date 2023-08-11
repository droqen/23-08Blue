use ambient_api::{
    core::{
        player::components::{player, user_id},
        transform::components::translation, primitives::components::cube,
        physics::components::{physics_controlled, dynamic, sphere_collider, }, ecs::components::children, app::components::name,
        // 
    },
    prelude::*,
};

use crate::skatemouse::components::{is_skatemouse, mouse_cheese, mouse_fwd, mouse_pace};
use crate::skatemouse::components::{mouse_ref, player_controlled};
use crate::demo::messages::SetMouseCheese;

pub fn setup() {
    spawn_query((player(), user_id())).bind(|plrs|for(plr,(_,uid)) in plrs{
        entity::add_component(plr, name(), "Player".to_string());
        let skatemouse = Entity::new() 
            .with(name(), "Skatemouse".to_string())
            .with(user_id(), uid)
            .with(player_controlled(), plr)
            .with(translation(), vec3(0., 0., 0.))
            .with(physics_controlled(), ())
            .with(dynamic(), true)
            .with(sphere_collider(), 1.0)
            .with(is_skatemouse(), ())
            .with(mouse_cheese(), vec3(0., 0., 0.))
            .with(mouse_fwd(), vec3(0., 1., 0.))
            .with(mouse_pace(), 0.)
            .spawn();

        // entity::add_child(plr, skatemouse);

        entity::add_component(plr, mouse_ref(), skatemouse);

    });
    SetMouseCheese::subscribe(|src,msg|{
        if let Some(plrent) = src.client_entity_id() {

            // if let Some(plrchildren) = entity::get_component(plrent, children()) {
            //     for child in plrchildren {
            //         if entity::has_component(child, mouse_cheese()) {
            //             entity::set_component(child, mouse_cheese(), msg.cheese);
            //         }
            //     }
            // }

            if let Some(mouse) = entity::get_component(plrent, mouse_ref()) {
                entity::add_component(mouse, mouse_cheese(), msg.cheese);
            }

        }
    });
}