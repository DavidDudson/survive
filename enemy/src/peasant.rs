use crate::enemy::{Enemy, ENEMY_COLLISION_GROUP};
use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::dynamics::ImpulseJoint;
use bevy_rapier2d::dynamics::RevoluteJoint;
use models::attack::Attack;
use models::draggable::Draggable;
use models::grounded;
use models::grounded::Grounded;
use models::hardness::Hardness;
use models::health::Health;
use models::name::Name;
use models::speed::Speed;
use models::textured::Textured;

#[derive(Component)]
pub struct Limb {
    pub parent: Entity,
    pub offset: Vec2,
}

#[derive(Component)]
#[require(
    Enemy,
    Name(peasant_name),
    Textured(peasant_texture),
    Health(peasant_health),
    Speed(peasant_speed),
    Attack(peasant_attack)
)]
pub struct Peasant;

#[derive(Component)]
pub struct WalkingAnimation {
    pub phase: f32,
    pub speed: f32,
}

impl Default for WalkingAnimation {
    fn default() -> Self {
        Self {
            phase: 0.0,
            speed: 2.0,
        }
    }
}

impl Peasant {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        info!("Spawning peasant");
        
        // Spawn main body (thin segment)
        let body = commands.spawn((
            Draggable,
            Peasant,
            Mesh2d(meshes.add(Rectangle::new(8., 32.))),
            MeshMaterial2d(materials.add(Color::from(WHITE))),
            Transform::from_xyz(-1920. / 2., 0., 0.),
            Collider::cuboid(4., 16.),
            LockedAxes::ROTATION_LOCKED,
            ENEMY_COLLISION_GROUP,
            ActiveEvents::CONTACT_FORCE_EVENTS,
            RigidBody::Dynamic,
            Damping {
                linear_damping: 0.8,
                angular_damping: 1.0,
            },
            Hardness(1),
            AdditionalMassProperties::Mass(10.0),
        )).id();

        // Spawn head
        let head = commands.spawn((
            Mesh2d(meshes.add(Circle::new(12.))),
            MeshMaterial2d(materials.add(Color::from(WHITE))),
            Transform::from_xyz(0., 24., 0.),
            Collider::ball(12.),
            ENEMY_COLLISION_GROUP,
            Limb { parent: body, offset: Vec2::new(0., 24.) },
            RigidBody::Dynamic,
            Damping {
                linear_damping: 0.95,
                angular_damping: 1.0,
            },
            AdditionalMassProperties::Mass(12.0),
            LockedAxes::ROTATION_LOCKED,
        )).id();

        // Create revolute joint for head
        let revolute = RevoluteJointBuilder::new()
            .local_anchor1(Vec2::new(0., 16.))
            .local_anchor2(Vec2::new(0., -12.))
            .limits([-std::f32::consts::PI/4., std::f32::consts::PI/4.])
            .build();
        commands.spawn(ImpulseJoint::new(body, revolute));

        // Spawn limbs
        let limb_offsets = [
            Vec2::new(-16., -8.),  // Left arm
            Vec2::new(16., -8.),   // Right arm
            Vec2::new(-8., -24.),  // Left leg
            Vec2::new(8., -24.),   // Right leg
        ];

        for (i, offset) in limb_offsets.iter().enumerate() {
            let limb = commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(6.,16.))),
                MeshMaterial2d(materials.add(Color::from(WHITE))),
                Transform::from_xyz(offset.x, offset.y, 0.),
                Collider::cuboid(3., 16.),
                ENEMY_COLLISION_GROUP,
                Limb { parent: body, offset: *offset },
                RigidBody::Dynamic,
                Damping {
                    linear_damping: 0.9,
                    angular_damping: 1.0,
                },
                AdditionalMassProperties::Mass(3.0),
                WalkingAnimation {
                    phase: if i < 2 { 
                        if i == 0 { std::f32::consts::PI } else { 0.0 }
                    } else {
                        if i == 2 { 0.0 } else { std::f32::consts::PI }
                    },
                    speed: 6.0,
                },
            )).id();

            // Create revolute joint for limb
            let revolute = RevoluteJointBuilder::new()
                .local_anchor1(*offset)
                .local_anchor2(Vec2::ZERO)
                .limits([-std::f32::consts::PI/2., std::f32::consts::PI/2.])
                .build();
            commands.spawn(ImpulseJoint::new(body, revolute));
        }
    }
}

fn peasant_name() -> Name {
    Name("Peasant".to_string())
}

fn peasant_health() -> Health {
    Health(5)
}

fn peasant_speed() -> Speed {
    Speed(100.)
}

fn peasant_attack() -> Attack {
    Attack::melee(Health(1))
}

fn peasant_texture() -> Textured {
    Textured {
        file: "enemy/Enemy.png".to_string(),
    }
}

pub fn check_grounded(
    mut commands: Commands,
    mut peasant_query: Query<(Entity, &Transform, &Velocity, &Option<Grounded>), With<Peasant>>,
) {
    if let Ok(ground_transform) = ground_query.get_single() {
        for (entity, transform, velocity, grounded) in &mut peasant_query {
            let is_grounded = transform.translation.y - 16.0 <= ground_transform.translation.y + 500.0
                && velocity.linvel.y.abs() < 0.1;
            
            if is_grounded {
                if (grounded.is_none()) {
                    commands.entity(entity).insert(Grounded);
                }
            } else {
                if (grounded.is_some()) {
                    commands.entity(entity).remove(Grounded);
                }
            }
        }
    }
}

pub fn animate_limbs(
    time: Res<Time>,
    peasant_query: Query<&Grounded, With<Peasant>>,
    mut limb_query: Query<(&mut Transform, &mut WalkingAnimation), With<Limb>>,
) {
    if let Ok(grounded) = peasant_query.get_single() {
        for (mut transform, mut animation) in &mut limb_query {
            if grounded.0 {
                animation.phase += animation.speed * time.delta_secs();
                transform.rotation = Quat::from_rotation_z(animation.phase.sin() * 0.5);
            } else {
                transform.rotation = Quat::IDENTITY;
            }
        }
    }
}
