use bevy::prelude::*;

pub mod components;
mod systems;

pub struct FwActorPlugin;

impl Plugin for FwActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                systems::start_standby_anim,
                systems::stop_standby_anim,
                systems::start_hit_anim,
            ),
        );
    }
}
