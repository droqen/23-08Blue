use ambient_api::prelude::*;

use crate::embers::a_nice_overhead_camera::messages::MouseRay;

use crate::embers::demo::components::{the_pointer, pointer_lmb, pointer_oob};

pub fn setup() {
    let pointer = Entity::new()
        .with(the_pointer(), vec3(0., 0., 0.))
        .with(pointer_lmb(), false)
        .with(pointer_oob(), false)
        .spawn();
    let pointer2 = pointer.clone();

    MouseRay::subscribe(move |_src,msg|{
        if let Some(point) = get_zplane_intersection(msg.origin, msg.dir, 0.) {
            entity::set_component(pointer, the_pointer(), point);
            entity::set_component(pointer, pointer_oob(), false);
        } else {
            entity::set_component(pointer, pointer_oob(), true);
        }
    });

    ambient_api::core::messages::Frame::subscribe(move |_|{
        let input = input::get();
        entity::set_component(pointer2, pointer_lmb(), input.mouse_buttons.contains(&MouseButton::Left));
    });
}

fn get_zplane_intersection(origin:Vec3, dir:Vec3, z:f32) -> Option<Vec3> {
    let dir_mult = (z-origin.z)/dir.z;
    if dir_mult.is_normal() && dir_mult.is_sign_positive() { Some(origin+dir_mult*dir) } else { None }
}