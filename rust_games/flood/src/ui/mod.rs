use crate::flood::GridPosition;
use bevy::{prelude::*, render::camera::RenderTarget};

pub struct PointerState {
    // just_happened: bool,
    pub grid_position: UVec2,
    pub world_position: Vec2,
}

pub struct PointerInfo {
    pub position: PointerState,
    // select: PointerState,
}

pub fn get_pointer_info(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    windows: &Res<Windows>,
) -> Option<PointerInfo> {
    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };
    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();
        let grid_pos = world_pos;

        Some(PointerInfo {
            position: PointerState {
                grid_position: UVec2::new(grid_pos.x as u32, grid_pos.y as u32),
                world_position: world_pos,
            },
        })
    } else {
        None
    }
}
