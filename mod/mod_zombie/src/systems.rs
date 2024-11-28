use bevy::prelude::*;
use consts::anim::{
    INDEX_ZOMBIE_CRITICAL, INDEX_ZOMBIE_EAT, INDEX_ZOMBIE_EAT_STOP, INDEX_ZOMBIE_FULL_DAMAGE,
    INDEX_ZOMBIE_HALF_DAMAGE, INDEX_ZOMBIE_MOVE, NAME_ZOMBIE_CRITICAL, NAME_ZOMBIE_EAT,
    NAME_ZOMBIE_EAT_STOP, NAME_ZOMBIE_FULL_DAMAGE, NAME_ZOMBIE_HALF_DAMAGE, NAME_ZOMBIE_MOVE,
};
use fw_actor::{looping_anim, oneshot_anim};

use crate::components::{
    AnimZombieCriticalTag, AnimZombieEatPlayingTag, AnimZombieEatStopTag, AnimZombieEatTag,
    AnimZombieFullDamageTag, AnimZombieHalfDamageTag, AnimZombieMovePlayingTag, AnimZombieMoveTag,
};

looping_anim!(
    AnimZombieMoveTag,
    AnimZombieMovePlayingTag,
    start_move_anim,
    stop_move_anim,
    INDEX_ZOMBIE_MOVE,
    NAME_ZOMBIE_MOVE
);

looping_anim!(
    AnimZombieEatTag,
    AnimZombieEatPlayingTag,
    start_eat_anim,
    stop_eat_anim,
    INDEX_ZOMBIE_EAT,
    NAME_ZOMBIE_EAT
);

oneshot_anim!(
    AnimZombieEatStopTag,
    start_eat_stop_anim,
    INDEX_ZOMBIE_EAT_STOP,
    NAME_ZOMBIE_EAT_STOP
);

oneshot_anim!(
    AnimZombieHalfDamageTag,
    start_half_damage_anim,
    INDEX_ZOMBIE_HALF_DAMAGE,
    NAME_ZOMBIE_HALF_DAMAGE
);

oneshot_anim!(
    AnimZombieFullDamageTag,
    start_full_damage_anim,
    INDEX_ZOMBIE_FULL_DAMAGE,
    NAME_ZOMBIE_FULL_DAMAGE
);

oneshot_anim!(
    AnimZombieCriticalTag,
    start_critical_anim,
    INDEX_ZOMBIE_CRITICAL,
    NAME_ZOMBIE_CRITICAL
);
