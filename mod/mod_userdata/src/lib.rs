use bevy::{prelude::*, utils::HashSet};
use mod_plant::metadata::PlantType;

pub struct ModUserdataPlugin;

impl Plugin for ModUserdataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UserData>();
    }
}

#[derive(Debug, Resource)]
pub struct UserData {
    pub unlock_plugins: HashSet<PlantType>,
    pub plant_solt_count: usize,
    pub pass_adventure_count: u32,
    pub adventure_progress: u32,
    pub unlock_minigame: bool,
    pub unlock_puzzle: bool,
    pub unlock_survial: bool,
    pub unlock_shovel: bool,
    pub unlock_almanac: bool,
    pub unlock_shop: bool,
    pub unlock_shop_taco_upgrade: bool,
    pub unlock_zen_garden: bool,
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            unlock_plugins: [PlantType::PeaShooter].into_iter().collect(),
            plant_solt_count: 6,
            pass_adventure_count: 0,
            adventure_progress: 1,
            unlock_minigame: false,
            unlock_puzzle: false,
            unlock_survial: false,
            unlock_shovel: false,
            unlock_almanac: false,
            unlock_shop: false,
            unlock_shop_taco_upgrade: false,
            unlock_zen_garden: false,
        }
    }
}
