use ambient_api::core::{
    primitives::components::cube,
    transform::{components::translation, concepts::make_transformable},
};
use ambient_api::prelude::*;

#[main]
pub fn main() {
    let camera = default_camera::spawn();
    packages::clicks_auto::messages::ConfigureCamera { camera: camera }.send_local_broadcast(false);
    packages::clicks_auto::messages::ConfigureZPlane { z: 0.0 }.send_local_broadcast(false);

    let cube = Entity::new()
        .with_merge(make_transformable())
        .with(cube(), ())
        .spawn();
    onclick_ease_cube::setup(cube);
}

mod onclick_ease_cube {
    use crate::{
        packages::{clicks_auto::components::*, ease::components::*, this::components::cubes_ease},
        vec3_point_to_point,
    };
    use ambient_api::{core::transform::components::translation, prelude::*};

    pub fn setup(cube: EntityId) {
        spawn_query(click_world_projected()).bind(move |clicks| {
            for (click, groundpos) in clicks {
                let ease = vec3_point_to_point::make(
                    entity::get_component(cube, translation()).unwrap(),
                    groundpos,
                    1.0,
                )
                // .with(ease_filter_paramblend(), 2.0)
                .with(ease_autoset_translation(), cube)
                .spawn();

                if let Some(prev_ease) = entity::get_component(cube, cubes_ease()) {
                    entity::despawn(prev_ease);
                }

                entity::add_component(cube, cubes_ease(), ease);
            }
        });
    }
}

mod vec3_point_to_point {
    use crate::packages::ease::components::{
        ease_end_time, ease_start_time, ease_vec3_a, ease_vec3_b,
    };
    use ambient_api::prelude::*;
    pub fn make(a: Vec3, b: Vec3, speed: f32) -> Entity {
        let start = game_time().as_secs_f32();
        let dur = a.distance(b) / speed;
        // dbg!(dur);
        Entity::new()
            .with(ease_vec3_a(), a)
            .with(ease_vec3_b(), b)
            .with(ease_start_time(), start)
            .with(ease_end_time(), start + dur)
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
