use ambient_api::{
    core::{
        app::components::main_scene,
        camera::{
            components::aspect_ratio_from_window,
            concepts::make_perspective_infinite_reverse_camera,
        },
        primitives::components::quad,
        transform::{
            components::{lookat_target, translation},
            concepts::make_transformable,
        },
        model::components::model_from_url,
    },
    prelude::*,
};

mod s_model_imp;

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with(model_from_url(), sae_model_imp_script::assets::url("MSH_Prop_Trashbag.glb"))
        .spawn();

        Entity::new()
            .with_merge(make_transformable())
            .with(translation(), vec3(1., -1., 0.))
            .with(model_from_url(), s_model_imp::get_asset_url())
            .spawn();

    println!("Hello, Ambient!");
}
