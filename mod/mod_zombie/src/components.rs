use std::sync::Arc;

use bevy::prelude::*;
use bevy_spine::SpineBundle;

use crate::metadata::{Hp, ZombieInfo};

#[derive(Debug, Component)]
pub struct ZombieMetadata(pub Arc<ZombieInfo>);

#[derive(Debug, Component)]
pub struct Zombie;

#[derive(Debug, Component)]
pub struct ZombieHp {
    pub hp: f32,
    pub armor_hp: Vec<f32>,
}

#[derive(Bundle)]
pub struct ZombieBundle {
    pub info: ZombieMetadata,
    pub hp: ZombieHp,
    pub zombie: Zombie,
    pub spine: SpineBundle,
}

// 僵尸移动动画
#[derive(Debug, Component)]
pub struct AnimZombieMoveTag;

#[derive(Debug, Component, Default)]
pub(crate) struct AnimZombieMovePlayingTag;

// 僵尸啃食动画
#[derive(Debug, Component)]
pub struct AnimZombieEatTag;

#[derive(Debug, Component, Default)]
pub(crate) struct AnimZombieEatPlayingTag;

// 僵尸啃食结束
#[derive(Debug, Component)]
pub struct AnimZombieEatStopTag;

// 僵尸半血动画
#[derive(Debug, Component)]
pub struct AnimZombieHalfDamageTag;

// 僵尸进入临界动画
#[derive(Debug, Component)]
pub struct AnimZombieFullDamageTag;

// 僵尸死亡动画
#[derive(Debug, Component)]
pub struct AnimZombieCriticalTag;

impl ZombieBundle {
    pub fn new(zombie: Arc<ZombieInfo>) -> Self {
        Self {
            info: ZombieMetadata(zombie.clone()),
            hp: ZombieHp::from(&zombie.hp),
            zombie: Zombie,
            spine: SpineBundle {
                skeleton: zombie.render.spine_skeleton.clone(),
                ..Default::default()
            },
        }
    }
}

impl From<&Hp> for ZombieHp {
    fn from(value: &Hp) -> Self {
        ZombieHp {
            hp: value.real + value.critical,
            armor_hp: value.armor.iter().map(|armor| armor.hp).collect(),
        }
    }
}