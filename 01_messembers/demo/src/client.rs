use ambient_api::prelude::*;

#[main]
pub async fn main() {
    sleep(1.).await; // necessary to allow the cam ember to set itself up before this message is sent (so it's not missed!)
    embers::cam::messages::NewCam { key: 0 }.send_local_broadcast(true);
}
