use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Button;

// 点击热区
#[derive(Component, Default)]
pub enum ButtonHotspot {
    #[default]
    None,
    Rects(Vec<Rect>),
    Polygon(Vec<Vec2>),
}

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

#[derive(Component, Default, Clone, Copy)]
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
        match self {
            ButtonHotspot::None => false,
            ButtonHotspot::Rects(vec) => vec.iter().any(|rect| rect.contains(point)),
            ButtonHotspot::Polygon(vec) => {
                // From tabnine:
                // 此算法基于 "点在多边形内的射线穿越测试"，
                // 它通过检查点是否在多边形的每条边的左侧来确定点是否在多边形内。
                // 如果点穿越了奇数个边，则点在多边形内。
                let mut c = false;
                let nvert = vec.len();
                let mut j = nvert - 1;

                for i in 0..nvert {
                    let vi = vec[i];
                    let vj = vec[j];

                    if ((vi.y > point.y) != (vj.y > point.y))
                        && (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x)
                    {
                        c = !c;
                    }
                    j = i;
                }
                c
            }
        }
    }
}
