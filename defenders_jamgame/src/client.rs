use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        transform::{
            components::{lookat_target, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

use ambient_api::prelude::*;

use embers::defenders_jamgame::{
    messages::DeleteGuy,
    components::is_walker
};

#[main]
pub fn main() {
    let mycamera = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    let getallwalkers = query((is_walker(), translation())).build();
    ambient_api::core::messages::Frame::subscribe(move |_|{
        let (delta, input) = input::get_delta();
        if (delta.mouse_buttons.contains(&MouseButton::Left)) {
            let ray = get_mouse_camera_ray(input.mouse_position, mycamera);
            let point = get_zplane_intersection(ray.origin, ray.dir, 0.).unwrap();
            for (walker,(_,pos)) in getallwalkers.evaluate() {
                if pos.distance_squared(point) < 1. {
                    DeleteGuy{guy: walker}.send_server_reliable();
                    break;
                }
            }
        }
    });

}

pub fn get_mouse_camera_ray(mouse_position : Vec2, camera_ent : EntityId) -> Ray {
    let input = input::get();
    let lmb_click_position = input.mouse_position;
    camera::screen_position_to_world_ray(camera_ent, lmb_click_position)
}

fn get_zplane_intersection(origin:Vec3, dir:Vec3, z:f32) -> Option<Vec3> {
    let dir_mult = (z-origin.z)/dir.z;
    if dir_mult.is_normal() && dir_mult.is_sign_positive() { Some(origin+dir_mult*dir) } else { None }
}