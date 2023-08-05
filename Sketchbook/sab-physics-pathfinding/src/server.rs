use std::f32::consts::PI;

use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        primitives::components::quad,
        transform::{
            components::{lookat_target, translation, rotation},
            concepts::make_transformable,
        },
    },
    prelude::*,
};

mod s_rollers;
mod s_lookdowncamera;
mod s_maze;

#[main]
pub async  fn main() {
    s_lookdowncamera::setup();
    s_maze::setup();

    println!("Hello, Ambient!");
    sleep(1.).await;
    s_rollers::setup();
}
