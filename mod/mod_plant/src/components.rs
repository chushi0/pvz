use std::{sync::Arc, time::Duration};

use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::metadata::PlantInfo;

#[derive(Debug, Component)]
pub struct PlantMetaData(pub Arc<PlantInfo>);

#[derive(Debug, Component)]
pub struct Plant;

#[derive(Debug, Component)]
pub struct PlantHp(pub f32);

#[derive(Bundle)]
pub struct PlantBundle {
    // 植物类型
    pub info: PlantMetaData,
    // 植物生命值
    pub hp: PlantHp,

    pub plant: Plant,
    pub spine: SpineBundle,
}

#[derive(Debug, Component)]
pub struct PlantSeed;

#[derive(Debug, Component)]
pub struct SunshineCost(pub u16);

#[derive(Debug, Component, Default)]
pub enum PlantCooldown {
    #[default]
    Ready,
    Cooldown(Duration),
}

#[derive(Debug, Component, Default)]
pub enum PlantUsable {
    #[default]
    Usable,
    Unusable,
}

#[derive(Component, Default)]
pub enum SeedHover {
    #[default]
    None,
    Hover,
}

#[derive(Component)]
pub struct AnimPlantShootTag;

#[derive(Component)]
pub struct AnimPlantProduceTag;

#[derive(Component)]
pub struct AnimPlantInstantTag;

#[derive(Bundle)]
pub struct PlantSeedBundle {
    // 植物类型
    pub info: PlantMetaData,
    // 冷却时间
    pub cooldown: PlantCooldown,
    // 是否可以使用
    pub usable: PlantUsable,

    pub plant_seed: PlantSeed,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,

    pub seed_hover: SeedHover,
}

impl PlantBundle {
    pub fn new(plant: Arc<PlantInfo>) -> Self {
        Self {
            info: PlantMetaData(plant.clone()),
            hp: PlantHp(plant.hp),
            plant: Plant,
            spine: SpineBundle {
                skeleton: plant.render.spine_skeleton.clone(),
                ..Default::default()
            },
        }
    }
}

impl PlantSeedBundle {
    pub fn new(plant: Arc<PlantInfo>) -> Self {
        Self {
            info: PlantMetaData(plant.clone()),
            cooldown: PlantCooldown::Ready,
            usable: PlantUsable::Usable,
            plant_seed: PlantSeed,
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            seed_hover: SeedHover::default(),
        }
    }
}

#[derive(Component)]
pub(crate) struct CooldownOverlay;

#[derive(Component)]
pub(crate) struct UnusedOverlay;
