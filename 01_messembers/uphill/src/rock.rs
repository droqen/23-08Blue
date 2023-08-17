use std::f32::consts::PI;

use ambient_api::prelude::*;

const YAW_PITCHES: [(f32, f32); 11] = [
    (0.10, 0.15),
    (0.00, 0.14),
    (-0.20, 0.23),
    (-0.15, 0.14),
    (-0.20, 0.23),
    (-1.00, 0.14),
    (0.04, 0.05),
    (-0.20, 0.23),
    (0.94, 0.14),
    (-0.20, 0.23),
    (0.10, 0.15),
];

pub fn rock_progress_to_position(prog: f32) -> Vec3 {
    let bigprog = prog * 10.;
    let mut pos = vec3(0., 0., 0.);
    for yaw_pitch_index in 0..10 {
        let subprog = (bigprog - yaw_pitch_index as f32).min(1.0);
        let (yaw, pitch) = YAW_PITCHES[yaw_pitch_index];
        pos += Quat::from_euler(glam::EulerRot::XYZ, 0., pitch, yaw - PI * 0.75)
            * vec3(0.0, 3.0, 1.0)
            * subprog;
        if subprog < 1.0 {
            break;
        }
    }
    println!("prog {prog} = pos {pos:?}");
    return pos;
}
pub fn rock_progress_to_grade(prog: f32) -> f32 {
    let lowerpos = rock_progress_to_position(prog - 0.005);
    let higherpos = rock_progress_to_position(prog + 0.005);
    println!("Grade: {}", (higherpos.z - lowerpos.z) / 0.1);
    return (higherpos.z - lowerpos.z) / 0.1;
}
