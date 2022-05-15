use bevy::{core::FixedTimestep, prelude::*};
use rug::Float;

const LABEL: &str = "SIMULATION_TIMESTEP";

const GRAVITATIONAL_CONSTANT: f32 = 6.674e-10_f32;

#[derive(Component)]
pub struct Simulated(pub String);

/// Entities with this component are targeted by the gravity caluclation system.
#[derive(Component)]
pub struct Gravity;

/// Entities with this component have mass, which is required for gravity calculations.
#[derive(Component)]
pub struct PhysicalProperties {
    pub mass: Float,
    pub estimated_radius: Float,
    pub acceleration: Vec3,
}

/// Forces are not applied to this object, but it's physical properties can still be simulated.
#[derive(Component)]
pub struct ReferenceFrame;

/// ECS Stage that represents when the simulation step is calculated.
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct SimulationUpdateStage;

/// ECS Plugin used to encapsulate the simulation update at a fixed timestep.
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::Update,
            SimulationUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(1. / 10.).with_label(LABEL))
                .with_system(simulation_step.exclusive_system()),
        );
    }
}

pub fn simulation_step(
    mut sim_query: Query<(&mut Transform, &mut PhysicalProperties, Entity), With<Simulated>>,
    ref_query: Query<Entity, With<ReferenceFrame>>,
) {
    let mut combinations = sim_query.iter_combinations_mut();

    let reference = ref_query.get_single();

    while let Some(
        [(mut a_transform, mut a_properties, a_entity), (mut b_transform, mut b_properties, b_entity)],
    ) = combinations.fetch_next()
    {
        // grab the distance between the physical objects
        let distance = a_transform.translation.distance(b_transform.translation);

        // get the normalized direction vectors
        let ab_direction_vec = (b_transform.translation - a_transform.translation).normalize();
        let ba_direction_vec = (a_transform.translation - b_transform.translation).normalize();

        // get the force between the two simulated entities.
        let force =
            (GRAVITATIONAL_CONSTANT * a_properties.mass.clone() * b_properties.mass.clone())
                / Float::with_val(128, distance.powi(2));

        // find the acceleration
        let a_acceleration =
            (force.clone() * a_properties.mass.clone()) / a_properties.estimated_radius.clone();
        let b_acceleration =
            (force.clone() * b_properties.mass.clone()) / b_properties.estimated_radius.clone();

        println!("{:?} {:?}", force.clone(), a_properties.mass);

        let a_acceleration_vec = ab_direction_vec * a_acceleration.to_f32();
        let b_acceleration_vec = ba_direction_vec * b_acceleration.to_f32();

        // apply previous acceleration
        if let Ok(ref_entity) = reference {
            if ref_entity != a_entity {
                a_transform.translation = a_transform.translation + a_properties.acceleration;
            }

            if ref_entity != b_entity {
                b_transform.translation = b_transform.translation + b_properties.acceleration;
            }
        }

        // set new acceleration
        a_properties.acceleration = a_acceleration_vec;
        b_properties.acceleration = b_acceleration_vec;
    }
}
