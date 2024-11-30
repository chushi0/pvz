use std::time::Duration;

use bevy::{prelude::*, render::camera::ScalingMode};
use fw_anim::{AnimationBundle, AnimationClip, AnimationClips, KeyFrame};
use fw_button::components::{ButtonBackground, ButtonBundle, ButtonEnabled, ButtonHotspot};
use fw_ftxm::{FtxmSource, MainMusicTable};
use mod_userdata::UserData;

use crate::tag::{
    AdventureButtonTag, ButtonTextTag, ExitButtonTag, HelpButtonTag, MinigameButtonTag,
    OptionButtonTag, PuzzleButtonTag, SceneTag, SurvialButtonTag,
};

pub(crate) fn start_bgm(mut commands: Commands, target: Query<Entity, With<FtxmSource>>) {
    // splash不会结束音乐，如果其已经存在，则复用
    for entity in &target {
        commands.entity(entity).insert(SceneTag);
    }

    // 如果没有，则新建
    if target.is_empty() {
        commands.spawn((
            FtxmSource {
                pot: MainMusicTable::Title.into(),
            },
            SceneTag,
        ));
    }
}

pub(crate) fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
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

pub(crate) fn spawn_bg(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_image(
        &mut commands,
        &asset_server,
        "reanim/SelectorScreen_BG.jpg",
        Vec2 { x: 0.0, y: 0.0 },
        Some(Vec2 { x: 800.0, y: 600.0 }),
        Vec2::default(),
        0.0,
        0.0,
    );
}

pub(crate) fn spawn_bg_center(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_image(
        &mut commands,
        &asset_server,
        "reanim/SelectorScreen_BG_Center.png",
        Vec2 {
            x: 0.0,
            y: -300.0 + 350.0 / 2.0,
        },
        None,
        Vec2 { x: 0.0, y: 50.0 },
        1.0,
        0.5,
    );
}

pub(crate) fn spawn_bg_left(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_image(
        &mut commands,
        &asset_server,
        "reanim/SelectorScreen_BG_Left.png",
        Vec2 {
            x: -400.0 + 476.0 / 2.0,
            y: -300.0 + 680.0 / 2.0,
        },
        None,
        Vec2 { x: 0.0, y: 80.0 },
        2.0,
        0.5,
    );
}

pub(crate) fn spawn_bg_right(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    userdata: Res<UserData>,
) {
    let entity = spawn_image(
        &mut commands,
        &asset_server,
        "reanim/SelectorScreen_BG_Right.png",
        Vec2 {
            x: 400.0 - 730.0 / 2.0,
            y: -300.0 + 560.0 / 2.0,
        },
        None,
        Vec2 { x: 0.0, y: 500.0 },
        3.0,
        0.5,
    );

    let font = asset_server.load("font/fzcgbk.ttf");

    commands.entity(entity).with_children(|parent| {
        // 冒险模式 or 开始冒险吧
        if userdata.pass_adventure_count == 0 && userdata.adventure_progress == 1 {
            // 开始冒险吧
            spawn_image_button(
                parent,
                asset_server.load("reanim/SelectorScreen_Shadow_StartAdventure.png"),
                asset_server.load("reanim/SelectorScreen_StartAdventure_Button1.png"),
                asset_server.load("reanim/SelectorScreen_StartAdventure_Highlight.png"),
                Vec3 {
                    x: 135.0,
                    y: 185.0,
                    z: 1.0,
                },
                true,
                ButtonHotspot::Polygon(vec![
                    Vec2 {
                        x: -165.5 + 16.0,
                        y: 73.0 - 26.0,
                    },
                    Vec2 {
                        x: -165.5 + 70.0,
                        y: 73.0 - 27.0,
                    },
                    Vec2 {
                        x: -165.5 + 100.0,
                        y: 73.0 - 5.0,
                    },
                    Vec2 {
                        x: -165.5 + 140.0,
                        y: 73.0 - 0.0,
                    },
                    Vec2 {
                        x: -165.5 + 208.0,
                        y: 73.0 - 3.0,
                    },
                    Vec2 {
                        x: -165.5 + 227.0,
                        y: 73.0 - 15.0,
                    },
                    Vec2 {
                        x: -165.5 + 248.0,
                        y: 73.0 - 42.0,
                    },
                    Vec2 {
                        x: -165.5 + 329.0,
                        y: 73.0 - 54.0,
                    },
                    Vec2 {
                        x: -165.5 + 315.0,
                        y: 73.0 - 137.0,
                    },
                    Vec2 {
                        x: -165.5 + 256.0,
                        y: 73.0 - 139.0,
                    },
                    Vec2 {
                        x: -165.5 + 151.0,
                        y: 73.0 - 118.0,
                    },
                    Vec2 {
                        x: -165.5 + 5.0,
                        y: 73.0 - 97.0,
                    },
                    Vec2 {
                        x: -165.5 + 4.0,
                        y: 73.0 - 37.0,
                    },
                ]),
                AdventureButtonTag,
            );
        } else {
            // 冒险模式
            spawn_image_button(
                parent,
                asset_server.load("reanim/SelectorScreen_Shadow_Adventure.png"),
                asset_server.load("reanim/SelectorScreen_Adventure_button.png"),
                asset_server.load("reanim/SelectorScreen_Adventure_highlight.png"),
                Vec3 {
                    x: 135.0,
                    y: 185.0,
                    z: 1.0,
                },
                true,
                ButtonHotspot::Polygon(vec![
                    Vec2 {
                        x: -165.0 + 16.0,
                        y: 60.0 - 3.0,
                    },
                    Vec2 {
                        x: -165.0 + 324.0,
                        y: 60.0 - 26.0,
                    },
                    Vec2 {
                        x: -165.0 + 313.0,
                        y: 60.0 - 116.0,
                    },
                    Vec2 {
                        x: -165.0 + 253.0,
                        y: 60.0 - 112.0,
                    },
                    Vec2 {
                        x: -165.0 + 204.0,
                        y: 60.0 - 102.0,
                    },
                    Vec2 {
                        x: -165.0 + 182.0,
                        y: 60.0 - 117.0,
                    },
                    Vec2 {
                        x: -165.0 + 103.0,
                        y: 60.0 - 104.0,
                    },
                    Vec2 {
                        x: -165.0 + 87.0,
                        y: 60.0 - 84.0,
                    },
                    Vec2 {
                        x: -165.0 + 1.0,
                        y: 60.0 - 72.0,
                    },
                    Vec2 {
                        x: -165.0 + 2.0,
                        y: 60.0 - 16.0,
                    },
                ]),
                AdventureButtonTag,
            );

            parent.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: (userdata.adventure_progress / 10 + 1).to_string(),
                        style: TextStyle {
                            font_size: 18.0,
                            ..Default::default()
                        },
                    }],
                    ..Default::default()
                },
                transform: Transform::from_xyz(125.0, 149.0, 1.1),
                ..Default::default()
            });

            parent.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: ((userdata.adventure_progress - 1) % 10 + 1).to_string(),
                        style: TextStyle {
                            font_size: 18.0,
                            ..Default::default()
                        },
                    }],
                    ..Default::default()
                },
                transform: Transform::from_xyz(150.0, 146.0, 1.1),
                ..Default::default()
            });
        }

        // 迷你游戏
        spawn_image_button(
            parent,
            asset_server.load("reanim/SelectorScreen_Shadow_Survival.png"),
            asset_server.load("reanim/SelectorScreen_Survival_button.png"),
            asset_server.load("reanim/SelectorScreen_Survival_highlight.png"),
            Vec3 {
                x: 125.0,
                y: 85.0,
                z: 1.0,
            },
            userdata.unlock_minigame,
            ButtonHotspot::Polygon(vec![
                Vec2 {
                    x: -156.0 + 4.0,
                    y: 66.5 - 6.0,
                },
                Vec2 {
                    x: -156.0 + 311.0,
                    y: 66.5 - 53.0,
                },
                Vec2 {
                    x: -156.0 + 293.0,
                    y: 66.5 - 127.0,
                },
                Vec2 {
                    x: -156.0 + 6.0,
                    y: 66.5 - 77.0,
                },
            ]),
            MinigameButtonTag,
        );

        // 解谜模式
        spawn_image_button(
            parent,
            asset_server.load("reanim/SelectorScreen_Shadow_Challenge.png"),
            asset_server.load("reanim/SelectorScreen_Challenges_button.png"),
            asset_server.load("reanim/SelectorScreen_Challenges_highlight.png"),
            Vec3 {
                x: 115.0,
                y: 5.0,
                z: 1.0,
            },
            userdata.unlock_puzzle,
            ButtonHotspot::Polygon(vec![
                Vec2 {
                    x: -143.0 + 4.0,
                    y: 61.0 - 2.0,
                },
                Vec2 {
                    x: -143.0 + 283.0,
                    y: 61.0 - 55.0,
                },
                Vec2 {
                    x: -143.0 + 268.0,
                    y: 61.0 - 115.0,
                },
                Vec2 {
                    x: -143.0 + 3.0,
                    y: 61.0 - 59.0,
                },
            ]),
            PuzzleButtonTag,
        );

        // 生存模式
        spawn_image_button(
            parent,
            asset_server.load("reanim/SelectorScreen_Shadow_ZenGarden.png"),
            asset_server.load("reanim/SelectorScreen_Vasebreaker_button.png"),
            asset_server.load("reanim/SelectorScreen_vasebreaker_highlight.png"),
            Vec3 {
                x: 105.0,
                y: -65.0,
                z: 1.0,
            },
            userdata.unlock_survial,
            ButtonHotspot::Polygon(vec![
                Vec2 {
                    x: -133.0 + 10.0,
                    y: 61.5 - 3.0,
                },
                Vec2 {
                    x: -133.0 + 262.0,
                    y: 61.5 - 60.0,
                },
                Vec2 {
                    x: -133.0 + 247.0,
                    y: 61.5 - 119.0,
                },
                Vec2 {
                    x: -133.0 + 7.0,
                    y: 61.5 - 57.0,
                },
            ]),
            SurvialButtonTag,
        );

        // 选项
        parent
            .spawn((
                ButtonBundle {
                    sprite: SpriteBundle {
                        transform: Transform::from_xyz(
                            534.0 - 730.0 / 2.0,
                            -464.0 + 560.0 / 2.0,
                            10.0,
                        ),
                        ..Default::default()
                    },
                    hotspot: ButtonHotspot::Rects(vec![Rect {
                        min: Vec2 { x: -30.0, y: -18.0 },
                        max: Vec2 { x: 30.0, y: 18.0 },
                    }]),
                    ..Default::default()
                },
                OptionButtonTag,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text2dBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "选项".to_owned(),
                                style: TextStyle {
                                    font: font.clone(),
                                    ..Default::default()
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ButtonTextTag,
                ));
            });

        // 帮助
        parent
            .spawn((
                ButtonBundle {
                    sprite: SpriteBundle {
                        transform: Transform::from_xyz(
                            602.0 - 730.0 / 2.0,
                            -491.0 + 560.0 / 2.0,
                            10.0,
                        ),
                        ..Default::default()
                    },
                    hotspot: ButtonHotspot::Rects(vec![Rect {
                        min: Vec2 { x: -30.0, y: -18.0 },
                        max: Vec2 { x: 30.0, y: 18.0 },
                    }]),
                    ..Default::default()
                },
                HelpButtonTag,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text2dBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "帮助".to_owned(),
                                style: TextStyle {
                                    font: font.clone(),
                                    ..Default::default()
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ButtonTextTag,
                ));
            });

        // 退出
        parent
            .spawn((
                ButtonBundle {
                    sprite: SpriteBundle {
                        transform: Transform::from_xyz(
                            674.0 - 730.0 / 2.0,
                            -476.0 + 560.0 / 2.0,
                            10.0,
                        ),
                        ..Default::default()
                    },
                    hotspot: ButtonHotspot::Rects(vec![Rect {
                        min: Vec2 { x: -30.0, y: -18.0 },
                        max: Vec2 { x: 30.0, y: 18.0 },
                    }]),
                    ..Default::default()
                },
                ExitButtonTag,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text2dBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "退出".to_owned(),
                                style: TextStyle {
                                    font: font.clone(),
                                    ..Default::default()
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ButtonTextTag,
                ));
            });
    });
}

#[allow(clippy::too_many_arguments)]
fn spawn_image(
    commands: &mut Commands,
    asset_server: &AssetServer,
    path: &'static str,
    pos: Vec2,
    custom_size: Option<Vec2>,
    move_to: Vec2,
    z: f32,
    move_time: f32,
) -> Entity {
    let image = asset_server.load(path);

    let entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size,
                    ..Default::default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, z),
                texture: image,
                ..Default::default()
            },
            SceneTag,
        ))
        .id();

    if move_time > 0.0 {
        commands.spawn((
            AnimationBundle {
                animation_clips: AnimationClips(vec![AnimationClip {
                    entity,
                    keyframes: vec![
                        KeyFrame {
                            time: Duration::ZERO,
                            transform: Some(Transform::from_xyz(
                                pos.x - move_to.x,
                                pos.y - move_to.y,
                                z,
                            )),
                            ..Default::default()
                        },
                        KeyFrame {
                            time: Duration::from_secs_f32(move_time),
                            transform: Some(Transform::from_xyz(pos.x, pos.y, z)),
                            ..Default::default()
                        },
                    ],
                }]),
                ..Default::default()
            },
            SceneTag,
        ));
    }

    entity
}

#[allow(clippy::too_many_arguments)]
fn spawn_image_button<B: Bundle>(
    parent: &mut ChildBuilder<'_>,
    shadow: Handle<Image>,
    normal_bg: Handle<Image>,
    press_bg: Handle<Image>,
    translation: Vec3,
    enabled: bool,
    hotspot: ButtonHotspot,
    extra_bundle: B,
) -> Entity {
    parent.spawn(SpriteBundle {
        texture: shadow,
        transform: Transform::from_translation(
            translation
                + Vec3 {
                    x: 5.0,
                    y: -5.0,
                    z: -0.1,
                },
        ),
        ..Default::default()
    });

    parent
        .spawn((
            ButtonBundle {
                hotspot,
                enabled: match enabled {
                    true => ButtonEnabled::Enabled,
                    false => ButtonEnabled::Disabled,
                },
                background: ButtonBackground {
                    normal: normal_bg.clone(),
                    hover: press_bg.clone(),
                    pressed: press_bg.clone(),
                    disabled: normal_bg.clone(),
                },
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: match enabled {
                            true => Color::WHITE,
                            false => Color::srgb(0.5, 0.5, 0.5),
                        },
                        ..Default::default()
                    },
                    transform: Transform::from_translation(translation),
                    ..Default::default()
                },
                ..Default::default()
            },
            extra_bundle,
        ))
        .id()
}

pub(crate) fn clear_scene(mut commands: Commands, target: Query<Entity, With<SceneTag>>) {
    for entity in &target {
        commands.entity(entity).despawn_recursive();
    }
}
