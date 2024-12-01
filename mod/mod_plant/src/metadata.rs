use core::f32;
use std::{ops::Deref, sync::Arc};

use bevy::{prelude::*, utils::HashMap};
use bevy_spine::SkeletonData;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, PartialOrd, Ord)]
pub enum PlantType {
    // 豌豆射手
    PeaShooter,
    // 向日葵
    Sunflower,
    // 樱桃炸弹
    CherryBomb,
    // 坚果墙
    WallNut,
    // 土豆雷
    PotatoMine,
    // 寒冰射手
    SnowPea,
    // 大嘴花
    Chomper,
    // 双发射手
    Repeater,
    // 小喷菇
    PuffShroom,
    // 阳光菇
    SunShroom,
    // 大喷菇
    FumeShroom,
    // 墓碑吞噬者
    GraveBuster,
    // 魅惑菇
    HypnoShroom,
    // 胆小菇
    ScaredyShroom,
    // 寒冰菇
    IceShroom,
    // 毁灭菇
    DoomShroom,
    // 睡莲
    LilyPad,
    // 窝瓜
    Squash,
    // 三线射手
    Threepeater,
    // 缠绕海草
    TangleKlep,
    // 火爆辣椒
    Jalapeno,
    // 地刺
    Spikeweed,
    // 火炬树桩
    Torchwood,
    // 高坚果
    TallNut,
    // 海蘑菇
    SeaShroom,
    // 路灯花
    Plantern,
    // 仙人掌
    Cactus,
    // 三叶草
    Blover,
    // 裂荚射手
    SplitPea,
    // 杨桃
    Starfruit,
    // 南瓜头
    Pumpkin,
    // 磁力菇
    MagnetShroom,
    // 卷心菜投手
    CabbagePult,
    // 花盆
    FlowerPot,
    // 玉米投手
    KernelPult,
    // 咖啡豆
    CoffeeBean,
    // 大蒜
    Gralic,
    // 叶子保护伞
    UmbrellaLeaf,
    // 金盏花
    Marigold,
    // 西瓜投手
    MelonPult,
    // 机枪射手
    GatlingPea,
    // 双子向日葵
    TwinSunflower,
    // 忧郁菇
    GloomShroom,
    // 香蒲
    Cattail,
    // 冰瓜
    WinterMelon,
    // 吸金磁
    GoldMagnet,
    // 地刺王
    Spikerock,
    // 玉米加农炮
    CobCannon,
    // 模仿者
    Imitater,
    // 爆炸坚果
    ExplodeNut,
    // 巨型坚果
    GiantWallNut,
}

#[derive(Debug, Deserialize)]
pub(super) struct Plants {
    #[serde(rename = "Plant")]
    pub(super) plants: Vec<PlantInfo>,
}

#[derive(Debug, Resource, Default)]
pub struct PlantRegistry(pub HashMap<PlantType, Arc<PlantInfo>>);

#[derive(Debug, Deserialize)]
pub struct PlantInfo {
    pub id: PlantType,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "Sunshine")]
    #[serde(default)]
    pub sunshine: u32,
    #[serde(rename = "Cooldown")]
    #[serde(default)]
    pub cooldown: f32,
    #[serde(rename = "Hp")]
    #[serde(default = "default_plant_hp")]
    pub hp: f32,
    #[serde(rename = "Render")]
    pub render: PlantRender,
    #[serde(rename = "Position")]
    #[serde(default)]
    pub position: PlantPosition,
    #[serde(rename = "PlantOn")]
    #[serde(default)]
    pub plant_on: PlantOn,
    #[serde(rename = "Shoot")]
    pub shoot: Option<PlantShoot>,
    #[serde(rename = "Produce")]
    pub produce: Option<PlantProduce>,
    #[serde(rename = "Instant")]
    pub instant: Option<PlantInstant>,
    #[serde(rename = "Bowling")]
    #[serde(default)]
    pub bowling: Option<PlantBowling>,
}

#[derive(Debug, Deserialize)]
pub struct PlantBowling {
    #[serde(rename = "Explode")]
    #[serde(default)]
    pub explode_range: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct PlantInstant {
    #[serde(rename = "AnimTime")]
    pub anim_time: f32,
    #[serde(rename = "Invincible")]
    #[serde(default = "default_true")]
    pub invincible: bool,
    #[serde(rename = "EnterSound")]
    #[serde(default)]
    pub enter_sound: Option<String>,
    #[serde(rename = "Effect")]
    pub effects: Vec<InstantEffect>,
}

#[derive(Debug, Deserialize)]
pub struct InstantEffect {
    #[serde(rename = "Type")]
    pub effect_type: InstantEffectType,
    #[serde(rename = "Time")]
    pub effect_time: f32,
    #[serde(rename = "Sound")]
    #[serde(default)]
    pub sound: Option<String>,
    #[serde(rename = "Particle")]
    #[serde(default)]
    pub particle: Option<Particle>,
}

#[derive(Debug, Deserialize)]
pub enum InstantEffectType {
    Explode {
        radius: f32,
        #[serde(default = "default_explode_damage")]
        damage: f32,
    },
}

#[derive(Debug, Deserialize)]
pub enum Particle {
    CherryBomb,
}

#[derive(Debug, Deserialize, Default)]
pub enum PlantPosition {
    #[default]
    Primary,
    Protect,
    Pot,
    Temp,
}

#[derive(Debug, Deserialize)]
pub struct PlantRender {
    #[serde(rename = "Atlas")]
    pub atlas: String,
    #[serde(rename = "Skeleton")]
    pub skeleton: String,
    #[serde(rename = "DamageSkin")]
    #[serde(default = "default_false")]
    pub damage_skin: bool,

    #[serde(skip)]
    pub spine_skeleton: Handle<SkeletonData>,
}

#[derive(Debug, Deserialize)]
pub struct PlantOn {
    #[serde(rename = "Dirt")]
    #[serde(default = "default_true")]
    pub dirt: bool,
    #[serde(rename = "River")]
    #[serde(default = "default_false")]
    pub river: bool,
    #[serde(rename = "Roof")]
    #[serde(default = "default_false")]
    pub roof: bool,
    #[serde(rename = "Lily")]
    #[serde(default = "default_true")]
    pub lily: bool,
    #[serde(rename = "Pot")]
    #[serde(default = "default_true")]
    pub pot: bool,
    #[serde(rename = "Grave")]
    #[serde(default = "default_false")]
    pub grave: bool,
    #[serde(rename = "Hole")]
    #[serde(default = "default_false")]
    pub hole: bool,
}

#[derive(Debug, Deserialize)]
pub struct PlantShoot {
    #[serde(default)]
    #[serde(rename = "Detect")]
    pub detect: PlantDetect,
    #[serde(rename = "Cooldown")]
    pub shoot_cooldown: f32,
    #[serde(rename = "Projectile")]
    pub projectiles: Vec<ShootProjectile>,
}

#[derive(Debug, Deserialize)]
pub struct PlantProduce {
    #[serde(rename = "Cooldown")]
    pub cooldown: f32,
    #[serde(rename = "CooldownSpread")]
    #[serde(default)]
    pub cooldown_spread: f32,
    #[serde(rename = "Product")]
    pub products: Vec<PlantProduct>,
}

#[derive(Debug, Deserialize)]
pub struct PlantProduct {
    #[serde(rename = "Resource")]
    pub resource_type: ResourceType,
    #[serde(rename = "Times")]
    #[serde(default = "f32_infinity")]
    pub max_times: f32,
    #[serde(rename = "Delay")]
    #[serde(default)]
    pub delay: f32,
    #[serde(rename = "Timing")]
    pub timing: f32,
    #[serde(rename = "OffsetX")]
    #[serde(default)]
    pub offset_x: f32,
    #[serde(rename = "OffsetY")]
    #[serde(default)]
    pub offset_y: f32,
}

#[derive(Debug, Deserialize)]
pub enum ResourceType {
    Sunshine,
}

#[derive(Debug, Deserialize, Default)]
pub enum PlantDetect {
    #[default]
    LaneFront,
    LaneBack,
    Lane,
    Rays {
        #[serde(rename = "Direction")]
        direction: f32,
    },
    Screen,
}

#[derive(Debug, Deserialize)]
pub struct ShootProjectile {
    #[serde(rename = "Type")]
    pub projectile_type: ProjectileType,
    #[serde(rename = "Track")]
    pub track: ProjectileTrack,
    #[serde(rename = "Timing")]
    pub shoot_timing: f32,
    #[serde(rename = "OffsetX")]
    #[serde(default)]
    pub offset_x: f32,
    #[serde(rename = "OffsetY")]
    #[serde(default)]
    pub offset_y: f32,
}

#[derive(Debug, Deserialize)]
pub enum ProjectileType {
    Pea,
    SnowPea,
    Cactus,
    Star,
}

#[derive(Debug, Deserialize)]
pub enum ProjectileTrack {
    Line { direction: f32 },
    Throw,
    Follow,
}

impl Deref for PlantRegistry {
    type Target = HashMap<PlantType, Arc<PlantInfo>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for PlantOn {
    fn default() -> Self {
        Self {
            dirt: true,
            river: false,
            roof: false,
            lily: true,
            pot: true,
            grave: false,
            hole: false,
        }
    }
}

const fn default_true() -> bool {
    true
}

const fn default_false() -> bool {
    false
}

const fn f32_infinity() -> f32 {
    f32::INFINITY
}

const fn default_explode_damage() -> f32 {
    1800.
}

const fn default_plant_hp() -> f32 {
    200.
}
