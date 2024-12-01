use bevy::prelude::*;
use consts::anim::{INDEX_HIT, INDEX_STANDBY, NAME_HIT, NAME_STANDBY};

use crate::components::{AnimHitTag, AnimStandbyPlayingTag, AnimStandbyTag};

#[macro_export]
macro_rules! play_anim {
    ($spine:expr, $index:expr, $name:expr, $looping:expr, $delay:expr) => {
        match $spine
            .animation_state
            .add_animation_by_name($index, $name, $looping, $delay)
        {
            Ok(_) => (),
            Err(e) => debug!("play spine anim err: {}, anim name: {}", e, $name),
        }
    };
}

#[macro_export]
macro_rules! looping_anim {
    ($tag:ty, $inner_tag:ty, $start_anim:ident, $stop_anim:ident, $index:expr, $name:expr) => {
        pub(crate) fn $start_anim(
            mut commands: ::bevy::prelude::Commands,
            mut targets: Query<
                (::bevy::prelude::Entity, &mut ::bevy_spine::prelude::Spine),
                (
                    ::bevy::prelude::With<$tag>,
                    ::bevy::prelude::Without<$inner_tag>,
                ),
            >,
        ) {
            for (entity, mut spine) in &mut targets {
                $crate::play_anim!(spine, $index, $name, true, 0.0);
                commands.entity(entity).try_insert(<$inner_tag>::default());
            }
        }

        pub(crate) fn $stop_anim(
            mut commands: ::bevy::prelude::Commands,
            mut targets: Query<
                (::bevy::prelude::Entity, &mut ::bevy_spine::prelude::Spine),
                (
                    ::bevy::prelude::Without<$tag>,
                    ::bevy::prelude::With<$inner_tag>,
                ),
            >,
        ) {
            for (entity, mut spine) in &mut targets {
                spine.animation_state.clear_track($index);
                commands.entity(entity).remove::<$inner_tag>();
            }
        }
    };
}

#[macro_export]
macro_rules! oneshot_anim {
    ($tag:ty, $start_anim:ident, $index:expr, $name:expr) => {
        pub(crate) fn $start_anim(
            mut commands: ::bevy::prelude::Commands,
            mut targets: Query<
                (::bevy::prelude::Entity, &mut ::bevy_spine::prelude::Spine),
                ::bevy::prelude::With<$tag>,
            >,
        ) {
            for (entity, mut spine) in &mut targets {
                $crate::play_anim!(spine, $index, $name, false, 0.0);
                commands.entity(entity).remove::<$tag>();
            }
        }
    };
}

looping_anim!(
    AnimStandbyTag,
    AnimStandbyPlayingTag,
    start_standby_anim,
    stop_standby_anim,
    INDEX_STANDBY,
    NAME_STANDBY
);

oneshot_anim!(AnimHitTag, start_hit_anim, INDEX_HIT, NAME_HIT);
