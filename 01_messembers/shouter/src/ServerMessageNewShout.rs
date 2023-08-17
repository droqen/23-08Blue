use ambient_api::{prelude::*, core::{text::components::text, app::components::{name, main_scene}, primitives::components::cube, transform::components::{local_to_world, mesh_to_local, mesh_to_world, scale}}};

use crate::embers::shouter::components::{
    shout_key,shout_prog,shout_base_pos,shout_text_center,
};

pub fn setup() {
    let shouts_parent = Entity::new()
        .with(name(), "Shouts".to_string())
        .spawn();

    crate::embers::shouter::messages::NewShout::subscribe(move|_src,msg|{
        entity::add_child(shouts_parent,
            Entity::new()
                .with(shout_key(), msg.key)
                .with(shout_prog(), 0.0)
                .with(shout_base_pos(), msg.pos)
                .with(shout_text_center(), vec3(msg.txtmsg.len() as f32 * 4., 15., 0.0))
                .with(text(), msg.txtmsg)
                .with(scale(), Vec3::ONE * 0.05)
                .with(local_to_world(), Mat4::IDENTITY)
                .with(mesh_to_local(), Mat4::IDENTITY)
                .with(mesh_to_world(), Mat4::IDENTITY)
                .with(main_scene(), ())
                .spawn()
        );
    });
}