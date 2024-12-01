use bevy::{
    prelude::*,
    sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle},
    text::Text2dBounds,
};
use bevy_spine::prelude::*;
use consts::anim::{
    INDEX_PLANT_DAMAGE_1, INDEX_PLANT_DAMAGE_2, INDEX_PLANT_INSTANT, INDEX_PLANT_PRODUCE,
    INDEX_PLANT_SHOOT, NAME_PLANT_DAMAGE_1, NAME_PLANT_DAMAGE_2, NAME_PLANT_INSTANT,
    NAME_PLANT_PRODUCE, NAME_PLANT_SHOOT,
};
use fw_actor::oneshot_anim;
use fw_cursor::CursorPosition;

use crate::components::{
    AnimPlantDamage1Tag, AnimPlantDamage2Tag, AnimPlantInstantTag, AnimPlantProduceTag,
    AnimPlantShootTag, CooldownOverlay, PlantCooldown, PlantMetaData, PlantSeed, PlantUsable,
    SeedHover, SunshineVisibility, UnusedOverlay,
};

#[allow(clippy::type_complexity)]
pub(crate) fn setup_seeds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    seeds: Query<
        (Entity, &PlantMetaData, &SunshineVisibility),
        (With<PlantSeed>, Without<Children>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rectangle_mesh: Local<Option<Mesh2dHandle>>,
    mut gray_color_material: Local<Option<Handle<ColorMaterial>>>,
) {
    if rectangle_mesh.is_none() {
        *rectangle_mesh = Some(meshes.add(Rectangle::default()).into())
    }
    let Some(rectangle_mesh) = &*rectangle_mesh else {
        return;
    };

    if gray_color_material.is_none() {
        *gray_color_material = Some(materials.add(Color::srgba(0.0, 0.0, 0.0, 0.8)));
    }
    let Some(gray_color_material) = &*gray_color_material else {
        return;
    };

    for (entity, PlantMetaData(plant_info), sunshine_visibility) in &seeds {
        let seed_background = asset_server.load("images/SeedPacket_Larger.png");
        commands.entity(entity).with_children(|parent| {
            // 种子包背景
            parent.spawn(SpriteBundle {
                texture: seed_background,
                transform: Transform::from_scale(Vec3 {
                    x: 0.5,
                    y: 0.5,
                    z: 1.0,
                }),
                ..Default::default()
            });

            // 阳光文字
            if matches!(sunshine_visibility, SunshineVisibility::Visible) {
                parent.spawn(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: plant_info.sunshine.to_string(),
                            style: TextStyle {
                                font_size: 14.0,
                                color: Color::BLACK,
                                ..Default::default()
                            },
                        }],
                        ..Default::default()
                    },
                    text_anchor: Anchor::CenterRight,
                    text_2d_bounds: Text2dBounds {
                        size: Vec2 { x: 60.0, y: 18.0 },
                    },
                    transform: Transform::from_translation(Vec3 {
                        x: 6.0,
                        y: -26.0,
                        z: 0.01,
                    }),
                    ..Default::default()
                });
            }

            // 植物
            parent.spawn(SpineBundle {
                skeleton: plant_info.render.spine_skeleton.clone(),
                transform: Transform::from_scale(Vec3 {
                    x: 0.3,
                    y: 0.3,
                    z: 1.0,
                })
                .with_translation(Vec3 {
                    x: 0.0,
                    y: -5.0,
                    z: 0.01,
                }),
                ..Default::default()
            });

            // 冷却
            parent.spawn((
                MaterialMesh2dBundle {
                    mesh: rectangle_mesh.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.02).with_scale(Vec3 {
                        x: 50.0,
                        y: 70.0,
                        z: 1.0,
                    }),
                    material: gray_color_material.clone(),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
                CooldownOverlay,
            ));

            // 无法使用
            parent.spawn((
                MaterialMesh2dBundle {
                    mesh: rectangle_mesh.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.03).with_scale(Vec3 {
                        x: 50.0,
                        y: 70.0,
                        z: 1.0,
                    }),
                    material: gray_color_material.clone(),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
                UnusedOverlay,
            ));
        });
    }
}

pub(crate) fn update_seed_hover(
    mut seeds: Query<(&GlobalTransform, &mut SeedHover), With<PlantSeed>>,
    cursor_position: Res<CursorPosition>,
) {
    // clear hover
    seeds
        .par_iter_mut()
        .for_each(|(_, mut hover)| *hover = SeedHover::None);

    let hover_seed = seeds
        .iter_mut()
        .filter(|(transform, _)| {
            let translation = transform.translation();
            let translation_2d = Vec2 {
                x: translation.x,
                y: translation.y,
            };

            let size = Vec2 { x: 100.0, y: 140.0 } * 0.5;

            Rect {
                min: translation_2d - size * 0.5,
                max: translation_2d + size * 0.5,
            }
            .contains(cursor_position.world_position)
        })
        .max_by(|(transform_a, _), (transform_b, _)| {
            transform_a
                .translation()
                .z
                .partial_cmp(&transform_b.translation().z)
                .unwrap()
        });

    if let Some((_, mut hover)) = hover_seed {
        *hover = SeedHover::Hover;
    }
}

pub(crate) fn update_cooldown_overlay(
    time: Res<Time>,
    mut seeds: Query<(&mut PlantCooldown, &PlantMetaData, &Children)>,
    mut overlay: Query<(&mut Visibility, &mut Transform), With<CooldownOverlay>>,
) {
    for (mut cooldown, PlantMetaData(plant_info), children) in &mut seeds {
        *cooldown = match *cooldown {
            PlantCooldown::Ready => PlantCooldown::Ready,
            PlantCooldown::Cooldown(duration) => {
                if duration > time.delta() {
                    PlantCooldown::Cooldown(duration - time.delta())
                } else {
                    PlantCooldown::Ready
                }
            }
        };
        for entity in children {
            let Ok((mut visibility, mut transform)) = overlay.get_mut(*entity) else {
                continue;
            };

            match *cooldown {
                PlantCooldown::Ready => {
                    *visibility = Visibility::Hidden;
                }
                PlantCooldown::Cooldown(duration) => {
                    *visibility = Visibility::Inherited;

                    let percent = duration.as_secs_f32() / plant_info.cooldown;
                    transform.scale.y = 70.0 * percent;
                    transform.translation.y = 35.0 * (1.0 - percent);
                }
            }
        }
    }
}

pub(crate) fn update_usable_overlay(
    seeds: Query<(&PlantUsable, &Children)>,
    mut overlay: Query<&mut Visibility, With<UnusedOverlay>>,
) {
    for (usable, children) in &seeds {
        for entity in children {
            let Ok(mut visibility) = overlay.get_mut(*entity) else {
                continue;
            };
            *visibility = match usable {
                PlantUsable::Usable => Visibility::Hidden,
                PlantUsable::Unusable => Visibility::Inherited,
            };
        }
    }
}

oneshot_anim!(
    AnimPlantShootTag,
    start_shoot_anim,
    INDEX_PLANT_SHOOT,
    NAME_PLANT_SHOOT
);

oneshot_anim!(
    AnimPlantProduceTag,
    start_produce_anim,
    INDEX_PLANT_PRODUCE,
    NAME_PLANT_PRODUCE
);

oneshot_anim!(
    AnimPlantInstantTag,
    start_instant_anim,
    INDEX_PLANT_INSTANT,
    NAME_PLANT_INSTANT
);

oneshot_anim!(
    AnimPlantDamage1Tag,
    start_damage1_anim,
    INDEX_PLANT_DAMAGE_1,
    NAME_PLANT_DAMAGE_1
);

oneshot_anim!(
    AnimPlantDamage2Tag,
    start_damage2_anim,
    INDEX_PLANT_DAMAGE_2,
    NAME_PLANT_DAMAGE_2
);
