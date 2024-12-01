use bevy::prelude::*;
use scene_base::GameScene;

mod setup;
mod tag;
mod update;

#[derive(Component)]
pub struct SceneRewardPlugin;

impl Plugin for SceneRewardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameScene::Reward),
            (
                setup::setup_bgm,
                setup::setup_camera,
                setup::setup_background,
                setup::setup_text,
                setup::setup_fadein,
                setup::setup_reward_info,
                setup::setup_conform_button,
                setup::setup_back_to_title_button,
            ),
        )
        .add_systems(OnExit(GameScene::Reward), setup::clear_scene)
        .add_systems(
            Update,
            (
                update::update_material_alpha,
                update::click_conform_button,
                update::click_back_to_title_button,
            ),
        );
    }
}
