use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    mechanical_components::generic::{
        GenericMechanicalComponentBundle, MyPosition, MyRigidBody, Shape,
    },
    player_plugin::Player,
};

#[derive(Component)]
struct Cube {
    color_handle: Handle<ColorMaterial>,
    color: Color,
}
#[derive(Component)]
struct Terrain;
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain);
    }
}

const CUBE_LENGTH: f32 = 100.;
fn spawn_cube(
    mut commands: Commands,
    color_handle: Handle<ColorMaterial>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    color: Color,
    position: MyPosition,
) {
    commands.spawn((
        Cube {
            color_handle: color_handle.clone(),
            color,
        },
        GenericMechanicalComponentBundle::new(
            MyRigidBody::Fixed,
            Shape::Rect {
                width: CUBE_LENGTH,
                heigt: CUBE_LENGTH,
            },
            color,
            position,
            &mut meshes,
            &mut materials,
        ),
    ));
}

fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let lenght = 2_000.;
    let square_size = 100.;
    let half_square_size = square_size / 2.;
    let square_nums = (lenght / square_size) as u32;
    //let terrain_curve = |x: f32| x.powi(2) / (lenght*3.);
    let gap = 5.;
    let color = Color::hsl(360. / square_nums as f32, 0.95, 0.6);

    let terrain_cube = commands.spawn((Terrain, Transform::from_xyz(0., 0., 0.), Visibility::default())).id();
    commands.entity(terrain_cube).with_children(|parent| {
        (0..square_nums).for_each(|i| {
            let x = -lenght / 2. + (square_size + gap) * (i as f32);
            //println!("{}",x);
            let y = -100.;
            let color = Color::hsl(360. * i as f32 / square_nums as f32, 0.95, 0.6);
            let color_handle = materials.add(color);

            parent.spawn((
                Cube {
                    color_handle: color_handle.clone(),
                    color,
                },
                GenericMechanicalComponentBundle::new(
                    MyRigidBody::Fixed,
                    Shape::Rect {
                        width: CUBE_LENGTH,
                        heigt: CUBE_LENGTH,
                    },
                    color,
                    MyPosition { x, y },
                    &mut meshes,
                    &mut materials,
                ),
            ));
        });
    });
}

fn update_material_color(materials: &mut ResMut<Assets<ColorMaterial>>, terrain_cube: &Cube) {
    let material = materials.get_mut(&terrain_cube.color_handle).unwrap();
    let old_color = terrain_cube.color.to_srgba();
    let increase_color = 2.0;
    let new_color = Color::srgb(
        old_color.red + increase_color,
        old_color.green + increase_color,
        old_color.blue + increase_color,
    );
    material.color = new_color;
}
