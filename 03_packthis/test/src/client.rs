use ambient_api::prelude::*;

#[main]
pub fn main() {
    tester::init();
}

mod tester {
    use crate::packages::this::components::*;
    use ambient_api::{core::app::components::name, prelude::*};
    pub fn init() {
        Entity::new()
            .with(name(), "Test".into())
            .with(test_comp(), "Hello".into())
            .spawn();
    }
}
