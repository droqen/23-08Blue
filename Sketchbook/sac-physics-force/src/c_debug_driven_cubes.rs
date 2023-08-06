use ambient_api::{
    core::{
        app::components::{main_scene, name},
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        layout::components::space_between_items,
        physics::components::sphere_collider,
        rendering::components::color,
        transform::components::{lookat_target, translation, rotation},
        rect::components::{
            background_color, border_color, border_radius, border_thickness, line_from, line_to,
            line_width,
        },
        layout::components::{width,height,},
    },
    prelude::*,
};

use crate::sac_physics_force::components::driven_power;

pub fn setup(camera : EntityId) {
    DebugDrivenPower::el(camera).spawn_interactive();
}

// DISPLAYS LINE INDICATING Glider_FORWARD
#[element_component]
pub fn DebugDrivenPower(hooks: &mut Hooks, camera : EntityId) -> Element {
    let driven_ents = hooks.use_query((translation(), driven_power(), name()));
    Group::el(
        driven_ents.iter().map(|(_ent,(pos,power,name))|{
            Text::el( format!("({}) {}", power, name) )
                .with( translation(), camera::world_to_screen(camera, *pos).extend(0.) + vec3(10., 0., 0.) )
        })
    )
}

// // DISPLAYS LINE INDICATING LOCAL_FORWARD
// #[element_component]
// pub fn LocalForwardUI(hooks: &mut Hooks, camera : EntityId) -> Element {
//     let forward_ents = hooks.use_query((translation(), rotation(), local_forward()));
//     Group::el(
//         forward_ents.iter().map(|(_ent,(pos,rot,local_fwd))|{
//             Line::el()
//                 .with(line_width(), 2.)
//                 .with(color(), vec4(1., 1., 0., 1.))
//                 .with(line_from(), camera::world_to_screen(camera, *pos).extend(0.))
//                 .with(line_to(), camera::world_to_screen(camera, *pos + (*rot * *local_fwd) * 10.).extend(0.))
//         })
//     )
// }

// // DISPLAYS DEBUG CIRCLE ON SPHERE COLLIDERS
// #[element_component]
// pub fn PhysCircleUI(hooks: &mut Hooks, camera: EntityId) -> Element {
//     let sphere_ents = hooks.use_query((translation(), sphere_collider()));
//     Group::el(
//         sphere_ents.iter().map(|(_id,(pos,_radius))|{
//             Centered::el([
//             Rectangle::el()
//                         .with(line_width(), 2.)
//                         .with(width(), 20.)
//                         .with(height(), 20.)
//             ])
//             .with(translation(), camera::world_to_screen(camera, *pos).extend(0.))
//             // Line::el()
//             //     .with(line_from(), camera::world_to_screen(camera, *pos).extend(0.))
//             //     .with(line_to(), camera::world_to_screen(camera, *hook_pos).extend(0.))
//             //     .with(background_color(), vec4(1.,0.,1.,1.))
//             //     .with(line_width(), 2.)
//             // // Text::el(name)
//             // //     .with(translation(), camera::world_to_screen(camera, *pos).extend(0.))
//         })
//     )
// }

// // DISPLAYS DEBUG LINE OF PLAYER'S PATH...
// #[element_component]
// pub fn HookLineUI(hooks: &mut Hooks, camera: EntityId) -> Element {
//     let hooky_ents = hooks.use_query((translation(), glider_hook_pos()));
//     Group::el(
//         hooky_ents.iter().map(|(_id,(pos,hook_pos))|{
//             Line::el()
//                 .with(line_from(), camera::world_to_screen(camera, *pos).extend(0.))
//                 .with(line_to(), camera::world_to_screen(camera, *hook_pos).extend(0.))
//                 .with(background_color(), vec4(1.,0.,1.,1.))
//                 .with(line_width(), 2.)
//             // Text::el(name)
//             //     .with(translation(), camera::world_to_screen(camera, *pos).extend(0.))
//         })
//     )
// }

// // DISPLAYS NAMES OF ALL ENTITIES THAT HAVE NAMES...
// #[element_component]
// pub fn NameUI(hooks: &mut Hooks, camera: EntityId) -> Element {
//     let named_ents = hooks.use_query((translation(), name()));
//     Group::el(
//         named_ents.iter().map(|(_id,(pos,name))|{
//             Text::el(name)
//                 .with(translation(), camera::world_to_screen(camera, *pos).extend(0.))
//         })
//     )
// }