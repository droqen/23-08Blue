use crate::packages::defender_ptt::components::{runner_spawnpos, runner_spawntime};
use ambient_api::prelude::*;

pub enum PredictionError {
    NotRunner,
    FailedPrediction,
}

pub fn try_calc_is_runner_done(
    runner: EntityId,
    time_offset: f32,
) -> Result<bool, PredictionError> {
    if let Some(spawntime) = entity::get_component(runner, runner_spawntime()) {
        Ok(game_time().as_secs_f32() + time_offset - spawntime > RUNNER_DEATHTIME)
    } else {
        Err(PredictionError::NotRunner)
    }
}

pub fn try_calc_runner_pos(runner: EntityId, time_offset: f32) -> Result<Vec3, PredictionError> {
    if let Some(spawnpos) = entity::get_component(runner, runner_spawnpos()) {
        if let Some(spawntime) = entity::get_component(runner, runner_spawntime()) {
            Ok(calc_runner_pos(
                spawnpos,
                game_time().as_secs_f32() + time_offset - spawntime,
            )
            .extend(0.0))
        } else {
            Err(PredictionError::NotRunner)
        }
    } else {
        Err(PredictionError::NotRunner)
    }
}

const RUNNER_SPEED: f32 = 7.0;

const RUNNER_DEATHTIME: f32 = 4.0;

fn calc_runner_pos(spawnpos: Vec2, lifetime: f32) -> Vec2 {
    spawnpos + Vec2::splat(lifetime * RUNNER_SPEED)
}
