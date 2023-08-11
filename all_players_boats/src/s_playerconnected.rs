use ambient_api::{prelude::*, core::player::components::{is_player, user_id}};
pub fn setup() {
    spawn_query((is_player(), user_id())).bind(|players|for (player,(_,uid)) in players {
        println!("!!! player connected !!! {uid:?} !!!");
    });
}
