use ambient_api::prelude::*;



#[main]
pub async fn main() {
    sleep(0.1).await;
    println!("playerboat uses a 'sleep await' to solve a race condition");
    on_playerboat_spawned_link_camera_and_setup_input();
}

use ambient_api::core::player::components::{user_id, local_user_id};
use boatblue_playerboat::messages::GotoRay;
use selfie_camera::components::selfie_focus_ent;
use boat::components::boat_forward;

fn on_playerboat_spawned_link_camera_and_setup_input() {
    let my_uid = entity::get_component(entity::resources(), local_user_id()).unwrap();
    spawn_query((boat_forward(), user_id())).bind(move |playerboats|{
        for (playerboat,(_, uid)) in playerboats {
            if uid == my_uid {
                link_camera_and_setup_input(playerboat);
            }
        }
    });
}
fn link_camera_and_setup_input(playerboat : EntityId) {
    if let Some((my_camera, _)) = query(selfie_focus_ent()).build().evaluate().first() {

        entity::add_component(*my_camera, selfie_focus_ent(), playerboat);

        setup_mouse_ray_input_broadcast(*my_camera);

    } else { println!("Err - NO CAMERA FOUND, camera link failed"); }
}

fn setup_mouse_ray_input_broadcast(camera : EntityId) {
    ambient_api::core::messages::Frame::subscribe(move |_|{
        let input = input::get();
        let mouse_ray: Ray = get_mouse_ray(camera);
        if input.mouse_buttons.contains(&MouseButton::Left) {
            GotoRay{origin: mouse_ray.origin, dir:mouse_ray.dir}.send_server_unreliable();
        } else {
            // ... do nothing if mouse button is not held
        }
    });
}

fn get_mouse_ray(camera_ent : EntityId) -> Ray {
    let input = input::get();
    let lmb_click_position = input.mouse_position;
    camera::screen_position_to_world_ray(camera_ent, lmb_click_position)
}



/*

pub fn build_query() -> GeneralQuery<(Component<EntityId>, Component<String>)> {
    query((plr_glidercam(), user_id())).build()
}

pub fn try_find_my_cam(query : GeneralQuery<(Component<EntityId>, Component<String>)>) -> Option<EntityId> {
    let local_uid =
        entity::get_component(entity::resources(), local_user_id()).unwrap();
    for (plr, (cam,uid)) in query.evaluate() {
        if uid == local_uid { return Some(cam); }
    }
    return None; // no camera found
}

pub fn my_cam_spawn_query(callback : fn(camera : EntityId)) {
    let local_uid =
        entity::get_component(entity::resources(), local_user_id()).unwrap();
    spawn_query((plr_glidercam(), user_id())).bind(move |cameras|{
        for (plr, (cam,uid)) in cameras {
            if uid == local_uid { callback(cam); }
        }
    });
}

*/