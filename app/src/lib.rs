use bevy::{log::LogPlugin, prelude::*};

pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Plant VS Zombie".to_string(),
                        resizable: false,
                        resize_constraints: WindowResizeConstraints {
                            min_width: 800.,
                            min_height: 600.,
                            max_width: 800.,
                            max_height: 600.,
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(LogPlugin {
                    #[cfg(debug_assertions)]
                    level: bevy::log::Level::DEBUG,
                    ..Default::default()
                }),
        )
        .add_plugins(bevy_spine::SpinePlugin)
        .add_plugins(fw_anim::FwAnimPlugin)
        .add_plugins(fw_ftxm::FwFtxmPlugin)
        .add_plugins(fw_cursor::FwCursorPlugin)
        .add_plugins(fw_button::FwButtonPlugin)
        .add_plugins(fw_actor::FwActorPlugin)
        .add_plugins(mod_plant::ModPlantPlugin)
        .add_plugins(mod_zombie::ModZombiePlugin)
        .add_plugins(mod_level::ModLevelPlugin)
        .add_plugins(mod_userdata::ModUserdataPlugin)
        .add_plugins(scene_base::SceneBasePlugin)
        .add_plugins(scene_title::SceneTitlePlugin)
        .add_plugins(scene_game::SceneGamePlugin)
        .run();
}
