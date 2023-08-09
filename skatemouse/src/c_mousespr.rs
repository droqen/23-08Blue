use core::panic;
use std::f32::consts::PI;

use ambient_api::{
    prelude::*,
    core::{
        ecs::components::parent,
        transform::components::{translation, scale, rotation},
        primitives::concepts::{make_sphere, make_capsule},
        model::components::model_from_url,
    },
};

use crate::skatemouse::components::{is_skatemouse, mouse_cheese, mouse_fwd, is_mouse_spr, is_cheese_spr, mouse_spr_fwd, };

const Y_POSITIVE : Vec3 = vec3(0., 1., 0.);

pub fn setup() {
    spawn_query((translation(), is_skatemouse())).bind(|mice|for(mouse,(pos,cheese)) in mice{
        let mouse_spr = Entity::new()
            .with(is_mouse_spr(), ())
            // .with_merge(make_sphere())
            .with(model_from_url(), crate::skatemouse::assets::url("low_poly_rat.glb"))
            .with(mouse_spr_fwd(), vec3(-1., 0., 0.))
            .spawn();
        entity::add_child(mouse, mouse_spr);
    });
    query((is_mouse_spr(), parent())).each_frame(|sprs|for(spr,(_,mouse))in sprs{
        let pos = entity::get_component(mouse, translation());
        let fwd = entity::get_component(mouse, mouse_fwd());
        let spr_fwd = entity::get_component(spr, mouse_spr_fwd()).unwrap_or(Y_POSITIVE);

        if pos.is_some() && fwd.is_some() {
            let pos = pos.unwrap();
            let fwd = fwd.unwrap();
            entity::add_component(spr, translation(), pos);
            entity::add_component(spr, rotation(), Quat::from_rotation_arc(spr_fwd, fwd));
        } else {
            panic!("bad value for pos/fwd: {:?}/{:?}",pos,fwd);
        }
    });
    spawn_query((translation(),mouse_cheese())).bind(|mice|for(mouse,(pos,cheese)) in mice{
        let cheese_spr = Entity::new()
            .with(is_cheese_spr(), ())
            .with_merge(make_capsule())
            .with(scale(), vec3(0.1, 0.1, 1.5))
            .spawn();
        entity::add_child(mouse, cheese_spr);
    });
    query((is_cheese_spr(), parent())).each_frame(|sprs|for(spr,(_,mouse))in sprs{
        if let Some(cheese_pos) = entity::get_component(mouse, mouse_cheese()) {
            entity::add_component(spr, translation(), cheese_pos);
        } else {
            entity::remove_component(spr, translation());
        }
    });
}
