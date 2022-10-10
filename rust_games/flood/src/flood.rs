use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::tasks::ComputeTaskPool;
use bevy::utils::Duration;
use bevy_prototype_lyon::prelude::*;

use crate::grid::Grid;

#[derive(Default, Clone, Copy)]
struct FloodCell {
    ground_height: f32,
    flood_height: f32,
}

impl FloodCell {
    pub fn total_height(&self) -> f32 {
        return self.ground_height + self.flood_height;
    }
}

const OFFSETS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const FLOOD_VISCOSITY: f32 = 0.7;
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

    pub fn get_ground_height(&self, x: usize, y: usize) -> f32 {
        self.dst_grid.get(x, y).ground_height
    }

    pub fn set_ground_height(&mut self, x: usize, y: usize, height: f32) {
        self.dst_grid.get_mut(x, y).ground_height = height;
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


                GROUND RULES:
                - transfer occurs based on difference
                    - capped at flood_height of tile with greater height


                PERMUTATIONS:
                - S and T are same height. -> basic transfer
                - S <
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
                    let raw_difference = target_tile.total_height() - source_tile.total_height();
                    let max_transfer = if raw_difference < 0.0 {
                        // leaving source tile
                        source_tile.flood_height
                    } else {
                        // entering source tile
                        target_tile.flood_height
                    };

                    let change = raw_difference.min(max_transfer) * FLOOD_VISCOSITY * delta;
                    new_height += change;
                }

                self.dst_grid.get_mut(x, y).flood_height = new_height.max(0.0);
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
    commands.insert_resource(flood);
}

pub fn update_flood_system(time: Res<Time>, mut flood: ResMut<Flood>) {
    flood.step(time.delta_seconds());
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
        extents: Vec2::new(10.0, 10.0),
        origin: RectangleOrigin::Center,
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
