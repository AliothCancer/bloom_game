use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::player_plugin::Player;

#[derive(Component)]
struct TerrainCube {
    color_handle: Handle<ColorMaterial>,
    color: Color
}
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain)
            .add_systems(Update, glow_at_hovering);
    }
}

fn glow_at_hovering(
    mut terrain_query: Query<(&mut TerrainCube, &Transform)>,
    player_query: Query<&Transform, With<Player>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_transform = player_query.single();
    terrain_query
        .iter_mut()
        .for_each(|(terrain_cube, terrain_transform)| {
            if (terrain_transform.translation.x - player_transform.translation.x).abs() < 100. {
                let material = materials.get_mut(&terrain_cube.color_handle).unwrap();
                let old_color = terrain_cube.color.to_srgba();
                let increase_color = 2.;
                let new_color = Color::srgb(old_color.red+ increase_color, old_color.green + increase_color, old_color.blue + increase_color);
                material.color = new_color;
            } else {
                let material = materials.get_mut(&terrain_cube.color_handle).unwrap();
                material.color = terrain_cube.color;
            }
        });
}

fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let square_size = 60.;
    let half_square_size = square_size / 2.;
    let square_nums = 60;

    for i in 0..square_nums {
        let color = Color::hsl(360. * i as f32 / square_nums as f32, 0.95, 0.7);
        let color_handle = materials.add(color);
        let x = -1000.0 + (square_size + 10.) * (i as f32);
        commands
            .spawn((
                TerrainCube {
                    color_handle: color_handle.clone(),
                    color
                },
                Mesh2d(meshes.add(Rectangle::new(square_size, square_size))),
                MeshMaterial2d(color_handle),
                Transform::from_xyz(x, -100.0, 0.),
            ))
            .insert(Collider::cuboid(half_square_size, half_square_size));
    }
}
