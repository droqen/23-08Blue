pub fn init_all() {
    // easemover_animated_by_ease::init(); // unused
}

// mod easymover_animated_by_ease {
//     use crate::packages::{ease::components::*, this::components::*};
//     use ambient_api::prelude::*;
//     pub fn init() {
//         query(emover_ease()).each_frame(|emovers| {
//             for (mover, ease) in emovers {
//                 if let Some(ease_value) = entity::get_component(ease, ease_vec2_value()) {
//                     entity::add_component(mover, emover_landpos(), ease_value);
//                 }
//                 // else no vec2 exists
//             }
//         });
//     }
// }

pub mod helpers {
    use crate::packages::{ease::components::*, this::components::*};
    use ambient_api::prelude::*;
    pub fn get_landpos(emover: EntityId) -> Option<Vec2> {
        if let Some(ease) = entity::get_component(emover, emover_ease()) {
            return entity::get_component(ease, ease_vec2_value());
        }
        return None;
    }
}
