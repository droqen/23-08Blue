pub fn client_demo() {
    default_camera::spawn();
    client_decorate_sprite_as_cube::init();
}

pub fn server_demo() {
    server_move_emover_around::init();
}

mod server_move_emover_around {
    use crate::packages::this::components::*;
    use ambient_api::prelude::*;
    pub fn init() {
        let emover = Entity::new()
            .with(effect_spawn_emover_at(), vec2(-5., -5.))
            .spawn();
        run_async(async move {
            loop {
                sleep(1.).await;
                let new_landgoal = random::<Vec2>() * 10. - 5.;
                println!("Move entity to {:?}", new_landgoal);
                entity::add_component(emover, emover_landgoal(), new_landgoal);
            }
        })
    }
}

mod client_decorate_sprite_as_cube {
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
