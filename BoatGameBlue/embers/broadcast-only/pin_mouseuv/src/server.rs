use ambient_api::prelude::*;

#[main]
pub fn main() {
    pin_mouseuv::messages::PinMouseUv::subscribe(|_src,msg|{
        pin_mouseuv::messages::PinMouseUv{ u:msg.u, v:msg.v }.send_local_broadcast(false);
    });
}
