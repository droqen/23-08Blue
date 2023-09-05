use ambient_api::prelude::*;

#[main]
pub fn main() {
    config_setting_messages::init();
    display_player_scores::init();
}

mod display_player_scores {
    use crate::packages::this::components::{
        player_score, player_score_position, score_config_camera,
    };
    use ambient_api::{core::transform::components::translation, prelude::*};
    pub fn init() {
        spawn_query(score_config_camera()).bind(|cfgs| {
            for (_cfg, camera) in cfgs {
                ScoreUI::el(camera).spawn_interactive();
            }
        });
        // let find_config_camera = query(score_config_camera()).build();
        // ambient_api::core::messages::Frame::subscribe(move |_| {
        //     if let Some(camera_result) = find_config_camera.evaluate().first() {
        //         let camera: EntityId = camera_result.1;
        //     }
        // });
    }

    // DISPLAYS SCORE TEXT above players who have score > 0
    #[element_component]
    pub fn ScoreUI(hooks: &mut Hooks, camera: EntityId) -> Element {
        let score_having_ents = hooks.use_query((player_score(), player_score_position()));
        Group::el(score_having_ents.iter().map(|(_ent, (score, pos))| {
            if *score > 0 {
                Text::el(format!("{}", score)).with(
                    translation(),
                    camera::world_to_screen(camera, *pos).extend(0.5) + vec3(-5., 0., 0.),
                )
            } else {
                Text::el("".to_string())
            }
        }))
    }
}

mod config_setting_messages {
    use crate::packages::this::{components::score_config_camera, messages::ConfigureCamera};
    use ambient_api::prelude::*;

    pub fn init() {
        let cfg = Entity::new().spawn();
        {
            let cfg = cfg.clone();
            ConfigureCamera::subscribe(move |_src, msg| {
                entity::add_component(cfg, score_config_camera(), msg.camera);
            });
        }
    }
}
