use bevy::prelude::*;
use libxm::XMContext;
use rodio::OutputStream;
use rodio::OutputStreamHandle;
use rodio::Sink;
use rodio::Source;
use std::fs::File;
use std::io::Read;
use std::mem::forget;
use std::u8;

pub struct FwFtxmPlugin;

#[derive(Component)]
pub struct FtxmAudioSink {
    pub sink: Sink,
}

#[derive(Component)]
pub struct FtxmSource {
    pub pot: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum MainMusicTable {
    // 白天 0x00
    Grasswalk,
    // 黑夜 0x30
    Moongrains,
    // 泳池 0x5E
    WateryGraves,
    // 选卡 0x7A
    ChooseYourSeeds,
    // 迷雾 0x7D
    RigorMormist,
    // 标题 0x98
    Title,
    // BOSS战 0x9E
    BrainiacManiac,
    // 小游戏 0xA6
    Loonboon,
    // 解谜 0xB1
    Cerebrawl,
    // 屋顶 0xB8
    GrazeTheRoof,
    // X-10 0xD4
    UltimateBattle,
    // 禅镜花园 0xDD
    ZenGarden,
}

struct FtxmPlaybackSource {
    context: XMContext,
    first_loop: bool,
    last_buffer: [f32; 4096],
    last_buffer_index: usize,
    last_buffer_len: usize,
}

#[derive(Resource)]
struct AudioOutput {
    handle: Option<OutputStreamHandle>,
}

impl Plugin for FwFtxmPlugin {
    fn build(&self, app: &mut App) {
        let audio_output_handle = if let Ok((stream, handle)) = OutputStream::try_default() {
            forget(stream);
            Some(handle)
        } else {
            None
        };

        app.insert_resource(AudioOutput {
            handle: audio_output_handle,
        })
        .add_systems(Update, start_playback);
    }
}

impl FtxmPlaybackSource {
    pub fn new(pot: u8) -> Self {
        let mut data = Vec::new();
        File::open("./assets/sounds/mainmusic.xm")
            .unwrap()
            .read_to_end(&mut data)
            .unwrap();

        let mut xm = XMContext::new(&data, 48000).unwrap();
        xm.seek(pot, 0, 0);
        xm.set_max_loop_count(1);
        for i in 21..=30 {
            xm.mute_channel(i, true);
        }

        Self {
            context: xm,
            first_loop: true,
            last_buffer: [0.0; 4096],
            last_buffer_index: 0,
            last_buffer_len: 0,
        }
    }
}

impl Source for FtxmPlaybackSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        48000
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl Iterator for FtxmPlaybackSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last_buffer_index >= self.last_buffer_len {
            if self.first_loop && self.context.loop_count() != 0 {
                self.context.set_max_loop_count(0);
                for i in 21..=30 {
                    self.context.mute_channel(i, false);
                }
                self.first_loop = false;
            }

            self.last_buffer_index = 0;
            self.last_buffer_len = self.context.generate_samples(&mut self.last_buffer);
        }

        let res = Some(self.last_buffer[self.last_buffer_index]);
        self.last_buffer_index += 1;
        res
    }
}

fn start_playback(
    mut commands: Commands,
    audio_output: Res<AudioOutput>,
    source: Query<(Entity, &FtxmSource), Without<FtxmAudioSink>>,
) {
    let Some(handle) = &audio_output.handle else {
        return;
    };

    for (entity, source) in &source {
        let Ok(sink) = Sink::try_new(handle) else {
            continue;
        };

        sink.append(FtxmPlaybackSource::new(source.pot));
        commands.entity(entity).insert(FtxmAudioSink { sink });
    }
}

impl From<MainMusicTable> for u8 {
    fn from(value: MainMusicTable) -> Self {
        match value {
            MainMusicTable::Grasswalk => 0x00,
            MainMusicTable::Moongrains => 0x30,
            MainMusicTable::WateryGraves => 0x5E,
            MainMusicTable::ChooseYourSeeds => 0x7A,
            MainMusicTable::RigorMormist => 0x7D,
            MainMusicTable::Title => 0x98,
            MainMusicTable::BrainiacManiac => 0x9E,
            MainMusicTable::Loonboon => 0xA6,
            MainMusicTable::Cerebrawl => 0xB1,
            MainMusicTable::GrazeTheRoof => 0xB8,
            MainMusicTable::UltimateBattle => 0xD4,
            MainMusicTable::ZenGarden => 0xDD,
        }
    }
}
