use std::f32::consts::PI;

use ambient_api::prelude::*;

const YAW_PITCHES: [(f32, f32); 21] = [
    (0.10, -0.04),
    (0.00, 0.07),
    (-0.20, 0.25),
    (-0.15, 0.03),
    (-0.20, 0.28),
    (-1.00, 0.54),
    (0.04, 0.17),
    (-0.20, 0.04),
    (0.94, 0.52),
    (-0.20, 0.68),
    (0.10, 0.33),
    (0.00, 0.15),
    (0.20, 0.34),
    (0.15, 0.88),
    (1.00, 0.50),
    (0.53, 0.14),
    (-0.04, 0.23),
    (-0.20, 0.30),
    (-0.34, 0.21),
    (0.20, -0.05),
    (0.10, 0.05),
];

pub fn rock_progress_to_position(prog: f32) -> Vec3 {
    let bigprog = prog * 20.;
    let mut pos = vec3(0., 0., 0.);
    for yaw_pitch_index in 0..20 {
        let subprog = (bigprog - yaw_pitch_index as f32).min(1.0);
        let (yaw, pitch) = YAW_PITCHES[yaw_pitch_index];
        pos += Quat::from_euler(glam::EulerRot::XYZ, 0., -pitch, yaw - PI * 0.75)
            * vec3(0.0, 3.0, 0.0)
            * subprog;
        if subprog < 1.0 {
            break;
        }
    }
    return pos;
}
pub fn rock_progress_to_position_raw(prog: f32) -> Vec3 {
    let bigprog = prog * 20.;
    let mut pos = vec3(0., 0., 0.);
    for yaw_pitch_index in 0..20 {
        let subprog = (bigprog - yaw_pitch_index as f32).min(1.0);
        let (yaw, pitch) = YAW_PITCHES[yaw_pitch_index];
        pos += Quat::from_euler(glam::EulerRot::XYZ, 0., -pitch, yaw - PI * 0.75)
            * vec3(0.0, 3.0, 0.0)
            * subprog;
        if subprog < 1.0 {
            break;
        }
    }
    return pos;
}
pub fn rock_progress_to_position_smoothed(prog: f32) -> Vec3 {
    return rock_progress_to_position(prog);
    // rock_progress_to_position_raw(prog - 0.04) * 0.25
    //     + rock_progress_to_position_raw(prog) * 0.5
    //     + rock_progress_to_position_raw(prog + 0.04) * 0.25
}
pub fn rock_progress_to_grade(prog: f32) -> f32 {
    let lowerpos = rock_progress_to_position(prog - 0.005);
    let higherpos = rock_progress_to_position(prog + 0.005);
    return (higherpos.z - lowerpos.z) / 0.1;
}

// pub fn get_nearest_prog(pos: Vec3) -> f32 {
//     let mut prog = 0.5;
//     let mut precision = 0.25;
//     for _ in 0..7 {
//         prog = get_nearer_prog(prog - precision, prog + precision, pos);
//         precision *= 0.5;
//     }
//     prog
// }

// fn get_nearer_prog(p1: f32, p2: f32, pos: Vec3) -> f32 {
//     if pos.distance_squared(rock_progress_to_position_raw(p1))
//         < pos.distance_squared(rock_progress_to_position_raw(p2))
//     {
//         return p1;
//     } else {
//         return p2;
//     }
// }

// fn dist_to_prog(prog: f32, pos: Vec3) -> f32 {
//     return;
// }
