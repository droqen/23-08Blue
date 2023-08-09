use ambient_api::prelude::*;

mod s_adjust_nice_camera;
mod s_spawn_quad;
mod s_spawn_boat;

#[main]
pub fn main() {
    s_adjust_nice_camera::setup();
    s_spawn_quad::setup();
    s_spawn_boat::setup();
}
