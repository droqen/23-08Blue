use ambient_api::prelude::*;

mod AnyHeadLookat;
mod AnyHeadTargetEnt;
mod ClientMessageNewCam;
mod ClientQueryCamTilt;
mod ClientQueryMouseSetsTilt;

#[main]
pub fn main() {
    AnyHeadLookat::setup();
    AnyHeadTargetEnt::setup();
    ClientMessageNewCam::setup();
    ClientQueryCamTilt::setup();
    ClientQueryMouseSetsTilt::setup();
}
