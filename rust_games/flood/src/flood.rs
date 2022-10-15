use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::tasks::ComputeTaskPool;
use bevy::utils::Duration;
use bevy_prototype_lyon::prelude::*;

use crate::grid::Grid;

pub struct FixedTime;

impl FixedTime {
    pub fn fixed_delta(&self) -> f32 {
        0.100
    }
}

#[derive(Default, Clone, Copy, Deref, DerefMut)]
pub struct GridPoint(IVec2);

#[derive(Default, Clone, Copy)]
struct GroundCell {
    height: f32,
}

#[derive(Default, Clone, Copy)]
struct FloodCell {
    height: f32,
}

const OFFSETS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const FLOOD_MIN_HEIGHT: f32 = 0.01;
const FLOOD_VISCOSITY: f32 = 0.9;
const FLOOD_GRID_WIDTH: usize = 160;
const FLOOD_GRID_HEIGHT: usize = 90;
const GRID_CELL_SIZE: f32 = 1.0;

pub struct Flood {
    width: usize,
    height: usize,
    ground_grid: Grid<GroundCell>,
    src_grid: Grid<FloodCell>,
    dst_grid: Grid<FloodCell>,
}

impl Flood {
    pub fn new(width: usize, height: usize) -> Self {
        Flood {
            width,
            height,
            ground_grid: Grid::new(width, height),
            src_grid: Grid::new(width, height),
            dst_grid: Grid::new(width, height),
        }
    }

    pub fn get_flood_height(&self, x: usize, y: usize) -> f32 {
        self.dst_grid.get(x, y).height
    }

    pub fn set_flood_height(&mut self, x: usize, y: usize, height: f32) {
        self.dst_grid.get_mut(x, y).height = height;
    }

    pub fn get_ground_height(&self, x: usize, y: usize) -> f32 {
        self.ground_grid.get(x, y).height
    }

    pub fn set_ground_height(&mut self, x: usize, y: usize, height: f32) {
        self.ground_grid.get_mut(x, y).height = height;
    }

    pub fn add_flood(&mut self, x: usize, y: usize, additional_flood: f32) {
        self.dst_grid.get_mut(x, y).height += additional_flood;
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }

    pub fn closest_flood(&self, position: Vec2, radius: i32) -> Option<GridPoint> {
        let target_x = position.x as i32;
        let target_y = position.y as i32;

        let mut found_flood = false;
        let mut min_distance = 0;
        let mut closest = IVec2::ZERO;

        for test_x in target_x - radius..target_x + radius {
            for test_y in target_y - radius..target_y + radius {
                if !self.in_bounds(test_x, test_y) {
                    continue;
                }

                if self.get_flood_height(test_x as usize, test_y as usize) > 0.0 {
                    let test_distance = test_x.abs_diff(target_x) + test_y.abs_diff(target_y);
                    if !found_flood || test_distance < min_distance {
                        min_distance = test_distance;
                        closest.x = test_x;
                        closest.y = test_y;
                        found_flood = true;
                    }
                }
            }
        }

        if found_flood {
            Some(GridPoint(closest))
        } else {
            None
        }
    }

    pub fn step(&mut self, delta: f32) {
        std::mem::swap(&mut self.src_grid, &mut self.dst_grid);

        /* Rules:
        - Preserve amount of flood
        - Naive approach for now please
         */
        for y in 0..self.height {
            for x in 0..self.width {
                /* For each neighbor:
                - calculate transfer amount by
                    - calculating height difference
                    - apply viscosity and time delta
                    - only update index in dst
                    - will require double work for now (only check down and right?)
                // SKIPPING ground height for now


                GROUND RULES:
                - transfer occurs based on difference
                    - capped at flood_height of tile with greater height
                 */

                let source_flood = self.src_grid.get(x, y);
                let source_ground = self.ground_grid.get(x, y);
                let mut new_height = source_flood.height;
                for (x_offset, y_offset) in OFFSETS {
                    let x_target = x as i32 + x_offset;
                    let y_target = y as i32 + y_offset;

                    if !self.in_bounds(x_target, y_target) {
                        continue;
                    }

                    let target_flood = self.src_grid.get(x_target as usize, y_target as usize);
                    let target_ground = self.ground_grid.get(x_target as usize, y_target as usize);
                    let raw_difference = target_flood.height + target_ground.height
                        - (source_flood.height + source_ground.height);
                    let max_transfer = if raw_difference <= 0.0 {
                        // leaving source tile
                        source_flood.height
                    } else {
                        // entering source tile
                        target_flood.height
                    };

                    // Need to take the sign of transfer separately so that
                    // transfer amounts work correctly without too many issues.
                    let change = raw_difference.signum()
                        * raw_difference.abs().min(max_transfer)
                        * FLOOD_VISCOSITY
                        * delta;
                    if change.abs() >= FLOOD_MIN_HEIGHT {
                        new_height += change;
                    }
                }

                new_height = new_height.max(0.0);
                if new_height < FLOOD_MIN_HEIGHT {
                    new_height = 0.0
                }
                self.dst_grid.get_mut(x, y).height = new_height;
            }
        }
    }
}

#[derive(Component)]
pub struct FloodTile {
    x: usize,
    y: usize,
}

#[derive(Component)]
pub struct GroundTile {
    x: usize,
    y: usize,
}

#[derive(Component)]
pub struct FloodEnt;

pub fn setup_flood_demo_system(mut commands: Commands) {
    let mut flood = Flood::new(FLOOD_GRID_WIDTH, FLOOD_GRID_HEIGHT);

    for i in 0..20 {
        let height = if i % 2 == 0 { 200000.0 } else { 10.0 };
        flood.set_ground_height(25, i + 20, height);
        flood.set_ground_height(26, i + 20, height);
        flood.set_ground_height(27, i + 20, height);
        flood.set_ground_height(25 - i, 20, height);
    }

    for y in 0..FLOOD_GRID_HEIGHT {
        for x in 0..FLOOD_GRID_WIDTH {
            let color_value = flood.get_ground_height(x, y) / 10.0;
            let color = Color::rgb(color_value, color_value, color_value);
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: color,
                        custom_size: Some(Vec2::new(GRID_CELL_SIZE, GRID_CELL_SIZE)),
                        anchor: Anchor::BottomLeft,
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        x as f32 * GRID_CELL_SIZE,
                        y as f32 * GRID_CELL_SIZE,
                        0.0,
                    ),
                    ..default()
                })
                .insert(GroundTile { x, y });
        }
    }

    for y in 0..FLOOD_GRID_HEIGHT {
        for x in 0..FLOOD_GRID_WIDTH {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::PURPLE,
                        custom_size: Some(Vec2::new(GRID_CELL_SIZE, GRID_CELL_SIZE)),
                        anchor: Anchor::BottomLeft,
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        x as f32 * GRID_CELL_SIZE,
                        y as f32 * GRID_CELL_SIZE,
                        1.0,
                    ),
                    ..default()
                })
                .insert(FloodTile { x, y });
        }
    }

    spawn_spawner(&mut commands, 0, 20, 10.0, 1.0);
    spawn_spawner(&mut commands, 20, 30, 1000.0, 1.0);
    spawn_spawner(&mut commands, 0, 0, 30.0, 1.0);
    spawn_spawner(&mut commands, 145, 45, 4000.0, 1.0);
    commands.insert_resource(flood);
    commands.insert_resource(MouseTimer(Timer::from_seconds(1.0, true)));
}

pub fn update_flood_system(fixed_time: Res<FixedTime>, mut flood: ResMut<Flood>) {
    flood.step(fixed_time.fixed_delta());
}

pub fn update_flood_render_system(flood: Res<Flood>, mut query: Query<(&FloodTile, &mut Sprite)>) {
    query.for_each_mut(|(tile, mut sprite)| {
        let flood_height = flood.get_flood_height(tile.x, tile.y);
        let color = Color::rgba(0.0, 0.0, 1.0, flood_height);
        sprite.color = color;
    });
}

#[derive(Component)]
pub struct FloodSpawner {
    discharge: f32,
    timer: Timer,
}

#[derive(Component)]
pub struct GridPosition {
    x: usize,
    y: usize,
}

fn spawn_spawner(commands: &mut Commands, x: usize, y: usize, discharge: f32, period: f32) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(1.0, 1.0),
        origin: RectangleOrigin::BottomLeft,
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::GREEN)),
            Transform::from_xyz(x as f32 * GRID_CELL_SIZE, y as f32 * GRID_CELL_SIZE, 2.0),
        ))
        .insert(GridPosition { x, y })
        .insert(FloodSpawner {
            discharge,
            timer: Timer::new(Duration::from_secs_f32(period), true),
        });
}

pub fn spawner_discharge_flood_system(
    fixed_time: Res<FixedTime>,
    mut flood: ResMut<Flood>,
    mut query: Query<(&GridPosition, &mut FloodSpawner)>,
) {
    for (grid_position, mut spawner) in &mut query {
        if spawner
            .timer
            .tick(Duration::from_secs_f32(fixed_time.fixed_delta()))
            .just_finished()
        {
            flood.add_flood(grid_position.x, grid_position.y, spawner.discharge);
        }
    }
}

#[derive(Component)]
pub struct MainCamera;

pub struct MouseTimer(Timer);

pub fn mouse_record_system(
    time: Res<Time>,
    flood: Res<Flood>,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut mouse_timer: ResMut<MouseTimer>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if mouse_timer.0.tick(time.delta()).just_finished() {
        let (camera, camera_transform) = q_camera.single();
        // get the window that the camera is displaying to (or the primary window)
        let wnd = if let RenderTarget::Window(id) = camera.target {
            windows.get(id).unwrap()
        } else {
            windows.get_primary().unwrap()
        };

        // check if the cursor is inside the window and get its position
        if let Some(screen_pos) = wnd.cursor_position() {
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
            let world_pos: Vec2 = world_pos.truncate();
            let grid_pos = world_pos / GRID_CELL_SIZE;

            eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
            eprintln!("Grid  coords: {}/{}", grid_pos.x, grid_pos.y);
            eprintln!(
                "Ground height: {}, Flood height: {}",
                flood.get_ground_height(grid_pos.x as usize, grid_pos.y as usize),
                flood.get_flood_height(grid_pos.x as usize, grid_pos.y as usize)
            );
        }
    }
}
