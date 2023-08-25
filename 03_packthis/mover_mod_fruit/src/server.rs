use ambient_api::prelude::*;

#[main]
pub fn main() {
    spawn_lots_of_fruit::setup();
}

mod spawn_lots_of_fruit {
    // use crate::packages::mover_this::MAZE_WIDTH;
    use ambient_api::{
        core::{
            primitives::concepts::make_sphere,
            rendering::components::color,
            transform::components::{scale, translation},
        },
        prelude::*,
    };
    pub fn setup() {
        for x in 0..8 {
            for y in 0..5 {
                Entity::new()
                    .with_merge(make_sphere())
                    .with(scale(), Vec3::splat(0.125))
                    .with(color(), vec4(1., 0., 0., 1.))
                    .with(
                        translation(),
                        (vec2(x as f32, y as f32) * 2.0 - 6.0).extend(0.0),
                    )
                    .spawn();
            }
        }
    }
}
