use ambient_api::prelude::*;

mod both;

#[main]
pub fn main() {
    add_sprite_to_emovers::init();
    sprite_update_landpos::init();
    default_camera::spawn();
    demo::init();
    both::init_all();
    packages::ease::messages::ConfigureClientTimeOffset { time_offset: 0.5 }
        .send_local_broadcast(false);
}

mod default_camera {
    use ambient_api::{
        core::{
            app::components::main_scene,
            camera::{
                components::aspect_ratio_from_window,
                concepts::make_perspective_infinite_reverse_camera,
            },
            transform::components::{lookat_target, translation},
        },
        prelude::*,
    };

    pub fn spawn() -> EntityId {
        Entity::new()
            .with_merge(make_perspective_infinite_reverse_camera())
            .with(aspect_ratio_from_window(), EntityId::resources())
            .with(main_scene(), ())
            .with(translation(), Vec3::ONE * 5.)
            .with(lookat_target(), vec3(0., 0., 0.))
            .spawn()
    }
}

mod demo {
    use crate::packages::this::components::*;
    use ambient_api::{
        core::{
            primitives::components::cube,
            transform::{components::translation, concepts::make_transformable},
        },
        prelude::*,
    };

    pub fn init() {
        spawn_query(esprite_landpos()).bind(|esprites| {
            for (sprite, landpos) in esprites {
                entity::add_components(
                    sprite,
                    make_transformable()
                        .with(cube(), ())
                        .with(translation(), landpos.extend(0.5)),
                );
            }
        });
        query(esprite_landpos()).each_frame(|esprites| {
            for (sprite, landpos) in esprites {
                entity::add_component(sprite, translation(), landpos.extend(0.5));
            }
        });
    }
}

mod add_sprite_to_emovers {
    use crate::both::helpers::get_landpos;
    use crate::packages::this::components::*;
    use ambient_api::core::app::components::name;
    use ambient_api::prelude::*;
    pub fn init() {
        spawn_query(()).requires(emover_ease()).bind(|emovers| {
            for (mover, _) in emovers {
                let landpos = get_landpos(mover).unwrap_or_default();
                Entity::new()
                    .with(name(), "Sprite".into())
                    .with(esprite_landpos(), landpos)
                    .with(esprite_mover(), mover)
                    .spawn();
            }
        });
    }
}

mod sprite_update_landpos {
    use crate::both::helpers::get_landpos;
    use crate::packages::this::components::*;
    use ambient_api::prelude::*;
    pub fn init() {
        query(esprite_mover()).each_frame(|sprites| {
            for (sprite, mover) in sprites {
                let landpos = get_landpos(mover).unwrap_or_default();
                entity::add_component(sprite, esprite_landpos(), landpos);
            }
        });
    }
}
