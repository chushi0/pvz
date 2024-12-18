use bevy::prelude::*;
use consts::anim::{
    INDEX_ZOMBIE_ARMOR_1, INDEX_ZOMBIE_ARMOR_2, INDEX_ZOMBIE_ARMOR_3, INDEX_ZOMBIE_CRITICAL,
    INDEX_ZOMBIE_EAT, INDEX_ZOMBIE_EAT_STOP, INDEX_ZOMBIE_FULL_DAMAGE, INDEX_ZOMBIE_HALF_DAMAGE,
    INDEX_ZOMBIE_MOVE, NAME_ZOMBIE_ARMOR_1, NAME_ZOMBIE_ARMOR_2, NAME_ZOMBIE_ARMOR_3,
    NAME_ZOMBIE_CRITICAL, NAME_ZOMBIE_EAT, NAME_ZOMBIE_EAT_STOP, NAME_ZOMBIE_FULL_DAMAGE,
    NAME_ZOMBIE_HALF_DAMAGE, NAME_ZOMBIE_MOVE,
};
use fw_actor::{looping_anim, oneshot_anim};

use crate::components::{
    AnimZombieArmor1Tag, AnimZombieArmor2Tag, AnimZombieArmor3Tag, AnimZombieCriticalTag,
    AnimZombieEatPlayingTag, AnimZombieEatStopTag, AnimZombieEatTag, AnimZombieFullDamageTag,
    AnimZombieHalfDamageTag, AnimZombieMovePlayingTag, AnimZombieMoveTag,
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

oneshot_anim!(
    AnimZombieArmor1Tag,
    start_armor_1_anim,
    INDEX_ZOMBIE_ARMOR_1,
    NAME_ZOMBIE_ARMOR_1
);

oneshot_anim!(
    AnimZombieArmor2Tag,
    start_armor_2_anim,
    INDEX_ZOMBIE_ARMOR_2,
    NAME_ZOMBIE_ARMOR_2
);

oneshot_anim!(
    AnimZombieArmor3Tag,
    start_armor_3_anim,
    INDEX_ZOMBIE_ARMOR_3,
    NAME_ZOMBIE_ARMOR_3
);
