use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::grid::Grid;

#[derive(Default, Clone, Copy)]
struct FloodCell {
    ground_height: f32,
    flood_height: f32,
}

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

    pub fn step(&mut self, delta: f32) {
        // let working_grid = self.src_grid;
        // self.src_grid = self.dst_grid;

        // self.dst_grid = working_grid;
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
    let size = 40;
    let mut flood = Flood::new(size, size);
    for i in 0..40 {
        flood.set_flood_height(i, 5, i as f32 / 40.0);
    }
    commands.insert_resource(flood);

    let tile_size = 20.0;
    let tile_shape = shapes::Rectangle {
        extents: Vec2::new(20.0, 20.0),
        origin: RectangleOrigin::Center,
    };

    for x in 0..size {
        for y in 0..size {
            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &tile_shape,
                    DrawMode::Fill(FillMode::color(Color::WHITE)),
                    // DrawMode::Fill(FillMode::color(Color::GREEN)),
                    Transform::from_xyz(
                        x as f32 * tile_size - 300.0,
                        y as f32 * tile_size - 300.0,
                        0.0,
                    ),
                ))
                .insert(FloodTile { x, y });
        }
    }
}

pub fn update_flood_system(time: Res<Time>, mut flood: ResMut<Flood>) {
    flood.step(time.delta_seconds());
}

pub fn update_flood_render_system(
    flood: Res<Flood>,
    mut query: Query<(&FloodTile, &mut DrawMode)>,
) {
    for (tile, mut draw_mode) in &mut query {
        let flood_height = flood.get_flood_height(tile.x, tile.y);
        let color = Color::rgb(0.0, 0.0, flood_height);
        *draw_mode = DrawMode::Fill(FillMode::color(color));
    }
}
