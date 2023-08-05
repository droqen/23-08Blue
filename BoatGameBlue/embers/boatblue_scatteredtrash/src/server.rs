
use ambient_api::prelude::*;

mod s_scatter_trash;

#[main]
pub fn main() {
    for i in 0..100 {
        s_scatter_trash::spawn_random_sky_trash(-50., 0.);
    }
}