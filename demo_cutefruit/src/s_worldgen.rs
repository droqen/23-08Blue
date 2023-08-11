use ambient_api::{prelude::*, core::transform::components::translation};

use crate::ww_gen::components::{block_size, block_door_side, block_decor_cube, };

use crate::demo_cutefruit::components::is_dot;

pub fn setup() {
    for x in 0..15 { for y in 0..15 {
        let pos = 6.0 * vec2(x as f32, y as f32).extend(0.0);
        if random::<f32>()<0.25 {
            Entity::new()
                .with(translation(), pos)
                .with(block_size(), vec3(6.0, 6.0, 10.0))
                .with(block_decor_cube(), ())
                .spawn();
        } else {
            Entity::new()
                .with(translation(), pos)
                .with(is_dot(), ())
                .spawn();
        }
    }}
    // Entity::new()
    //     .with(translation(), vec3(0., 0., 0.))
    //     .with(block_size(), vec3(6.0, 6.0, 10.0))
    //     .with(block_decor_cube(), ())
    //     .spawn();

    // Entity::new()
    //     .with(translation(), vec3(6.0, -6.0, 0.))
    //     .with(block_size(), vec3(6.0, 6.0, 10.0))
    //     .with(block_door_side(), 0)
    //     .with(block_decor_cube(), ())
    //     .spawn();

    // Entity::new()
    //     .with(translation(), vec3(12.0, -12.0, 0.))
    //     .with(block_size(), vec3(6.0, 6.0, 10.0))
    //     .with(block_door_side(), 1)
    //     .with(block_decor_cube(), ())
    //     .spawn();
}