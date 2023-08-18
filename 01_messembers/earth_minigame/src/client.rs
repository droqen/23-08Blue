use ambient_api::prelude::*;

#[main]
pub fn main() {
    mobile_player::init();
    sky_stars::init();
}

mod sky_stars {
    use ambient_api::{
        core::{primitives::concepts::make_sphere, transform::components::translation},
        prelude::*,
    };

    pub fn init() {
        for _ in 0..100 {
            Entity::new()
                .with_merge(make_sphere())
                .with(translation(), random::<Quat>() * vec3(0.0, 100.0, 0.0))
                .spawn();
        }
    }
}

mod mobile_player {
    use ambient_api::{
        core::{
            primitives::concepts::make_sphere,
            transform::{components::translation, concepts::make_transformable},
        },
        prelude::*,
    };

    pub fn init() {
        let playersphere = Entity::new()
            .with_merge(make_transformable())
            .with_merge(make_sphere())
            .spawn();

        ambient_api::core::messages::Frame::subscribe(move |_| {
            let pin = input::get();
            let mut leftstick = vec2(0., 0.);
            if !pin.keys.is_empty() {
                if pin.keys.contains(&input::KeyCode::Left) || pin.keys.contains(&input::KeyCode::A)
                {
                    leftstick.x -= 1.;
                }
                if pin.keys.contains(&input::KeyCode::Right)
                    || pin.keys.contains(&input::KeyCode::D)
                {
                    leftstick.x += 1.;
                }
                if pin.keys.contains(&input::KeyCode::Up) || pin.keys.contains(&input::KeyCode::W) {
                    leftstick.y -= 1.;
                }
                if pin.keys.contains(&input::KeyCode::Down) || pin.keys.contains(&input::KeyCode::S)
                {
                    leftstick.y += 1.;
                }
            }
            entity::mutate_component(playersphere, translation(), |pos| {
                *pos = *pos + leftstick.extend(0.0) * delta_time()
            });
        });
    }
}
