use ambient_api::prelude::*;

mod s_mice_boatparams;
mod s_mice_collect_dots;
mod s_score;
mod s_spawn_dots;
mod s_timer;
mod s_worldgen;

#[main]
pub fn main() {
    s_mice_boatparams::setup();
    s_mice_collect_dots::setup();
    s_score::setup();
    s_spawn_dots::setup();
    s_timer::setup();
    s_worldgen::setup();
}
