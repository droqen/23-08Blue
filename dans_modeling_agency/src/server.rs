use ambient_api::prelude::*;

mod s_adjust_nice_camera;
mod s_spawn_quad;
mod s_spawn_boat;
mod s_spawn_snickers_bar;
mod s_spawn_cube_city;

#[main]
pub fn main() {
    s_adjust_nice_camera::setup();
    // s_spawn_quad::setup();
    // s_spawn_boat::setup();
    // s_spawn_snickers_bar::setup();
    s_spawn_cube_city::setup();
}
