use ambient_api::prelude::*;
use ambient_api::core::{
    app::components::main_scene,
    camera::{
        components::aspect_ratio_from_window,
        concepts::make_perspective_infinite_reverse_camera,
    },
    transform::components::{lookat_target, translation},
};
use crate::nice_overhead_camera::components::{head_pitch,head_yaw,head_relpos};

pub fn setup() {
    query((lookat_target(), head_pitch(), head_yaw(), head_relpos())).each_frame(|heads|
        for (head,(locus, pitch, yaw, relpos)) in heads {
            entity::add_component(head, translation(), locus + Quat::from_rotation_z(-yaw) * (Quat::from_rotation_x(pitch) * relpos));
        }
    );
}

pub fn make_head() -> Entity {
    Entity::new()
        .with(translation(), vec3(0., -5., 5.))
        .with(lookat_target(), vec3(0., 0., 0.))
        .with(head_pitch(), 0.)
        .with(head_yaw(), 0.)
        .with(head_relpos(), vec3(0., -5., 5.))
}

pub fn make_fully_functional_head_camera() -> Entity {
    make_head()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(main_scene(), ()) // is this needed?
        .with(aspect_ratio_from_window(), entity::resources())
}