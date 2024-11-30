use std::{any::TypeId, time::Duration};

use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    sprite::{Anchor, MaterialMesh2dBundle},
    text::Text2dBounds,
};
use fw_anim::{AnimationBundle, AnimationClip, AnimationClips, CustomAnimationTrigger, KeyFrame};
use fw_button::components::{ButtonBackground, ButtonBundle, ButtonHotspot};
use fw_ftxm::{FtxmSource, MainMusicTable};
use mod_level::{CurrentLevel, Reward};
use mod_plant::{components::PlantSeedBundle, metadata::PlantRegistry};

use crate::tag::{BackToTitleButtonTag, ConformButtonTag, MaterialColorAnim, SceneTag};

pub(crate) fn setup_bgm(mut commands: Commands) {
    commands.spawn((
        FtxmSource {
            pot: MainMusicTable::ZenGarden.into(),
        },
        SceneTag,
    ));
}

pub(crate) fn setup_camera(mut commands: Commands) {
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

pub(crate) fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("images/AwardScreen_Back.jpg");
    commands.spawn((
        SpriteBundle {
            texture: image,
            ..Default::default()
        },
        SceneTag,
    ));
}

pub(crate) fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("font/fzcgbk.ttf");
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "你得到一株新植物！".to_owned(),
                    style: TextStyle {
                        font,
                        color: Color::srgb(0.8, 0.5, 0.2),
                        ..Default::default()
                    },
                }],
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 250., 1.0),
            ..Default::default()
        },
        SceneTag,
    ));
}

pub(crate) fn setup_fadein(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                material: materials.add(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                transform: Transform::from_xyz(0.0, 0.0, 70.0).with_scale(Vec3::ONE * 1000.),
                ..Default::default()
            },
            SceneTag,
            CustomAnimationTrigger::default(),
            MaterialColorAnim,
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
                            [(TypeId::of::<MaterialColorAnim>(), 1.0)]
                                .into_iter()
                                .collect(),
                        ),
                        ..Default::default()
                    },
                    KeyFrame {
                        time: Duration::from_secs_f32(2.0),
                        custom_animation_triggers: Some(
                            [(TypeId::of::<MaterialColorAnim>(), 0.0)]
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

pub(crate) fn setup_plant_info(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    plant_registry: Res<PlantRegistry>,
    asset_server: Res<AssetServer>,
) {
    let Some(Reward::Plant { plant }) = current_level.reward else {
        return;
    };

    let Some(plant_info) = plant_registry.get(&plant) else {
        return;
    };

    let mut plant_bundle = PlantSeedBundle::new(plant_info.clone());
    plant_bundle.transform = Transform::from_xyz(0.0, 100.0, 1.0).with_scale(Vec3::ONE * 2.);
    commands.spawn((plant_bundle, SceneTag));

    let font = asset_server.load("font/fzcgbk.ttf");

    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: plant_info.name.clone(),
                    style: TextStyle {
                        font: font.clone(),
                        color: Color::srgb(0.8, 0.5, 0.2),
                        ..Default::default()
                    },
                }],
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -20., 1.0),
            ..Default::default()
        },
        SceneTag,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: plant_info.description.clone(),
                    style: TextStyle {
                        font,
                        color: Color::srgb(0.2, 0.5, 0.8),
                        ..Default::default()
                    },
                }],
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -105., 1.0),
            text_2d_bounds: Text2dBounds {
                size: Vec2 { x: 250., y: 90. },
            },
            ..Default::default()
        },
        SceneTag,
    ));
}

pub(crate) fn setup_conform_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("font/fzcgbk.ttf");

    commands
        .spawn((
            ButtonBundle {
                background: ButtonBackground {
                    normal: asset_server.load("images/SeedChooser_Button.png"),
                    hover: asset_server.load("images/SeedChooser_Button_Glow.png"),
                    pressed: asset_server.load("images/SeedChooser_Button_Glow.png"),
                    disabled: asset_server.load("images/SeedChooser_Button_Disabled.png"),
                },
                sprite: SpriteBundle {
                    sprite: Sprite {
                        anchor: Anchor::Center,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, -230., 10.0),
                    ..Default::default()
                },
                hotspot: ButtonHotspot::Rects(vec![Rect {
                    min: Vec2 { x: -78.0, y: -21.0 },
                    max: Vec2 { x: 78.0, y: 21.0 },
                }]),
                ..Default::default()
            },
            ConformButtonTag,
            SceneTag,
        ))
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "下一关".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            color: Color::srgb(0.83, 0.67, 0.07),
                            ..Default::default()
                        },
                    }],
                    ..Default::default()
                },
                text_anchor: Anchor::Center,
                transform: Transform::from_xyz(0.0, 0.0, 0.01),
                ..Default::default()
            });
        });
}

pub(crate) fn setup_back_to_title_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("font/fzcgbk.ttf");

    commands
        .spawn((
            ButtonBundle {
                background: ButtonBackground {
                    normal: asset_server.load("images/SeedChooser_Button2.png"),
                    hover: asset_server.load("images/SeedChooser_Button2_Glow.png"),
                    pressed: asset_server.load("images/SeedChooser_Button2_Glow.png"),
                    disabled: asset_server.load("images/SeedChooser_Button2.png"),
                },
                sprite: SpriteBundle {
                    sprite: Sprite {
                        anchor: Anchor::Center,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(330.0, 270., 10.0),
                    ..Default::default()
                },
                hotspot: ButtonHotspot::Rects(vec![Rect {
                    min: Vec2 { x: -56.0, y: -13.0 },
                    max: Vec2 { x: 56.0, y: 13.0 },
                }]),
                ..Default::default()
            },
            BackToTitleButtonTag,
            SceneTag,
        ))
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "主菜单".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            color: Color::BLACK,
                            font_size: 14.0,
                        },
                    }],
                    ..Default::default()
                },
                text_anchor: Anchor::Center,
                transform: Transform::from_xyz(0.0, 0.0, 0.01),
                ..Default::default()
            });
        });
}

pub(crate) fn clear_scene(mut commands: Commands, targets: Query<Entity, With<SceneTag>>) {
    for entity in &targets {
        commands.entity(entity).despawn_recursive();
    }
}
