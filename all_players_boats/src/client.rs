use ambient_api::prelude::*;

mod c_mouseinput;

#[main]
pub fn main() {
    c_mouseinput::setup();
}
