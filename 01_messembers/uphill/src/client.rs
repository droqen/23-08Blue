use ambient_api::core::app::components::name;
use ambient_api::core::primitives::concepts::make_sphere;
use ambient_api::core::transform::components::translation;
use ambient_api::prelude::*;

use embers::cam::components::{cam_key, head_targetent};
use embers::uphill::components::rock_progress;

mod rock;

#[main]
pub async fn main() {
    sleep(0.5).await;

    embers::cam::messages::NewCam { key: 1 }.send_local_broadcast(false); // spawn a cam

    sleep(0.5).await;

    // each rock (1) gets a clientside rock spr
    spawn_query(rock_progress()).bind(|rocks| {
        for (rock, progress) in rocks {
            let rock_spr = Entity::new()
                .with_merge(make_sphere())
                .with(name(), "Rock spr".to_string())
                .spawn();
            // follow.
            let cameras = query(cam_key()).build().evaluate();
            // dbg!(cameras);
            for (cam, _key) in cameras {
                entity::add_component(cam, head_targetent(), rock_spr.clone());
            }
            ambient_api::core::messages::Frame::subscribe(move |_| {
                if let Some(serv_rock_prog) = entity::get_component(rock, rock_progress()) {
                    let pos = rock::rock_progress_to_position_smoothed(serv_rock_prog);
                    entity::add_component(rock_spr, translation(), pos);
                }
            });
        }
    });

    // // draw a clientside sphere for the rock
    // let find_rock = query(rock_progress()).build();
    // // let rocks = find_rock.evaluate();
    // if let Some((rock, _)) = find_rock.evaluate().first() {
    //     let rock_spr = Entity::new().with_merge(make_sphere()).spawn();
    //     ambient_api::core::messages::Frame::subscribe(move |_| {
    //         if let Some(serv_rock_prog) = entity::get_component(*rock, rock_progress()) {
    //             let pos = rock::rock_progress_to_position(serv_rock_prog);
    //             entity::set_component(rock_spr, translation(), pos);
    //         }
    //     });
    // } else {
    //     panic!("No rock found :( (spr)");
    // }

    mouse_click_sends_push_message::init();
}

mod mouse_click_sends_push_message {
    use ambient_api::prelude::*;
    pub fn init() {
        ambient_api::core::messages::Frame::subscribe(move |_| {
            let (delta, input) = input::get_delta();
            if (delta.mouse_buttons.contains(&MouseButton::Left)) {
                crate::embers::uphill::messages::PushRock { force: 1.0 }.send_server_reliable();
            }
        });
    }
}

// mod mouse_click_moves_cursor_cube {
//     use ambient_api::prelude::*;
//     pub fn init() {
//         ambient_api::core::messages::Frame::subscribe(move |_| {
//             let (delta, input) = input::get_delta();
//             if (delta.mouse_buttons.contains(&MouseButton::Left)) {
//                 crate::rock::get_nearest_prog(delta.mouse_position)
//                 crate::embers::uphill::messages::PushRock { force: 1.0 }.send_server_reliable();
//             }
//         });
//     }
// }
