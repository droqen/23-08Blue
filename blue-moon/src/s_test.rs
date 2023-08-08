use ambient_api::prelude::*;
use ambient_api::core::app::components::name;
use crate::blue_moon::components::test_component;

pub fn setup() {
    Entity::new()
        .with(name(), "s_test".to_string())
        .with(test_component(), ())
        .spawn();
    println!("s_test::setup() called successfully...");
}