use ambient_api::prelude::*;

mod c_score_ui;
mod c_timer_ui;
mod c_mice_boatmodels;
mod c_camera_follows_mouse;

#[main]
pub async fn main() {
    c_score_ui::setup();
    c_timer_ui::setup();
    c_mice_boatmodels::setup();
    c_camera_follows_mouse::adjust_camera_params();
    run_async(async{
        c_camera_follows_mouse::setup_async().await;
    });
}
