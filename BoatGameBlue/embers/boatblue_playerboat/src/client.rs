use ambient_api::prelude::*;
use ambient_api::core::transform::components::translation;
use boat::components::boat_steer;

mod x_findpair;

#[main]
pub async fn main() {

    run_async(async {
        let get_boats = query((translation(), boat_steer())).build();
        loop {
            for (boat, (pos,_)) in get_boats.evaluate() {
                if pos.length() > 20. {
                    entity::set_component(boat, boat_steer(),-pos.truncate().normalize());
                } else {
                    entity::set_component(boat, boat_steer(),
                        (random::<Vec2>()-Vec2::splat(0.5)).normalize_or_zero()
                    );
                }
            }
            sleep(1.).await;
        }
    });

    on_boat_spawned_spawn_model();

    run_async(on_playerboat_spawned_link_camera_and_setup_input());

}

use boat::components::image_of_boat;
use ambient_api::core::model::components::model_from_url;
fn on_boat_spawned_spawn_model() {
    spawn_query(image_of_boat()).bind(|images|{
        for (image, _boat) in images {
            entity::add_component(image, model_from_url(), asset::url("boatblue_playerboat/assets/MSH_Boat.glb").unwrap());
        }
    });
}

use ambient_api::core::player::components::{user_id, local_user_id};
use boatblue_playerboat::messages::GotoRay;
use selfie_camera::components::selfie_focus_ent;
use boat::components::{
    boat_forward,
    // boat_forward_rotvel,
};

async fn on_playerboat_spawned_link_camera_and_setup_input() {

    let my_uid = entity::get_component(entity::resources(), local_user_id()).unwrap();
    let (playerboat, camera) = x_findpair::find_first_pair_async(
        spawn_query((boat_forward(), user_id())), move |boat| { entity::get_component(boat, user_id()).unwrap_or_default() == my_uid },
        spawn_query(selfie_focus_ent()), |_|true,
    ).await;

    entity::add_component(camera, selfie_focus_ent(), playerboat);
    setup_mouse_ray_input_broadcast(camera);

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