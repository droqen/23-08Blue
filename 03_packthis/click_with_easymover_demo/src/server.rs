use ambient_api::{core::player::components::is_player, prelude::*};

#[main]
pub fn main() {
    let emover = spawn_query(()).requires(is_player()).bind(|plrs| {
        for (plr, _) in plrs {
            entity::add_component(
                plr,
                packages::easymover::components::effect_spawn_emover_at(),
                random::<Vec2>() * 5. - 4.,
            );
        }
    });
    packages::this::messages::MovePlayer::subscribe(|src, msg| {
        if let Some(plr) = src.client_entity_id() {
            entity::add_component(
                plr,
                packages::easymover::components::emover_landgoal(),
                msg.pos2,
            );
        }
    });
}
