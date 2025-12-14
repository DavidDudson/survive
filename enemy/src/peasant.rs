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
#[require(Enemy, Name, Textured, Health, Speed, Attack)]
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
        commands
            .spawn((
                Draggable,
                Peasant,
                Name("Peasant".to_string()),
                Textured {
                    file: "enemy/Enemy.png".to_string(),
                },
                Health(5),
                Speed(100.),
                Attack::melee(Health(1)),
                Mesh2d(meshes.add(Rectangle::new(64., 64.))),
                MeshMaterial2d(materials.add(Color::from(WHITE))),
                Transform::from_xyz(-1920. / 2., 0., 0.),
                Collider::cuboid(64. / 2., 64. / 2.),
                LockedAxes::ROTATION_LOCKED,
                ENEMY_COLLISION_GROUP,
                ActiveEvents::CONTACT_FORCE_EVENTS,
                Hardness(1),
            ))
            .insert(Damping {
                linear_damping: 0.5,
                angular_damping: 1.0,
            });
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
