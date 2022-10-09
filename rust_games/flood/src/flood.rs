use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool;
use bevy::utils::Duration;
use bevy_prototype_lyon::prelude::*;

use crate::grid::Grid;

#[derive(Default, Clone, Copy)]
struct FloodCell {
    ground_height: f32,
    flood_height: f32,
}

const OFFSETS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const FLOOD_VISCOSITY: f32 = 0.5;
const FLOOD_GRID_WIDTH: usize = 80;
const FLOOD_GRID_HEIGHT: usize = 45;
const GRID_CELL_SIZE: f32 = 10.0;

pub struct Flood {
    width: usize,
    height: usize,
    src_grid: Grid<FloodCell>,
    dst_grid: Grid<FloodCell>,
}

impl Flood {
    pub fn new(width: usize, height: usize) -> Self {
        Flood {
            width,
            height,
            src_grid: Grid::new(width, height),
            dst_grid: Grid::new(width, height),
        }
    }

    pub fn get_flood_height(&self, x: usize, y: usize) -> f32 {
        self.dst_grid.get(x, y).flood_height
    }

    pub fn set_flood_height(&mut self, x: usize, y: usize, height: f32) {
        self.dst_grid.get_mut(x, y).flood_height = height;
    }

    pub fn add_flood(&mut self, x: usize, y: usize, additional_flood: f32) {
        self.dst_grid.get_mut(x, y).flood_height += additional_flood;
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
                 */

                let source_tile = self.src_grid.get(x, y);
                let mut new_height = source_tile.flood_height;
                for (x_offset, y_offset) in OFFSETS {
                    let x_target = x as i32 + x_offset;
                    let y_target = y as i32 + y_offset;

                    if x_target < 0
                        || x_target as usize >= self.width
                        || y_target < 0
                        || y_target as usize >= self.height
                    {
                        continue;
                    }

                    let target_tile = self.src_grid.get(x_target as usize, y_target as usize);
                    let height_difference = target_tile.flood_height - source_tile.flood_height;
                    let change = height_difference * FLOOD_VISCOSITY * delta;
                    new_height += change;
                }

                self.dst_grid.get_mut(x, y).flood_height = new_height;
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
pub struct FloodEnt;

pub fn setup_flood_demo_system(mut commands: Commands) {
    let flood = Flood::new(FLOOD_GRID_WIDTH, FLOOD_GRID_HEIGHT);
    commands.insert_resource(flood);

    let tile_shape = shapes::Rectangle {
        extents: Vec2::new(GRID_CELL_SIZE, GRID_CELL_SIZE),
        origin: RectangleOrigin::Center,
    };

    for y in 0..FLOOD_GRID_HEIGHT {
        for x in 0..FLOOD_GRID_WIDTH {
            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &tile_shape,
                    DrawMode::Fill(FillMode::color(Color::WHITE)),
                    Transform::from_xyz(x as f32 * GRID_CELL_SIZE, y as f32 * GRID_CELL_SIZE, 0.0),
                ))
                .insert(FloodTile { x, y });
        }
    }

    spawn_spawner(&mut commands, 0, 20, 10.0, 1.0);
    spawn_spawner(&mut commands, 20, 30, 10.0, 1.0);
    spawn_spawner(&mut commands, 0, 0, 30.0, 1.0);
}

pub fn update_flood_system(time: Res<Time>, mut flood: ResMut<Flood>) {
    flood.step(time.delta_seconds());
}

pub fn update_flood_render_system(
    flood: Res<Flood>,
    mut query: Query<(&FloodTile, &mut DrawMode)>,
) {
    query.for_each_mut(|(tile, mut draw_mode)| {
        let flood_height = flood.get_flood_height(tile.x, tile.y);
        let color = Color::rgb(0.0, 0.0, flood_height);
        *draw_mode = DrawMode::Fill(FillMode::color(color));
    });
}

#[derive(Component)]
pub struct FloodSpawner {
    discharge: f32,
    period: f32,
    timer: Timer,
}

#[derive(Component)]
pub struct GridPosition {
    x: usize,
    y: usize,
}

fn spawn_spawner(commands: &mut Commands, x: usize, y: usize, discharge: f32, period: f32) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(10.0, 10.0),
        origin: RectangleOrigin::Center,
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::GREEN)),
            Transform::from_xyz(x as f32 * GRID_CELL_SIZE, y as f32 * GRID_CELL_SIZE, 0.0),
        ))
        .insert(GridPosition { x, y })
        .insert(FloodSpawner {
            discharge,
            period,
            timer: Timer::new(Duration::from_secs_f32(period), true),
        });
}

pub fn spawner_discharge_flood_system(
    time: Res<Time>,
    mut flood: ResMut<Flood>,
    mut query: Query<(&GridPosition, &mut FloodSpawner)>,
) {
    for (grid_position, mut spawner) in &mut query {
        if spawner.timer.tick(time.delta()).just_finished() {
            flood.add_flood(grid_position.x, grid_position.y, spawner.discharge);
        }
    }
}
