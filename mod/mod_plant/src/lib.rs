use std::{fs::File, sync::Arc};

use bevy::prelude::*;
use bevy_spine::SkeletonData;
use metadata::{PlantRegistry, Plants};

pub mod components;
pub mod metadata;
mod systems;

pub struct ModPlantPlugin;

impl Plugin for ModPlantPlugin {
    fn build(&self, app: &mut App) {
        let plant_registry = load_registry(app.world_mut());

        app.insert_resource(plant_registry)
            .add_systems(PreUpdate, systems::update_seed_hover)
            .add_systems(
                Update,
                (
                    (
                        systems::setup_seeds,
                        (
                            systems::update_cooldown_overlay,
                            systems::update_usable_overlay,
                        ),
                    )
                        .chain(),
                    systems::start_shoot_anim,
                ),
            );
    }
}

fn load_registry(world: &mut World) -> PlantRegistry {
    let plants: Plants =
        serde_xml_rs::from_reader(File::open("./assets/data/plants.xml").unwrap()).unwrap();
    debug!("loaded plant infos: {plants:?}");

    PlantRegistry(
        plants
            .plants
            .into_iter()
            .map(|mut plant| {
                let asset_server = world.get_resource::<AssetServer>().unwrap();
                let skeleton = asset_server.load(&plant.render.skeleton);
                let atlas = asset_server.load(&plant.render.atlas);

                let mut skeletons = world.get_resource_mut::<Assets<SkeletonData>>().unwrap();
                plant.render.spine_skeleton =
                    skeletons.add(SkeletonData::new_from_binary(skeleton, atlas));
                (plant.id, Arc::new(plant))
            })
            .collect(),
    )
}
