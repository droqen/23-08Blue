use ambient_api::prelude::*;
mod s_allplayermice;
mod s_micefloat;
#[main]
pub fn main() {
    s_allplayermice::setup();
    s_micefloat::setup();
}
