use ambient_api::prelude::*;

mod x_head;

#[main]
pub fn main() {
    x_head::setup();
}

pub fn make_head() -> Entity { x_head::make_head() }
pub fn make_make_fully_functional_head_camera() -> Entity { x_head::make_fully_functional_head_camera() }
