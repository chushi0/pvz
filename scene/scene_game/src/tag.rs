use std::sync::Arc;

use bevy::prelude::*;
use mod_plant::metadata::PlantInfo;
use mod_zombie::metadata::ZombieType;

use crate::GameState;

// 场景中所有实体均需包含此组件，用于退出场景时进行清理
#[derive(Component)]
pub(crate) struct SceneTag;

// 游戏计时器，配合GameTimerTag进行延迟任务
#[derive(Component)]
pub(crate) struct GameTimer(pub(crate) Timer);

// 计时结束后进行操作
#[derive(Component)]
pub(crate) enum GameTimerTag {
    // 相机移动到右侧（看僵尸，选卡）
    CameraToRightAnim,
    // 相机移动到中间（战场）
    CameraToCenterAnim,
    // 相机移动到左侧（战败）
    CameraToLeftAnim,

    // 植物包
    FadeInSeedBank,
    // 选植物
    FadeInSeedChooser,
    FadeOutSeedChooser,
    // 车
    FadeInCars,
    // 游戏进程进度条
    ShowLevelProgress,

    // “准备”
    TextReady,
    // “安放”
    TextSet,
    // “植物”
    TextPlants,
    // “一大波僵尸正在接近”
    TextHugeWave,
    // “最后一波”
    TextFinalWave,
    // “僵尸吃掉了你的脑子”
    TextZombieEatYourBrain,

    // 音效 准备安放植物
    SoundReadySetPlant,
    // 音效 僵尸开始入侵
    SoundZombieReached,
    // 音效 一大波僵尸正在接近
    SoundHugeWave,
    // 音效 大波僵尸接近
    SoundHugeWaveZombieReached,
    // 音效 最后一波
    SoundFinalWave,
    // 音效 失败
    SoundLose,
    // 音效 啃食
    SoundEat,
    // 音效 失败 惊叫
    SoundScream,
    // 音效 胜利
    SoundWin,

    // 停止bgm
    StopBgm,

    // 切换游戏状态
    EnterState(GameState),
    // 升级无草坪之地
    UpgradeSod,
    // 结束本局游戏
    Finish,
    // 重启游戏
    Reset,
}

// 应用颜色动画
#[derive(Component)]
pub(crate) struct ColorAlphaFade;

// 顶部种子栏标记
#[derive(Component)]
pub(crate) struct SeedbankTag;

// 选卡栏
#[derive(Component)]
pub(crate) struct SeedChooserTag;

// 可以捡起来种的种子
#[derive(Component)]
pub(crate) struct PickableSeed;

// 阳光文本，将阳光显示到对应位置
#[derive(Component)]
pub(crate) struct SunshineText;

// 清理车标记
#[derive(Component)]
pub(crate) struct CleanerCar {
    pub lane: u8,
}

// 已启动的清理车
#[derive(Component)]
pub(crate) struct BootCleanerCar;

// 应用图片裁剪动画
#[derive(Component)]
pub(crate) struct ImageCutAnim;

// 跟随鼠标位置
#[derive(Component)]
pub(crate) struct FollowCursorTag;

// 捡起的种子
#[derive(Component)]
pub(crate) struct PickSeed {
    pub seed: Entity,
    pub plant_info: Arc<PlantInfo>,
}

/// 植物槽
#[derive(Component, Default)]
pub(crate) struct PlantSolt {
    // 主要植物
    pub primary: Option<Entity>,
    // 保护类植物（南瓜）
    pub protect: Option<Entity>,
    // 花盆类植物（花盆、睡莲）
    pub pot: Option<Entity>,
    // 临时辅助植物（咖啡豆）
    pub temp: Option<Entity>,

    // 植物槽类型
    pub solt_type: SoltType,

    // 墓碑
    pub grave: Option<Entity>,
    // （毁灭菇留下的）坑
    pub hole: Option<Entity>,
}

#[derive(Default, Clone, Copy)]
pub(crate) enum SoltType {
    #[default]
    Dirt,
    River,
    Roof,
}

// 植物
#[derive(Component)]
pub(crate) struct PlantTag {
    // 对应的植物槽
    pub solt: Entity,
    // 植物数据
    pub metadata: Arc<PlantInfo>,
}

// 在展示 level progress ui 时，位置左移
#[derive(Component)]
pub(crate) struct ShowLevelProgressShiftLeft;

// 关卡进度条，进度部分，通过裁剪图片实现进度展示
#[derive(Component)]
pub(crate) struct LevelProgressProgressTag;

// 关卡进度条上的旗帜
#[derive(Component)]
pub(crate) enum LevelProgressFlagTag {
    NotReach,
    Reach,
}

// 关卡进度条上的僵尸头
#[derive(Component)]
pub(crate) struct LevelProgressHeadTag;

// 关卡预览僵尸
#[derive(Component)]
pub(crate) struct StandbyZombieTag;

// 投掷物冷却控制
#[derive(Component)]
pub(crate) struct ProjectileCooldown {
    pub cooldown: f32,
    pub max_cooldown: f32,
}

// 生产控制
#[derive(Component)]
pub(crate) struct PlantProduceTag {
    // 冷却
    pub cooldown: f32,
    // 总时间
    pub elaspse: f32,
}

// 植物是否要进行攻击（是否侦测到攻击范围内有敌人）
#[derive(Component)]
pub(crate) enum PlantShootTag {
    Standby,
    Shoot,
}

// 投掷物标记
#[derive(Component)]
pub(crate) struct ProjectileTag;

// 移动速度
#[derive(Component)]
pub(crate) struct MoveVelocity(pub Vec2);

// 移动加速度
#[derive(Component)]
pub(crate) struct MoveAcceleration(pub Vec2);

// 冻结，不计算移动
#[derive(Component)]
pub(crate) struct Freeze;

// 延迟入场，在计时结束后解除Freeze和DelayShow
#[derive(Component)]
pub(crate) struct DelayShow {
    pub timer: Timer,
    pub se: Option<&'static str>,
}

// 僵尸标记
#[derive(Component)]
pub(crate) struct ZombieTag;

// 可攻击的僵尸，会吸引子弹
#[derive(Component)]
pub(crate) struct ZombieAttackableTag;

// 僵尸正在啃食状态，并标记啃食的植物
#[derive(Component)]
pub(crate) struct ZombieEatTag {
    // 僵尸啃食的植物
    pub target_plant: Entity,
    // 计时器
    pub timer: Timer,
}

// 僵尸血量动画
#[derive(Component, Default)]
pub(crate) struct ZombieHpAnim {
    pub trigger_half_damage_anim: bool,
    pub trigger_full_damage_anim: bool,
    pub trigger_critical_anim: bool,
    pub trigger_armor_anims_1: bool,
    pub trigger_armor_anims_2: bool,
    pub trigger_armor_anims_3: bool,
}

// 无敌，不会造成伤害
#[derive(Component)]
pub(crate) struct InvincibleTag;

// 僵尸进入临界值后的标签
#[derive(Component)]
pub(crate) struct ZombieCriticalTag;

// 计时器结束后，从场景中移除
#[derive(Component)]
pub(crate) struct ToDespawn(pub Timer);

// 在场景中的位置
#[derive(Component, Default)]
pub(crate) struct LanePosition {
    // 哪一路
    pub lane: u8,
    pub x: f32,
}

// 刷怪点
#[derive(Component)]
pub(crate) struct ZombieSolt {
    pub solt_type: SoltType,
}

// 在计时结束后，在该位置生成僵尸并移除此实体
#[derive(Component)]
pub(crate) struct ToSpawnZombie {
    pub timer: Timer,
    pub zombie_type: ZombieType,
    pub zombie_solt: Entity,
}

// 自然阳光
#[derive(Component)]
pub(crate) struct NaturalSunshineTag {
    pub target_y: f32,
}

// 阳光
#[derive(Component)]
pub(crate) struct SunshineTag {
    pub count: u32,
}

// 阳光生产槽
#[derive(Component)]
pub(crate) struct NaturalSunshineSolt {
    pub next_sunshine_timer: Timer,
}

// 游戏失败时隐藏
#[derive(Component)]
pub(crate) struct GameUiTag;

// 奖励槽
#[derive(Component)]
pub(crate) struct RewardSolt;

// 奖励
#[derive(Component)]
pub(crate) struct RewardTag;

// 材质透明度动画
#[derive(Component)]
pub(crate) struct MaterialColorAnim;

// 跟随相机
#[derive(Component)]
pub(crate) struct FollowCameraTag;

// 开始游戏按钮
#[derive(Component)]
pub(crate) struct StartGameButtonTag;

// 可以进行选择的种子
#[derive(Component)]
pub(crate) struct ChooseableSeedTag;

// 种子（在选择器里的）变换信息，用于复原
#[derive(Component)]
pub(crate) struct SeedTransformInChooserBox(pub Transform);

// 移动计时器，在时间结束后移除速度和加速度
#[derive(Component)]
pub(crate) struct MoveTimer(pub Timer);

impl PlantSolt {
    // 按默认僵尸啃食顺序返回植物
    pub fn plants(&self) -> [Option<Entity>; 4] {
        [self.temp, self.protect, self.primary, self.pot]
    }

    pub fn plants_mut(&mut self) -> [&mut Option<Entity>; 4] {
        [
            &mut self.temp,
            &mut self.protect,
            &mut self.primary,
            &mut self.pot,
        ]
    }
}
