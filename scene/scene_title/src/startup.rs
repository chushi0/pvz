use bevy::prelude::*;
use fw_ftxm::{FtxmSource, MainMusicTable};

use crate::tag::SceneTag;

pub(crate) fn start_bgm(mut commands: Commands, target: Query<Entity, With<FtxmSource>>) {
    // splash不会结束音乐，如果其已经存在，则复用
    for entity in &target {
        commands.entity(entity).insert(SceneTag);
    }

    // 如果没有，则新建
    if target.is_empty() {
        commands.spawn((
            FtxmSource {
                pot: MainMusicTable::Title.into(),
            },
            SceneTag,
        ));
    }
}
