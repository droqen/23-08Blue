use ambient_api::prelude::*;

#[main]
pub fn main() {
    get_message_new_shout::init();
    query_shout_particles_and_animate_them::init();
}

mod get_message_new_shout {
    use ambient_api::{
        core::{
            app::components::{main_scene, name},
            primitives::components::cube,
            text::components::text,
            transform::components::{local_to_world, mesh_to_local, mesh_to_world, scale},
        },
        prelude::*,
    };

    use crate::embers::shouter::components::{
        shout_base_pos, shout_key, shout_prog, shout_text_center,
    };

    pub fn init() {
        let shouts_parent = Entity::new().with(name(), "Shouts".to_string()).spawn();

        crate::embers::shouter::messages::NewShout::subscribe(move |_src, msg| {
            entity::add_child(
                shouts_parent,
                Entity::new()
                    .with(shout_key(), msg.key)
                    .with(shout_prog(), 0.0)
                    .with(shout_base_pos(), msg.pos)
                    .with(
                        shout_text_center(),
                        vec3(msg.txtmsg.len() as f32 * 4., 15., 0.0),
                    )
                    .with(text(), msg.txtmsg)
                    .with(scale(), Vec3::ONE * 0.05)
                    .with(local_to_world(), Mat4::IDENTITY)
                    .with(mesh_to_local(), Mat4::IDENTITY)
                    .with(mesh_to_world(), Mat4::IDENTITY)
                    .with(main_scene(), ())
                    .spawn(),
            );
        });
    }
}

mod query_shout_particles_and_animate_them {

    use std::f32::consts::PI;

    use ambient_api::{
        core::transform::components::{scale, translation},
        prelude::*,
    };

    use crate::embers::shouter::components::{
        shout_base_pos, shout_key, shout_prog, shout_text_center,
    };

    pub fn init() {
        query((shout_base_pos(), shout_prog())).each_frame(|shouts| {
            for (shout, (base_pos, prog)) in shouts {
                let prog2 = prog + delta_time() * 0.5;
                if prog2 >= 1. {
                    entity::despawn(shout);
                } else {
                    let center =
                        entity::get_component(shout, shout_text_center()).unwrap_or(Vec3::ZERO);
                    entity::set_component(shout, shout_prog(), prog2);
                    let desired_scale = Vec3::splat(0.03 + 0.07 * (prog2 * PI).sin());
                    entity::add_component(shout, scale(), desired_scale);
                    entity::add_component(
                        shout,
                        translation(),
                        base_pos + vec3(0., (prog2 * PI).sin(), 0.) - desired_scale * center,
                    );
                }
            }
        });
    }
}
