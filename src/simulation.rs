use bevy::{core::FixedTimestep, prelude::*};

const LABEL: &str = "SIMULATION_TIMESTEP";

const GRAVITATIONAL_CONSTANT: f32 = 6.674e-11_f32;

#[derive(Component)]
pub struct Simulated(pub String);

/// Entities with this component are targeted by the gravity caluclation system.
#[derive(Component)]
pub struct Gravity;

/// Entities with this component have mass, which is required for gravity calculations.
#[derive(Component)]
pub struct PhysicalProperties {
    pub mass: f32,
    pub estimated_radius: f32,
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
                .with_run_criteria(FixedTimestep::step(1. / 1000.).with_label(LABEL))
                .with_system(simulation_step),
        );
    }
}

pub fn simulation_step(
    mut sim_query: Query<(&mut Transform, &PhysicalProperties, Entity), With<Simulated>>,
    ref_query: Query<Entity, With<ReferenceFrame>>,
) {
    let mut combinations = sim_query.iter_combinations_mut();
    while let Some(
        [(mut a_transform, a_properties, a_entity), (mut b_transform, b_properties, b_entity)],
    ) = combinations.fetch_next()
    {
        // grab the distance between the physical objects
        let distance = a_transform.translation.distance(b_transform.translation);

        // get the normalized direction vectors
        let ab_direction_vec = (b_transform.translation - a_transform.translation).normalize();
        let ba_direction_vec = (a_transform.translation - b_transform.translation).normalize();

        // get the force between the two simulated entities.
        let force = GRAVITATIONAL_CONSTANT * ((a_properties.mass * b_properties.mass) / distance);

        // find the acceleration
        let a_acceleration = force / a_properties.mass;
        let b_acceleration = force / b_properties.mass;
    }
}
