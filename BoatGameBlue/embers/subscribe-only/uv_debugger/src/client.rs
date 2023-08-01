use ambient_api::prelude::*;

#[main]
pub fn main() {
    pin_mouseuv::messages::PinMouseUv::subscribe(|_src,msg|{
        dbg!(msg);
    });
}