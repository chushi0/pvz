use std::{any::TypeId, time::Duration};

use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    sprite::{Anchor, MaterialMesh2dBundle},
    text::Text2dBounds,
};
use fw_actor::components::AnimStandbyTag;
use fw_anim::{AnimationBundle, AnimationClip, AnimationClips, CustomAnimationTrigger, KeyFrame};
use fw_button::components::{ButtonBackground, ButtonBundle, ButtonHotspot};
use fw_ftxm::{FtxmSource, MainMusicTable};
use mod_level::{CurrentLevel, LevelBackground, SodType, Zombie};
use mod_plant::{
    components::{PlantSeed, PlantSeedBundle, PlantUsable},
    metadata::{PlantRegistry, PlantType},
};
use mod_userdata::UserData;
use mod_zombie::{
    components::{AnimZombieEatTag, AnimZombieMoveTag, ZombieBundle},
    metadata::ZombieRegistry,
};
use rand::{thread_rng, Rng};

use crate::{
    resource::{Sunshine, ZombieWaveController},
    tag::{
        ChooseableSeedTag, CleanerCar, FollowCameraTag, GameTimer, GameTimerTag, GameUiTag,
        LanePosition, MaterialColorAnim, NaturalSunshineSolt, PickableSeed, PlantSolt, RewardSolt,
        RewardTag, SceneTag, SeedChooserTag, SeedTransformInChooserBox, SeedbankTag,
        ShowLevelProgressShiftLeft, SoltType, StandbyZombieTag, StartGameButtonTag, SunshineText,
        ZombieSolt, ZombieTag,
    },
    GameState,
};

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
            transform: Transform::from_xyz(-220., 0., 0.),
            ..Default::default()
        },
        SceneTag,
    ));
}

pub(crate) fn setup_background(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    asset_server: Res<AssetServer>,
) {
    // 背景图
    let image = asset_server.load(match current_level.background {
        LevelBackground::Day {
            sod_type: SodType::SodRow5,
            ..
        } => "images/background1.jpg",
        LevelBackground::Day { .. } => "images/background1unsodded.jpg",
        LevelBackground::Night => "images/background2.jpg",
        LevelBackground::Swim => "images/background3.jpg",
        LevelBackground::SwimFog => "images/background4.jpg",
        LevelBackground::Roof => "images/background5.jpg",
        LevelBackground::RoofNight => "images/background6.jpg",
    });
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::CenterLeft,
                ..Default::default()
            },
            texture: image,
            transform: Transform::from_xyz(-620., 0., 0.),
            ..Default::default()
        },
        SceneTag,
    ));

    // 白天无草皮之地
    if let LevelBackground::Day { sod_type, .. } = &current_level.background {
        if let Some(image) = match sod_type {
            SodType::SodRow1 => Some("images/sod1row.png"),
            SodType::SodRow3 => Some("images/sod3row.png"),
            SodType::None | SodType::SodRow5 => None,
        } {
            let image = asset_server.load(image);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        anchor: Anchor::CenterLeft,
                        ..Default::default()
                    },
                    texture: image,
                    transform: Transform::from_xyz(-620. + 240., -25., 0.01),
                    ..Default::default()
                },
                SceneTag,
            ));
        }
    }

    // TODO: 如果是泳池，需要处理河道
}

pub(crate) fn setup_plant_solt(mut commands: Commands, current_level: Res<CurrentLevel>) {
    match &current_level.background {
        LevelBackground::Day {
            sod_type,
            upgrade_sod_type,
        } => {
            let lane = match (sod_type, upgrade_sod_type) {
                (SodType::None, false) => vec![],
                (SodType::SodRow1, false) | (SodType::None, true) => vec![2],
                (SodType::SodRow3, false) | (SodType::SodRow1, true) => vec![1, 2, 3],
                (SodType::SodRow5, _) | (SodType::SodRow3, true) => vec![0, 1, 2, 3, 4],
            };
            lane.into_iter().for_each(|lane| {
                for i in 0..9 {
                    commands.spawn((
                        PlantSolt::default(),
                        Transform::from_xyz(
                            -320.0 + i as f32 * 80.0,
                            lane as f32 * 100.0 - 110.0 * 2.0,
                            10.0,
                        ),
                        GlobalTransform::default(),
                        SceneTag,
                        LanePosition {
                            lane,
                            ..Default::default()
                        },
                    ));
                }
            });
        }
        LevelBackground::Night => {
            for lane in 0..5 {
                for i in 0..9 {
                    commands.spawn((
                        PlantSolt::default(),
                        Transform::from_xyz(
                            -320.0 + i as f32 * 80.0,
                            lane as f32 * 100.0 - 110.0 * 2.0,
                            10.0,
                        ),
                        GlobalTransform::default(),
                        SceneTag,
                        LanePosition {
                            lane,
                            ..Default::default()
                        },
                    ));
                }
            }
        }
        LevelBackground::Swim | LevelBackground::SwimFog => {
            for (lane, solt_type) in [
                SoltType::Dirt,
                SoltType::Dirt,
                SoltType::River,
                SoltType::River,
                SoltType::Dirt,
                SoltType::Dirt,
            ]
            .into_iter()
            .enumerate()
            {
                for i in 0..9 {
                    commands.spawn((
                        PlantSolt {
                            solt_type,
                            ..Default::default()
                        },
                        // TODO: 调整坐标
                        Transform::from_xyz(
                            -320.0 + i as f32 * 80.0,
                            lane as f32 * 100.0 - 110.0 * 2.0,
                            10.0,
                        ),
                        GlobalTransform::default(),
                        SceneTag,
                        LanePosition {
                            lane: lane as u8,
                            ..Default::default()
                        },
                    ));
                }
            }
        }
        LevelBackground::Roof | LevelBackground::RoofNight => {
            for lane in 0..5 {
                for i in 0..9 {
                    commands.spawn((
                        PlantSolt {
                            solt_type: SoltType::Roof,
                            ..Default::default()
                        },
                        // TODO: 调整坐标
                        Transform::from_xyz(
                            -320.0 + i as f32 * 80.0,
                            lane as f32 * 100.0 - 110.0 * 2.0,
                            10.0,
                        ),
                        GlobalTransform::default(),
                        SceneTag,
                        LanePosition {
                            lane,
                            ..Default::default()
                        },
                    ));
                }
            }
        }
    }
}

pub(crate) fn setup_zombie_solt(mut commands: Commands, current_level: Res<CurrentLevel>) {
    match &current_level.background {
        LevelBackground::Day {
            sod_type,
            upgrade_sod_type,
        } => {
            let lane = match (sod_type, upgrade_sod_type) {
                (SodType::None, false) => vec![],
                (SodType::SodRow1, false) | (SodType::None, true) => vec![2],
                (SodType::SodRow3, false) | (SodType::SodRow1, true) => vec![1, 2, 3],
                (SodType::SodRow5, _) | (SodType::SodRow3, true) => vec![0, 1, 2, 3, 4],
            };
            lane.into_iter().for_each(|lane| {
                commands.spawn((
                    ZombieSolt {
                        solt_type: SoltType::Dirt,
                    },
                    Transform::from_xyz(500., lane as f32 * 100.0 - 110.0 * 2.0 - 15.0, 10.0),
                    GlobalTransform::default(),
                    SceneTag,
                    LanePosition {
                        lane,
                        ..Default::default()
                    },
                ));
            });
        }
        LevelBackground::Night => {
            for lane in 0..5 {
                commands.spawn((
                    ZombieSolt {
                        solt_type: SoltType::Dirt,
                    },
                    Transform::from_xyz(500., lane as f32 * 100.0 - 110.0 * 2.0 - 15.0, 10.0),
                    GlobalTransform::default(),
                    SceneTag,
                    LanePosition {
                        lane,
                        ..Default::default()
                    },
                ));
            }
        }
        LevelBackground::Swim | LevelBackground::SwimFog => {
            for (lane, solt_type) in [
                SoltType::Dirt,
                SoltType::Dirt,
                SoltType::River,
                SoltType::River,
                SoltType::Dirt,
                SoltType::Dirt,
            ]
            .into_iter()
            .enumerate()
            {
                commands.spawn((
                    ZombieSolt { solt_type },
                    // TODO: 调整坐标
                    Transform::from_xyz(500., lane as f32 * 100.0 - 110.0 * 2.0 - 15.0, 10.0),
                    GlobalTransform::default(),
                    SceneTag,
                    LanePosition {
                        lane: lane as u8,
                        ..Default::default()
                    },
                ));
            }
        }
        LevelBackground::Roof | LevelBackground::RoofNight => {
            for lane in 0..5 {
                commands.spawn((
                    ZombieSolt {
                        solt_type: SoltType::Roof,
                    },
                    // TODO: 调整坐标
                    Transform::from_xyz(500., lane as f32 * 100.0 - 110.0 * 2.0 - 15.0, 10.0),
                    GlobalTransform::default(),
                    SceneTag,
                    LanePosition {
                        lane,
                        ..Default::default()
                    },
                ));
            }
        }
    }
}

pub(crate) fn setup_seedbank(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    userdata: Res<UserData>,
) {
    let seedbank = asset_server.load("images/seedbank.png");

    commands
        .spawn((
            FollowCameraTag,
            Transform::default(),
            GlobalTransform::default(),
            ViewVisibility::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            SceneTag,
            GameUiTag,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            anchor: Anchor::TopLeft,
                            custom_size: Some(Vec2 {
                                x: 80.0 + 12.0 + 5.0 + 55.0 * userdata.plant_solt_count as f32,
                                y: 87.0,
                            }),
                            ..Default::default()
                        },
                        texture: seedbank,
                        visibility: Visibility::Hidden,
                        transform: Transform::from_xyz(-400.0, 300.0, 1.0),
                        ..Default::default()
                    },
                    ImageScaleMode::Sliced(TextureSlicer {
                        border: BorderRect {
                            left: 80.0,
                            right: 12.0,
                            top: 8.0,
                            bottom: 8.0,
                        },
                        center_scale_mode: SliceScaleMode::Stretch,
                        sides_scale_mode: SliceScaleMode::Stretch,
                        max_corner_scale: 1.0,
                    }),
                    SeedbankTag,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text2dBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "50".to_string(),
                                    style: TextStyle {
                                        color: Color::BLACK,
                                        ..Default::default()
                                    },
                                }],
                                ..Default::default()
                            },
                            text_anchor: Anchor::Center,
                            text_2d_bounds: Text2dBounds {
                                size: Vec2 { x: 56.0, y: 22.0 },
                            },
                            transform: Transform::from_xyz(39.0, -73.0, 0.01),
                            ..Default::default()
                        },
                        SunshineText,
                    ));
                });
        });
}

pub(crate) fn setup_cleanup_car(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
) {
    let car = asset_server.load("reanim-merge/LawnMower.png");
    let car_lane = match &current_level.background {
        LevelBackground::Day {
            sod_type,
            upgrade_sod_type,
        } => match (sod_type, upgrade_sod_type) {
            (SodType::None, false) => vec![],
            (SodType::None, true) | (SodType::SodRow1, false) => vec![2],
            (SodType::SodRow1, true) | (SodType::SodRow3, false) => vec![1, 2, 3],
            (SodType::SodRow3, true) | (SodType::SodRow5, _) => vec![0, 1, 2, 3, 4],
        },
        LevelBackground::Night => vec![0, 1, 2, 3, 4],
        LevelBackground::Swim | LevelBackground::SwimFog => vec![0, 1, 2, 3, 4, 5],
        LevelBackground::Roof | LevelBackground::RoofNight => vec![0, 1, 2, 3, 4],
    };

    for lane in car_lane {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                visibility: Visibility::Hidden,
                texture: car.clone(),
                ..Default::default()
            },
            SceneTag,
            CleanerCar { lane },
            LanePosition { lane, x: 0. },
        ));
    }
}

pub(crate) fn setup_resources(
    current_level: Res<CurrentLevel>,
    mut sunshine: ResMut<Sunshine>,
    mut zombie_wave_controller: ResMut<ZombieWaveController>,
) {
    sunshine.0 = current_level.sunshine;
    zombie_wave_controller.next_wave_timer = Timer::new(
        Duration::from_secs_f32(current_level.first_wave_time),
        TimerMode::Once,
    );
    zombie_wave_controller.next_wave_index = 0;
    zombie_wave_controller.trigger_huge_wave = false;
}

pub(crate) fn setup_init_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Init);
}

pub(crate) fn setup_init_bgm(mut commands: Commands) {
    commands.spawn((
        FtxmSource {
            pot: MainMusicTable::ChooseYourSeeds.into(),
        },
        SceneTag,
    ));
}

pub(crate) fn setup_sunshine_solt(mut commands: Commands, current_level: Res<CurrentLevel>) {
    if current_level.nature_sunshine {
        commands.spawn((
            NaturalSunshineSolt {
                next_sunshine_timer: Timer::new(Duration::from_secs_f32(7.0), TimerMode::Once),
            },
            SceneTag,
        ));
    }
}

pub(crate) fn setup_enter_timer(mut commands: Commands, current_level: Res<CurrentLevel>) {
    let mut time = 0.0;
    // 摄像机移动到中间，进入战场
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::CameraToCenterAnim,
        SceneTag,
    ));
    time += 2.0;

    // 无草坪之地，铺草地
    if let LevelBackground::Day {
        sod_type,
        upgrade_sod_type,
    } = &current_level.background
    {
        if match (sod_type, upgrade_sod_type) {
            (_, false) => false,
            (SodType::SodRow5, true) => false,
            (SodType::None | SodType::SodRow1 | SodType::SodRow3, true) => true,
        } {
            commands.spawn((
                GameTimer(Timer::from_seconds(time, TimerMode::Once)),
                GameTimerTag::UpgradeSod,
                SceneTag,
            ));
            time += 2.0;
        }
    }

    // 植物包、车就绪
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::FadeInSeedBank,
        SceneTag,
    ));
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::FadeInCars,
        SceneTag,
    ));
    time += 2.0;

    // 准备安放植物
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::StopBgm,
        SceneTag,
    ));
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::SoundReadySetPlant,
        SceneTag,
    ));
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::TextReady,
        SceneTag,
    ));
    time += 0.5;
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::TextSet,
        SceneTag,
    ));
    time += 0.5;
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::TextPlants,
        SceneTag,
    ));
    time += 1.0;

    // 游戏开始
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::EnterState(GameState::Main),
        SceneTag,
    ));
}

// 设定游戏启动时间轴
pub(crate) fn setup_init_timer(
    mut commands: Commands,
    userdata: Res<UserData>,
    plant_registry: Res<PlantRegistry>,
) {
    let mut time = 3.0;

    // 摄像机移动到右侧，查看僵尸
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::CameraToRightAnim,
        SceneTag,
    ));
    time += 4.0;

    // 检查是否需要选卡
    // TODO: 目前仅判断植物槽数量
    if userdata.unlock_plugins.len() <= userdata.plant_solt_count {
        // 植物数量小于植物槽，无需选卡
        // 需要在seedbank位置生成对应植物
        let mut unlock_plugins = userdata.unlock_plugins.iter().collect::<Vec<_>>();
        unlock_plugins.sort();
        for (i, plant) in unlock_plugins.into_iter().enumerate() {
            commands
                .spawn((
                    FollowCameraTag,
                    Transform::default(),
                    GlobalTransform::default(),
                    ViewVisibility::default(),
                    Visibility::default(),
                    InheritedVisibility::default(),
                    SceneTag,
                    GameUiTag,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        PlantSeedBundle {
                            transform: Transform::from_translation(Vec3 {
                                x: 85.0 + 55.0 * i as f32 + 55.0 * 0.5 - 400.0,
                                y: -8.0 - 35.0 + 300.0,
                                z: 1.1,
                            }),
                            visibility: Visibility::Hidden,
                            ..PlantSeedBundle::new(plant_registry.get(plant).unwrap().clone())
                        },
                        SeedbankTag,
                    ));
                });
        }

        // 进Enter状态
        commands.spawn((
            GameTimer(Timer::from_seconds(time, TimerMode::Once)),
            GameTimerTag::EnterState(GameState::Enter),
            SceneTag,
        ));
        return;
    }

    // 需要选卡，进选卡状态
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::EnterState(GameState::ChooseSeed),
        SceneTag,
    ));
}

pub(crate) fn insert_seed_pickable_tag(
    mut commands: Commands,
    seeds: Query<Entity, (With<PlantSeed>, With<SeedbankTag>)>,
) {
    for entity in &seeds {
        commands.entity(entity).insert(PickableSeed);
    }
}

pub(crate) fn setup_seed_chooser(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    userdata: Res<UserData>,
    plant_registry: Res<PlantRegistry>,
) {
    let seed_chooser = asset_server.load("images/SeedChooser_Background.png");
    let font = asset_server.load("font/fzcgbk.ttf");

    // 底图
    commands
        .spawn((
            FollowCameraTag,
            Transform::default(),
            GlobalTransform::default(),
            ViewVisibility::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            SceneTag,
            GameUiTag,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            anchor: Anchor::BottomLeft,
                            ..Default::default()
                        },
                        texture: seed_chooser,
                        visibility: Visibility::Hidden,
                        transform: Transform::from_xyz(-400., -300., 1.0),
                        ..Default::default()
                    },
                    SeedChooserTag,
                ))
                .with_children(|parent| {
                    // 标题
                    parent.spawn(Text2dBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "选择你的植物".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    color: Color::srgb(0.83, 0.67, 0.07),
                                    ..Default::default()
                                },
                            }],
                            ..Default::default()
                        },
                        text_anchor: Anchor::Center,
                        transform: Transform::from_xyz(232.0, 513.0 - 14.0, 0.01),
                        ..Default::default()
                    });

                    // 按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                background: ButtonBackground {
                                    normal: asset_server.load("images/SeedChooser_Button.png"),
                                    hover: asset_server.load("images/SeedChooser_Button_Glow.png"),
                                    pressed: asset_server
                                        .load("images/SeedChooser_Button_Glow.png"),
                                    disabled: asset_server
                                        .load("images/SeedChooser_Button_Disabled.png"),
                                },
                                sprite: SpriteBundle {
                                    sprite: Sprite {
                                        anchor: Anchor::Center,
                                        ..Default::default()
                                    },
                                    transform: Transform::from_xyz(232.0, 513.0 - 475.0, 0.01),
                                    ..Default::default()
                                },
                                hotspot: ButtonHotspot::Rects(vec![Rect {
                                    min: Vec2 { x: -78.0, y: -21.0 },
                                    max: Vec2 { x: 78.0, y: 21.0 },
                                }]),
                                ..Default::default()
                            },
                            StartGameButtonTag,
                        ))
                        .with_children(|parent| {
                            parent.spawn(Text2dBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "一起摇滚吧！".to_string(),
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
                });
        });

    // 所有已解锁植物，供选择
    for plants in &userdata.unlock_plugins {
        // 转为数字，以便计算坐标
        let i = *plants as i32;
        let x = i % 8;
        let y = i / 8;

        // 位置
        let seed_transform = Transform::from_translation(Vec3 {
            x: 25.0 + (50.0 * 0.9 + 5.0) * x as f32 + 55.0 * 0.5 - 400.0,
            y: -25.0 - y as f32 * (70. * 0.9 + 5.) + 165.0,
            z: 1.5,
        })
        .with_scale(Vec3::ONE * 0.95);

        // 可点击的种子包
        commands
            .spawn((
                FollowCameraTag,
                Transform::default(),
                GlobalTransform::default(),
                ViewVisibility::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                GameUiTag,
                SceneTag,
            ))
            .with_children(|parent| {
                parent.spawn((
                    PlantSeedBundle {
                        transform: seed_transform,
                        visibility: Visibility::Hidden,
                        ..PlantSeedBundle::new(
                            plant_registry.get(&PlantType::PeaShooter).unwrap().clone(),
                        )
                    },
                    SeedChooserTag,
                    ChooseableSeedTag,
                    SeedTransformInChooserBox(seed_transform),
                ));
            });

        // 底下再生成一个不可选择的暗色种子包，用于在选择后依然可以知道这里原本是什么植物
        commands
            .spawn((
                FollowCameraTag,
                Transform::default(),
                GlobalTransform::default(),
                ViewVisibility::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                GameUiTag,
                SceneTag,
            ))
            .with_children(|parent| {
                parent.spawn((
                    PlantSeedBundle {
                        transform: seed_transform.with_translation(Vec3 {
                            x: seed_transform.translation.x,
                            y: seed_transform.translation.y,
                            z: seed_transform.translation.z - 0.1,
                        }),
                        visibility: Visibility::Hidden,
                        usable: PlantUsable::Unusable,
                        ..PlantSeedBundle::new(
                            plant_registry.get(&PlantType::PeaShooter).unwrap().clone(),
                        )
                    },
                    SeedChooserTag,
                ));
            });
    }
}

pub(crate) fn setup_choose_seed_timer(mut commands: Commands) {
    commands.spawn((
        GameTimer(Timer::from_seconds(0.0, TimerMode::Once)),
        GameTimerTag::FadeInSeedBank,
        SceneTag,
    ));
    commands.spawn((
        GameTimer(Timer::from_seconds(0.0, TimerMode::Once)),
        GameTimerTag::FadeInSeedChooser,
        SceneTag,
    ));
}

pub(crate) fn setup_game_bgm(mut commands: Commands) {
    commands.spawn((
        FtxmSource {
            pot: MainMusicTable::Grasswalk.into(),
        },
        SceneTag,
    ));
}

pub(crate) fn setup_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
) {
    let font: Handle<Font> = asset_server.load("font/fzcgbk.ttf");
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: current_level.name.clone(),
                    style: TextStyle {
                        font: font.clone(),
                        color: Color::srgb(0.8, 0.5, 0.2),
                        font_size: 24.0,
                    },
                }],
                ..Default::default()
            },
            text_anchor: Anchor::BottomRight,
            transform: Transform::from_xyz(390.0, -300.0, 50.1),
            ..Default::default()
        },
        SceneTag,
        ShowLevelProgressShiftLeft,
        GameUiTag,
    ));

    for (x, y) in [(1., 0.), (0., 1.), (-1., 0.), (0., -1.)] {
        commands.spawn((
            Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: current_level.name.clone(),
                        style: TextStyle {
                            font: font.clone(),
                            color: Color::BLACK,
                            font_size: 24.0,
                        },
                    }],
                    ..Default::default()
                },
                text_anchor: Anchor::BottomRight,
                transform: Transform::from_xyz(390.0 + x, -300.0 + y, 50.0),
                ..Default::default()
            },
            SceneTag,
            ShowLevelProgressShiftLeft,
            GameUiTag,
        ));
    }
}

pub(crate) fn setup_standby_zombie(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    zombie_registry: Res<ZombieRegistry>,
) {
    let mut rng = thread_rng();
    for Zombie { zombie, count } in &current_level.preview_zombies {
        let zombie = zombie_registry.get(zombie).unwrap();

        for _ in 0..*count {
            let mut zombie_bundle = ZombieBundle::new(zombie.clone());
            let y = rng.gen_range(-300.0..150.0);
            let x = rng.gen_range(0.0..175.0) - y * 0.1;
            zombie_bundle.spine.transform = Transform::from_xyz(x + 500.0, y, 1.0 - y * 0.001);
            commands.spawn((zombie_bundle, StandbyZombieTag, AnimStandbyTag, SceneTag));
        }
    }
}

pub(crate) fn clear_scene(mut commands: Commands, entities: Query<Entity, With<SceneTag>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

pub(crate) fn clear_standby_zombie(
    mut commands: Commands,
    zombies: Query<Entity, (With<StandbyZombieTag>, Without<ZombieTag>)>,
) {
    for entity in &zombies {
        commands.entity(entity).despawn_recursive();
    }
}

pub(crate) fn clean_game_ui(mut commands: Commands, targets: Query<Entity, With<GameUiTag>>) {
    for entity in &targets {
        commands.entity(entity).despawn_recursive();
    }
}

pub(crate) fn setup_gameover_timer(mut commands: Commands) {
    let mut time = 0.0;

    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::SoundLose,
        SceneTag,
    ));
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::StopBgm,
        SceneTag,
    ));
    time += 1.0;

    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::CameraToLeftAnim,
        SceneTag,
    ));
    time += 2.0;

    for _ in 0..4 {
        commands.spawn((
            GameTimer(Timer::from_seconds(time, TimerMode::Once)),
            GameTimerTag::SoundEat,
            SceneTag,
        ));
        time += 0.5;
    }

    time += 1.0;
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::SoundScream,
        SceneTag,
    ));
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::TextZombieEatYourBrain,
        SceneTag,
    ));

    // TODO: 游戏结束对话框，重新开始或返回
    time += 5.5;
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::Reset,
        SceneTag,
    ));
}

pub(crate) fn stop_standby_anims(
    mut commands: Commands,
    targets: Query<Entity, With<AnimStandbyTag>>,
) {
    for entity in &targets {
        commands.entity(entity).remove::<AnimStandbyTag>();
    }
}

pub(crate) fn stop_zombie_anims(mut commands: Commands, zombies: Query<Entity, With<ZombieTag>>) {
    for entity in &zombies {
        commands
            .entity(entity)
            .remove::<(AnimZombieMoveTag, AnimZombieEatTag)>();
    }
}

pub(crate) fn setup_reward_solt(mut commands: Commands) {
    commands.spawn((
        RewardSolt,
        Transform::default(),
        GlobalTransform::default(),
        SceneTag,
    ));
}

pub(crate) fn setup_exit_timer(mut commands: Commands) {
    commands.spawn((
        GameTimer(Timer::from_seconds(0.0, TimerMode::Once)),
        GameTimerTag::SoundWin,
        SceneTag,
    ));

    commands.spawn((
        GameTimer(Timer::from_seconds(0.0, TimerMode::Once)),
        GameTimerTag::StopBgm,
        SceneTag,
    ));

    commands.spawn((
        GameTimer(Timer::from_seconds(7.0, TimerMode::Once)),
        GameTimerTag::Finish,
        SceneTag,
    ));
}

pub(crate) fn setup_exit_reward_anim(
    mut commands: Commands,
    reward: Query<(Entity, &Transform), With<RewardTag>>,
) {
    for (entity, transform) in &reward {
        commands.spawn((
            AnimationBundle {
                animation_clips: AnimationClips(vec![AnimationClip {
                    entity,
                    keyframes: vec![
                        KeyFrame {
                            time: Duration::ZERO,
                            transform: Some(*transform),
                            ..Default::default()
                        },
                        KeyFrame {
                            time: Duration::from_secs_f32(3.0),
                            transform: Some(Transform::from_translation(Vec3 {
                                x: 0.,
                                y: 200.,
                                z: 50.,
                            })),
                            ..Default::default()
                        },
                    ],
                }]),
                ..Default::default()
            },
            SceneTag,
        ));
    }
}

pub(crate) fn setup_exit_overlay(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 生成覆盖层
    let entity = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                material: materials.add(Color::srgba(1.0, 1.0, 1.0, 0.0)),
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
                            [(TypeId::of::<MaterialColorAnim>(), 0.0)]
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
                    KeyFrame {
                        time: Duration::from_secs_f32(7.0),
                        custom_animation_triggers: Some(
                            [(TypeId::of::<MaterialColorAnim>(), 1.0)]
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
