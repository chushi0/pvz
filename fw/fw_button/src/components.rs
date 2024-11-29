use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Button;

// 点击热区
#[derive(Component, Default)]
pub struct ButtonHotspot(pub Vec<Rect>);

// 交互
#[derive(Component, Default)]
pub enum ButtonInteraction {
    #[default]
    None,
    // 悬浮
    Hover,
    // 按住
    Pressed,
    // 点击（仅持续1帧）
    Click,
    // 取消（按住后移出热区）
    Cancel,
}

// 背景
#[derive(Component, Default)]
pub struct ButtonBackground {
    pub normal: Handle<Image>,
    pub hover: Handle<Image>,
    pub pressed: Handle<Image>,
    pub disabled: Handle<Image>,
}

#[derive(Component, Default)]
pub enum ButtonEnabled {
    #[default]
    Enabled,
    Disabled,
}

#[derive(Bundle, Default)]
pub struct ButtonBundle {
    pub button: Button,
    pub hotspot: ButtonHotspot,
    pub interaction: ButtonInteraction,
    pub enabled: ButtonEnabled,
    pub background: ButtonBackground,
    pub sprite: SpriteBundle,
}

impl ButtonHotspot {
    pub fn contains(&self, point: Vec2) -> bool {
        self.0.iter().any(|rect| rect.contains(point))
    }
}
