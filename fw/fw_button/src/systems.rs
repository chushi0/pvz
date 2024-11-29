use bevy::prelude::*;
use fw_cursor::CursorPosition;

use crate::components::{ButtonBackground, ButtonEnabled, ButtonHotspot, ButtonInteraction};

pub(crate) fn update_interaction(
    mut buttons: Query<(
        &mut ButtonInteraction,
        &ButtonEnabled,
        &GlobalTransform,
        &ButtonHotspot,
    )>,
    cursor_position: Res<CursorPosition>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    let just_pressed = mouse_button_input.just_pressed(MouseButton::Left);
    let just_released = mouse_button_input.just_released(MouseButton::Left);
    let cursor_position = cursor_position.world_position;

    buttons
        .par_iter_mut()
        .for_each(|(mut interaction, disabled, transform, hotspot)| {
            let translation = transform.translation();
            let in_range = hotspot.contains(Vec2 {
                x: cursor_position.x - translation.x,
                y: cursor_position.y - translation.y,
            });

            *interaction = match disabled {
                ButtonEnabled::Disabled => ButtonInteraction::None,
                ButtonEnabled::Enabled => {
                    match (&*interaction, in_range, just_pressed, just_released) {
                        (ButtonInteraction::None | ButtonInteraction::Click, false, _, _) => {
                            ButtonInteraction::None
                        }
                        (ButtonInteraction::None | ButtonInteraction::Click, true, _, _) => {
                            ButtonInteraction::Hover
                        }
                        (ButtonInteraction::Hover, true, false, _) => ButtonInteraction::Hover,
                        (ButtonInteraction::Hover, true, true, _) => ButtonInteraction::Pressed,
                        (ButtonInteraction::Hover, false, _, _) => ButtonInteraction::None,
                        (ButtonInteraction::Pressed, true, _, false) => ButtonInteraction::Pressed,
                        (ButtonInteraction::Pressed, true, _, true) => ButtonInteraction::Click,
                        (ButtonInteraction::Pressed, false, _, _) => ButtonInteraction::Cancel,
                        (ButtonInteraction::Cancel, _, _, false) => ButtonInteraction::Cancel,
                        (ButtonInteraction::Cancel, _, _, true) => ButtonInteraction::None,
                    }
                }
            };
        });
}

pub(crate) fn update_background(
    mut buttons: Query<(
        &ButtonInteraction,
        &ButtonEnabled,
        &ButtonBackground,
        &mut Handle<Image>,
    )>,
) {
    buttons
        .par_iter_mut()
        .for_each(|(interaction, disabled, background, mut image)| {
            let image_should_be = match disabled {
                ButtonEnabled::Disabled => &background.disabled,
                ButtonEnabled::Enabled => match interaction {
                    ButtonInteraction::None | ButtonInteraction::Cancel => &background.normal,
                    ButtonInteraction::Hover | ButtonInteraction::Click => &background.hover,
                    ButtonInteraction::Pressed => &background.pressed,
                },
            };

            if *image_should_be != *image {
                *image = image_should_be.clone();
            }
        });
}
