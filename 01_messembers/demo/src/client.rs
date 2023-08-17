use ambient_api::prelude::*;

#[main]
pub async fn main() {
    sleep(1.).await; // necessary to allow the cam ember to set itself up before this message is sent (so it's not missed!)
                     // demo_king_of_ephyra::init();
    demo_king_of_ephyra::init();
}

mod demo_shouter {
    pub fn init() {
        crate::spawn_cam::init();
    }
}

mod demo_king_of_ephyra {
    use crate::embers::cam::{
        components::{head_relpos, head_targetent},
        messages::MouseRay,
    };
    use ambient_api::{
        core::{primitives::components::cube, transform::components::translation},
        prelude::*,
    };
    pub fn init() {
        crate::spawn_cam::init_and(|cam| {
            entity::add_component(cam, head_relpos(), vec3(0., 2., 20.));
            entity::add_component(cam, head_targetent(), sphere);
        });
        let mousecube = Entity::new().with(cube(), ()).spawn();
        MouseRay::subscribe(move |_src, msg| {
            let mouse_plane_pos = get_zplane_intersection(msg.origin, msg.dir, 0.);
            entity::set_component(mousecube, translation(), mouse_plane_pos);
        });
    }
    fn get_zplane_intersection(origin: Vec3, dir: Vec3, z: f32) -> Vec3 {
        let dir_mult = (z - origin.z) / dir.z;
        origin + dir_mult * dir
    }
}

mod spawn_cam {
    use crate::embers::cam::{
        components::{cam_key, head_relpos},
        messages::NewCam,
    };
    use ambient_api::{message::Listener, prelude::*};
    pub fn init() {
        // spawn_query(cam_key()).bind(|cams| {
        //     for (cam, key) in cams {
        //         if key == 103 {
        //             entity::add_component(cam, head_relpos(), vec3(0., 2., 10.));
        //         }
        //     }
        // });
        NewCam { key: 103 }.send_local_broadcast(true);
    }
    pub fn init_and(function: impl Fn(EntityId) -> () + 'static) {
        spawn_query(cam_key()).bind(move |cams| {
            for (cam, key) in cams {
                if key == 103 {
                    function(cam);
                }
            }
        });
        NewCam { key: 103 }.send_local_broadcast(true);
    }
}
