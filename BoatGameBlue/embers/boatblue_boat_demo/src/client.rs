use ambient_api::prelude::*;

use ambient_api::core::model::components::model_from_url;

use boat::components::{image_of_boat};

#[main]
pub fn main() {
    spawn_query(image_of_boat()).bind(|images|{
        for (image, _boat) in images {
            entity::add_component(image, model_from_url(), asset::url("boatblue_boat_demo/assets/MSH_Boat.glb").unwrap());
        }
    });
}
