use ambient_api::prelude::*;

mod both;
mod usage;

#[main]
pub fn main() {
    add_sprite_to_emovers::init();
    sprite_update_landpos::init();
    usage::client_demo();
    both::init_all();
    packages::ease::messages::ConfigureClientTimeOffset { time_offset: 0.5 }
        .send_local_broadcast(false);
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
