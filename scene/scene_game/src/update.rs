use std::{any::TypeId, time::Duration};

use bevy::{audio::PlaybackMode, ecs::system::SystemId, prelude::*, sprite::Anchor};
use bevy_spatial::{kdtree::KDTree2, SpatialAccess};
use bevy_spine::{SkeletonData, Spine, SpineBundle};
use consts::anim::{INDEX_SUN_FADE_OUT, NAME_SUN_FADE_OUT};
use fw_actor::{
    components::{AnimHitTag, AnimStandbyTag},
    play_anim,
};
use fw_anim::{AnimationBundle, AnimationClip, AnimationClips, CustomAnimationTrigger, KeyFrame};
use fw_cursor::CursorPosition;
use fw_ftxm::FtxmAudioSink;
use mod_plant::{
    components::{
        AnimPlantShootTag, PlantBundle, PlantCooldown, PlantHp, PlantMetaData, PlantSeed,
        PlantSeedBundle, PlantUsable, SeedHover,
    },
    metadata::{
        PlantDetect, PlantPosition, PlantRegistry, PlantType, ProjectileTrack, ProjectileType,
    },
};
use mod_zombie::{
    components::{
        AnimZombieCriticalTag, AnimZombieEatStopTag, AnimZombieEatTag, AnimZombieFullDamageTag,
        AnimZombieHalfDamageTag, AnimZombieMoveTag, ZombieBundle, ZombieHp, ZombieMetadata,
    },
    metadata::ZombieRegistry,
};
use rand::{seq::SliceRandom, thread_rng, Rng};
use scene_base::GameScene;
use scene_res_game::{Background, GameSceneSettings, Reward, SodType, WaveType};

use crate::{
    resource::ZombieWaveController,
    tag::{
        BootCleanerCar, CleanerCar, ColorAlphaFade, DelayShowProjectile, FollowCursorTag, Freeze,
        GameTimer, GameTimerTag, GameUiTag, ImageCutAnim, InvincibleTag, LanePosition,
        LevelProgressFlagTag, LevelProgressHeadTag, LevelProgressProgressTag, MaterialColorAnim,
        MoveAcceleration, MoveVelocity, NaturalSunshineSolt, PickSeed, PickableSeed, PlantShootTag,
        PlantSolt, PlantTag, ProjectileCooldown, ProjectileTag, RewardSolt, RewardTag, SceneTag,
        SeedbankTag, ShowLevelProgressShiftLeft, SoltType, SunshineTag, SunshineText, ToDespawn,
        ToSpawnZombie, ZombieAttackableTag, ZombieCriticalTag, ZombieEatTag, ZombieHpAnim,
        ZombieSolt, ZombieTag,
    },
    GameState, Sunshine,
};

pub(crate) struct UpdateTimerSystem {
    move_camera_to_right: SystemId,
    move_camera_to_center: SystemId,
    move_camera_to_left: SystemId,
    fade_in_seed_bank: SystemId,
    fade_in_conveyer_belt: SystemId,
    fade_in_cars: SystemId,
    show_level_progress: SystemId,
    text_ready: SystemId,
    text_set: SystemId,
    text_plants: SystemId,
    text_huge_wave: SystemId,
    text_final_wave: SystemId,
    text_zombie_eat_your_brain: SystemId,
    sound_ready_set_plant: SystemId,
    sound_zombie_reached: SystemId,
    sound_huge_wave: SystemId,
    sound_huge_wave_zombie_reached: SystemId,
    sound_final_wave: SystemId,
    sound_lose: SystemId,
    sound_eat: SystemId,
    sound_scream: SystemId,
    sound_win: SystemId,
    stop_bgm: SystemId,
    upgrade_sod: SystemId,
    finish: SystemId,
}

// 计时器 延迟执行
pub(crate) fn update_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Query<(Entity, &mut GameTimer, &GameTimerTag)>,
    mut state: ResMut<NextState<GameState>>,
    mut timer_trigger: Local<Option<UpdateTimerSystem>>,
) {
    // 初始化trigger
    if timer_trigger.is_none() {
        *timer_trigger = Some(UpdateTimerSystem {
            move_camera_to_right: commands.register_one_shot_system(trigger_move_camera_to_right),
            move_camera_to_center: commands.register_one_shot_system(trigger_move_camera_to_center),
            move_camera_to_left: commands.register_one_shot_system(trigger_move_camera_to_left),
            fade_in_seed_bank: commands.register_one_shot_system(trigger_fade_in_seed_bank),
            fade_in_conveyer_belt: commands.register_one_shot_system(trigger_fade_in_conveyer_belt),
            fade_in_cars: commands.register_one_shot_system(trigger_fade_in_cars),
            show_level_progress: commands.register_one_shot_system(trigger_show_level_progress),
            text_ready: commands.register_one_shot_system(trigger_text_ready),
            text_set: commands.register_one_shot_system(trigger_text_set),
            text_plants: commands.register_one_shot_system(trigger_text_plants),
            text_huge_wave: commands.register_one_shot_system(trigger_text_huge_wave),
            text_final_wave: commands.register_one_shot_system(trigger_text_final_wave),
            text_zombie_eat_your_brain: commands
                .register_one_shot_system(trigger_text_zombie_eat_your_brain),
            sound_ready_set_plant: commands.register_one_shot_system(trigger_sound_ready_set_plant),
            sound_zombie_reached: commands.register_one_shot_system(trigger_sound_zombie_reached),
            sound_huge_wave: commands.register_one_shot_system(trigger_sound_huge_wave),
            sound_huge_wave_zombie_reached: commands
                .register_one_shot_system(trigger_sound_huge_wave_zombie_reached),
            sound_final_wave: commands.register_one_shot_system(trigger_sound_final_wave),
            sound_lose: commands.register_one_shot_system(trigger_sound_lose),
            sound_eat: commands.register_one_shot_system(trigger_sound_eat),
            sound_scream: commands.register_one_shot_system(trigger_sound_scream),
            sound_win: commands.register_one_shot_system(trigger_sound_win),
            stop_bgm: commands.register_one_shot_system(trigger_stop_bgm),
            upgrade_sod: commands.register_one_shot_system(trigger_upgrade_sod),
            finish: commands.register_one_shot_system(trigger_finish),
        })
    }
    let Some(timer_trigger) = &*timer_trigger else {
        return;
    };

    let delta = time.delta();

    for (entity, mut timer, tag) in &mut timer {
        timer.0.tick(delta);

        if !timer.0.just_finished() {
            continue;
        }

        if matches!(timer.0.mode(), TimerMode::Once) {
            commands.entity(entity).despawn_recursive();
        }
        let trigger_system = match tag {
            GameTimerTag::CameraToRightAnim => timer_trigger.move_camera_to_right,
            GameTimerTag::CameraToCenterAnim => timer_trigger.move_camera_to_center,
            GameTimerTag::CameraToLeftAnim => timer_trigger.move_camera_to_left,
            GameTimerTag::FadeInSeedBank => timer_trigger.fade_in_seed_bank,
            GameTimerTag::FadeInConveyerBelt => timer_trigger.fade_in_conveyer_belt,
            GameTimerTag::FadeInCars => timer_trigger.fade_in_cars,
            GameTimerTag::ShowLevelProgress => timer_trigger.show_level_progress,
            GameTimerTag::TextReady => timer_trigger.text_ready,
            GameTimerTag::TextSet => timer_trigger.text_set,
            GameTimerTag::TextPlants => timer_trigger.text_plants,
            GameTimerTag::TextHugeWave => timer_trigger.text_huge_wave,
            GameTimerTag::TextFinalWave => timer_trigger.text_final_wave,
            GameTimerTag::TextZombieEatYourBrain => timer_trigger.text_zombie_eat_your_brain,
            GameTimerTag::SoundReadySetPlant => timer_trigger.sound_ready_set_plant,
            GameTimerTag::SoundZombieReached => timer_trigger.sound_zombie_reached,
            GameTimerTag::SoundHugeWave => timer_trigger.sound_huge_wave,
            GameTimerTag::SoundHugeWaveZombieReached => {
                timer_trigger.sound_huge_wave_zombie_reached
            }
            GameTimerTag::SoundFinalWave => timer_trigger.sound_final_wave,
            GameTimerTag::SoundLose => timer_trigger.sound_lose,
            GameTimerTag::SoundEat => timer_trigger.sound_eat,
            GameTimerTag::SoundScream => timer_trigger.sound_scream,
            GameTimerTag::SoundWin => timer_trigger.sound_win,
            GameTimerTag::StopBgm => timer_trigger.stop_bgm,
            GameTimerTag::EnterState(game_state) => {
                state.set(*game_state);
                continue;
            }
            GameTimerTag::UpgradeSod => timer_trigger.upgrade_sod,
            GameTimerTag::Finish => timer_trigger.finish,
        };

        commands.run_system(trigger_system);
    }
}

fn trigger_move_camera_to_right(
    mut commands: Commands,
    camera: Query<(Entity, &Transform), With<Camera>>,
) {
    move_camera(&mut commands, &camera, 380.);
}

fn trigger_move_camera_to_center(
    mut commands: Commands,
    camera: Query<(Entity, &Transform), With<Camera>>,
) {
    move_camera(&mut commands, &camera, 0.);
}

fn trigger_move_camera_to_left(
    mut commands: Commands,
    camera: Query<(Entity, &Transform), With<Camera>>,
) {
    move_camera(&mut commands, &camera, -220.);
}

fn trigger_fade_in_seed_bank(
    mut commands: Commands,
    mut seedbank: Query<(Entity, &mut Visibility), With<SeedbankTag>>,
) {
    let mut clips = Vec::new();
    for (entity, mut visiblity) in &mut seedbank {
        *visiblity = Visibility::Visible;
        clips.push(AnimationClip {
            entity,
            keyframes: vec![
                KeyFrame {
                    time: Duration::ZERO,
                    transform: Some(Transform::from_xyz(-370.0, 400.0, 1.0)),
                    ..Default::default()
                },
                KeyFrame {
                    time: Duration::from_secs_f32(0.2),
                    transform: Some(Transform::from_xyz(-370.0, 300.0, 1.0)),
                    ..Default::default()
                },
            ],
        });
    }

    commands.spawn((
        AnimationBundle {
            animation_clips: AnimationClips(clips),
            ..Default::default()
        },
        SceneTag,
    ));
}

fn trigger_fade_in_conveyer_belt() {}

fn trigger_fade_in_cars(
    mut commands: Commands,
    mut cars: Query<(Entity, &mut Visibility, &CleanerCar), Without<SeedbankTag>>,
) {
    let mut clips = Vec::new();
    for (entity, mut visiblity, car) in &mut cars {
        *visiblity = Visibility::Visible;
        let y = car.lane as f32 * 100.0 - 100.0 * 2.;

        clips.push(AnimationClip {
            entity,
            keyframes: vec![
                KeyFrame {
                    time: Duration::ZERO,
                    transform: Some(Transform::from_xyz(-450.0, y, 10.0)),
                    ..Default::default()
                },
                KeyFrame {
                    time: Duration::from_secs_f32(0.15 * car.lane as f32 + 0.05),
                    transform: Some(Transform::from_xyz(-450.0, y, 10.0)),
                    ..Default::default()
                },
                KeyFrame {
                    time: Duration::from_secs_f32(0.15 * car.lane as f32 + 0.15),
                    transform: Some(Transform::from_xyz(-410.0, y, 10.0)),
                    ..Default::default()
                },
            ],
        });
    }

    commands.spawn((
        AnimationBundle {
            animation_clips: AnimationClips(clips),
            ..Default::default()
        },
        SceneTag,
    ));
}

fn trigger_show_level_progress(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSceneSettings>,
    mut shift_left: Query<&mut Transform, With<ShowLevelProgressShiftLeft>>,
) {
    let meter = asset_server.load("images/FlagMeter.png");
    let level_progress = asset_server.load("images/FlagMeterLevelProgress.png");
    let parts = asset_server.load("images/FlagMeterParts.png");

    // 背景
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                rect: Some(Rect {
                    min: Vec2 { x: 0.0, y: 0.0 },
                    max: Vec2 { x: 158.0, y: 27.0 },
                }),
                anchor: Anchor::BottomRight,
                ..Default::default()
            },
            texture: meter.clone(),
            transform: Transform::from_xyz(390.0, -290.0, 50.0),
            ..Default::default()
        },
        SceneTag,
    ));

    // 进度条
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                rect: Some(Rect {
                    min: Vec2 { x: 158.0, y: 27.0 },
                    max: Vec2 { x: 158.0, y: 54.0 },
                }),
                anchor: Anchor::BottomRight,
                ..Default::default()
            },
            texture: meter,
            transform: Transform::from_xyz(390.0, -290.0, 50.1),
            ..Default::default()
        },
        SceneTag,
        LevelProgressProgressTag,
    ));

    // 旗帜
    let flag_count = settings
        .zombie_waves
        .iter()
        .filter(|wave| matches!(wave.wave_type, WaveType::HugeWave))
        .count();
    let split_part = 158.0 / flag_count as f32;
    for i in 0..flag_count {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::CenterLeft,
                    rect: Some(Rect {
                        min: Vec2 { x: 25.0, y: 0.0 },
                        max: Vec2 { x: 50.0, y: 25.0 },
                    }),
                    ..Default::default()
                },
                texture: parts.clone(),
                transform: Transform::from_xyz(
                    390.0 - 154.0 + split_part * i as f32,
                    -275.0,
                    50.19,
                ),
                ..Default::default()
            },
            SceneTag,
        ));
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::CenterLeft,
                    rect: Some(Rect {
                        min: Vec2 { x: 50.0, y: 0.0 },
                        max: Vec2 { x: 75.0, y: 25.0 },
                    }),
                    ..Default::default()
                },
                texture: parts.clone(),
                transform: Transform::from_xyz(390.0 - 154.0 + split_part * i as f32, -275.0, 50.2),
                ..Default::default()
            },
            SceneTag,
            LevelProgressFlagTag::NotReach,
        ));
    }

    // 关卡进程
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomRight,
                ..Default::default()
            },
            texture: level_progress,
            transform: Transform::from_xyz(390.0 - 36.0, -295.0, 50.3),
            ..Default::default()
        },
        SceneTag,
    ));

    // 头
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                rect: Some(Rect {
                    min: Vec2 { x: 0.0, y: 0.0 },
                    max: Vec2 { x: 25.0, y: 25.0 },
                }),
                anchor: Anchor::Center,
                ..Default::default()
            },
            texture: parts.clone(),
            transform: Transform::from_xyz(390.0, -275.0, 50.4),
            ..Default::default()
        },
        SceneTag,
        LevelProgressHeadTag,
    ));

    for mut transform in &mut shift_left {
        transform.translation.x -= 175.0;
    }
}

fn trigger_text_ready(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_image(
        &mut commands,
        &asset_server,
        "reanim/StartReady.png",
        0.0,
        1.0,
        0.5,
        0.0,
        0.0,
    );
}

fn trigger_text_set(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_image(
        &mut commands,
        &asset_server,
        "reanim/StartSet.png",
        0.0,
        1.0,
        0.5,
        0.0,
        0.0,
    );
}

fn trigger_text_plants(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_image(
        &mut commands,
        &asset_server,
        "reanim/StartPlant.png",
        0.0,
        1.0,
        1.0,
        0.0,
        0.0,
    );
}

fn trigger_text_huge_wave(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_image(
        &mut commands,
        &asset_server,
        "reanim/HugeWave.png",
        0.2,
        1.5,
        6.0,
        1.0,
        0.0,
    );
}

fn trigger_text_final_wave(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_image(
        &mut commands,
        &asset_server,
        "reanim/FinalWave.png",
        1.0,
        5.0,
        2.0,
        0.5,
        0.0,
    );
}

fn trigger_text_zombie_eat_your_brain(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_image(
        &mut commands,
        &asset_server,
        "reanim/ZombiesWon.png",
        2.5,
        0.0,
        2.0,
        0.0,
        -220.0,
    );
}

fn trigger_sound_ready_set_plant(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_se(&mut commands, &asset_server, "sounds/readysetplant.ogg")
}

fn trigger_sound_zombie_reached(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_se(&mut commands, &asset_server, "sounds/awooga.ogg");
}

fn trigger_sound_huge_wave(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_se(&mut commands, &asset_server, "sounds/hugewave.ogg");
}

fn trigger_sound_huge_wave_zombie_reached(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_se(&mut commands, &asset_server, "sounds/siren.ogg");
}

fn trigger_sound_final_wave(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_se(&mut commands, &asset_server, "sounds/finalwave.ogg");
}

fn trigger_sound_lose(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_se(&mut commands, &asset_server, "sounds/losemusic.ogg");
}

fn trigger_sound_eat(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = thread_rng();
    spawn_se(
        &mut commands,
        &asset_server,
        ["sounds/chomp.ogg", "sounds/chomp2.ogg"]
            .choose(&mut rng)
            .unwrap(),
    );
}

fn trigger_sound_scream(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_se(&mut commands, &asset_server, "sounds/scream.ogg");
}

fn trigger_sound_win(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_se(&mut commands, &asset_server, "sounds/winmusic.ogg");
}

fn trigger_stop_bgm(mut commands: Commands, bgm: Query<Entity, With<FtxmAudioSink>>) {
    for entity in &bgm {
        commands.entity(entity).despawn_recursive();
    }
}

fn trigger_upgrade_sod(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSceneSettings>,
) {
    let Background::Day {
        sod_type,
        upgrade_sod_type: _,
    } = &settings.background
    else {
        return;
    };

    let (image, lanes, x, y, cut_start, cut_end, height) = match sod_type {
        SodType::None => ("images/sod1row.png", vec![2], 240., 25.0, 0.0, 771.0, 127.0),
        SodType::SodRow1 => (
            "images/sod3row.png",
            vec![1, 3],
            240.,
            25.0,
            0.0,
            771.0,
            355.0,
        ),
        SodType::SodRow3 => (
            "images/background1.jpg",
            vec![0, 4],
            0.,
            0.,
            240.0,
            240.0 + 771.0,
            600.0,
        ),
        SodType::SodRow5 => return,
    };

    let cover_image = asset_server.load(image);
    let roll_image = asset_server.load("reanim/SodRoll.png");
    let roll_cap_image = asset_server.load("reanim/SodRollCap.png");

    let cover_entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::CenterLeft,
                    rect: Some(Rect {
                        min: Vec2 { x: 0.0, y: 0.0 },
                        max: Vec2 {
                            x: cut_end * 0.5,
                            y: height,
                        },
                    }),
                    ..Default::default()
                },
                texture: cover_image,
                transform: Transform::from_xyz(-620. + x, -y, 0.02),
                ..Default::default()
            },
            SceneTag,
            CustomAnimationTrigger::default(),
            ImageCutAnim,
        ))
        .id();
    let roll_entity = lanes
        .iter()
        .map(|row| {
            (
                *row,
                commands
                    .spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                anchor: Anchor::TopLeft,
                                custom_size: Some(Vec2 { x: 70., y: 100. }),
                                ..Default::default()
                            },
                            texture: roll_image.clone(),
                            ..Default::default()
                        },
                        SceneTag,
                    ))
                    .id(),
            )
        })
        .collect::<Vec<_>>();
    let roll_cap_entity = lanes
        .iter()
        .map(|row| {
            (
                *row,
                commands
                    .spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                anchor: Anchor::TopLeft,
                                ..Default::default()
                            },
                            texture: roll_cap_image.clone(),
                            ..Default::default()
                        },
                        SceneTag,
                    ))
                    .id(),
            )
        })
        .collect::<Vec<_>>();

    let mut animation_clips = Vec::new();
    for (y, entity) in roll_entity {
        animation_clips.push(AnimationClip {
            entity,
            keyframes: vec![
                KeyFrame {
                    time: Duration::ZERO,
                    transform: Some(
                        Transform::from_xyz(-620. + 240., -y as f32 * 100. + 100. * 2.5, 0.3)
                            .with_scale(Vec3 {
                                x: 1.0,
                                y: 0.7,
                                z: 1.0,
                            }),
                    ),
                    ..Default::default()
                },
                KeyFrame {
                    time: Duration::from_secs_f32(2.0),
                    transform: Some(
                        Transform::from_xyz(
                            -620. + 240. + 771.,
                            -y as f32 * 100. + 100. * 2.5,
                            0.3,
                        )
                        .with_scale(Vec3 {
                            x: 0.0,
                            y: 1.0,
                            z: 1.0,
                        }),
                    ),
                    ..Default::default()
                },
            ],
        });
    }

    for (y, entity) in roll_cap_entity {
        animation_clips.push(AnimationClip {
            entity,
            keyframes: vec![
                KeyFrame {
                    time: Duration::ZERO,
                    transform: Some(
                        Transform::from_xyz(
                            -620. + 240.,
                            -y as f32 * 100. + 100. * 2.5 - 30.,
                            0.31,
                        )
                        .with_scale(Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        }),
                    ),
                    ..Default::default()
                },
                KeyFrame {
                    time: Duration::from_secs_f32(2.0),
                    transform: Some(
                        Transform::from_xyz(
                            -620. + 240. + 771.,
                            -y as f32 * 100. + 100. * 2.5 - 100.,
                            0.31,
                        )
                        .with_scale(Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        })
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::PI * 6.)),
                    ),
                    ..Default::default()
                },
            ],
        });
    }

    animation_clips.push(AnimationClip {
        entity: cover_entity,
        keyframes: vec![
            KeyFrame {
                time: Duration::ZERO,
                custom_animation_triggers: Some(
                    [(TypeId::of::<ImageCutAnim>(), cut_start)]
                        .into_iter()
                        .collect(),
                ),
                ..Default::default()
            },
            KeyFrame {
                time: Duration::from_secs_f32(2.0),
                custom_animation_triggers: Some(
                    [(TypeId::of::<ImageCutAnim>(), cut_end)]
                        .into_iter()
                        .collect(),
                ),
                ..Default::default()
            },
        ],
    });

    commands.spawn(AnimationBundle {
        animation_clips: AnimationClips(animation_clips),
        ..Default::default()
    });
}

fn trigger_finish(mut next_screen: ResMut<NextState<GameScene>>) {
    // TODO: 游戏结束，切到其他画面
    next_screen.set(GameScene::Title);
}

#[inline]
fn move_camera(
    commands: &mut Commands,
    camera: &Query<(Entity, &Transform), With<Camera>>,
    x: f32,
) {
    let (camera_entity, camera_transform) = camera.single();
    commands.spawn((
        AnimationBundle {
            animation_clips: AnimationClips(vec![AnimationClip {
                entity: camera_entity,
                keyframes: vec![
                    KeyFrame {
                        time: Duration::ZERO,
                        transform: Some(*camera_transform),
                        ..Default::default()
                    },
                    KeyFrame {
                        time: Duration::from_secs_f32(1.5),
                        transform: Some(Transform::from_xyz(x, 0., 0.)),
                        ..Default::default()
                    },
                ],
            }]),
            ..Default::default()
        },
        SceneTag,
    ));
}

#[inline]
fn spawn_se(commands: &mut Commands, asset_server: &AssetServer, path: &'static str) {
    let se = asset_server.load(path);
    commands.spawn((
        AudioBundle {
            source: se,
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..Default::default()
            },
        },
        SceneTag,
    ));
}

#[inline]
fn spawn_image(
    commands: &mut Commands,
    asset_server: &AssetServer,
    path: &'static str,
    scale_time_secs: f32,
    scale_start: f32,
    stay_time: f32,
    fade_out_time: f32,
    x: f32,
) {
    let image = asset_server.load(path);
    let mut entity = commands.spawn((
        SpriteBundle {
            texture: image,
            ..Default::default()
        },
        SceneTag,
        CustomAnimationTrigger::default(),
        ColorAlphaFade,
        GameUiTag,
    ));

    entity.insert(AnimationBundle {
        animation_clips: AnimationClips(vec![AnimationClip {
            entity: entity.id(),
            keyframes: vec![
                KeyFrame {
                    time: Duration::ZERO,
                    transform: Some(Transform::from_xyz(x, 0., 100.).with_scale(Vec3 {
                        x: scale_start,
                        y: scale_start,
                        z: 1.,
                    })),
                    custom_animation_triggers: Some(
                        [(TypeId::of::<ColorAlphaFade>(), 0.5)]
                            .into_iter()
                            .collect(),
                    ),
                },
                KeyFrame {
                    time: Duration::from_secs_f32(scale_time_secs),
                    transform: Some(Transform::from_xyz(x, 0., 100.)),
                    custom_animation_triggers: Some(
                        [(TypeId::of::<ColorAlphaFade>(), 1.0)]
                            .into_iter()
                            .collect(),
                    ),
                },
                KeyFrame {
                    time: Duration::from_secs_f32(scale_time_secs + stay_time),
                    transform: Some(Transform::from_xyz(x, 0., 100.)),
                    custom_animation_triggers: Some(
                        [(TypeId::of::<ColorAlphaFade>(), 1.0)]
                            .into_iter()
                            .collect(),
                    ),
                },
                KeyFrame {
                    time: Duration::from_secs_f32(scale_time_secs + stay_time + fade_out_time),
                    transform: Some(Transform::from_xyz(x, 0., 100.)),
                    custom_animation_triggers: Some(
                        [(TypeId::of::<ColorAlphaFade>(), 0.0)]
                            .into_iter()
                            .collect(),
                    ),
                },
            ],
        }]),
        ..Default::default()
    });
}

// 根据动画组件更新透明度
pub(crate) fn update_alpha_color(
    mut targets: Query<(&mut Sprite, &CustomAnimationTrigger), With<ColorAlphaFade>>,
) {
    for (mut sprite, trigger) in &mut targets {
        let Some(alpha) = trigger.animation_value.get(&TypeId::of::<ColorAlphaFade>()) else {
            continue;
        };
        sprite.color = sprite.color.with_alpha(*alpha);
    }
}

// 更新阳关文本
pub(crate) fn update_sunshine_text(
    sunshine: Res<Sunshine>,
    mut text: Query<&mut Text, With<SunshineText>>,
) {
    for mut text in &mut text {
        text.sections[0].value = sunshine.0.to_string();
    }
}

// 更新图片裁剪动画
pub(crate) fn update_image_cut(
    mut targets: Query<(&mut Sprite, &CustomAnimationTrigger), With<ImageCutAnim>>,
) {
    for (mut sprite, trigger) in &mut targets {
        let Some(x) = trigger.animation_value.get(&TypeId::of::<ImageCutAnim>()) else {
            continue;
        };
        if let Some(ref mut rect) = &mut sprite.rect {
            rect.max.x = *x;
        }
    }
}

// 输入：捡起种子
pub(crate) fn input_pick_seed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    seed: Query<
        (
            Entity,
            &PlantMetaData,
            &PlantCooldown,
            &PlantUsable,
            &SeedHover,
        ),
        (With<PlantSeed>, With<PickableSeed>),
    >,
    mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
    pick_seed: Query<(Entity, &PickSeed)>,
    sunshine: Res<Sunshine>,
) {
    // 左键点击
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    // 点击的种子卡
    let Some((entity, PlantMetaData(metadata), cooldown, usable, _)) = seed
        .iter()
        .filter(|(_, _, _, _, hover)| matches!(hover, SeedHover::Hover))
        .next()
    else {
        return;
    };

    // 如果点到了种子卡，则清除点击事件
    mouse_button_input.clear_just_pressed(MouseButton::Left);

    // 正在捡起的种子
    for (pick_seed_entity, PickSeed { seed, .. }) in &pick_seed {
        if entity == *seed {
            commands.entity(pick_seed_entity).despawn_recursive();
        }
    }

    // 如果有种子，则不再捡起新的种子
    if !pick_seed.is_empty() {
        return;
    }

    // 检查是否可以使用
    // 1. 阳光是否足够
    // 2. 是否已经冷却
    // 3. 其他原因不可使用
    let can_use_plant = sunshine.0 >= metadata.sunshine
        && matches!(cooldown, PlantCooldown::Ready)
        && matches!(usable, PlantUsable::Usable);

    // 无法使用植物
    if !can_use_plant {
        // 音效
        spawn_se(&mut commands, &asset_server, "sounds/buzzer.ogg");
        return;
    }

    // 生成跟随鼠标的植物
    commands
        .spawn((
            Transform::default(),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            FollowCursorTag,
            SceneTag,
            PickSeed {
                seed: entity,
                plant_info: metadata.clone(),
            },
            GameUiTag,
        ))
        .with_children(|parent| {
            let mut plant_bundle = PlantBundle::new(metadata.clone());
            plant_bundle.spine.transform =
                Transform::from_xyz(0.0, 0.0, 10.0).with_scale(Vec3::ONE * 0.7);
            parent.spawn(plant_bundle);
        });

    // 音效
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/seedlift.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..Default::default()
            },
        },
        SceneTag,
    ));
}

// 实体跟随鼠标
pub(crate) fn update_follow_cursor(
    cursor_position: Res<CursorPosition>,
    mut target: Query<&mut Transform, With<FollowCursorTag>>,
) {
    let Vec2 { x, y } = cursor_position.world_position;
    for mut transform in &mut target {
        *transform = Transform::from_xyz(x, y, 0.0);
    }
}

// 输入：种植物
pub(crate) fn plant_seed(
    mut commands: Commands,
    mut solts: Query<(Entity, &mut PlantSolt, &GlobalTransform, &LanePosition)>,
    pick_seed: Query<(Entity, &PickSeed)>,
    mut seeds: Query<&mut PlantCooldown, (With<PlantSeed>, With<PickableSeed>)>,
    cursor_position: Res<CursorPosition>,
    mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
    plants: Query<&PlantMetaData>,
    asset_server: Res<AssetServer>,
    mut sunshine: ResMut<Sunshine>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }
    let Some((entity, PickSeed { seed, plant_info })) = pick_seed.iter().next() else {
        return;
    };

    // 点击的格子
    let Some((solt_entity, mut solt, solt_translation, lane_position, _)) = solts
        .iter_mut()
        .filter_map(|(entity, solt, transform, lane_position)| {
            let solt_translation = transform.translation();
            let solt_position = Vec2 {
                x: solt_translation.x,
                y: solt_translation.y,
            };

            let distance_square = solt_position.distance_squared(cursor_position.world_position);
            if distance_square > 55.0 * 55.0 {
                return None;
            }

            Some((
                entity,
                solt,
                solt_translation,
                lane_position,
                distance_square,
            ))
        })
        .min_by(|(_, _, _, _, ds1), (_, _, _, _, ds2)| ds1.partial_cmp(ds2).unwrap())
    else {
        return;
    };

    // 如果点到了格子，清除点击事件
    mouse_button_input.clear_just_pressed(MouseButton::Left);

    // 检查植物种植位置，有花盆优先考虑花盆
    let pot_plant = solt
        .pot
        .map(|entity| plants.get(entity).ok())
        .flatten()
        .map(|metadata| metadata.0.id);
    let can_plant_on = if solt.grave.is_some() {
        plant_info.plant_on.grave
    } else if solt.hole.is_some() {
        plant_info.plant_on.hole
    } else {
        match (pot_plant, &solt.solt_type) {
            (Some(PlantType::LilyPad), _) => plant_info.plant_on.lily,
            (Some(PlantType::FlowerPot), _) => plant_info.plant_on.pot,
            (Some(_), _) => false, // 未知花盆类型，默认不可种植
            (None, SoltType::Dirt) => plant_info.plant_on.dirt,
            (None, SoltType::River) => plant_info.plant_on.river,
            (None, SoltType::Roof) => plant_info.plant_on.roof,
        }
    };
    if !can_plant_on {
        return;
    }

    // 对应植物槽位置
    let solt_position = match &plant_info.position {
        PlantPosition::Primary => &mut solt.primary,
        PlantPosition::Protect => &mut solt.protect,
        PlantPosition::Pot => &mut solt.pot,
        PlantPosition::Temp => &mut solt.temp,
    };

    // 对应位置已有植物，忽略
    if solt_position.is_some() {
        return;
    }

    // 再次检查阳光是否充足
    if sunshine.0 < plant_info.sunshine {
        return;
    }

    // 移除跟随鼠标的植物
    commands.entity(entity).despawn_recursive();

    // 在格子位置种植物
    let mut plant_bundle = PlantBundle::new(plant_info.clone());
    plant_bundle.spine.transform =
        Transform::from_xyz(solt_translation.x, solt_translation.y - 15.0, 10.0)
            .with_scale(Vec3::ONE * 0.7);
    let mut plant_entity = commands.spawn((
        plant_bundle,
        PlantTag {
            solt: solt_entity,
            metadata: plant_info.clone(),
        },
        AnimStandbyTag,
        SceneTag,
        LanePosition {
            lane: lane_position.lane,
            x: solt_translation.x,
        },
    ));
    if let Some(shoot) = &plant_info.shoot {
        plant_entity.insert((
            ProjectileCooldown {
                cooldown: shoot.shoot_cooldown,
                max_cooldown: shoot.shoot_cooldown,
            },
            PlantShootTag::Standby,
        ));
    }
    *solt_position = Some(plant_entity.id());

    // 扣除阳光
    sunshine.0 -= plant_info.sunshine;

    // 种子重新装填
    if let Ok(mut cooldown) = seeds.get_mut(*seed) {
        *cooldown = PlantCooldown::Cooldown(Duration::from_secs_f32(plant_info.cooldown));
    }

    // 音效
    spawn_se(
        &mut commands,
        &asset_server,
        ["sounds/plant.ogg", "sounds/plant2.ogg"]
            .choose(&mut thread_rng())
            .unwrap(),
    );
}

// 取消捡起的种子
pub(crate) fn cancel_pick_seed(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    seed: Query<Entity, With<PickSeed>>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Right) {
        return;
    }

    for entity in &seed {
        commands.entity(entity).despawn_recursive();
    }
}

// 植物射击
pub(crate) fn plant_shoot(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut plants: Query<(
        Entity,
        &mut ProjectileCooldown,
        &PlantShootTag,
        &PlantTag,
        &GlobalTransform,
    )>,
) {
    let delta = time.delta().as_secs_f32();
    for (entity, mut cooldown, shoot, plant, global_transform) in &mut plants {
        // 射击冷却
        cooldown.cooldown -= delta;
        if cooldown.cooldown > 0.0 {
            continue;
        }

        // 是否射击
        if !matches!(shoot, PlantShootTag::Shoot) {
            continue;
        }

        cooldown.cooldown = cooldown.max_cooldown;

        // 播放射击动画
        commands.entity(entity).insert(AnimPlantShootTag);

        // 生成射弹
        let plant_original = global_transform.translation();
        for projectile in plant
            .metadata
            .shoot
            .as_ref()
            .map(|shoot| shoot.projectiles.iter())
            .unwrap_or_default()
        {
            let projectile_image = asset_server.load(match &projectile.projectile_type {
                ProjectileType::Pea => "images/ProjectilePea.png",
                ProjectileType::SnowPea => "images/ProjectileSnowPea.png",
                ProjectileType::Cactus => "images/ProjectileCactus.png",
                ProjectileType::Star => "images/Projectile_star.png",
            });

            let mut projectile_entity = commands.spawn((
                SpriteBundle {
                    texture: projectile_image,
                    transform: Transform::from_translation(
                        plant_original
                            + Vec3 {
                                x: projectile.offset_x,
                                y: projectile.offset_y,
                                z: 1.0,
                            } * 0.7,
                    ),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
                SceneTag,
                ProjectileTag,
                Freeze,
                DelayShowProjectile {
                    timer: Timer::new(
                        Duration::from_secs_f32(projectile.shoot_timing),
                        TimerMode::Once,
                    ),
                },
            ));

            // 速度
            match &projectile.track {
                ProjectileTrack::Line { direction } => {
                    projectile_entity.insert(MoveVelocity(Vec2::from_angle(*direction) * 500.0));
                }
                ProjectileTrack::Throw => todo!(),
                ProjectileTrack::Follow => todo!(),
            }
        }
    }
}

// 投掷物延迟展示
pub(crate) fn update_projectile_show(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut projectile: Query<(Entity, &mut DelayShowProjectile, &mut Visibility)>,
) {
    let delta = time.delta();
    for (entity, mut config, mut visibility) in &mut projectile {
        // 计时
        config.timer.tick(delta);
        if !config.timer.just_finished() {
            continue;
        }

        // 显示
        *visibility = Visibility::Visible;

        // 音效
        spawn_se(
            &mut commands,
            &asset_server,
            ["sounds/throw.ogg", "sounds/throw2.ogg"]
                .choose(&mut thread_rng())
                .unwrap(),
        );

        // 标签更新
        commands
            .entity(entity)
            .remove::<(Freeze, DelayShowProjectile)>();
    }
}

// 更新实体速度
pub(crate) fn update_velocity(
    time: Res<Time>,
    mut targets: Query<(&MoveAcceleration, &mut MoveVelocity), Without<Freeze>>,
) {
    let delta = time.delta().as_secs_f32();
    targets.par_iter_mut().for_each(|(acc, ref mut vec)| {
        vec.0 += acc.0 * delta;
    });
}

// 更新实体位置
pub(crate) fn update_movement(
    time: Res<Time>,
    mut targets: Query<(&MoveVelocity, &mut Transform), Without<Freeze>>,
) {
    let delta = time.delta().as_secs_f32();
    targets.par_iter_mut().for_each(|(vec, ref mut transform)| {
        transform.translation.x += delta * vec.0.x;
        transform.translation.y += delta * vec.0.y;
    });
}

// 僵尸进入啃食状态
pub(crate) fn check_zombie_eat_start(
    mut commands: Commands,
    mut zombie: Query<
        (Entity, &GlobalTransform),
        (
            With<ZombieTag>,
            Without<ZombieEatTag>,
            Without<ZombieCriticalTag>,
        ),
    >,
    plant_kdtree: Res<KDTree2<PlantTag>>,
    plant_tag: Query<&PlantTag>,
    plant_solt: Query<&PlantSolt>,
) {
    for (zombie_entity, zombie_transform) in &mut zombie {
        // 僵尸位置
        let translation = zombie_transform.translation();
        let zombie_position = Vec2 {
            x: translation.x,
            y: translation.y,
        };
        // 最近的植物
        let Some((plant_position, Some(nearest_plant))) =
            plant_kdtree.nearest_neighbour(zombie_position)
        else {
            continue;
        };

        // 距离判断
        if zombie_position.distance_squared(plant_position) > 40.0 * 40.0 {
            continue;
        }

        // 从槽位判断应该吃的植物
        let Ok(plant_tag) = plant_tag.get(nearest_plant) else {
            continue;
        };
        let Ok(plant_solt) = plant_solt.get(plant_tag.solt) else {
            continue;
        };
        let Some(eat_plant) = plant_solt.plants().into_iter().flatten().next() else {
            continue;
        };

        // 僵尸啃食标签
        let mut timer = Timer::new(Duration::from_secs_f32(0.5), TimerMode::Repeating);
        timer.set_elapsed(Duration::from_secs_f32(0.5));

        commands
            .entity(zombie_entity)
            .remove::<AnimZombieMoveTag>()
            .insert((
                AnimZombieEatTag,
                ZombieEatTag {
                    target_plant: eat_plant,
                    timer,
                },
                Freeze,
            ));
    }
}

// 僵尸结束啃食状态
pub(crate) fn check_zombie_eat_end(
    mut commands: Commands,
    mut zombie: Query<(Entity, &ZombieEatTag), Without<ZombieCriticalTag>>,
) {
    for (entity, eat_tag) in &mut zombie {
        if commands.get_entity(eat_tag.target_plant).is_some() {
            continue;
        }

        commands
            .entity(entity)
            .remove::<(AnimZombieEatTag, ZombieEatTag, Freeze)>()
            .insert((AnimZombieMoveTag, AnimZombieEatStopTag));
    }
}

// 僵尸结算投掷物伤害
pub(crate) fn zombie_projectile_damage(
    mut commands: Commands,
    mut zombies: Query<(&mut ZombieHp, Option<&InvincibleTag>), With<ZombieAttackableTag>>,
    projectiles: Query<(Entity, &GlobalTransform), With<ProjectileTag>>,
    zombie_kdtree: Res<KDTree2<ZombieAttackableTag>>,
    asset_server: Res<AssetServer>,
) {
    for (projectile_entity, transform) in &projectiles {
        // 投掷物位置
        let projectile_translation = transform.translation();
        let projectile_position = Vec2 {
            x: projectile_translation.x,
            y: projectile_translation.y,
        };

        // 最近的僵尸
        let Some((zombie_position, Some(zombie_entity))) =
            zombie_kdtree.nearest_neighbour(projectile_position)
        else {
            continue;
        };

        // 僵尸碰撞箱
        let zombie_collision = Rect {
            min: zombie_position - Vec2 { x: 31.0, y: 14.0 },
            max: zombie_position + Vec2 { x: 15.0, y: 131.0 },
        };

        // 碰撞检测
        if !zombie_collision.contains(projectile_position) {
            continue;
        }

        // 僵尸信息
        let Ok((mut zombie_hp, invincible)) = zombies.get_mut(zombie_entity) else {
            continue;
        };

        // 如果不处于无敌状态，则计算伤害
        if invincible.is_none() {
            zombie_hp.hp -= 20.0;
        }

        // 音效
        spawn_se(
            &mut commands,
            &asset_server,
            ["sounds/splat.ogg", "sounds/splat2.ogg", "sounds/splat3.ogg"]
                .choose(&mut thread_rng())
                .unwrap(),
        );

        // 受击动画
        commands.entity(zombie_entity).insert(AnimHitTag);

        // 移除投掷物
        commands.entity(projectile_entity).despawn_recursive();
    }
}

// 根据僵尸血量播放动画
pub(crate) fn update_zombie_hp_anim(
    mut commands: Commands,
    mut zombie: Query<(Entity, &ZombieMetadata, &ZombieHp, &mut ZombieHpAnim)>,
) {
    for (entity, ZombieMetadata(metadata), hp, mut hp_anim) in &mut zombie {
        if hp.hp <= metadata.hp.real / 2.0 + metadata.hp.critical
            && !hp_anim.trigger_half_damage_anim
        {
            commands.entity(entity).insert(AnimZombieHalfDamageTag);
            hp_anim.trigger_half_damage_anim = true;
        }
        if hp.hp <= metadata.hp.critical && !hp_anim.trigger_full_damage_anim {
            commands.entity(entity).insert(AnimZombieFullDamageTag);
            hp_anim.trigger_full_damage_anim = true;
        }
        if hp.hp <= 0.0 && !hp_anim.trigger_critical_anim {
            commands
                .entity(entity)
                .remove::<AnimStandbyTag>()
                .insert(AnimZombieCriticalTag);
            hp_anim.trigger_critical_anim = true;
        }
    }
}

// 僵尸进入临界状态
pub(crate) fn update_zombie_enter_critical(
    mut commands: Commands,
    zombies: Query<
        (Entity, &ZombieMetadata, &ZombieHp),
        (With<ZombieTag>, Without<ZombieCriticalTag>),
    >,
) {
    for (entity, ZombieMetadata(metadata), hp) in &zombies {
        if hp.hp > metadata.hp.critical {
            continue;
        }

        commands.entity(entity).insert(ZombieCriticalTag);
    }
}

// 僵尸临界状态更新逻辑
pub(crate) fn update_zombie_critical(
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
    mut zombies: Query<&mut ZombieHp, With<ZombieCriticalTag>>,
) {
    if timer.is_none() {
        *timer = Some(Timer::new(
            Duration::from_secs_f32(1.0),
            TimerMode::Repeating,
        ));
    }
    let Some(ref mut timer) = &mut *timer else {
        return;
    };
    timer.tick(time.delta());

    if !timer.just_finished() {
        return;
    }

    zombies.par_iter_mut().for_each(|mut hp| hp.hp -= 20.0);
}

// 僵尸死亡
pub(crate) fn update_zombie_die(
    mut commands: Commands,
    zombies: Query<(Entity, &ZombieHp), Without<ToDespawn>>,
) {
    for (entity, hp) in &zombies {
        if hp.hp > 0.0 {
            continue;
        }
        commands
            .entity(entity)
            .insert(ToDespawn(Timer::new(
                Duration::from_secs_f32(1.0),
                TimerMode::Once,
            )))
            .remove::<(MoveVelocity, MoveAcceleration, ZombieAttackableTag)>();
    }
}

// 延迟移除实体
pub(crate) fn despawn_schedule_entity(
    mut commands: Commands,
    time: Res<Time>,
    mut entities: Query<(Entity, &mut ToDespawn)>,
) {
    let delta = time.delta();

    for (entity, mut to_despawn) in &mut entities {
        to_despawn.0.tick(delta);
        if to_despawn.0.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// 更新实体在场景中的位置
pub(crate) fn update_lane_position(mut target: Query<(&mut LanePosition, &GlobalTransform)>) {
    target.par_iter_mut().for_each(|(mut position, transform)| {
        position.x = transform.translation().x;
    });
}

// 更新植物侦测，判断是否需要射击
pub(crate) fn update_plant_shoot_enable(
    mut plants: Query<(
        &PlantTag,
        &mut PlantShootTag,
        &LanePosition,
        &GlobalTransform,
    )>,
    zombies: Query<(&LanePosition, &GlobalTransform), With<ZombieAttackableTag>>,
) {
    plants
        .par_iter_mut()
        .for_each(|(plant, mut shoot, plant_position, plant_transform)| {
            let found_enemy = match plant.metadata.shoot.as_ref().map(|shot| &shot.detect) {
                Some(PlantDetect::LaneFront) => zombies.iter().any(|(zombie_position, _)| {
                    zombie_position.lane == plant_position.lane
                        && zombie_position.x >= plant_position.x
                }),
                Some(PlantDetect::LaneBack) => zombies.iter().any(|(zombie_position, _)| {
                    zombie_position.lane == plant_position.lane
                        && zombie_position.x <= plant_position.x
                }),
                Some(PlantDetect::Lane) => zombies
                    .iter()
                    .any(|(zombie_position, _)| zombie_position.lane == plant_position.lane),
                Some(PlantDetect::Rays { direction }) => {
                    let plant_translation = plant_transform.translation();
                    let plant_origin = Vec2 {
                        x: plant_translation.x,
                        y: plant_translation.y,
                    };
                    let direction = Vec2::from_angle(*direction);

                    zombies.iter().any(|(_, zombie_transform)| {
                        let zombie_translation = zombie_transform.translation();
                        let zombie_position = Vec2 {
                            x: zombie_translation.x,
                            y: zombie_translation.y,
                        };

                        // 计算点 zombie_position 到射线 plant_origin direction 的距离
                        let distance_square = {
                            // 点在射线上的投影
                            let projection = (zombie_position - plant_origin).dot(direction);
                            if projection < 0.0 {
                                // 投影超出射线，距离为到端点的距离
                                zombie_position.distance_squared(plant_origin)
                            } else {
                                // 投影在射线上，找到垂点
                                let q = plant_origin + projection * direction;
                                // 距离为点到垂点的距离
                                zombie_position.distance_squared(q)
                            }
                        };

                        // 距离足够短则发射子弹
                        distance_square < 50.0 * 50.0
                    })
                }
                Some(PlantDetect::Screen) => !zombies.is_empty(),
                None => return,
            };

            if found_enemy {
                *shoot = PlantShootTag::Shoot;
            } else {
                *shoot = PlantShootTag::Standby;
            }
        });
}

// 移除超出范围的投掷物
pub(crate) fn remove_outrange_projectile(
    mut commands: Commands,
    projectiles: Query<(Entity, &GlobalTransform), With<ProjectileTag>>,
) {
    for (entity, transform) in &projectiles {
        let translation = transform.translation();
        let position = Vec2 {
            x: translation.x,
            y: translation.y,
        };

        let screen_rect = Rect {
            min: Vec2 {
                x: -500.0,
                y: -400.0,
            },
            max: Vec2 { x: 500.0, y: 400.0 },
        };

        if !screen_rect.contains(position) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// 刷新僵尸流
pub(crate) fn update_zombie_wave(
    mut commands: Commands,
    time: Res<Time>,
    mut zombie_wave_controller: ResMut<ZombieWaveController>,
    settings: Res<GameSceneSettings>,
    zombies: Query<
        (),
        Or<(
            (With<ZombieAttackableTag>, Without<ZombieCriticalTag>),
            With<ToSpawnZombie>,
        )>,
    >,
    zombie_registry: Res<ZombieRegistry>,
    zombie_solts: Query<(Entity, &ZombieSolt)>,
) {
    if zombie_wave_controller.next_wave_index >= settings.zombie_waves.len() {
        return;
    }

    zombie_wave_controller.next_wave_timer.tick(time.delta());
    if !zombie_wave_controller.next_wave_timer.just_finished() {
        // 所有僵尸死亡后，就不再关心计时器，直接转到下一波
        // 但需要排除：
        //  - 第一波，因为玩家需要先发展一段时间
        //  - 已经触发大波僵尸到达的提示，这里固定等10秒
        if zombie_wave_controller.next_wave_index == 0 || zombie_wave_controller.trigger_huge_wave {
            return;
        }

        if !zombies.is_empty() {
            return;
        }

        debug!("wave controller: all zombie cleaned, next wave")
    } else {
        debug!("wave controller: times up, next wave");
    }

    // 一大波僵尸正在接近！
    if !zombie_wave_controller.trigger_huge_wave
        && settings
            .zombie_waves
            .get(zombie_wave_controller.next_wave_index)
            .map(|wave| matches!(wave.wave_type, WaveType::HugeWave))
            .unwrap_or(false)
    {
        zombie_wave_controller.trigger_huge_wave = true;

        zombie_wave_controller.next_wave_timer =
            Timer::new(Duration::from_secs_f32(10.0), TimerMode::Once);

        commands.spawn((
            GameTimer(Timer::from_seconds(0.0, TimerMode::Once)),
            GameTimerTag::TextHugeWave,
            SceneTag,
        ));
        commands.spawn((
            GameTimer(Timer::from_seconds(0.0, TimerMode::Once)),
            GameTimerTag::SoundHugeWave,
            SceneTag,
        ));
        commands.spawn((
            GameTimer(Timer::from_seconds(10.0, TimerMode::Once)),
            GameTimerTag::SoundHugeWaveZombieReached,
            SceneTag,
        ));

        return;
    }

    zombie_wave_controller.trigger_huge_wave = false;

    // 第一波
    if zombie_wave_controller.next_wave_index == 0 {
        commands.spawn((
            GameTimer(Timer::from_seconds(0.0, TimerMode::Once)),
            GameTimerTag::SoundZombieReached,
            SceneTag,
        ));
        commands.spawn((
            GameTimer(Timer::from_seconds(0.0, TimerMode::Once)),
            GameTimerTag::ShowLevelProgress,
            SceneTag,
        ));
    }

    // 最后一波
    if zombie_wave_controller.next_wave_index == settings.zombie_waves.len() - 1 {
        debug!("wave controller: final wave");
        commands.spawn((
            GameTimer(Timer::from_seconds(0.0, TimerMode::Once)),
            GameTimerTag::TextFinalWave,
            SceneTag,
        ));
        commands.spawn((
            GameTimer(Timer::from_seconds(1.0, TimerMode::Once)),
            GameTimerTag::SoundFinalWave,
            SceneTag,
        ));
    }

    // 波次数据
    let Some(wave) = settings
        .zombie_waves
        .get(zombie_wave_controller.next_wave_index)
    else {
        return;
    };

    // 生成僵尸
    debug!("wave controller: summon zombies: {:?}", wave.zombies);
    let mut rng = thread_rng();
    for (zombie_type, count) in &wave.zombies {
        for _ in 0..*count {
            // 僵尸信息
            let zombie_info = zombie_registry.get(zombie_type).unwrap();
            // 所有可选择的生成点位
            let solts = zombie_solts
                .iter()
                .filter(|(_, solt)| match solt.solt_type {
                    SoltType::Dirt => zombie_info.summon_on.dirt,
                    SoltType::River => zombie_info.summon_on.river,
                    SoltType::Roof => zombie_info.summon_on.roof,
                })
                .collect::<Vec<_>>();
            // 随机一个生成僵尸的点位
            let Some((entity, _)) = solts.choose(&mut rng) else {
                break;
            };

            // 随机延迟 0~5秒
            let delay = if zombie_info.summon_delay {
                rng.gen_range(0.0..5.0)
            } else {
                0.0
            };

            // 生成
            commands.spawn((
                ToSpawnZombie {
                    timer: Timer::new(Duration::from_secs_f32(delay), TimerMode::Once),
                    zombie_type: *zombie_type,
                    zombie_solt: *entity,
                },
                SceneTag,
            ));
        }
    }

    // 下一波
    zombie_wave_controller.next_wave_index += 1;

    // 下一波计时
    zombie_wave_controller.next_wave_timer =
        Timer::new(Duration::from_secs_f32(30.0), TimerMode::Once);
}

// 更新关卡进度条
pub(crate) fn update_level_progress(
    settings: Res<GameSceneSettings>,
    zombie_wave_controller: Res<ZombieWaveController>,
    mut level_progress: Query<&mut Sprite, With<LevelProgressProgressTag>>,
    time: Res<Time>,
) {
    // 旗帜数量
    let flag_count = settings
        .zombie_waves
        .iter()
        .filter(|wave| matches!(wave.wave_type, WaveType::HugeWave))
        .count()
        .max(1);
    let split_part = 158.0 / flag_count as f32;

    // 当前位置
    let huge_wave = {
        let mut count = 0;
        for i in 0..zombie_wave_controller.next_wave_index {
            if matches!(settings.zombie_waves[i].wave_type, WaveType::HugeWave) {
                count += 1;
            }
        }
        count
    };
    let wave_after_huge = {
        let mut count = 0;
        for i in 0..zombie_wave_controller.next_wave_index {
            let i = zombie_wave_controller.next_wave_index - i - 1;
            if matches!(settings.zombie_waves[i].wave_type, WaveType::HugeWave) {
                break;
            }
            count += 1;
        }
        count
    };
    let wave_before_huge = {
        let mut count = 0;
        for i in zombie_wave_controller.next_wave_index..settings.zombie_waves.len() {
            if matches!(settings.zombie_waves[i].wave_type, WaveType::HugeWave) {
                break;
            }
            count += 1;
        }
        count
    };

    // 进度计算
    let mut progress = huge_wave as f32
        + (wave_after_huge + 1) as f32 / (wave_before_huge + wave_after_huge + 2) as f32;
    progress = split_part * progress - 15.;
    progress = progress.min(158.).max(0.);

    // special case: 没有huge wave时，最后一波展示会有问题，特殊处理下
    if zombie_wave_controller.next_wave_index == settings.zombie_waves.len() {
        progress = 158.;
    }

    // 应用（限制移动速度）
    level_progress.iter_mut().for_each(|mut sprite| {
        if let Some(rect) = &mut sprite.rect {
            rect.min.x = (158.0 - progress).max(rect.min.x - time.delta().as_secs_f32() * 20.0);
        }
    });
}

// 更新关卡进度条（僵尸头）
pub(crate) fn update_level_progress_head(
    level_progress: Query<&Sprite, With<LevelProgressProgressTag>>,
    mut level_progress_head: Query<&mut Transform, With<LevelProgressHeadTag>>,
) {
    let Some(progress) = level_progress
        .iter()
        .filter_map(|sprite| sprite.rect)
        .map(|rect| rect.min.x)
        .next()
    else {
        return;
    };

    level_progress_head
        .iter_mut()
        .for_each(|mut transform| transform.translation.x = 390. - 158. + progress);
}

// 更新关卡进度条（旗帜）
pub(crate) fn update_level_progress_flag(
    mut commands: Commands,
    level_progress: Query<&Sprite, With<LevelProgressProgressTag>>,
    mut level_progress_flag: Query<(
        Entity,
        &mut LevelProgressFlagTag,
        &Transform,
        &GlobalTransform,
    )>,
) {
    let Some(progress) = level_progress
        .iter()
        .filter_map(|sprite| sprite.rect)
        .map(|rect| rect.min.x)
        .next()
    else {
        return;
    };
    let x = 390. - 158. + progress - 15.;
    for (entity, mut flag_tag, transform, global_transform) in &mut level_progress_flag {
        if matches!(*flag_tag, LevelProgressFlagTag::Reach) {
            continue;
        }
        if global_transform.translation().x < x {
            continue;
        }
        *flag_tag = LevelProgressFlagTag::Reach;

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
                            time: Duration::from_secs_f32(1.0),
                            transform: Some({
                                let mut new_transform = *transform;
                                new_transform.translation.y += 10.0;
                                new_transform
                            }),
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

// 僵尸吃植物 伤害结算
pub(crate) fn update_zombie_eat(
    time: Res<Time>,
    zombies: Query<(&ZombieMetadata, &ZombieEatTag), Without<ZombieCriticalTag>>,
    mut plants: Query<&mut PlantHp, With<PlantTag>>,
) {
    let delta = time.delta().as_secs_f32();
    for (ZombieMetadata(metadata), eat_tag) in &zombies {
        let Ok(mut plant_hp) = plants.get_mut(eat_tag.target_plant) else {
            continue;
        };

        plant_hp.0 -= metadata.attack * delta;
    }
}

// 植物死亡
pub(crate) fn update_plant_die(
    mut commands: Commands,
    plants: Query<(Entity, &PlantHp, &PlantTag)>,
    mut plant_solt: Query<&mut PlantSolt>,
) {
    for (entity, hp, plant_tag) in &plants {
        if hp.0 > 0.0 {
            continue;
        };

        // 移除植物
        commands.entity(entity).despawn_recursive();

        // 清除对应植物槽标记
        let Ok(mut solt) = plant_solt.get_mut(plant_tag.solt) else {
            continue;
        };

        solt.plants_mut()
            .into_iter()
            .filter(|plant| {
                plant
                    .map(|plant_entity| plant_entity == entity)
                    .unwrap_or(false)
            })
            .for_each(|plant| *plant = None);
    }
}

// 僵尸啃食逻辑
pub(crate) fn update_zombie_eat_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut zombies: Query<&mut ZombieEatTag, Without<ZombieCriticalTag>>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = thread_rng();
    for mut eat_tag in &mut zombies {
        // 计时器
        eat_tag.timer.tick(time.delta());
        if !eat_tag.timer.just_finished() {
            continue;
        }

        // 播放音效
        spawn_se(
            &mut commands,
            &asset_server,
            ["sounds/chomp.ogg", "sounds/chomp2.ogg"]
                .choose(&mut rng)
                .unwrap(),
        );

        // 植物受击闪烁
        if let Some(mut plant) = commands.get_entity(eat_tag.target_plant) {
            plant.try_insert(AnimHitTag);
        }
    }
}

// 延迟生成僵尸
pub(crate) fn update_summon_zombie(
    mut commands: Commands,
    mut to_spawn_zombie: Query<(Entity, &mut ToSpawnZombie)>,
    time: Res<Time>,
    zombie_registry: Res<ZombieRegistry>,
    zombie_solts: Query<(&LanePosition, &GlobalTransform), With<ZombieSolt>>,
) {
    for (entity, mut to_spawn_zombie) in &mut to_spawn_zombie {
        // 计时器
        to_spawn_zombie.timer.tick(time.delta());
        if !to_spawn_zombie.timer.just_finished() {
            continue;
        }

        // 移除计时器
        commands.entity(entity).despawn_recursive();

        // 僵尸信息
        let Some(zombie_info) = zombie_registry.get(&to_spawn_zombie.zombie_type) else {
            continue;
        };

        // 查找生成点
        let Ok((lane_position, global_transform)) = zombie_solts.get(to_spawn_zombie.zombie_solt)
        else {
            continue;
        };

        // 生成位置
        let translation = global_transform.translation();

        // 生成
        let mut zombie_bundle = ZombieBundle::new(zombie_info.clone());
        zombie_bundle.spine.transform =
            Transform::from_xyz(translation.x, translation.y, 15.0 - translation.y * 0.001);
        commands.spawn((
            zombie_bundle,
            AnimStandbyTag,
            AnimZombieMoveTag,
            SceneTag,
            ZombieTag,
            ZombieAttackableTag,
            MoveVelocity(Vec2 {
                x: -zombie_info.speed,
                y: 0.0,
            }),
            ZombieHpAnim::default(),
            LanePosition {
                lane: lane_position.lane,
                x: translation.x,
            },
        ));
    }
}

// 检查植物种子是否可以使用
pub(crate) fn check_plant_seed_usable(
    sunshine: Res<Sunshine>,
    mut seeds: Query<
        (&mut PlantUsable, &PlantMetaData, &PlantCooldown),
        (With<PlantSeed>, With<PickableSeed>),
    >,
) {
    seeds
        .par_iter_mut()
        .for_each(|(mut usable, PlantMetaData(plant_info), cooldown)| {
            if sunshine.0 >= plant_info.sunshine && matches!(cooldown, PlantCooldown::Ready) {
                *usable = PlantUsable::Usable;
            } else {
                *usable = PlantUsable::Unusable;
            }
        });
}

// 更新阳光逻辑
pub(crate) fn update_sunshine(
    time: Res<Time>,
    mut sunshine: Query<(&SunshineTag, &mut Transform)>,
) {
    sunshine
        .par_iter_mut()
        .for_each(|(sunshine, mut transform)| {
            transform.translation.y =
                (transform.translation.y - time.delta().as_secs_f32() * 50.).max(sunshine.target_y);
        });
}

// 自然生产阳光
pub(crate) fn update_natural_sunshine(
    mut commands: Commands,
    time: Res<Time>,
    mut sunshine_solt: Query<&mut NaturalSunshineSolt>,
    asset_server: Res<AssetServer>,
    mut skeletions: ResMut<Assets<SkeletonData>>,
    mut sunshine_skeleton: Local<Option<Handle<SkeletonData>>>,
) {
    if sunshine_skeleton.is_none() {
        *sunshine_skeleton = Some(skeletions.add(SkeletonData::new_from_binary(
            asset_server.load("reanim-spine/sun.skel"),
            asset_server.load("reanim-spine/sun.atlas"),
        )));
    }
    let Some(sunshine_skeleton) = &*sunshine_skeleton else {
        return;
    };

    let mut rng = thread_rng();

    for mut solt in &mut sunshine_solt {
        solt.next_sunshine_timer.tick(time.delta());

        if !solt.next_sunshine_timer.just_finished() {
            continue;
        }

        solt.next_sunshine_timer = Timer::new(
            Duration::from_secs_f32(rng.gen_range(7.0..15.0)),
            TimerMode::Once,
        );

        debug!("summon natural sun");
        commands.spawn((
            SpineBundle {
                skeleton: sunshine_skeleton.clone(),
                transform: Transform::from_xyz(rng.gen_range(-350.0..350.0), 300.0, 30.0),
                ..Default::default()
            },
            SunshineTag {
                target_y: rng.gen_range(-200.0..200.0),
                count: 25,
            },
            SceneTag,
            AnimStandbyTag,
            ToDespawn(Timer::new(Duration::from_secs_f32(30.), TimerMode::Once)),
        ));
    }
}

// 收集阳光
pub(crate) fn collect_sunshine(
    mut commands: Commands,
    mut sunshine: ResMut<Sunshine>,
    mut sunshines: Query<(
        Entity,
        &mut Spine,
        &SunshineTag,
        &Transform,
        &GlobalTransform,
    )>,
    cursor_position: Res<CursorPosition>,
    mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    // 点击的阳光
    let Some((sunshine_entity, mut spine, sunshine_tag, transform)) = sunshines
        .iter_mut()
        .filter_map(|(entity, spine, sunshine, transform, global_transform)| {
            let translation = global_transform.translation();
            let position = Vec2 {
                x: translation.x,
                y: translation.y,
            };

            let distance_square = position.distance_squared(cursor_position.world_position);
            if distance_square > 55.0 * 55.0 {
                return None;
            }
            Some((entity, spine, sunshine, transform))
        })
        .next()
    else {
        return;
    };

    // 如果点到了阳光，清除点击事件
    mouse_button_input.clear_just_pressed(MouseButton::Left);

    // 移除阳光标记，动画移动到左上角
    let target_transform = Transform::from_xyz(-370. + 40., 300. - 35., transform.translation.z);
    commands
        .entity(sunshine_entity)
        .remove::<(SunshineTag, ToDespawn)>()
        .insert(AnimationBundle {
            animation_clips: AnimationClips(vec![AnimationClip {
                entity: sunshine_entity,
                keyframes: vec![
                    KeyFrame {
                        time: Duration::ZERO,
                        transform: Some(*transform),
                        ..Default::default()
                    },
                    KeyFrame {
                        time: Duration::from_secs_f32(0.5),
                        transform: Some(target_transform),
                        ..Default::default()
                    },
                ],
            }]),
            ..Default::default()
        });

    // 消失淡出动画
    play_anim!(spine, INDEX_SUN_FADE_OUT, NAME_SUN_FADE_OUT, false, 0.0);

    // 阳光记数
    sunshine.0 += sunshine_tag.count;

    // 音效
    spawn_se(&mut commands, &asset_server, "sounds/points.ogg");
}

// 触发清理车
pub(crate) fn trigger_cleanup_car(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cars: Query<(Entity, &LanePosition), (With<CleanerCar>, Without<BootCleanerCar>)>,
    zombies: Query<&LanePosition, (With<ZombieTag>, Without<ZombieCriticalTag>)>,
) {
    for (entity, car_position) in &cars {
        // 是否有僵尸进入范围内
        if !zombies.iter().any(|zombie_position| {
            car_position.lane == zombie_position.lane && car_position.x + 40. >= zombie_position.x
        }) {
            continue;
        }

        debug!("clean car boot, lane: {}", car_position.lane);

        // 清理车启动
        commands
            .entity(entity)
            .insert((BootCleanerCar, MoveVelocity(Vec2 { x: 200.0, y: 0.0 })));

        // 播放音效
        spawn_se(&mut commands, &asset_server, "sounds/lawnmower.ogg");
    }
}

// 清理车杀僵尸
pub(crate) fn cleanup_car_kill_zombie(
    cars: Query<&LanePosition, With<BootCleanerCar>>,
    mut zombies: Query<(&mut ZombieHp, &LanePosition), With<ZombieTag>>,
) {
    for car_position in &cars {
        zombies
            .par_iter_mut()
            .for_each(|(mut hp, zombie_position)| {
                if zombie_position.lane == car_position.lane
                    && (zombie_position.x - car_position.x).abs() < 40.0
                {
                    hp.hp = 0.0;
                    hp.armor_hp.fill(0.0);
                }
            });
    }
}

// 清理超出屏幕的车
pub(crate) fn remove_outrange_car(
    mut commands: Commands,
    cars: Query<(Entity, &LanePosition), With<BootCleanerCar>>,
) {
    for (entity, position) in &cars {
        if position.x > 600.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// 游戏结束
pub(crate) fn check_game_over(
    zombies: Query<&LanePosition, (With<ZombieTag>, Without<ZombieCriticalTag>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let gameover = zombies.iter().any(|position| position.x < -450.0);
    if !gameover {
        return;
    }
    debug!("gameover: zombie win");
    next_state.set(GameState::Fail);
}

// 更新奖励槽位置
pub(crate) fn update_reward_solt(
    mut reward_solt: Query<&mut Transform, With<RewardSolt>>,
    zombies: Query<&GlobalTransform, (With<ZombieTag>, Without<ZombieCriticalTag>)>,
) {
    let Some(zombie_transform) = zombies.iter().next() else {
        return;
    };

    for mut transform in &mut reward_solt {
        transform.translation = zombie_transform.translation();
    }
}

// 检查并生成关卡奖励
pub(crate) fn check_summon_reward(
    mut commands: Commands,
    reward_solt: Query<(Entity, &GlobalTransform), With<RewardSolt>>,
    zombie_wave_controller: Res<ZombieWaveController>,
    settings: Res<GameSceneSettings>,
    zombies: Query<
        (),
        Or<(
            (With<ZombieAttackableTag>, Without<ZombieCriticalTag>),
            With<ToSpawnZombie>,
        )>,
    >,
    plant_registry: Res<PlantRegistry>,
) {
    // 必须存在奖励槽
    let Some((reward_solt_entity, reward_solt_transform)) = reward_solt.iter().next() else {
        return;
    };

    // 所有波次必须均已释放
    if zombie_wave_controller.next_wave_index < settings.zombie_waves.len() {
        return;
    }

    // 必须无存活僵尸
    if !zombies.is_empty() {
        return;
    }

    debug!("gameover: plant win");

    // 销毁奖励槽，避免重复创建奖励
    commands.entity(reward_solt_entity).despawn_recursive();

    // 生成奖励
    let reward_start_translation = reward_solt_transform.translation();
    match settings.reward {
        Reward::PlantSeed(plant_type) => commands
            .spawn((
                PlantSeedBundle {
                    transform: Transform::from_translation(Vec3 {
                        x: reward_start_translation.x,
                        y: reward_start_translation.y,
                        z: 60.0,
                    }),
                    ..PlantSeedBundle::new(plant_registry.get(&plant_type).unwrap().clone())
                },
                RewardTag,
                SceneTag,
            ))
            .id(),
        Reward::MoneyBag => todo!(),
    };
}

pub(crate) fn input_pick_reward_seed(
    seed: Query<&SeedHover, (With<PlantSeed>, With<RewardTag>)>,
    mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // 左键点击
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    // 点击的种子卡
    let Some(_) = seed
        .iter()
        .filter(|hover| matches!(hover, SeedHover::Hover))
        .next()
    else {
        return;
    };

    // 如果点到了种子卡，则清除点击事件
    mouse_button_input.clear_just_pressed(MouseButton::Left);

    // 结束状态
    next_state.set(GameState::Exit);
}

// 更新材质alpha
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