use std::{fs::File, ops::Deref, sync::Arc};

use bevy::{prelude::*, utils::HashMap};
use mod_item::ItemType;
use mod_plant::metadata::PlantType;
use mod_zombie::metadata::ZombieType;
use serde::Deserialize;

pub struct ModLevelPlugin;

#[derive(Resource)]
pub struct LevelRegistry(pub HashMap<LevelType, Arc<Level>>);

#[derive(Resource)]
pub struct CurrentLevel(pub Arc<Level>);

impl Plugin for ModLevelPlugin {
    fn build(&self, app: &mut App) {
        let levels = load_levels();
        let current_level = levels
            .0
            .get(&LevelType::Adventure { level: 1 })
            .unwrap()
            .clone();

        app.insert_resource(levels)
            .insert_resource(CurrentLevel(current_level));
    }
}

fn load_levels() -> LevelRegistry {
    let levels: Levels =
        serde_xml_rs::from_reader(File::open("./assets/data/levels.xml").unwrap()).unwrap();
    debug!("loaded levels: {:?}", levels);

    LevelRegistry(
        levels
            .levels
            .into_iter()
            .map(|level| (level.id, Arc::new(level)))
            .collect(),
    )
}

impl Deref for LevelRegistry {
    type Target = HashMap<LevelType, Arc<Level>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for CurrentLevel {
    type Target = Level;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
pub enum LevelType {
    Adventure { level: u32 },
    PlantZombie,
}

#[derive(Debug, Deserialize)]
struct Levels {
    #[serde(rename = "Level")]
    levels: Vec<Level>,
}

#[derive(Debug, Deserialize)]
pub struct Level {
    #[serde(alias = "LevelType")]
    pub id: LevelType,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Background")]
    pub background: LevelBackground,
    #[serde(rename = "Sunshine")]
    #[serde(default = "default_sunshine")]
    pub sunshine: u32,
    #[serde(rename = "NatureSunshine")]
    #[serde(default = "default_true")]
    pub nature_sunshine: bool,
    #[serde(rename = "FirstWaveTime")]
    #[serde(default = "default_first_wave_time")]
    pub first_wave_time: f32,
    #[serde(rename = "Reward")]
    pub reward: Option<Reward>,
    #[serde(rename = "PreviewZombie")]
    pub preview_zombies: Vec<Zombie>,
    #[serde(rename = "Wave")]
    pub waves: Vec<Wave>,
}

#[derive(Debug, Deserialize)]
pub enum LevelBackground {
    // 白天
    Day {
        #[serde(rename = "type")]
        #[serde(default)]
        sod_type: SodType,
        #[serde(rename = "upgrade")]
        #[serde(default = "default_false")]
        upgrade_sod_type: bool,
    },
    // 黑夜
    Night,
    // 泳池
    Swim,
    // 迷雾
    SwimFog,
    // 屋顶
    Roof,
    // BOSS战（屋顶黑夜）
    RoofNight,
}

#[derive(Debug, Deserialize, Default)]
pub enum SodType {
    None,
    SodRow1,
    SodRow3,
    #[default]
    SodRow5,
}

#[derive(Debug, Deserialize)]
pub struct Zombie {
    #[serde(rename = "$value")]
    pub zombie: ZombieType,
    #[serde(default = "default_zombie_count")]
    pub count: u8,
}

#[derive(Debug, Deserialize)]
pub struct Wave {
    #[serde(rename = "type")]
    #[serde(default)]
    pub wave_type: WaveType,
    #[serde(rename = "Zombie")]
    pub zombies: Vec<Zombie>,
}

#[derive(Debug, Deserialize, Default)]
pub enum WaveType {
    #[default]
    Normal,
    HugeWave,
}

#[derive(Debug, Deserialize)]
pub enum Reward {
    Plant {
        #[serde(rename = "type")]
        plant: PlantType,
    },
    Item {
        #[serde(rename = "type")]
        item: ItemType,
    },
    Note {
        #[serde(rename = "path")]
        note_path: String,
    },
}

const fn default_zombie_count() -> u8 {
    1
}

const fn default_sunshine() -> u32 {
    50
}

const fn default_first_wave_time() -> f32 {
    30.0
}

const fn default_true() -> bool {
    true
}

const fn default_false() -> bool {
    false
}
