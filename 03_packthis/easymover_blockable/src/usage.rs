pub fn server_demo() {
    // server_move_emover_around::init();
    // spawn_automatic_mover::Init();
    // spawn_basic_obstacle::init();
}

pub fn client_demo() {
    // decorate_sprite_as_cube::init();
    // default_camera::spawn();
}

mod spawn_automatic_mover {
    use ambient_api::prelude::*;
    pub fn init() {}
}

mod spawn_basic_obstacle {
    pub fn init() {}
}

mod decorate_sprite_as_cube {
    use crate::packages::easymover::components::*;
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

mod server_move_emover_around {

    // This shows how to spawn an emover_blockable and how to move one.

    use crate::packages::easymover::components::*;
    use crate::packages::this::components::*;
    use ambient_api::prelude::*;
    pub fn init() {
        let emover = Entity::new()
            .with(effect_spawn_emover_at(), vec2(-5., -5.))
            .with(emover_movespeed(), 5.0)
            .spawn();
        run_async(async move {
            loop {
                sleep(2.).await;
                let landgoal = random::<Vec2>() * 10. - 5.;
                println!("Move entity (blockable) to {:?}", landgoal);
                entity::add_component(emover, effect_blockable_move(), landgoal);
            }
        })
    }
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
