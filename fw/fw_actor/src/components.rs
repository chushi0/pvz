use bevy::prelude::*;

#[derive(Component)]
pub struct AnimStandbyTag;

#[derive(Component)]
pub struct AnimHitTag;

#[derive(Default, Component)]
pub(crate) struct AnimStandbyPlayingTag;
