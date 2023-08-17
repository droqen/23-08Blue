use ambient_api::prelude::*;

#[main]
pub fn main() {
    camera_registration::init();
    // player_registration::init();
}

mod camera_registration {
    use crate::embers::babel::messages::{OneCameraRegistered, RegisterCamera};
    use ambient_api::prelude::*;
    pub fn init() {
        let mut camera: Option<EntityId> = None;
        RegisterCamera::subscribe(move |_src, msg| {
            if camera.is_none() {
                println!("Registered camera");
                camera = Some(msg.entity);
                OneCameraRegistered { camera: msg.entity }.send_local_broadcast(false);
            } else {
                println!("Error - Ignoring registering a second camera. Bad bad bad.");
            }
        });
    }
}

mod player_registration {
    use crate::embers::babel::messages::{OnePlayerRegistered, RegisterPlayer};
    use ambient_api::prelude::*;
    pub fn init() {
        let mut player: Option<EntityId> = None;
        RegisterPlayer::subscribe(move |_src, msg| {
            if player.is_none() {
                println!("Registered player");
                player = Some(msg.entity);
                OnePlayerRegistered { player: msg.entity }.send_local_broadcast(false);
            } else {
                println!("Error - Ignoring registering a second player. Bad bad bad.");
            }
        });
    }
}
