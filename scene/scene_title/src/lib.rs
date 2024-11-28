use bevy::prelude::*;
use scene_base::GameScene;

pub struct SceneTitlePlugin;

impl Plugin for SceneTitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::Title), start_game_scene);
    }
}

fn start_game_scene(mut next_scene: ResMut<NextState<GameScene>>) {
    debug!("debug: no title scene here, start game scene");
    next_scene.set(GameScene::Game);
}
