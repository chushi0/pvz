use bevy::prelude::*;
use bevy_spatial::*;
use resource::{Sunshine, ZombieWaveController};
use scene_base::GameScene;
use tag::{PlantTag, ZombieAttackableTag};

mod resource;
mod setup;
mod tag;
mod update;

pub struct SceneGamePlugin;

#[derive(Debug, Default, PartialEq, Eq, Hash, States, Clone, Copy)]
enum GameState {
    #[default]
    Init, // 初始状态
    ChooseSeed, // 选卡界面
    Enter,      // 进入战场
    Main,       // 开始对局
    Exit,       // 结算
    Fail,       // 游戏结束
}

impl Plugin for SceneGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<Sunshine>()
            .init_resource::<ZombieWaveController>()
            .add_plugins(
                AutomaticUpdate::<PlantTag>::new()
                    .with_spatial_ds(SpatialStructure::KDTree2)
                    .with_transform(TransformMode::GlobalTransform),
            )
            .add_plugins(
                AutomaticUpdate::<ZombieAttackableTag>::new()
                    .with_spatial_ds(SpatialStructure::KDTree2)
                    .with_transform(TransformMode::GlobalTransform),
            )
            .add_systems(
                OnEnter(scene_base::GameScene::Game),
                (
                    setup::setup_camera,
                    setup::setup_background,
                    setup::setup_plant_solt,
                    setup::setup_zombie_solt,
                    setup::setup_seedbank,
                    setup::setup_seed_chooser,
                    setup::setup_cleanup_car,
                    setup::setup_standby_zombie,
                    setup::setup_resources,
                    setup::setup_sunshine_solt,
                    setup::setup_init_state,
                    setup::setup_init_bgm,
                    setup::setup_init_timer,
                    setup::setup_reward_solt,
                )
                    .chain(),
            )
            .add_systems(
                OnEnter(GameState::ChooseSeed),
                setup::setup_choose_seed_timer,
            )
            .add_systems(
                OnEnter(GameState::Enter),
                (setup::setup_enter_timer, setup::insert_seed_pickable_tag),
            )
            .add_systems(
                OnEnter(GameState::Main),
                (
                    setup::setup_game_bgm,
                    setup::setup_game_ui,
                    setup::clear_standby_zombie,
                )
                    .run_if(in_state(GameScene::Game)),
            )
            .add_systems(
                OnEnter(GameState::Fail),
                (
                    setup::clean_game_ui,
                    setup::stop_standby_anims,
                    setup::stop_zombie_anims,
                    setup::setup_gameover_timer,
                )
                    .run_if(in_state(GameScene::Game)),
            )
            .add_systems(
                OnEnter(GameState::Exit),
                (
                    setup::setup_exit_timer,
                    setup::setup_exit_reward_anim,
                    setup::setup_exit_overlay,
                )
                    .run_if(in_state(GameScene::Game)),
            )
            .add_systems(OnExit(GameScene::Game), setup::clear_scene)
            .add_systems(
                Update,
                (
                    // 全局逻辑
                    (
                        update::update_timer,
                        update::update_sunshine_text,
                        update::update_alpha_color,
                        update::update_image_cut,
                        update::update_material_alpha,
                        update::update_follow_camera,
                    ),
                    // 选卡逻辑
                    (
                        update::update_start_button_enabled,
                        update::input_select_seed,
                        update::input_giveup_seed,
                        update::start_game_button,
                    )
                        .run_if(in_state(GameState::ChooseSeed)),
                    // 游戏主逻辑
                    (
                        // 输入逻辑
                        (
                            // 左键点击事件
                            (
                                update::plant_seed,
                                update::collect_sunshine,
                                update::input_pick_seed,
                                update::input_pick_reward_seed,
                            )
                                .chain(),
                            update::cancel_pick_seed,
                            update::update_follow_cursor,
                            update::check_plant_seed_usable,
                        ),
                        // 植物逻辑
                        (
                            update::update_plant_shoot_enable,
                            update::plant_shoot,
                            update::plant_product,
                            update::update_projectile_show,
                            update::update_plant_die,
                            update::update_zombie_eat,
                        ),
                        // 僵尸逻辑
                        (
                            update::update_summon_zombie,
                            update::check_zombie_eat_start,
                            update::check_zombie_eat_end,
                            update::zombie_projectile_damage,
                            update::update_zombie_hp_anim,
                            update::update_zombie_enter_critical,
                            update::update_zombie_critical,
                            update::update_zombie_die,
                            update::update_zombie_eat_timer,
                        ),
                        // 流程控制
                        (
                            (update::update_zombie_wave, update::check_summon_reward).chain(),
                            update::update_level_progress,
                            update::update_level_progress_head,
                            update::update_level_progress_flag,
                            update::update_sunshine,
                            update::update_natural_sunshine,
                            update::trigger_cleanup_car,
                            update::cleanup_car_kill_zombie,
                            update::remove_outrange_car,
                            update::check_game_over,
                            update::update_reward_solt,
                        ),
                    )
                        .run_if(in_state(GameState::Main)),
                    // 通用逻辑
                    // 放在这里，以便捡起奖励后，各种物理事件不会停滞
                    // 但在游戏失败后，不能执行这部分逻辑，需要冻结画面
                    (
                        update::update_velocity,
                        update::update_movement,
                        update::check_remove_movement,
                        update::despawn_schedule_entity,
                        update::update_lane_position,
                        update::remove_outrange_projectile,
                    )
                        .run_if(in_state(GameState::Main).or_else(in_state(GameState::Exit))),
                )
                    .run_if(in_state(GameScene::Game)),
            );
    }
}
