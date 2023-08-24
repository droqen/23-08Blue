use ambient_api::{
    core::{
        app::components::{main_scene, name},
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        primitives::components::{cube, quad},
        transform::{
            components::{lookat_target, translation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

use packages::this::components::*;

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with(main_scene(), ())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    let mut prevfish: Option<EntityId> = None;

    for i in 1..100 {
        let fish = Entity::new()
            .with(name(), format!("Fish#{}", i))
            .with(fish_swim_dir(), random::<Quat>() * vec3(1., 0., 0.))
            .with(translation(), random::<Vec3>())
            .spawn();
        {
            if let Some(prevfish) = prevfish {
                Entity::new()
                    .with(rel_fish_follower(), ())
                    .with(left_relative(), prevfish)
                    .with(right_relative(), fish)
                    .spawn();
            }
        }
        prevfish = Some(fish);

        let body = Entity::new()
            .with(translation(), random::<Vec3>())
            .with(cube(), ())
            .spawn();

        // create a relationship entity
        Entity::new()
            .with(rel_fishs_body(), ())
            .with(left_relative(), fish)
            .with(right_relative(), body)
            .spawn();
    }

    query((left_relative(), right_relative()))
        .requires(rel_fishs_body())
        .each_frame(|rels| {
            for (rel, (fish, body)) in rels {
                let fish_pos = entity::get_component(fish, translation()).unwrap_or(Vec3::ZERO);
                entity::mutate_component_with_default(body, translation(), fish_pos, |pos| {
                    *pos = 0.5 * *pos + 0.5 * fish_pos
                });
            }
        });

    query(fish_swim_dir()).each_frame(|fishies| {
        for (fish, dir) in fishies {
            entity::mutate_component(fish, translation(), |pos| *pos += dir * 10.0 * delta_time());
            if random::<f32>() < 0.03 {
                entity::add_component(fish, fish_swim_dir(), random::<Quat>() * vec3(1., 0., 0.));
            }
        }
    });

    query((left_relative(), right_relative()))
        .requires(rel_fish_follower())
        .each_frame(|rels| {
            for (rel, (fish, follower)) in rels {
                if random::<f32>() < 0.95 {
                    let fish_pos = entity::get_component(fish, translation()).unwrap_or(Vec3::ZERO);
                    let follower_pos =
                        entity::get_component(follower, translation()).unwrap_or(Vec3::ZERO);
                    if let Some(follow_dir) = (fish_pos - follower_pos).try_normalize() {
                        // println!(
                        //     "{:?} is following {:?}",
                        //     entity::get_component(follower, name()),
                        //     entity::get_component(fish, name()),
                        // );
                        entity::set_component(follower, fish_swim_dir(), follow_dir);
                    }
                }
            }
        });
}
