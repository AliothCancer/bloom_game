use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player_plugin::Player;

#[derive(Component)]
struct TerrainCube {
    color_handle: Handle<ColorMaterial>,
    color: Color,
}
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain)
            .add_systems(Update, glow_at_hovering);
    }
}
fn _glow_at_collision(
    mut terrain_query: Query<(Entity, &mut TerrainCube), With<TerrainCube>>,
    player_query: Single<Entity, With<Player>>,
    rapier_context: ReadDefaultRapierContext,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_entity = player_query.into_inner();
    terrain_query
        .iter_mut()
        .for_each(|(terrain_entity, terrain_cube)| {
            rapier_context
                .contact_pairs()
                .for_each(|contact_pair_view| {
                    if _is_player_terrain_collision(
                        contact_pair_view.collider1(),
                        contact_pair_view.collider2(),
                        terrain_entity,
                        player_entity,
                    ) {
                        update_material_color(&mut materials, &terrain_cube);
                    }
                });
        });
}

fn _is_player_terrain_collision(
    entity_a: Entity,
    entity_b: Entity,
    terrain_entity: Entity,
    player_entity: Entity,
) -> bool {
    (entity_a.index() == terrain_entity.index() && entity_b.index() == player_entity.index())
        || (entity_b.index() == terrain_entity.index() && entity_a.index() == player_entity.index())
}

fn glow_at_hovering(
    mut terrain_query: Query<(&mut TerrainCube, &Transform)>,
    player_query: Query<(&Transform, &Player)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (player_transform, player) = player_query.single();
    terrain_query
        .iter_mut()
        .for_each(|(terrain_cube, terrain_transform)| {
            if terrain_transform
                .translation
                .distance(player_transform.translation)
                .abs()
                < player.side_lenght * 1.4
            {
                let material = materials.get_mut(&terrain_cube.color_handle).unwrap();
                let old_color = terrain_cube.color.to_srgba();
                let increase_color = 2.;
                let new_color = Color::srgb(
                    old_color.red + increase_color,
                    old_color.green + increase_color,
                    old_color.blue + increase_color,
                );
                material.color = new_color;
            } else {
                let material = materials.get_mut(&terrain_cube.color_handle).unwrap();
                material.color.mix_assign(terrain_cube.color, 0.05);
            }
        });
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
    for i in 0..square_nums {
        let x = -lenght / 2. + (square_size + gap) * (i as f32);
        let y = - 100.;

        let color = Color::hsl(360. * i as f32 / square_nums as f32, 0.95, 0.6);
        let color_handle = materials.add(color);

        commands
            .spawn((
                TerrainCube {
                    color_handle: color_handle.clone(),
                    color,
                },
                Mesh2d(meshes.add(Rectangle::new(square_size, square_size))),
                MeshMaterial2d(color_handle),
                Transform::from_xyz(x, y, 0.),
            ))
            .insert(Collider::cuboid(half_square_size, half_square_size))
            .insert(ActiveEvents::COLLISION_EVENTS);
    }
}

fn update_material_color(
    materials: &mut ResMut<Assets<ColorMaterial>>,
    terrain_cube: &TerrainCube,
) {
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
