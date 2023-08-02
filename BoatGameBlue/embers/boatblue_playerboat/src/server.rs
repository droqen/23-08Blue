use ambient_api::prelude::*;

#[main]
pub fn main() {
    on_player_spawned_spawn_playerboat();
    on_goto_message_set_player_goto();
    each_frame_set_player_steer_by_goto();

    create_plane_for_mouse_ray_to_hit();
}


use ambient_api::core::{
    transform::components::translation,
    transform::concepts::make_transformable,
    physics::components::{
        physics_controlled,
        dynamic,
        sphere_collider,
        plane_collider,
        visualize_collider,
    },
    player::components::{player, user_id},
    ecs::components::children,
};
use boatblue_playerboat::components::playerboat_goto;
use boatblue_playerboat::messages::GotoRay;
fn on_player_spawned_spawn_playerboat() {
    spawn_query((player(), user_id())).bind(|players|{
        for (player,(_,uid)) in players {
            entity::add_child(
                player,
                spawn_player_boat(uid)
            );
        }
    });
}
fn spawn_player_boat(uid : String) -> EntityId {
    Entity::new()
        .with_merge(make_transformable())
        .with(physics_controlled(), ())
        .with(dynamic(), true)
        .with(sphere_collider(), 1.0) // how big?
        .with(user_id(), uid)
        .with(visualize_collider(), ())
        // .with(playerboat_goto(), Vec3:)
        .with_merge(make_boat())
        .with_merge(make_buoy())
        .spawn()
}
fn on_goto_message_set_player_goto() {
    GotoRay::subscribe(|src, msg|{
        dbg!(&msg);
        if let Some(player_ent) = src.client_entity_id() {
            if let Some(children) = entity::get_component(player_ent, children()) {
                for child in children {
                    if entity::has_component(child, boat_steer()) {
                        for hit in physics::raycast(msg.origin, msg.dir) {
                            if entity::has_component(hit.entity, plane_collider()) {
                                entity::add_component(child, playerboat_goto(), hit.position.truncate());
                            }
                        }
                    }
                }
            }
        }
    });
}
fn each_frame_set_player_steer_by_goto() {
    query((translation(), playerboat_goto())).each_frame(|playerboats|{
        for (playerboat, (my_pos, goto_pos)) in playerboats {
            let goal_vec : Vec2 = goto_pos - my_pos.truncate();
            let goal_dist : f32 = goal_vec.length();
            if goal_dist < 0.05 {
                // entity::remove_component(playerboat, playerboat_goto()); // ? should i ?
                entity::set_component(playerboat, boat_steer(), Vec2::ZERO);
            } else {
                let steer_power = invlerp(0.00, 4.00, goal_vec.length()).clamp(0.10, 1.00);
                entity::set_component(playerboat, boat_steer(), goal_vec.normalize() * steer_power);
            }
        }
    });
}
fn create_plane_for_mouse_ray_to_hit() {
    Entity::new()
        .with(translation(), vec3(0., 0., -5.))
        .with(plane_collider(), ())
        .spawn();
}

use boat::components::{
    boat_vel, boat_steer, boat_forward, boat_forward_rotvel,
};
fn make_boat() -> Entity {
    Entity::new()
        .with(boat_vel(), vec2(0., 0.))
        .with(boat_steer(), vec2(0., 0.))
        .with(boat_forward(), vec2(0., 1.))
        .with(boat_forward_rotvel(), 0.)
}

use matter::components::{
    matter_gravity, matter_local_center, buoy_max_force, buoy_max_drag, buoy_radius, buoy_water_level,
};
fn make_buoy() -> Entity {
    Entity::new()
        .with(matter_gravity(), 9.82)
        .with(matter_local_center(), vec3(0., 0., -2.))
        .with(buoy_max_force(), 15.0)
        .with(buoy_max_drag(), 4.0) 
        .with(buoy_radius(), 1.0)
        .with(buoy_water_level(), 0.)
}


// fn lerp(from : f32, to : f32, rel : f32) -> f32 { ((1. - rel) * from) + (rel * to) }
fn invlerp(from : f32, to : f32, value : f32) -> f32 { (value - from) / (to - from) }