use ambient_api::prelude::*;

mod s_spawn_maze;
mod s_spawn_mice;
mod s_micefloat;

#[main]
pub fn main() {
    s_spawn_maze::setup();
    s_spawn_mice::setup();
    s_micefloat::setup();
}
