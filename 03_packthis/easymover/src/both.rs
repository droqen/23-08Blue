pub fn init_all() {
    landpos_driven::init(); // unused
}

mod landpos_driven {
    use crate::packages::{ease::components::*, this::components::*};
    use ambient_api::{entity::despawn, prelude::*};
    pub fn init() {
        change_query((ease_propagate_landpos_to(), ease_vec2_value()))
            .track_change(ease_vec2_value())
            .bind(|eases| {
                for (ease, (emover, landpos)) in eases {
                    entity::add_component(emover, emover_landpos(), landpos);
                }
            });
    }
}
