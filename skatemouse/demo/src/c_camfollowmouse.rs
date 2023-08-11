use ambient_api::{
    core::{
        player::components::{local_user_id, user_id},
        transform::components::{translation, lookat_target},
    },
    prelude::*,
};

use crate::embers::demo::messages::{MyPointerClick, MyMousePos, SetMouseCheese, };
use crate::embers::demo::components::{the_pointer, pointer_lmb, pointer_oob};
use crate::embers::skatemouse::components::{is_skatemouse, mouse_cheese};
use crate::embers::a_nice_overhead_camera::components::the_nice_camera;

pub fn setup() {
    query((the_pointer(), pointer_lmb())).each_frame(|pointers|for(the_pointer,(pointer_pos,lmb)) in pointers{
        if lmb && !entity::get_component(the_pointer, pointer_oob()).unwrap_or(false) {
            MyPointerClick{click: pointer_pos}.send_local_broadcast(true);
        }
    });

    let local_uid = entity::get_component(entity::resources(), local_user_id()).unwrap();
    spawn_query((is_skatemouse(), translation(), user_id())).bind(move |mice|for(mouse,(_,pos,uid)) in mice{
        if uid != local_uid { continue; }
        let my_mouse = mouse.clone();
        ambient_api::core::messages::Frame::subscribe(move |_|{
            MyMousePos{pos: entity::get_component(my_mouse, translation()).unwrap()}.send_local_broadcast(true);
        });
        let my_mouse = mouse.clone();
        MyPointerClick::subscribe(move |_src,msg|{
            entity::add_component(my_mouse, mouse_cheese(), msg.click); // todo; should this wait to get a response?
            SetMouseCheese{cheese: msg.click}.send_server_reliable();
        });
    });

    spawn_query(the_nice_camera()).bind(|cameras|for (the_camera,_) in cameras{
        MyMousePos::subscribe(move |_src,msg|{ // this updates the camera
            // entity::add_component(the_camera, lookat_target(), msg.pos);
        });
    });
}