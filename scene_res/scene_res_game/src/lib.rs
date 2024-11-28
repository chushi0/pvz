use bevy::{prelude::*, utils::HashMap};
use mod_plant::metadata::PlantType;
use mod_zombie::metadata::ZombieType;

pub struct SceneResGamePlugin;

impl Plugin for SceneResGamePlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<GameSceneSettings>();
        app.insert_resource(GameSceneSettings {
            background: Background::Day {
                sod_type: SodType::None,
                upgrade_sod_type: true,
            },
            levelname: "关卡 1-1".to_owned(),
            sunshine: 150,
            natural_sunshine: true,
            reward: Reward::PlantSeed(PlantType::Sunflower),
            zombie_waves: vec![
                ZombieWave {
                    wave_type: WaveType::Normal,
                    zombies: [(ZombieType::Basic, 1)].into_iter().collect(),
                },
                ZombieWave {
                    wave_type: WaveType::Normal,
                    zombies: [(ZombieType::Basic, 1)].into_iter().collect(),
                },
                ZombieWave {
                    wave_type: WaveType::Normal,
                    zombies: [(ZombieType::Basic, 1)].into_iter().collect(),
                },
                ZombieWave {
                    wave_type: WaveType::Normal,
                    zombies: [(ZombieType::Basic, 2)].into_iter().collect(),
                },
            ],
            ..Default::default()
        });
    }
}

#[derive(Resource, Default)]
pub struct GameSceneSettings {
    // 地皮背景
    pub background: Background,
    // 关卡名
    pub levelname: String,
    // 初始阳光
    pub sunshine: u32,
    // 蘑菇是否需要咖啡豆
    pub day: bool,
    // 自然生产阳光
    pub natural_sunshine: bool,
    // 刷怪
    pub zombie_waves: Vec<ZombieWave>,
    // 墓碑
    pub gravestone: Gravestone,
    // 珊瑚
    pub coral: Coral,
    // 蹦极
    pub bungee: Bungee,
    // 关卡奖励
    pub reward: Reward,
}

pub enum Background {
    // 白天
    Day {
        sod_type: SodType,
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

pub enum SodType {
    None,
    SodRow1,
    SodRow3,
    SodRow5,
}

impl Default for Background {
    fn default() -> Self {
        Self::Day {
            sod_type: SodType::SodRow5,
            upgrade_sod_type: false,
        }
    }
}

pub struct ZombieWave {
    pub wave_type: WaveType,
    pub zombies: HashMap<ZombieType, u32>,
}

pub enum WaveType {
    Normal,
    HugeWave,
}

#[derive(Default)]
pub struct Gravestone {
    // 开始游戏时生成的墓碑数量
    pub start_game: u8,
    // 最后一波时生成的墓碑数量
    pub final_wave: u8,
    // 生成边界
    pub bundary: u8,
}

#[derive(Default)]
pub struct Coral {
    // 最后一波时生成的珊瑚僵尸数量
    pub count: u8,
    // 生成边界
    pub bundary: u8,
}

#[derive(Default)]
pub struct Bungee {
    // 一大波僵尸到来时生成的蹦极僵尸数量
    pub huge_wave: u8,
    // 最后一波时生成的蹦极僵尸数量（额外）
    pub final_wave: u8,
    // 生成边界
    pub bundary: u8,
}

#[derive(Default)]
pub enum Reward {
    PlantSeed(PlantType),
    #[default]
    MoneyBag,
}
