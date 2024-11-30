use bevy::prelude::*;
use fw_transition::FwTransitionPlugin;

#[derive(Debug, States, Hash, PartialEq, Eq, PartialOrd, Clone, Copy, Default)]
pub enum GameScene {
    #[default]
    Splash,
    Title,
    Game,
}

pub struct SceneBasePlugin;

impl Plugin for SceneBasePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameScene>()
            .add_plugins(FwTransitionPlugin::<GameScene>::default());
    }
}
