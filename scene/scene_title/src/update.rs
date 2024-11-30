use bevy::prelude::*;
use fw_button::components::{ButtonEnabled, ButtonInteraction};
use mod_level::{CurrentLevel, LevelRegistry, LevelType};
use mod_userdata::UserData;
use scene_base::GameScene;

use crate::tag::{AdventureButtonTag, ButtonTextTag, ExitButtonTag};

pub(crate) fn button_hover_text(
    button: Query<&ButtonInteraction>,
    mut button_text: Query<(&mut Text, &Parent), With<ButtonTextTag>>,
) {
    for (mut text, parent) in &mut button_text {
        let Ok(button_interaction) = button.get(parent.get()) else {
            continue;
        };

        text.sections[0].style.color = match button_interaction {
            ButtonInteraction::None | ButtonInteraction::Cancel => Color::BLACK,
            ButtonInteraction::Hover | ButtonInteraction::Pressed | ButtonInteraction::Click => {
                Color::srgb(0.05, 1.0, 0.0)
            }
        }
    }
}

pub(crate) fn click_adventure_button(
    mut button: Query<(&mut ButtonEnabled, &ButtonInteraction), With<AdventureButtonTag>>,
    mut next_state: ResMut<NextState<GameScene>>,
    userdata: Res<UserData>,
    mut current_level: ResMut<CurrentLevel>,
    level_registry: Res<LevelRegistry>,
) {
    let Some((mut enabled, interaction)) = button.iter_mut().next() else {
        return;
    };

    if !matches!(interaction, ButtonInteraction::Click) {
        return;
    }

    *enabled = ButtonEnabled::Disabled;

    current_level.0 = level_registry
        .get(&LevelType::Adventure {
            level: userdata.adventure_progress,
        })
        .unwrap()
        .clone();

    next_state.set(GameScene::Game);
}

pub(crate) fn click_exit_button(
    mut button: Query<(&mut ButtonEnabled, &ButtonInteraction), With<ExitButtonTag>>,
    mut app_exit_event: EventWriter<AppExit>,
) {
    let Some((mut enabled, interaction)) = button.iter_mut().next() else {
        return;
    };

    if !matches!(interaction, ButtonInteraction::Click) {
        return;
    }

    *enabled = ButtonEnabled::Disabled;
    app_exit_event.send(AppExit::Success);
}
