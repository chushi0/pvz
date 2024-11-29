use bevy::prelude::*;
use mod_level::{CurrentLevel, LevelRegistry, LevelType};
use mod_userdata::UserData;
use scene_base::GameScene;

pub struct SceneTitlePlugin;

impl Plugin for SceneTitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::Title), start_game_scene);
    }
}

fn start_game_scene(
    mut next_scene: ResMut<NextState<GameScene>>,
    levels: Res<LevelRegistry>,
    mut current_level: ResMut<CurrentLevel>,
    userdata: Res<UserData>,
) {
    debug!("debug: no title scene here, start game scene");
    next_scene.set(GameScene::Game);

    current_level.0 = levels
        .get(&LevelType::Adventure {
            level: userdata.adventure_progress,
        })
        .unwrap()
        .clone();
}
