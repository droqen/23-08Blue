use ambient_api::prelude::*;

mod ServerMessageNewSkater;
mod ServerQuerySkaterMvmt;
#[main]
pub fn main() {
    ServerMessageNewSkater::setup();
    ServerQuerySkaterMvmt::setup();
}

