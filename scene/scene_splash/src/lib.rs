use bevy::prelude::*;
use scene_base::GameScene;

mod setup;
mod tag;
mod update;

pub struct SceneSplashPlugin;

impl Plugin for SceneSplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameScene::Splash),
            (
                setup::start_bgm,
                setup::setup_camera,
                setup::setup_studio_logo,
                setup::setup_background,
                setup::setup_pvz_logo,
                setup::setup_load_bar,
            ),
        )
        .add_systems(
            Update,
            (
                update::update_alpha_anim,
                update::update_button_color,
                update::input_button_click,
            )
                .run_if(in_state(GameScene::Splash)),
        )
        .add_systems(OnExit(GameScene::Splash), setup::clear_scene);
    }
}
