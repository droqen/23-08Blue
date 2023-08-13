use std::f32::consts::PI;

use ambient_api::{prelude::*, core::{transform::components::{translation, lookat_target}, player::components::{user_id, is_player}}, message::Listener};

use crate::embers::a_nice_overhead_camera::components::{
    the_nice_camera, nice_input_tilt,
    nice_yaw_pitch_tilting, nice_yaw_pitch_tilting_base,
    head_relpos, head_target, target_relpos
};
use crate::embers::skatemouse::components::mouse_ref;

pub fn adjust_camera_params() {
    spawn_query(()).requires(the_nice_camera()).bind(|cams|for(the_camera, _)in cams{
        // nice_yaw_pitch_tilting_base: x = base yaw, y = base pitch
        entity::add_component(the_camera, nice_yaw_pitch_tilting_base(), vec2(PI * 0.25, 0.));
        // nice_yaw_pitch_tilting: x = max yaw tilt, y = max pitch tilt (using mouse)
        entity::add_component(the_camera, nice_yaw_pitch_tilting(), vec2(0.25, -0.15));
    });
    
    // // manually applying tilt? // nah 
    // query(nice_input_tilt()).each_frame(|cams|{
    //     for (cam,input_tilt) in cams {
    //         entity::add_component(cam, head_yaw(), yaw_pitch_base.x + yaw_pitch_tilt.x * -input_tilt.x);
    //         entity::add_component(cam, head_pitch(), yaw_pitch_base.y + yaw_pitch_tilt.y * -input_tilt.y);
    //     }
    // });
    
    query(nice_input_tilt()).requires(the_nice_camera()).each_frame(move |cams|for (the_camera,tilt) in cams{
        let lookside = tilt.x * 2.0;
        let mut lookcloser = lerpby((-7.0, 7.0), (tilt.y*0.5+0.5));
        if lookcloser < 0.0 { lookcloser *= 0.5; }
        // adjust lookat by tilt
        entity::add_component(the_camera, head_relpos(), vec3(0., 20. - lookcloser, 40.));
        entity::add_component(the_camera, target_relpos(), vec3(lookside, lookcloser, 0.));
    });
}

fn lerp(from : f32, to : f32, rel : f32) -> f32 { ((1. - rel) * from) + (rel * to) }
fn lerpby(range : (f32, f32), rel : f32) -> f32 { lerp(range.0,range.1,rel) }

pub async fn setup_async() {
    if let Some(mouse) = entity::wait_for_component(player::get_local(), mouse_ref()).await {
        _followmouse_no_multi_safety(mouse);
    } else {
        println!("CAMFOLLOWMOUSE dbg - (ERR?) Player deleted, never got mouse_ref");
    }
}

fn _followmouse_no_multi_safety(mouse:EntityId) {
    spawn_query(nice_input_tilt()).requires(the_nice_camera()).bind(move |cams| for (the_camera, _) in cams {
        println!("CAMFOLLOWMOUSE dbg - assigning head_target (this should appear !one time only!)");
        entity::add_component(the_camera, head_target(), mouse);
    });
}

fn _followmouse_listener_safety(mouse:EntityId) {
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

fn _followmouse_mut_safety(mouse:EntityId) {
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