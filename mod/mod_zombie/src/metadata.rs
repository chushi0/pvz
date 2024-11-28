use std::{ops::Deref, sync::Arc};

use bevy::{prelude::*, utils::HashMap};
use bevy_spine::SkeletonData;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum ZombieType {
    // 普通僵尸
    Basic,
    // 摇旗僵尸
    Flag,
    // 路障僵尸
    Conehead,
    // 撑杆僵尸
    PoleVaulting,
    // 铁桶僵尸
    Buckethead,
    // 读报僵尸
    Newspaper,
    // 铁栅栏僵尸
    ScreenDoor,
    // 橄榄球僵尸
    Football,
    // 舞王僵尸
    Dancing,
    // 伴舞僵尸
    Backup,
    // 鸭子救生圈僵尸
    Swim,
    // 潜水僵尸
    Snorkel,
    // 雪橇车僵尸
    Zomboni,
    // 雪橇僵尸小队
    BobsledTeam,
    // 海豚骑士僵尸
    Dolphin,
    // 小丑僵尸
    Joker,
    // 气球僵尸
    Balloon,
    // 矿工僵尸
    Digger,
    // 弹跳僵尸
    Pogo,
    // 雪人僵尸
    Snowman,
    // 蹦极僵尸
    Bungee,
    // 扶梯僵尸
    Ladder,
    // 投石车僵尸
    Busketball,
    // 伽刚特尔
    Gargantuar,
    // 小鬼僵尸
    Imp,
    // 僵王博士
    Zomboss,
}

#[derive(Debug, Deserialize)]
pub(super) struct Zombies {
    #[serde(rename = "Zombie")]
    pub(super) zombies: Vec<ZombieInfo>,
}

#[derive(Debug, Resource, Default)]
pub struct ZombieRegistry(pub HashMap<ZombieType, Arc<ZombieInfo>>);

#[derive(Debug, Deserialize)]
pub struct ZombieInfo {
    pub id: ZombieType,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Hp")]
    pub hp: Hp,
    #[serde(rename = "Speed")]
    pub speed: f32,
    #[serde(rename = "Attack")]
    pub attack: f32,
    #[serde(rename = "SummonDelay")]
    #[serde(default = "default_true")]
    pub summon_delay: bool,
    #[serde(rename = "Render")]
    pub render: Render,
    #[serde(rename = "SummonOn")]
    #[serde(default)]
    pub summon_on: SummonOn,
    #[serde(rename = "Jump")]
    pub jump: Option<Jump>,
}

#[derive(Debug, Deserialize)]
pub struct Hp {
    #[serde(rename = "Real")]
    pub real: f32,
    #[serde(rename = "Critical")]
    pub critical: f32,
    #[serde(rename = "Armor")]
    #[serde(default)]
    pub armor: Vec<Armor>,
}

#[derive(Debug, Deserialize)]
pub struct Armor {
    #[serde(default)]
    pub iron: bool,
    #[serde(rename = "$value")]
    pub hp: f32,
}

#[derive(Debug, Deserialize)]
pub struct Render {
    #[serde(rename = "Atlas")]
    pub atlas: String,
    #[serde(rename = "Skeleton")]
    pub skeleton: String,
    #[serde(skip)]
    pub spine_skeleton: Handle<SkeletonData>,
}

#[derive(Debug, Deserialize)]
pub struct SummonOn {
    #[serde(rename = "Dirt")]
    #[serde(default = "default_true")]
    pub dirt: bool,
    #[serde(rename = "River")]
    #[serde(default = "default_false")]
    pub river: bool,
    #[serde(rename = "Roof")]
    #[serde(default = "default_true")]
    pub roof: bool,
}

#[derive(Debug, Deserialize)]
pub struct Jump {
    // 可以跳跃的次数
    #[serde(rename = "Times")]
    pub times: u8,
    // 前摇
    #[serde(rename = "PrecastDelay")]
    #[serde(default)]
    pub precast_delay: f32,
    // 跳跃后速度变更
    #[serde(rename = "Speed")]
    pub speed: Option<f32>,
}

const fn default_true() -> bool {
    true
}

const fn default_false() -> bool {
    false
}

impl Deref for ZombieRegistry {
    type Target = HashMap<ZombieType, Arc<ZombieInfo>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for SummonOn {
    fn default() -> Self {
        Self {
            dirt: true,
            river: false,
            roof: true,
        }
    }
}
