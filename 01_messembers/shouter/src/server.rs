use ambient_api::prelude::*;

mod ServerShoutParticleAnim;
mod ServerMessageNewShout;

#[main]
pub fn main() {
    ServerShoutParticleAnim::setup();
    ServerMessageNewShout::setup();
}
