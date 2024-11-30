use bevy::prelude::*;
use scene_base::GameScene;

mod setup;
mod tag;
mod update;

pub struct SceneTitlePlugin;

impl Plugin for SceneTitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameScene::Title),
            (
                setup::start_bgm,
                setup::spawn_camera,
                setup::spawn_bg,
                setup::spawn_bg_center,
                setup::spawn_bg_left,
                setup::spawn_bg_right,
            ),
        )
        .add_systems(OnExit(GameScene::Title), setup::clear_scene)
        .add_systems(
            Update,
            (
                update::button_hover_text,
                update::click_adventure_button,
                update::click_exit_button,
            )
                .run_if(in_state(GameScene::Title)),
        );
    }
}
