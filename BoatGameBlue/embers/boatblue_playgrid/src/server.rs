use ambient_api::prelude::*;

mod s_mkgrid_basic;

#[main]
pub fn main() {
    s_mkgrid_basic::setup_grid_entities();
    s_mkgrid_basic::setup_shack_colliders();
    s_mkgrid_basic::setup_shack_models();
}
