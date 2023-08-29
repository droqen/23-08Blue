mod vec3_point_to_point {
    use crate::packages::ease::components::{
        ease_end_time, ease_start_time, ease_vec3_a, ease_vec3_b,
    };
    use ambient_api::prelude::*;
    pub fn make(a: Vec3, b: Vec3, speed: f32) -> Entity {
        let start = game_time().as_secs_f32();
        let dur = a.distance(b) / speed;
        Entity::new()
            .with(ease_vec3_a(), a)
            .with(ease_vec3_b(), b)
            .with(ease_start_time(), start)
            .with(ease_end_time(), start + dur)
    }
}

mod vec2_point_to_point {
    use crate::packages::ease::components::{
        ease_end_time, ease_start_time, ease_vec2_a, ease_vec2_b,
    };
    use ambient_api::prelude::*;
    pub fn make(a: Vec2, b: Vec2, speed: f32) -> Entity {
        let start = game_time().as_secs_f32();
        let dur = a.distance(b) / speed;
        Entity::new()
            .with(ease_vec2_a(), a)
            .with(ease_vec2_b(), b)
            .with(ease_start_time(), start)
            .with(ease_end_time(), start + dur)
    }
}

mod tween_f32 {
    use crate::packages::ease::components::{
        ease_end_time, ease_f32_a, ease_f32_b, ease_start_time,
    };
    use ambient_api::prelude::*;
    pub fn make(a: f32, b: f32, speed: f32) -> Entity {
        let start = game_time().as_secs_f32();
        let dur = a.distance(b) / speed;
        Entity::new()
            .with(ease_f32_a(), a)
            .with(ease_f32_b(), b)
            .with(ease_start_time(), start)
            .with(ease_end_time(), start + dur)
    }
}
