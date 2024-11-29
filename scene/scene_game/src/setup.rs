use std::{any::TypeId, time::Duration};

use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    sprite::{Anchor, MaterialMesh2dBundle},
    text::Text2dBounds,
};
use bevy_spine::prelude::*;
use fw_actor::components::AnimStandbyTag;
use fw_anim::{AnimationBundle, AnimationClip, AnimationClips, CustomAnimationTrigger, KeyFrame};
use fw_ftxm::{FtxmSource, MainMusicTable};
use mod_plant::{components::PlantSeedBundle, metadata::PlantRegistry};
use mod_userdata::UserData;
use mod_zombie::components::{AnimZombieEatTag, AnimZombieMoveTag};
use rand::{thread_rng, Rng};
use scene_res_game::{Background, GameSceneSettings, SodType};

use crate::{
    resource::{Sunshine, ZombieWaveController},
    tag::{
        CleanerCar, GameTimer, GameTimerTag, GameUiTag, LanePosition, MaterialColorAnim,
        NaturalSunshineSolt, PickableSeed, PlantSolt, RewardSolt, RewardTag, SceneTag, SeedbankTag,
        ShowLevelProgressShiftLeft, SoltType, StandbyZombieTag, SunshineText, ZombieSolt,
        ZombieTag,
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
    settings: Res<GameSceneSettings>,
    asset_server: Res<AssetServer>,
) {
    // 背景图
    let image = asset_server.load(match settings.background {
        Background::Day {
            sod_type: SodType::SodRow5,
            ..
        } => "images/background1.jpg",
        Background::Day { .. } => "images/background1unsodded.jpg",
        Background::Night => "images/background2.jpg",
        Background::Swim => "images/background3.jpg",
        Background::SwimFog => "images/background4.jpg",
        Background::Roof => "images/background5.jpg",
        Background::RoofNight => "images/background6.jpg",
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
    if let Background::Day { sod_type, .. } = &settings.background {
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

pub(crate) fn setup_plant_solt(mut commands: Commands, settings: Res<GameSceneSettings>) {
    match &settings.background {
        Background::Day {
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
        Background::Night => {
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
        Background::Swim | Background::SwimFog => {
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
        Background::Roof | Background::RoofNight => {
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

pub(crate) fn setup_zombie_solt(mut commands: Commands, settings: Res<GameSceneSettings>) {
    match &settings.background {
        Background::Day {
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
        Background::Night => {
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
        Background::Swim | Background::SwimFog => {
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
        Background::Roof | Background::RoofNight => {
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
            SceneTag,
            SeedbankTag,
            GameUiTag,
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
}

pub(crate) fn setup_cleanup_car(mut commands: Commands, asset_server: Res<AssetServer>) {
    let car = asset_server.load("reanim-merge/LawnMower.png");
    for lane in 0..5 {
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
    settings: Res<GameSceneSettings>,
    mut sunshine: ResMut<Sunshine>,
    mut zombie_wave_controller: ResMut<ZombieWaveController>,
) {
    sunshine.0 = settings.sunshine;
    zombie_wave_controller.next_wave_timer = Timer::new(
        Duration::from_secs_f32(if cfg!(debug_assertions) { 3.0 } else { 30.0 }),
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

pub(crate) fn setup_sunshine_solt(mut commands: Commands, settings: Res<GameSceneSettings>) {
    if settings.natural_sunshine {
        commands.spawn((
            NaturalSunshineSolt {
                next_sunshine_timer: Timer::new(Duration::from_secs_f32(7.0), TimerMode::Once),
            },
            SceneTag,
        ));
    }
}

pub(crate) fn setup_enter_timer(mut commands: Commands, settings: Res<GameSceneSettings>) {
    let mut time = 0.0;
    // 摄像机移动到中间，进入战场
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::CameraToCenterAnim,
        SceneTag,
    ));
    time += 2.0;

    // 无草坪之地，铺草地
    if let Background::Day {
        sod_type,
        upgrade_sod_type,
    } = &settings.background
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
    if userdata.unlock_plugins.len() < userdata.plant_solt_count {
        // 植物数量小于植物槽，无需选卡
        // 需要在seedbank位置生成对应植物
        let mut unlock_plugins = userdata.unlock_plugins.iter().collect::<Vec<_>>();
        unlock_plugins.sort();
        for (i, plant) in unlock_plugins.into_iter().enumerate() {
            commands.spawn((
                PlantSeedBundle {
                    transform: Transform::from_translation(Vec3 {
                        x: 85.0 + 55.0 * i as f32 + 55.0 * 0.5 - 400.0,
                        y: -8.0 - 35.0 + 300.0,
                        z: 1.1,
                    }),
                    visibility: Visibility::Hidden,
                    ..PlantSeedBundle::new(plant_registry.get(plant).unwrap().clone())
                },
                SceneTag,
                PickableSeed,
                GameUiTag,
                SeedbankTag,
            ));
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
    settings: Res<GameSceneSettings>,
) {
    let font: Handle<Font> = asset_server.load("font/fzcgbk.ttf");
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: settings.levelname.clone(),
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
                        value: settings.levelname.clone(),
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
    asset_server: Res<AssetServer>,
    mut skeletons: ResMut<Assets<SkeletonData>>,
) {
    let zombie = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("reanim-spine/zombie.skel"),
        asset_server.load("reanim-spine/zombie.atlas"),
    ));

    let mut rng = thread_rng();
    for _ in 0..3 {
        let y = rng.gen_range(-300.0..150.0);
        let x = rng.gen_range(0.0..175.0) - y * 0.1;
        commands.spawn((
            SpineBundle {
                skeleton: zombie.clone(),
                transform: Transform::from_xyz(x + 500.0, y, 1.0 - y * 0.001),
                ..Default::default()
            },
            StandbyZombieTag,
            AnimStandbyTag,
            SceneTag,
        ));
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
    // 暂时用 FINISH 代替
    time += 5.5;
    commands.spawn((
        GameTimer(Timer::from_seconds(time, TimerMode::Once)),
        GameTimerTag::Finish,
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
