use bevy::prelude::*;

#[derive(Debug, States, Hash, PartialEq, Eq, PartialOrd, Clone, Copy, Default)]
pub enum GameScene {
    #[default]
    Title,
    Game,
}

pub struct SceneBasePlugin;

impl Plugin for SceneBasePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameScene>();
    }
}
