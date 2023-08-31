use ambient_api::prelude::*;

mod easing_system;

#[main]
pub fn main() {
    easing_system::setup();
    client_cfg_messages::setup();
}

mod client_cfg_messages {
    use crate::packages::this::{
        components::{ease_start_time, ease_time_offset},
        messages::ConfigureClientTimeOffset,
    };
    use ambient_api::prelude::*;
    pub fn setup() {
        ConfigureClientTimeOffset::subscribe(|_src, msg| {
            spawn_query(())
                .requires(ease_start_time())
                .bind(move |eases| {
                    for (ease, _) in eases {
                        entity::add_component(ease, ease_time_offset(), msg.time_offset);
                    }
                });
        });
    }
}
