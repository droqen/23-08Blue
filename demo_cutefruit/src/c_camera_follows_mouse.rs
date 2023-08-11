use std::f32::consts::PI;

use ambient_api::{prelude::*, core::{transform::components::{translation, lookat_target}, player::components::{user_id, player}}, message::Listener};

use crate::a_nice_overhead_camera::components::{the_nice_camera, head_relpos, nice_yaw_pitch_tilting, nice_yaw_pitch_tilting_base};
use crate::skatemouse::components::mouse_ref;

pub fn adjust_camera_params() {
    spawn_query(()).requires(the_nice_camera()).bind(|cams|for(the_camera, _)in cams{
        // head_relpos: y = move back, z = move up
        entity::add_component(the_camera, lookat_target(), vec3(60., 60., 0.));

        // head_relpos: y = move back, z = move up
        entity::add_component(the_camera, head_relpos(), vec3(0., 40., 40.));
        
        // nice_yaw_pitch_tilting_base: x = base yaw, y = base pitch
        entity::add_component(the_camera, nice_yaw_pitch_tilting_base(), vec2(PI * 0.25, 0.));

        // nice_yaw_pitch_tilting: x = max yaw tilt, y = max pitch tilt (using mouse)
        entity::add_component(the_camera, nice_yaw_pitch_tilting(), vec2(0.25, -0.25));
    });
}

pub async fn setup_async() {
    if let Some(mouse) = entity::wait_for_component(player::get_local(), mouse_ref()).await {
        _use_nothing_allow_multiple(mouse);
    } else {
        println!("CAMFOLLOWMOUSE dbg - (ERR?) Player deleted, never got mouse_ref");
    }
}

fn _use_nothing_allow_multiple(mouse:EntityId) {
    spawn_query(()).requires(the_nice_camera()).bind(move |cams| for (the_camera, _) in cams {
        println!("CAMFOLLOWMOUSE dbg - found do_once, creating query (this should appear !one time only!)");
        query(()).requires(the_nice_camera()).each_frame(move |cams|for (the_camera,_) in cams{
            if let Some(mouse_pos) = entity::get_component(mouse, translation()) {
                entity::set_component(the_camera, lookat_target(), mouse_pos);
            }
        });
    });
}

fn _use_listener(mouse:EntityId) {
    // doesn't work.
    
    // let mut listener : Option<Listener> = None;
    // listener = Some(spawn_query(()).requires(the_nice_camera()).bind(move |cams| for (the_camera, _) in cams {
    //     if listener.is_some() {
    //         listener.take().unwrap().stop();
    //         println!("CAMFOLLOWMOUSE dbg - found listener, creating query (this should appear !one time only!)");
    //         query(()).requires(the_nice_camera()).each_frame(move |cams|for (the_camera,_) in cams{
    //             if let Some(mouse_pos) = entity::get_component(mouse, translation()) {
    //                 entity::set_component(the_camera, lookat_target(), mouse_pos);
    //             }
    //         });
    //     } else {
    //         println!("CAMFOLLOWMOUSE dbg - (ERR?) Listener is none.");
    //     }
    // }));
}

fn _use_mut_do_once(mouse:EntityId) {
    // doesn't work.

    // let mut do_once = true;
    // spawn_query(()).requires(the_nice_camera()).bind(move |cams| for (the_camera, _) in cams {
    //     if do_once {
    //         do_once = false;
    //         println!("CAMFOLLOWMOUSE dbg - found do_once, creating query (this should appear !one time only!)");
    //         query(()).requires(the_nice_camera()).each_frame(|cams|for (the_camera,_) in cams{
    //             if let Some(mouse_pos) = entity::get_component(mouse, translation()) {
    //                 entity::set_component(the_camera, lookat_target(), mouse_pos);
    //             }
    //         });
    //     } else {
    //         println!("CAMFOLLOWMOUSE dbg - (ERR?) do_once is false.");
    //     }
    // });
}