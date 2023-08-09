use ambient_api::{
    core::{
        physics::components::linear_velocity,
        transform::components::translation,
    },
    prelude::*
};

use crate::skatemouse::components::mouse_cheese;

pub fn setup() {
    query((translation(), mouse_cheese(), linear_velocity())).each_frame(|mice|for(mouse,(pos,cheese,vel)) in mice{
        physics::add_force(mouse, cheese-pos-vel); // dead simple: move from pos to cheese, and add friction opposite vel
    });
}
