use bevy::{
    prelude::*, render::camera::NormalizedRenderTarget, utils::HashMap, window::PrimaryWindow,
};

pub struct FwCursorPlugin;

#[derive(Debug, Resource, Default)]
pub struct CursorPosition {
    pub world_position: Vec2,
}

impl Plugin for FwCursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .add_systems(PreUpdate, update_cursor_position);
    }
}

fn update_cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    touches_input: Res<Touches>,
    camera: Query<(Entity, &Camera, &GlobalTransform)>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
    windows: Query<&Window>,
) {
    let primary_window = primary_window.iter().next();

    let camera_cursor_position: HashMap<Entity, Vec2> = camera
        .iter()
        .filter_map(|(entity, camera, camera_transform)| {
            let Some(NormalizedRenderTarget::Window(window_ref)) =
                camera.target.normalize(primary_window)
            else {
                return None;
            };

            let viewport_position = camera
                .logical_viewport_rect()
                .map(|rect| rect.min)
                .unwrap_or_default();

            windows
                .get(window_ref.entity())
                .ok()
                .and_then(|window| window.cursor_position())
                .or_else(|| touches_input.first_pressed_position())
                .and_then(|cursor_position| {
                    camera
                        .viewport_to_world_2d(camera_transform, cursor_position - viewport_position)
                })
                .map(|position| (entity, position))
        })
        .collect();

    assert!(camera_cursor_position.len() <= 1);

    let Some(cursor) = camera_cursor_position
        .iter()
        .next()
        .map(|(_, val)| val)
        .copied()
    else {
        return;
    };

    cursor_position.world_position = cursor;
}
