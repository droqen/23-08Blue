use ambient_api::prelude::*;

mod x_head;
mod c_nice_cam;

#[main]
pub fn main() {
    x_head::setup();
    c_nice_cam::setup();
}
