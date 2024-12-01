use std::{any::TypeId, ops::Range, time::Duration};

use bevy::prelude::*;
use fw_anim::{AnimationBundle, AnimationClip, AnimationClips, CustomAnimationTrigger, KeyFrame};
use rand::{thread_rng, Rng};

use crate::tag::{CherryBombParticleTag, ColorAlphaFade, SceneTag, ToDespawn};

// 樱桃炸弹粒子效果
pub(crate) fn apply_cherry_bomb_particle(
    mut commands: Commands,
    particle: Query<(Entity, &GlobalTransform), With<CherryBombParticleTag>>,
    asset_server: Res<AssetServer>,
) {
    if particle.is_empty() {
        return;
    }

    let cloud = asset_server.load("particles/ExplosionCloud.png");
    let powie = asset_server.load("particles/ExplosionPowie.png");

    for (entity, transform) in &particle {
        commands.entity(entity).despawn_recursive();
        let translation = transform.translation();

        for _ in 0..10 {
            gen_particle(
                &mut commands,
                cloud.clone(),
                translation,
                0.5,
                Color::srgb(0.9, 0.4, 0.0),
                -130.0..130.0,
            );
        }

        for _ in 0..10 {
            gen_particle(
                &mut commands,
                cloud.clone(),
                translation,
                1.5,
                Color::srgb(0.9, 0.6, 0.0),
                -80.0..80.0,
            );
        }

        commands.spawn((
            SpriteBundle {
                texture: powie.clone(),
                transform: Transform::from_translation(Vec3 {
                    x: translation.x,
                    y: translation.y,
                    z: 40.1,
                }),
                ..Default::default()
            },
            ToDespawn(Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once)),
            SceneTag,
        ));
    }
}

fn gen_particle(
    commands: &mut Commands,
    texture: Handle<Image>,
    translation: Vec3,
    scale: f32,
    color: Color,
    range: Range<f32>,
) {
    let mut rng = thread_rng();

    let start_transform = Transform::from_scale(Vec3::ONE * scale).with_translation(Vec3 {
        x: translation.x,
        y: translation.y,
        z: 40.0,
    });
    let end_transform = Transform::from_scale(Vec3::ONE * scale).with_translation(Vec3 {
        x: translation.x + rng.gen_range(range.clone()),
        y: translation.y + rng.gen_range(range),
        z: 40.0,
    });

    let mut entity = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                ..Default::default()
            },
            texture: texture.clone(),
            transform: start_transform,
            ..Default::default()
        },
        SceneTag,
        ColorAlphaFade,
        CustomAnimationTrigger::default(),
    ));
    let entity_id = entity.id();
    entity.insert(AnimationBundle {
        animation_clips: AnimationClips(vec![AnimationClip {
            entity: entity_id,
            keyframes: vec![
                KeyFrame {
                    time: Duration::ZERO,
                    transform: Some(start_transform),
                    custom_animation_triggers: Some(
                        [(TypeId::of::<ColorAlphaFade>(), 1.0)]
                            .into_iter()
                            .collect(),
                    ),
                },
                KeyFrame {
                    time: Duration::from_secs_f32(0.25),
                    transform: Some(Transform::from_scale(Vec3::ONE * scale).with_translation(
                        (start_transform.translation + end_transform.translation) * 0.5,
                    )),
                    custom_animation_triggers: Some(
                        [(TypeId::of::<ColorAlphaFade>(), 1.0)]
                            .into_iter()
                            .collect(),
                    ),
                },
                KeyFrame {
                    time: Duration::from_secs_f32(0.5),
                    transform: Some(end_transform),
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
