use std::any::TypeId;

use bevy::prelude::*;
use fw_anim::CustomAnimationTrigger;
use fw_button::components::ButtonInteraction;
use mod_level::{CurrentLevel, LevelRegistry, LevelType};
use scene_base::GameScene;

use crate::tag::{BackToTitleButtonTag, ConformButtonTag, MaterialColorAnim};

pub(crate) fn update_material_alpha(
    mut targets: Query<
        (&mut Handle<ColorMaterial>, &CustomAnimationTrigger),
        With<MaterialColorAnim>,
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut material, trigger) in &mut targets {
        let Some(alpha) = trigger
            .animation_value
            .get(&TypeId::of::<MaterialColorAnim>())
        else {
            continue;
        };

        *material = materials.add(Color::srgba(1.0, 1.0, 1.0, *alpha));
    }
}

pub(crate) fn click_conform_button(
    button: Query<&ButtonInteraction, With<ConformButtonTag>>,
    mut next_screen: ResMut<NextState<GameScene>>,
    mut current_level: ResMut<CurrentLevel>,
    level_registry: Res<LevelRegistry>,
) {
    let Some(interaction) = button.iter().next() else {
        return;
    };
    if !matches!(interaction, ButtonInteraction::Click) {
        return;
    }

    let LevelType::Adventure { level } = current_level.id else {
        return;
    };

    current_level.0 = level_registry
        .get(&LevelType::Adventure { level: level + 1 })
        .unwrap()
        .clone();
    next_screen.set(GameScene::Game);
}

pub(crate) fn click_back_to_title_button(
    button: Query<&ButtonInteraction, With<BackToTitleButtonTag>>,
    mut next_screen: ResMut<NextState<GameScene>>,
) {
    let Some(interaction) = button.iter().next() else {
        return;
    };
    if !matches!(interaction, ButtonInteraction::Click) {
        return;
    }
    next_screen.set(GameScene::Title);
}
