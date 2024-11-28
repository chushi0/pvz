use bevy::prelude::*;

#[derive(Resource, Default)]
pub(crate) struct Sunshine(pub u32);

#[derive(Resource, Default)]
pub(crate) struct ZombieWaveController {
    pub next_wave_timer: Timer,
    pub next_wave_index: usize,
    pub trigger_huge_wave: bool,
}
