use bevy::prelude::*;

pub mod components;
mod systems;

pub struct FwButtonPlugin;

impl Plugin for FwButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, systems::update_background)
            .add_systems(PreUpdate, systems::update_interaction);
    }
}
