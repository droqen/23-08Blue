
use ambient_api::prelude::*;

use boatblue_scatteredtrash::components::*;

mod s_scatter_trash;

#[main]
pub fn main() {
    for _ in 0..100 {
        s_scatter_trash::spawn_random_sky_trash(-50., 0.);
    }
}