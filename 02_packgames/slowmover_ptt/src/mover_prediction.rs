use crate::packages::slowmover_ptt::components::{
    mover_pos_end, mover_pos_start, mover_time_start,
};
use ambient_api::prelude::*;

const MOVER_SPEED: f32 = 3.0;

pub fn get_mover_pos2(mover: EntityId, time_offset: f32) -> Vec2 {
    try_calculate_mover_position(mover, time_offset).unwrap_or_default()
}

pub fn get_mover_pos3(mover: EntityId, time_offset: f32) -> Vec3 {
    get_mover_pos2(mover, time_offset).extend(0.5)
}

pub fn try_calculate_mover_position(mover: EntityId, time_offset: f32) -> Option<Vec2> {
    let start: Vec2 = entity::get_component(mover, mover_pos_start())?;
    let end: Vec2 = entity::get_component(mover, mover_pos_end())?;
    let start_time: f32 = entity::get_component(mover, mover_time_start())?;
    let dist: f32 = (end - start).length();
    if dist <= 0. {
        return Some(end);
    }
    let end_time: f32 = start_time + dist / MOVER_SPEED;
    let current_time = game_time().as_secs_f32() + time_offset;
    if current_time > end_time {
        return Some(end);
    }

    return Some(start.lerp(
        end,
        invlerp(start_time, end_time, current_time).clamp(0., 1.),
    ));
}

fn lerp(from: f32, to: f32, rel: f32) -> f32 {
    ((1. - rel) * from) + (rel * to)
}

fn invlerp(from: f32, to: f32, value: f32) -> f32 {
    (value - from) / (to - from)
}
