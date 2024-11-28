use std::{fs::File, sync::Arc};

use bevy::prelude::*;
use bevy_spine::SkeletonData;
use metadata::{ZombieRegistry, Zombies};

pub mod components;
pub mod metadata;
mod systems;

pub struct ModZombiePlugin;

impl Plugin for ModZombiePlugin {
    fn build(&self, app: &mut App) {
        let zombie_registry = load_registry(app.world_mut());

        app.insert_resource(zombie_registry).add_systems(
            Update,
            (
                systems::start_move_anim,
                systems::stop_move_anim,
                systems::start_eat_anim,
                systems::stop_eat_anim,
                systems::start_eat_stop_anim,
                systems::start_half_damage_anim,
                systems::start_full_damage_anim,
                systems::start_critical_anim,
            ),
        );
    }
}

fn load_registry(world: &mut World) -> ZombieRegistry {
    let zombies: Zombies =
        serde_xml_rs::from_reader(File::open("./assets/data/zombies.xml").unwrap()).unwrap();
    debug!("loaded zombie infos: {zombies:?}");

    ZombieRegistry(
        zombies
            .zombies
            .into_iter()
            .map(|mut zombie| {
                let asset_server = world.get_resource::<AssetServer>().unwrap();
                let skeleton = asset_server.load(&zombie.render.skeleton);
                let atlas = asset_server.load(&zombie.render.atlas);

                let mut skeletons = world.get_resource_mut::<Assets<SkeletonData>>().unwrap();
                zombie.render.spine_skeleton =
                    skeletons.add(SkeletonData::new_from_binary(skeleton, atlas));
                (zombie.id, Arc::new(zombie))
            })
            .collect(),
    )
}
