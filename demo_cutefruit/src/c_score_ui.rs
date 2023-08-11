use std::f32::consts::PI;

use ambient_api::{prelude::*, core::{transform::components::{translation, spherical_billboard, scale, rotation, mesh_to_world}, text::components::text, app::components::name, primitives::components::cube}};

use crate::embers::demo_cutefruit::components::{score, scorebob_parent, scorebob_child, scorebob_throb};

const THROB_SCALE_RANGE : (f32, f32) = (0.1, 0.2);
const THROB_SCALE_LERP : f32 = 0.05;

pub fn setup() {
    spawn_query((translation(),score())).bind(|score_havers|for(score_haver, (pos, score))in score_havers{
        let scorebob = Entity::new()
            .with(name(), "Scorebob".to_string())
            .with(translation(), pos)
            .with(rotation(), Quat::from_rotation_z(PI * -0.25))
            .with(scale(), vec3(0., 0., 0.))
            .with(cube(), ())
            .with(text(), "-0-".to_string())
            .with(scorebob_parent(), score_haver)
            .with(scorebob_throb(), 1.0)
            // .with(cube(), ())
            // .with(spherical_billboard(), ())
            .spawn();
        entity::add_component(score_haver, scorebob_child(), scorebob);
    });
    query((scorebob_parent(), scorebob_throb())).each_frame(|scorebobs|
        for(scorebob,(score_haver,throb))in scorebobs{
            if let Some(score_haver_pos) = entity::get_component(score_haver, translation()) {
                entity::add_component(scorebob, translation(), score_haver_pos + vec3(0., 0., 4.));
            }
            let dt = delta_time();
            if throb > dt {
                entity::set_component(scorebob, scorebob_throb(), throb - dt);
            } else {
                entity::set_component(scorebob, scorebob_throb(), 0.0);
            }
            
            entity::mutate_component(scorebob, scale(), |s|{
                *s=s.lerp(
                    Vec3::splat(lerp(THROB_SCALE_RANGE.0, THROB_SCALE_RANGE.1, throb.sqrt())),
                    0.25,
                );
            });
        }
    );
    change_query((scorebob_child(), score())).track_change(score()).bind(|score_havers|for(score_haver,(scorebob,score))in score_havers{
        entity::add_component(scorebob, text(), format!("-{}-", score));
        entity::mutate_component_with_default(scorebob, scorebob_throb(), 1.0, |throb|*throb=*throb*0.5+1.00);
    });
}

fn lerp(from : f32, to : f32, rel : f32) -> f32 { ((1. - rel) * from) + (rel * to) }