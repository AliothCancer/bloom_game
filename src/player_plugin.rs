use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

#[derive(Component)]
pub struct Player {
    pub side_lenght: f32,
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player = Player { side_lenght: 100. };
    
    //let half = player.side_lenght / 2.;
    //let rectangle_mesh = Mesh2d(meshes.add(Rectangle::new(player.side_lenght, player.side_lenght)));
    //let rectangle_collider = Collider::cuboid(half, half);
    //let mesh_and_collider = (rectangle_collider, rectangle_mesh);
    let circle_mesh = Mesh2d(meshes.add(Circle::new(player.side_lenght)));
    let circle_collider = Collider::ball(player.side_lenght);
    let mesh_and_collider = (circle_collider, circle_mesh);
    let position = Transform::from_xyz(0., 200., 0.);

    commands
        .spawn(player)
        // physic
        .insert(RigidBody::Dynamic)
        .insert(Restitution::default())
        .insert(ColliderMassProperties::Density(0.002))
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.,
        })
        .insert(Damping {
            linear_damping: 3.5,
            angular_damping: 2.0,
        })
        .insert(GravityScale(1.2))
        .insert(ActiveEvents::COLLISION_EVENTS)
        // position, shape, color
        .insert(position)
        .insert(mesh_and_collider)
        .insert(MeshMaterial2d(materials.add(Color::srgb(4., 1., 4.))));
}

const PLAYER_ACCELERATION_FORCE: f32 = 280_000.;

fn move_player(
    //mut player: Query<&mut Transform, With<Player>>,
    ext_forces: Single<&mut ExternalForce, With<Player>>,
    //mut grav_scale: Query<&mut GravityScale>,
    //time: Res<Time>,
    //mut controller: Query<&mut KinematicCharacterController>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;
    let mut torque_rotation = 0f32;
    if kb_input.pressed(KeyCode::KeyW) {
        torque_rotation = -1.;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        torque_rotation = 1.;
    }
    //if kb_input.pressed(KeyCode::KeyW) {
    //    direction += Vec2 { x: 0.0, y: 1. };
    //}
    //if kb_input.pressed(KeyCode::KeyS) {
    //    direction += Vec2 { x: 0.0, y: -1. };
    //}
    if kb_input.pressed(KeyCode::KeyA) {
        direction += Vec2 { x: -1.0, y: 0. };
    }
    if kb_input.pressed(KeyCode::KeyD) {
        direction += Vec2 { x: 1.0, y: 0. };
    }
    if direction.x.abs() + direction.y.abs() > 1. {
        direction *= Vec2 {
            x: 1. / 2.0f32.sqrt(),
            y: 1. / 2.0f32.sqrt(),
        }
    }
    //  dbg!(**ext_forces);
    let mut ext_force = ext_forces.into_inner();
    ext_force.force = direction * PLAYER_ACCELERATION_FORCE;
    ext_force.torque = torque_rotation * PLAYER_ACCELERATION_FORCE * 100.;
}
