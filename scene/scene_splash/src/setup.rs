use std::{any::TypeId, time::Duration};

use bevy::{prelude::*, render::camera::ScalingMode, sprite::Anchor};
use fw_anim::{AnimationBundle, AnimationClip, AnimationClips, CustomAnimationTrigger, KeyFrame};
use fw_button::components::{ButtonBackground, ButtonBundle, ButtonHotspot};
use fw_ftxm::{FtxmSource, MainMusicTable};

use crate::tag::{AnimSpriteAlpha, LoadingGrass, LoadingText, SceneTag, StartGameButtonTag};

pub(crate) fn start_bgm(mut commands: Commands) {
    commands.spawn(FtxmSource {
        pot: MainMusicTable::Title.into(),
    });
}

pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scaling_mode: ScalingMode::Fixed {
                    width: 800.,
                    height: 600.,
                },
                ..Default::default()
            },
            ..Default::default()
        },
        SceneTag,
    ));
}

pub(crate) fn setup_studio_logo(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("images/PopCap_Logo.jpg");
    let entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                    ..Default::default()
                },
                texture: image,
                ..Default::default()
            },
            SceneTag,
            AnimSpriteAlpha,
            CustomAnimationTrigger::default(),
        ))
        .id();
    commands.spawn((
        AnimationBundle {
            animation_clips: AnimationClips(vec![AnimationClip {
                entity,
                keyframes: vec![
                    KeyFrame {
                        time: Duration::ZERO,
                        custom_animation_triggers: Some(
                            [(TypeId::of::<AnimSpriteAlpha>(), 0.0)]
                                .into_iter()
                                .collect(),
                        ),
                        ..Default::default()
                    },
                    KeyFrame {
                        time: Duration::from_secs_f32(0.5),
                        custom_animation_triggers: Some(
                            [(TypeId::of::<AnimSpriteAlpha>(), 0.0)]
                                .into_iter()
                                .collect(),
                        ),
                        ..Default::default()
                    },
                    KeyFrame {
                        time: Duration::from_secs_f32(1.0),
                        custom_animation_triggers: Some(
                            [(TypeId::of::<AnimSpriteAlpha>(), 1.0)]
                                .into_iter()
                                .collect(),
                        ),
                        ..Default::default()
                    },
                    KeyFrame {
                        time: Duration::from_secs_f32(2.0),
                        custom_animation_triggers: Some(
                            [(TypeId::of::<AnimSpriteAlpha>(), 1.0)]
                                .into_iter()
                                .collect(),
                        ),
                        ..Default::default()
                    },
                    KeyFrame {
                        time: Duration::from_secs_f32(2.5),
                        custom_animation_triggers: Some(
                            [(TypeId::of::<AnimSpriteAlpha>(), 0.0)]
                                .into_iter()
                                .collect(),
                        ),
                        ..Default::default()
                    },
                ],
            }]),
            ..Default::default()
        },
        SceneTag,
    ));
}

pub(crate) fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("images/titlescreen.jpg");
    let entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                    ..Default::default()
                },
                texture: image,
                ..Default::default()
            },
            SceneTag,
            AnimSpriteAlpha,
            CustomAnimationTrigger::default(),
        ))
        .id();
    commands.spawn((
        AnimationBundle {
            animation_clips: AnimationClips(vec![AnimationClip {
                entity,
                keyframes: vec![
                    KeyFrame {
                        time: Duration::ZERO,
                        custom_animation_triggers: Some(
                            [(TypeId::of::<AnimSpriteAlpha>(), 0.0)]
                                .into_iter()
                                .collect(),
                        ),
                        ..Default::default()
                    },
                    KeyFrame {
                        time: Duration::from_secs_f32(2.5),
                        custom_animation_triggers: Some(
                            [(TypeId::of::<AnimSpriteAlpha>(), 0.0)]
                                .into_iter()
                                .collect(),
                        ),
                        ..Default::default()
                    },
                    KeyFrame {
                        time: Duration::from_secs_f32(3.0),
                        custom_animation_triggers: Some(
                            [(TypeId::of::<AnimSpriteAlpha>(), 1.0)]
                                .into_iter()
                                .collect(),
                        ),
                        ..Default::default()
                    },
                ],
            }]),
            ..Default::default()
        },
        SceneTag,
    ));
}

pub(crate) fn setup_pvz_logo(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("images/PvZ_Logo.png");

    let transform_start = Transform::from_xyz(0.0, 430.0, 1.0);
    let transform_end = Transform::from_xyz(0.0, 230.0, 1.0);

    let entity = commands
        .spawn((
            SpriteBundle {
                texture: image,
                transform: transform_start,
                ..Default::default()
            },
            SceneTag,
        ))
        .id();
    commands.spawn((
        AnimationBundle {
            animation_clips: AnimationClips(vec![AnimationClip {
                entity,
                keyframes: vec![
                    KeyFrame {
                        time: Duration::ZERO,
                        transform: Some(transform_start),
                        ..Default::default()
                    },
                    KeyFrame {
                        time: Duration::from_secs_f32(3.0),
                        transform: Some(transform_start),
                        ..Default::default()
                    },
                    KeyFrame {
                        time: Duration::from_secs_f32(4.0),
                        transform: Some(transform_end),
                        ..Default::default()
                    },
                ],
            }]),
            ..Default::default()
        },
        SceneTag,
    ));
}

pub(crate) fn setup_load_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
    let dirt = asset_server.load("images/LoadBar_dirt.png");
    let grass = asset_server.load("images/LoadBar_grass.png");
    let font = asset_server.load("font/fzcgbk.ttf");

    let transform_start = Transform::from_xyz(0.0, -350.0, 1.0);
    let transform_end = Transform::from_xyz(0.0, -250.0, 1.0);

    let entity = commands
        .spawn((
            ButtonBundle {
                hotspot: ButtonHotspot(vec![Rect {
                    min: Vec2 { x: -160., y: -26.0 },
                    max: Vec2 { x: 160., y: 26.0 },
                }]),
                background: ButtonBackground {
                    normal: dirt.clone(),
                    hover: dirt.clone(),
                    pressed: dirt.clone(),
                    disabled: dirt,
                },
                sprite: SpriteBundle {
                    transform: transform_start,
                    ..Default::default()
                },
                ..Default::default()
            },
            StartGameButtonTag,
            SceneTag,
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: grass,
                    transform: Transform::from_xyz(-6.0, 28.0, 1.0),
                    ..Default::default()
                },
                LoadingGrass,
            ));

            parent.spawn((
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "点击开始".to_owned(),
                            style: TextStyle {
                                font,
                                font_size: 18.0,
                                color: Color::srgb(0.8, 0.5, 0.2),
                            },
                        }],
                        ..Default::default()
                    },
                    text_anchor: Anchor::Center,
                    transform: Transform::from_xyz(-5.0, 5.0, 1.0),
                    ..Default::default()
                },
                LoadingText,
            ));
        })
        .id();
    commands.spawn((
        AnimationBundle {
            animation_clips: AnimationClips(vec![AnimationClip {
                entity,
                keyframes: vec![
                    KeyFrame {
                        time: Duration::ZERO,
                        transform: Some(transform_start),
                        ..Default::default()
                    },
                    KeyFrame {
                        time: Duration::from_secs_f32(3.5),
                        transform: Some(transform_start),
                        ..Default::default()
                    },
                    KeyFrame {
                        time: Duration::from_secs_f32(4.0),
                        transform: Some(transform_end),
                        ..Default::default()
                    },
                ],
            }]),
            ..Default::default()
        },
        SceneTag,
    ));
}

pub(crate) fn clear_scene(mut commands: Commands, targets: Query<Entity, With<SceneTag>>) {
    for entity in &targets {
        commands.entity(entity).despawn_recursive();
    }
}
