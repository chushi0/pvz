use std::{any::TypeId, time::Duration};

use bevy::{prelude::*, utils::HashMap};

pub struct FwAnimPlugin;

/// 一个动画可能会同时操作许多entity，此component存储所有相关信息
#[derive(Component, Default)]
pub struct AnimationClips(pub Vec<AnimationClip>);

/// 动画播放状态，改为Init可让动画重新播放
#[derive(Component, Default)]
pub enum AnimationState {
    #[default]
    Init,
    Playing(Duration),
    Stop,
}

#[derive(Component, Default)]
pub enum AnimationAutoCleanup {
    None,
    #[default]
    AutoCleanup,
}

/// 在场景中spawn该bundle即可播放动画
#[derive(Bundle, Default)]
pub struct AnimationBundle {
    pub animation_clips: AnimationClips,
    pub animation_state: AnimationState,
    pub auto_cleanup: AnimationAutoCleanup,
}

/// 管理一个entity的动画数据
pub struct AnimationClip {
    /// 目标entity
    pub entity: Entity,
    /// 关键帧序列，必须严格按照时间顺序排列，且所有需要修改的属性必须在所有关键帧中均存在
    pub keyframes: Vec<KeyFrame>,
}

/// 关键帧
#[derive(Default)]
pub struct KeyFrame {
    /// 时间（从动画开始播放计时）
    pub time: Duration,
    pub transform: Option<Transform>,
    pub custom_animation_triggers: Option<HashMap<TypeId, f32>>,
}

/// 用于自定义动画行动的组件，提供平滑过渡的数值，使得组件可以自行设计动画更新逻辑
#[derive(Component, Default)]
pub struct CustomAnimationTrigger {
    pub animation_value: HashMap<TypeId, f32>,
}

impl Plugin for FwAnimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, clean_stop_animation).add_systems(
            PostUpdate,
            update_animation.before(TransformSystem::TransformPropagate),
        );
    }
}

fn clean_stop_animation(
    mut commands: Commands,
    animations: Query<(Entity, &AnimationState, &AnimationAutoCleanup)>,
) {
    for (entity, state, auto_cleanup) in &animations {
        if matches!(
            (state, auto_cleanup),
            (AnimationState::Stop, AnimationAutoCleanup::AutoCleanup)
        ) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// 刷新动画
fn update_animation(
    time: Res<Time>,
    mut animations: Query<(&mut AnimationState, &AnimationClips)>,
    mut animation_targets: Query<AnyOf<(&mut Transform, &mut CustomAnimationTrigger)>>,
) {
    let delta = time.delta();
    for (mut animation_state, AnimationClips(animation_clips)) in &mut animations {
        // 计算动画播放时间
        let time = match &mut *animation_state {
            AnimationState::Init => {
                *animation_state = AnimationState::Playing(Duration::ZERO);
                Duration::ZERO
            }
            AnimationState::Playing(duration) => {
                *duration += delta;
                *duration
            }
            AnimationState::Stop => continue,
        };

        let mut anim_active = false;

        // 对所有影响的实体进行处理
        for animation_clip in animation_clips {
            let Ok(target) = animation_targets.get_mut(animation_clip.entity) else {
                continue;
            };

            // 当前所处的关键帧
            let Some(keyframe_index) = animation_clip
                .keyframes
                .iter()
                .enumerate()
                .filter_map(|(index, keyframe)| {
                    if keyframe.time > time {
                        Some(index)
                    } else {
                        None
                    }
                })
                .min()
            else {
                // 如果已经结束，则按照最后处理的关键帧应用效果
                apply_anim_keyframe(
                    target,
                    &animation_clip.keyframes,
                    animation_clip.keyframes.len() - 1,
                    1.,
                );
                continue;
            };

            // 动画仍在继续
            anim_active = true;

            // 应用动画效果
            let delta = if keyframe_index == 0 {
                0.0
            } else {
                (time.as_secs_f32()
                    - animation_clip.keyframes[keyframe_index - 1]
                        .time
                        .as_secs_f32())
                    / (animation_clip.keyframes[keyframe_index].time.as_secs_f32()
                        - animation_clip.keyframes[keyframe_index - 1]
                            .time
                            .as_secs_f32())
            };
            apply_anim_keyframe(target, &animation_clip.keyframes, keyframe_index, delta);
        }

        // 如果动画已经结束，则停止动画
        if !anim_active {
            *animation_state = AnimationState::Stop;
        }
    }
}

// 应用动画效果
fn apply_anim_keyframe(
    target: (Option<Mut<Transform>>, Option<Mut<CustomAnimationTrigger>>),
    keyframes: &[KeyFrame],
    index: usize,
    delta: f32,
) {
    let (transform, custom_animation_trigger) = target;

    // transform
    if let (Some(keyframe_transform), Some(mut target_transform)) =
        (&keyframes[index].transform, transform)
    {
        let last_keyframe_transform = if index > 0 {
            keyframes[index - 1].transform
        } else {
            None
        };

        match last_keyframe_transform {
            Some(last_keyframe_transform) => {
                let translation = last_keyframe_transform.translation * (1.0 - delta)
                    + keyframe_transform.translation * delta;
                let rotation = last_keyframe_transform
                    .rotation
                    .slerp(keyframe_transform.rotation, delta);
                let scale = last_keyframe_transform.scale * (1.0 - delta)
                    + keyframe_transform.scale * delta;
                *target_transform = Transform {
                    translation,
                    rotation,
                    scale,
                };
            }
            None => *target_transform = keyframe_transform.clone(),
        }
    };

    // custom animation trigger
    if let (Some(keyframe_custom_trigger), Some(mut target_custom_trigger)) = (
        &keyframes[index].custom_animation_triggers,
        custom_animation_trigger,
    ) {
        let last_keyframe_custom_trigger = if index > 0 {
            keyframes[index - 1].custom_animation_triggers.as_ref()
        } else {
            None
        };

        match last_keyframe_custom_trigger {
            Some(last_keyframe_custom_trigger) => {
                for (typeid, value) in keyframe_custom_trigger {
                    let Some(last_value) = last_keyframe_custom_trigger.get(typeid) else {
                        continue;
                    };

                    target_custom_trigger
                        .animation_value
                        .insert(*typeid, *last_value * (1. - delta) + *value * delta);
                }
            }
            None => {
                for (typeid, value) in keyframe_custom_trigger {
                    target_custom_trigger
                        .animation_value
                        .insert(*typeid, *value);
                }
            }
        }
    };
}
