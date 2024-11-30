use bevy::prelude::*;
use scene_base::GameScene;

mod startup;
mod tag;
mod update;

pub struct SceneTitlePlugin;

impl Plugin for SceneTitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::Title), startup::start_bgm);
    }
}
