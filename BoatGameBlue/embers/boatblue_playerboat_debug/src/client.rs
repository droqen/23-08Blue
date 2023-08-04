use ambient_api::prelude::*;

use ambient_api::core::transform::components::translation;

use boatblue_selfie_camera::components::selfie_focus_ent;

use boatblue_boat::components::{
    boat_steer,
    // 
};

#[main]
pub fn main() {
    spawn_query(selfie_focus_ent()).bind(|cameras|
        for (camera, _) in cameras {
            PlayerboatDebugLines::el(camera).spawn_interactive();
        }
    );
}

#[element_component]
fn PlayerboatDebugLines(hooks : &mut Hooks, c : EntityId) -> Element {
    println!("Debug!");
    let forward_ents = hooks.use_query((translation(), boat_steer()));
    Group::el(
        forward_ents.iter().map(|(_ent,(pos,steer_vec))|{
            let a = *pos;
            let b = a + steer_vec.extend(0.) + vec3(0., 0., 10.);
            elline(a, b, c)
        })
    )
}

use ambient_api::core::{
    rect::components::{line_from, line_to, line_width},
    rendering::components::color,
};

fn elline(from : Vec3, to: Vec3, camera : EntityId) -> Element {
    Line::el()
        .with(line_width(), 2.)
        .with(color(), vec4(1., 1., 0., 1.))
        .with(line_from(), camera::world_to_screen(camera, from).extend(0.))
        .with(line_to(), camera::world_to_screen(camera, to).extend(0.))
}