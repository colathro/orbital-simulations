use bevy::{
    core::{FixedTimestep, FixedTimesteps},
    prelude::*,
};

const LABEL: &str = "SIMULATION_TIMESTEP";

/// Entities with this component are targeted by the gravity caluclation system.
#[derive(Component)]
pub struct Gravity;

/// Entities with this component have mass, which is required for gravity calculations.
#[derive(Component)]
pub struct Mass;

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
                .with_run_criteria(FixedTimestep::step(1. / 30.).with_label(LABEL))
                .with_system(simulation_step),
        );
    }
}

pub fn simulation_step(
    mut last_time: Local<f64>,
    time: Res<Time>,
    fixed_timesteps: Res<FixedTimesteps>,
) {
}
