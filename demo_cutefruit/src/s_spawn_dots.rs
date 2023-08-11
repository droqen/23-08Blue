use ambient_api::{
    prelude::*, core::{
        primitives::concepts::make_sphere, transform::components::translation,
        // 
    }
};

use crate::embers::demo_cutefruit::components::is_dot;

pub fn setup() {
    spawn_query((translation(), is_dot())).bind(|dots|for (dot,(pos,_)) in dots{
        entity::add_child(dot,
            Entity::new()
                .with(translation(), pos)
                .with_merge(make_sphere())
                .spawn()
        );
    });
}