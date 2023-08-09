use ambient_api::{
    prelude::*,
    core::{
        ecs::components::parent,
        transform::components::{translation, scale},
        primitives::concepts::{make_sphere, make_capsule},
    },
};

use crate::skatemouse::components::{is_skatemouse, mouse_cheese, is_mouse_spr, is_cheese_spr, };

pub fn setup() {
    spawn_query((translation(), is_skatemouse())).bind(|mice|for(mouse,(pos,cheese)) in mice{
        let mouse_spr = Entity::new()
            .with(is_mouse_spr(), ())
            .with_merge(make_sphere())
            .spawn();
        entity::add_child(mouse, mouse_spr);
    });
    query((is_mouse_spr(), parent())).each_frame(|sprs|for(spr,(_,mouse))in sprs{
        if let Some(mouse_pos) = entity::get_component(mouse, translation()) {
            entity::add_component(spr, translation(), mouse_pos);
        } else {
            entity::remove_component(spr, translation());
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
