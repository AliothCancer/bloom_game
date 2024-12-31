use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;


#[derive(Component)]
struct Terrain;
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain)
            //.add_systems(Update, glow_at_hovering)
            ;
    }
}

//fn glow_at_hovering(
//    mut terrains: Query<&mut > 
//){}

fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let square_size = 60.;
    let half_square_size = square_size / 2.;
    let square_nums = 60;
    for i in 0..square_nums {
        //let color = Color::hsl(360. * i as f32 / square_nums as f32, 0.95, 0.7);
        
        let x = -1000.0 + (square_size+ 10.) * (i as f32);
        commands.spawn((
            Terrain,
            Mesh2d(meshes.add(Rectangle::new(square_size,square_size))),
            MeshMaterial2d(materials.add(Color::srgb(2., 7., 4.))),
            Transform::from_xyz(x, -100.0, 0.)
        ))
        .insert(Collider::cuboid(half_square_size, half_square_size))
        ;
    }
}
