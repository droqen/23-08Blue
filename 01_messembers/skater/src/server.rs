use ambient_api::prelude::*;

#[main]
pub fn main() {
    rcvmsg_NewSkater::init();
    query_skater_movement::init();
}

mod rcvmsg_NewSkater {

    pub fn init() -> Listener {
        crate::embers::skater::messages::NewSkater::subscribe(|_src, msg| {
            Entity::new()
                .with(skater_key(), msg.key)
                .with(translation(), msg.pos)
                .with(skater_target(), msg.pos)
                .with(skater_fwd(), vec3(0., 1., 0.))
                .with(skater_pace(), 0.)
                .spawn();
        })
    }

    use ambient_api::{core::transform::components::translation, message::Listener, prelude::*};

    use crate::embers::skater::components::{skater_fwd, skater_key, skater_pace, skater_target};
}

mod query_skater_movement {
    const GOALPACE_FROM_REACH: (f32, f32) = (1.0, 5.0);
    const TURNSPDMULT_FROM_REACH: (f32, f32) = (1.0, 2.0);
    const TURNSPD_BY_PACE: (f32, f32) = (5.0, 2.5);
    const TURNSPD_MAX_LERP: f32 = 0.1;
    const PACE_CHANGE_RATE: f32 = 1.0;
    const FRICTION_BY_PACE: (f32, f32) = (1.0, 0.1);
    const ACCEL_BY_PACE: (f32, f32) = (5.0, 4.0);
    const GOALSPEED_BY_PACE: (f32, f32) = (0.5, 10.0);
    const AUTOFWDACCEL_BY_PACE: (f32, f32) = (0.0, 2.0);

    pub fn setup() -> Listener {
        query((
            translation(),
            skater_target(),
            skater_fwd(),
            skater_pace(),
            linear_velocity(),
        ))
        .each_frame(|mice| {
            for (mouse, (pos, cheese, fwd, pace, vel)) in mice {
                let to_cheese: Vec3 = cheese - pos;
                let dist_to_cheese = to_cheese.length();
                let goalpace: f32 = invlerpfrom(GOALPACE_FROM_REACH, dist_to_cheese).clamp(0., 1.);
                let dt: f32 = delta_time();
                entity::set_component(
                    mouse,
                    skater_pace(),
                    tow(pace, goalpace, PACE_CHANGE_RATE * dt),
                );
                if dist_to_cheese > TURNSPDMULT_FROM_REACH.0 {
                    let goalfwd: Vec3 = to_cheese.normalize();
                    let rota: Quat = Quat::from_rotation_arc(fwd, fwd);
                    let rotb: Quat = Quat::from_rotation_arc(fwd, goalfwd);
                    let rotdist: f32 = rota.angle_between(rotb);
                    let turnspd: f32 = lerpby(TURNSPD_BY_PACE, pace)
                        .min(TURNSPD_MAX_LERP * rotdist / dt)
                        * invlerpfrom(TURNSPDMULT_FROM_REACH, dist_to_cheese).clamp(0., 1.);
                    if rotdist > turnspd * dt {
                        entity::set_component(
                            mouse,
                            skater_fwd(),
                            rota.lerp(rotb, turnspd * dt / rotdist) * fwd,
                        );
                    } else {
                        entity::set_component(mouse, skater_fwd(), goalfwd);
                    }
                }
                // entity::set_component(mouse, skater_fwd(), tow(pace,goalpace,PACE_CHANGE_RATE*dt));
                let friction: f32 = lerpby(FRICTION_BY_PACE, pace);
                let accel: f32 = lerpby(ACCEL_BY_PACE, pace);
                let autofwdaccel: f32 = lerpby(AUTOFWDACCEL_BY_PACE, pace);
                let goalspeed: f32 = lerpby(GOALSPEED_BY_PACE, pace);
                let goalvel: Vec3 = goalspeed * to_cheese.normalize_or_zero();
                let to_goalvel = goalvel - vel;
                physics::add_force(
                    mouse,
                    accel * to_goalvel // move directly to cheese
                +
                friction * -vel // friction applied opposite vel
                +
                autofwdaccel * fwd, // a force pushes you skater_fwd
                );
            }
        })
    }

    fn tow(a: f32, b: f32, rate: f32) -> f32 {
        if a + rate < b {
            a + rate
        } else if a - rate > b {
            a - rate
        } else {
            b
        }
    }
    fn lerp(from: f32, to: f32, rel: f32) -> f32 {
        ((1. - rel) * from) + (rel * to)
    }
    fn invlerp(from: f32, to: f32, value: f32) -> f32 {
        (value - from) / (to - from)
    }
    fn lerpby(range: (f32, f32), rel: f32) -> f32 {
        lerp(range.0, range.1, rel)
    }
    fn invlerpfrom(range: (f32, f32), value: f32) -> f32 {
        invlerp(range.0, range.1, value)
    }

    use ambient_api::{
        core::{physics::components::linear_velocity, transform::components::translation},
        message::Listener,
        prelude::*,
    };

    use crate::embers::skater::components::{skater_fwd, skater_pace, skater_target};
}
