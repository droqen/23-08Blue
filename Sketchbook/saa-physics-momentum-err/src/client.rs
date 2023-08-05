use ambient_api::prelude::*;
mod c_debug_driven_cubes;
use boatblue_selfie_camera::components::selfie_focus_ent;
#[main]
pub fn main() {
    spawn_query(()).requires(selfie_focus_ent()).bind(|cameras|
        for (camera, _) in cameras {
            // should only do this once
            c_debug_driven_cubes::setup(camera);
        }
    );
}
