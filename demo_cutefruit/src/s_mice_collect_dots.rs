use ambient_api::{
    prelude::*,
    core::transform::components::translation,
};

use crate::skatemouse::components::is_skatemouse;
use crate::demo_cutefruit::components::{is_dot, score};

const COLLECTION_RADIUS : f32 = 2.1;

pub fn setup() {
    let find_dots = query(translation()).requires(is_dot()).build();
    query(translation()).requires(is_skatemouse()).each_frame(move|mice|for(mouse,mouse_pos)in mice{
        for (dot,dot_pos) in find_dots.evaluate() {
            if mouse_pos.distance_squared(dot_pos) < COLLECTION_RADIUS_SQUARED {
                entity::despawn_recursive(dot); // eaten
                entity::mutate_component_with_default(mouse, score(), 1, |score|*score+=1);
            }
        }
    });
}

const COLLECTION_RADIUS_SQUARED : f32 = COLLECTION_RADIUS * COLLECTION_RADIUS;