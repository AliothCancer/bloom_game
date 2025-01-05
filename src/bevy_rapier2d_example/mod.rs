

use bevy::{math::VectorSpace, prelude::*};
use bevy_rapier2d::prelude::*;

pub struct BevyRapierExamplePlugin;

#[derive(Component, Default)]
pub struct Despawn;

#[derive(Resource, Default)]
pub struct DespawnResource {
    timer: Timer,
}

impl Plugin for BevyRapierExamplePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(DespawnResource::default()) 

            // DESPAWN EXAMPLE   
            //.add_systems(Startup, despawn_setup)
            //.add_systems(Update, despawn)

            // PLAYER_MOVEMENT EXAMPLE
            //.add_systems(Startup, spawn_player)
            //.add_systems(Update, player_movement)

            // MULTIPLE COLLIDER
            .add_systems(Startup, multi_collider)
            ;
    }
}

#[derive(Component)]
pub struct Player(f32);

pub fn spawn_player(mut commands: Commands, mut rapier_config: Query<&mut RapierConfiguration>) {
    let mut rapier_config = rapier_config.single_mut();
    // Set gravity to 0.0 and spawn camera.
    rapier_config.gravity = Vec2::ZERO;
    //commands.spawn(Camera2d::default());

    let sprite_size = 100.0;

    // Spawn entity with `Player` struct as a component for access in movement query.
    commands.spawn((
        Sprite {
            color: Color::srgba(10.0, 0.0, 10.0, 0.5),
            custom_size: Some(Vec2::new(sprite_size, sprite_size)),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Velocity::zero(),
        Collider::ball(sprite_size / 2.0),
        Player(300.0),
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_info: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut rb_vels) in &mut player_info {
        let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
        let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
        let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
        let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32)*player.0;
        if move_delta != Vec2::ZERO {
            rb_vels.linvel = move_delta;
        }
        rb_vels.linvel = rb_vels.linvel.lerp(Vec2::ZERO, 0.1);
        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
    }
}


pub fn despawn_setup(mut commands: Commands, mut despawn: ResMut<DespawnResource>) {
    despawn.timer = Timer::from_seconds(4.0, TimerMode::Once);

    // Build the rigid body.
    let rad = 4.0;
    let numi = 40; // Num vertical nodes.
    let numk = 40; // Num horizontal nodes.
    let shift = 10.0;

    let mut body_entities = Vec::new();

    for k in 0..numk {
        for i in 0..numi {
            let fk = k as f32;
            let fi = i as f32;

            let rigid_body = if i == 0 && (k % 4 == 0 || k == numk - 1) {
                RigidBody::Fixed
            } else {
                RigidBody::Dynamic
            };

            let child_entity = commands
                .spawn((
                    Transform::from_xyz(fk * shift, -fi * shift, 0.0),
                    rigid_body,
                    Collider::cuboid(rad, rad),
                ))
                .id();

            // Vertical joint.
            if i > 0 {
                let parent_entity = *body_entities.last().unwrap();
                let joint = RevoluteJointBuilder::new().local_anchor2(Vec2::new(0.0, shift));
                commands.entity(child_entity).with_children(|cmd| {
                    // NOTE: we want to attach multiple impulse joints to this entity, so
                    //       we need to add the components to children of the entity. Otherwise
                    //       the second joint component would just overwrite the first one.
                    let mut entity = cmd.spawn(ImpulseJoint::new(parent_entity, joint));
                    if i == (numi / 2) || (k % 4 == 0 || k == numk - 1) {
                        entity.insert(Despawn);
                    }
                });
            }

            // Horizontal joint.
            if k > 0 {
                let parent_index = body_entities.len() - numi;
                let parent_entity = body_entities[parent_index];
                let joint = RevoluteJointBuilder::new().local_anchor2(Vec2::new(-shift, 0.0));
                commands.entity(child_entity).with_children(|cmd| {
                    // NOTE: we want to attach multiple impulse joints to this entity, so
                    //       we need to add the components to children of the entity. Otherwise
                    //       the second joint component would just overwrite the first one.
                    let mut entity = cmd.spawn(ImpulseJoint::new(parent_entity, joint));
                    if i == (numi / 2) || (k % 4 == 0 || k == numk - 1) {
                        entity.insert(Despawn);
                    }
                });
            }

            body_entities.push(child_entity);
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut despawn: ResMut<DespawnResource>,
    query: Query<Entity, With<Despawn>>,
) {
    if despawn.timer.tick(time.delta()).just_finished() {
        for e in &query {
            println!("Despawning joint entity");
            commands.entity(e).despawn();
        }
    }
}

fn multi_collider(mut commands: Commands) {
    /*
     * Ground
     */
    let ground_size = 500.0;
    let ground_height = 1.0;

    commands.spawn((
        Transform::from_xyz(0.0, -ground_height, 0.0),
        Collider::cuboid(ground_size, ground_height),
    ));

    /*
     * Create the cubes
     */
    let num = 4;
    let rad = 2.0;

    let shift = rad * 4.0 + rad;
    let centerx = shift * (num / 2) as f32;
    let centery = shift / 2.0;

    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;

    for j in 0usize..20 {
        for i in 0..num {
            let x = i as f32 * shift * 5.0 - centerx + offset;
            let y = j as f32 * (shift * 5.0) + centery + 3.0;

            commands
                .spawn((Transform::from_xyz(x, y, 0.0), RigidBody::Dynamic))
                .with_children(|children| {
                    // these cuboids forms e container with a top open, like a glass
                    children.spawn(Collider::cuboid(rad * 10.0, rad));
                    children.spawn((
                        Transform::from_xyz(rad * 10.0, rad * 10.0, 0.0),
                        Collider::cuboid(rad, rad * 10.0),
                    ));
                    children.spawn((
                        Transform::from_xyz(rad * 10.0, rad * 10.0, 0.0),
                        Collider::ball(10.),
                    ));
                    children.spawn((
                        Transform::from_xyz(-rad * 10.0, rad * 10.0, 0.0),
                        Collider::cuboid(rad, rad * 10.0),
                    ));
                });
        }

        offset -= 0.05 * rad * (num as f32 - 1.0);
    }
}


fn rope_joint(mut commands: Commands) {
    let rad = 100.0;
    let rope_length = rad * 10.0;

    let parent = commands
        .spawn((
            Transform::from_xyz(0.0, 0.0, 0.0),
            RigidBody::Fixed,
            Collider::cuboid(rad, rad),
        ))
        .id();

    let joint = RopeJointBuilder::new(300.).local_anchor2(Vec2::new(rad, 0.0));

    commands
        .spawn((
            Transform::from_xyz(-rad * 2.0, 0.0, 0.0),
            RigidBody::Dynamic,
            Collider::cuboid(rad, rad),
        ))
        .insert(ImpulseJoint::new(parent, joint));
}

fn multi_joints(mut commands: Commands) {
    // Build the rigid body.
    let rad = 4.0;
    let numi = 40; // Num vertical nodes.
    let numk = 40; // Num horizontal nodes.
    let shift = 10.0;

    let mut body_entities = Vec::new();

    for k in 0..numk {
        for i in 0..numi {
            let fk = k as f32;
            let fi = i as f32;

            let rigid_body = if i == 0 && (k % 4 == 0 || k == numk - 1) {
                RigidBody::Fixed
            } else {
                RigidBody::Dynamic
            };

            let child_entity = commands
                .spawn((
                    Transform::from_xyz(fk * shift, -fi * shift, 0.0),
                    rigid_body,
                    Collider::cuboid(rad, rad),
                ))
                .id();

            // Vertical joint.
            if i > 0 {
                let parent_entity = *body_entities.last().unwrap();
                let joint = RevoluteJointBuilder::new().local_anchor2(Vec2::new(0.0, shift));
                commands.entity(child_entity).with_children(|cmd| {
                    // NOTE: we want to attach multiple impulse joints to this entity, so
                    //       we need to add the components to children of the entity. Otherwise
                    //       the second joint component would just overwrite the first one.
                    cmd.spawn(ImpulseJoint::new(parent_entity, joint));
                });
            }

            // Horizontal joint.
            if k > 0 {
                let parent_index = body_entities.len() - numi;
                let parent_entity = body_entities[parent_index];
                let joint = RevoluteJointBuilder::new().local_anchor2(Vec2::new(-shift, 0.0));
                commands.entity(child_entity).with_children(|cmd| {
                    // NOTE: we want to attach multiple impulse joints to this entity, so
                    //       we need to add the components to children of the entity. Otherwise
                    //       the second joint component would just overwrite the first one.
                    cmd.spawn(ImpulseJoint::new(parent_entity, joint));
                });
            }

            body_entities.push(child_entity);
        }
    }
}