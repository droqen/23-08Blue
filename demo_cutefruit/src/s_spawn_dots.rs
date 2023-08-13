use ambient_api::{
    prelude::*, core::{
        primitives::concepts::make_sphere,
        transform::components::{translation, rotation},
        physics::components::{angular_velocity, physics_controlled, dynamic, cube_collider},
        model::components::model_from_url,
        // 
    }
};

use crate::embers::demo_cutefruit::components::is_dot;
use crate::embers::buoy::components::buoy_local_center;

pub fn setup() {
    spawn_query((translation(), is_dot())).bind(move|dots|for(dot,(pos,_))in dots{
        let pos = pos + vec3(0., 0., random::<f32>()*10.);
        entity::add_child(dot,
            Entity::new()
                .with(physics_controlled(), ())
                .with(dynamic(), true)
                .with(cube_collider(), Vec3::splat(1.0))
                .with(translation(), pos)
                .with(rotation(), random::<Quat>())
                // .with(angular_velocity(), random::<Quat>() * random::<Vec3>())
                .with(model_from_url(), crate::embers::demo_cutefruit::assets::url("MSH_Prop_Crate.glb"))
                .with(buoy_local_center(),random::<Vec3>()-0.5)
                .with_merge(make_sphere())
                .with_merge(make_buoy())
                .spawn()
        );
    });
}

use ambient_api::prelude::*;
use crate::embers::buoy::components::{buoy_max_drag,buoy_max_force,buoy_radius,buoy_water_level};
// use crate::embers::buoy::components::{buoy_local_center,buoy_max_drag,buoy_max_force,buoy_radius,buoy_submerged,buoy_submerged_center,buoy_water_level};
pub fn make_buoy() -> Entity {
    Entity::new()
        .with(buoy_max_drag(), 1.0)
        .with(buoy_max_force(), 9.81 * 2.)
        .with(buoy_radius(), 0.5)
        .with(buoy_water_level(), 0.)
}