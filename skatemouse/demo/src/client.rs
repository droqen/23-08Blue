use ambient_api::prelude::*;

mod c_pointerplane;
mod c_pointerplanepreview;
mod c_camfollowmouse;

#[main]
pub fn main() {
    c_pointerplane::setup();
    c_pointerplanepreview::setup();
    c_camfollowmouse::setup();
}
