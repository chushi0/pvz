use std::any::TypeId;

use bevy::prelude::*;
use fw_anim::CustomAnimationTrigger;
use fw_button::components::ButtonInteraction;
use scene_base::GameScene;

use crate::tag::{AnimSpriteAlpha, LoadingText, StartGameButtonTag};

pub(crate) fn update_alpha_anim(
    mut targets: Query<(&mut Sprite, &CustomAnimationTrigger), With<AnimSpriteAlpha>>,
) {
    for (mut sprite, trigger) in &mut targets {
        let Some(alpha) = trigger
            .animation_value
            .get(&TypeId::of::<AnimSpriteAlpha>())
        else {
            continue;
        };
        sprite.color.set_alpha(*alpha);
    }
}

pub(crate) fn update_button_color(
    button: Query<&ButtonInteraction, With<StartGameButtonTag>>,
    mut text: Query<&mut Text, With<LoadingText>>,
) {
    let Some(interaction) = button.iter().next() else {
        return;
    };

    for mut text in &mut text {
        text.sections[0].style.color = match interaction {
            ButtonInteraction::None | ButtonInteraction::Cancel => Color::srgb(0.8, 0.5, 0.3),
            ButtonInteraction::Hover => Color::srgb(0.9, 0.55, 0.33),
            ButtonInteraction::Pressed | ButtonInteraction::Click => Color::srgb(0.7, 0.45, 0.27),
        };
    }
}

pub(crate) fn input_button_click(
    button: Query<&ButtonInteraction, With<StartGameButtonTag>>,
    mut next_state: ResMut<NextState<GameScene>>,
) {
    let Some(interaction) = button.iter().next() else {
        return;
    };

    if matches!(interaction, ButtonInteraction::Click) {
        next_state.set(GameScene::Title);
    }
}
