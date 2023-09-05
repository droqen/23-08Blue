use ambient_api::prelude::*;

#[main]
pub fn main() {
    create_obstacles::init();
    players_are_blockable_emovers::init();
    handle_move_message::init();
}

mod create_obstacles {
    use crate::packages::{
        easymover::components::effect_spawn_emover_at,
        easymover_blockable::components::emover_blockable_radius,
    };
    use ambient_api::{
        core::{
            physics::components::{cube_collider, sphere_collider},
            primitives::{
                components::{cube, sphere},
                concepts::make_sphere,
            },
            transform::{
                components::{scale, translation},
                concepts::make_transformable,
            },
        },
        prelude::*,
    };
    pub fn init() {
        Entity::new()
            .with(cube(), ())
            .with(cube_collider(), vec3(1.0, 1.0, 1.0))
            .with_merge(make_transformable())
            .with(translation(), vec3(0., 0., 0.5))
            .spawn();
    }
}

mod players_are_blockable_emovers {
    use crate::packages::{
        easymover::components::effect_spawn_emover_at,
        easymover_blockable::components::emover_blockable_radius,
    };
    use ambient_api::{core::player::components::is_player, prelude::*};
    pub fn init() {
        spawn_query(()).requires(is_player()).bind(|plrs| {
            for (plr, _) in plrs {
                entity::add_components(
                    plr,
                    Entity::new()
                        .with(effect_spawn_emover_at(), random::<Vec2>() * 5. - 4.)
                        .with(emover_blockable_radius(), 0.25),
                );
            }
        });
    }
}

mod handle_move_message {
    use crate::packages::{
        easymover_blockable::components::effect_blockable_move, this::messages::MovePlayerBlockable,
    };
    use ambient_api::prelude::*;
    pub fn init() {
        MovePlayerBlockable::subscribe(|src, msg| {
            if let Some(plr) = src.client_entity_id() {
                entity::add_component(plr, effect_blockable_move(), msg.pos2);
            }
        });
    }
}
