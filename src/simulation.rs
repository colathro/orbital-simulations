use bevy::{core::FixedTimestep, prelude::*};
use rug::Float;

pub const LABEL: &str = "SIMULATION_TIMESTEP";

const GRAVITATIONAL_CONSTANT: f32 = 6.674e-10_f32;

const DEFAULT_PRECISION: u32 = 128;

#[derive(Component)]
pub struct Simulated;

/// Entities with this component are targeted by the gravity caluclation system.
#[derive(Component)]
pub struct Gravity;

#[derive(Component)]
pub struct Rotating {
    pub degrees_per_second: Float,
}

/// Entities with this component have mass, which is required for gravity calculations.
#[derive(Component)]
pub struct PhysicalProperties {
    pub mass: Float,
    pub estimated_radius: Float,
    pub acceleration: HPVec3,
    pub translation: HPVec3,
}

/// High Precision Vec3 for floating point calculations.
#[derive(Clone, Debug)]
pub struct HPVec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl HPVec3 {
    pub fn add(a: &HPVec3, b: &HPVec3) -> HPVec3 {
        HPVec3::new(
            Float::with_val(DEFAULT_PRECISION, &a.x + &b.x),
            Float::with_val(DEFAULT_PRECISION, &a.y + &b.y),
            Float::with_val(DEFAULT_PRECISION, &a.z + &b.z),
        )
    }

    pub fn add_self(&mut self, b: &HPVec3) {
        self.x = Float::with_val(DEFAULT_PRECISION, &self.x + &b.x);
        self.y = Float::with_val(DEFAULT_PRECISION, &self.y + &b.y);
        self.z = Float::with_val(DEFAULT_PRECISION, &self.z + &b.z);
    }

    pub fn sub(a: &HPVec3, b: &HPVec3) -> HPVec3 {
        HPVec3::new(
            Float::with_val(DEFAULT_PRECISION, &a.x - &b.x),
            Float::with_val(DEFAULT_PRECISION, &a.y - &b.y),
            Float::with_val(DEFAULT_PRECISION, &a.z - &b.z),
        )
    }

    pub fn scalar_mul(a: &HPVec3, b: &Float) -> HPVec3 {
        HPVec3::new(
            Float::with_val(DEFAULT_PRECISION, &a.x * b),
            Float::with_val(DEFAULT_PRECISION, &a.y * b),
            Float::with_val(DEFAULT_PRECISION, &a.z * b),
        )
    }

    pub fn distance(&mut self, b: &HPVec3) -> Float {
        (Float::with_val(DEFAULT_PRECISION, &self.x - &b.x).square()
            + Float::with_val(DEFAULT_PRECISION, &self.y - &b.y).square()
            + Float::with_val(DEFAULT_PRECISION, &self.z - &b.z).square())
        .sqrt()
    }

    pub fn normalize(&self) -> HPVec3 {
        let magnitude = (Float::with_val(DEFAULT_PRECISION, self.x.square_ref())
            + Float::with_val(DEFAULT_PRECISION, self.y.square_ref())
            + Float::with_val(DEFAULT_PRECISION, self.z.square_ref()))
        .sqrt();

        HPVec3 {
            x: Float::with_val(DEFAULT_PRECISION, &self.x / &magnitude),
            y: Float::with_val(DEFAULT_PRECISION, &self.y / &magnitude),
            z: Float::with_val(DEFAULT_PRECISION, &self.z / &magnitude),
        }
    }

    #[inline(always)]
    pub fn new(x: Float, y: Float, z: Float) -> HPVec3 {
        HPVec3 { x, y, z }
    }

    pub fn zero() -> HPVec3 {
        HPVec3 {
            x: Float::with_val(DEFAULT_PRECISION, 0.0),
            y: Float::with_val(DEFAULT_PRECISION, 0.0),
            z: Float::with_val(DEFAULT_PRECISION, 0.0),
        }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x.to_f32(), self.y.to_f32(), self.z.to_f32())
    }

    pub fn from_vec3(vec: &Vec3) -> HPVec3 {
        HPVec3 {
            x: Float::with_val(DEFAULT_PRECISION, vec.x),
            y: Float::with_val(DEFAULT_PRECISION, vec.y),
            z: Float::with_val(DEFAULT_PRECISION, vec.z),
        }
    }
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
        /*         app.add_stage_after(
            CoreStage::Update,
            SimulationUpdateStage,
            SystemStage::single_threaded()
                .with_run_criteria(FixedTimestep::step(1. / 10000.).with_label(LABEL))
                .with_system(simulation_step.exclusive_system()),
        ); */
        app.add_system(simulation_step);
        app.add_system(rotation_step);
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
        let distance = a_properties.translation.distance(&b_properties.translation);

        // get the normalized direction vectors
        let ab_direction_vec =
            HPVec3::sub(&b_properties.translation, &a_properties.translation).normalize();
        let ba_direction_vec =
            HPVec3::sub(&a_properties.translation, &b_properties.translation).normalize();

        // get the force between the two simulated entities.
        let force =
            (GRAVITATIONAL_CONSTANT * a_properties.mass.clone() * b_properties.mass.clone())
                / distance.square();

        // find the acceleration
        let a_acceleration = force.clone() / a_properties.mass.clone();
        let b_acceleration = force.clone() / b_properties.mass.clone();

        let a_acceleration_vec = HPVec3::scalar_mul(&ab_direction_vec, &a_acceleration);
        let b_acceleration_vec = HPVec3::scalar_mul(&ba_direction_vec, &b_acceleration);

        // apply previous acceleration
        if let Ok(ref_entity) = reference {
            if ref_entity != a_entity {
                a_properties.translation =
                    HPVec3::add(&a_properties.translation, &a_properties.acceleration);
            }

            if ref_entity != b_entity {
                b_properties.translation =
                    HPVec3::add(&b_properties.translation, &b_properties.acceleration);
            }
        }

        // set new acceleration
        a_properties.acceleration.add_self(&a_acceleration_vec);
        b_properties.acceleration.add_self(&b_acceleration_vec);

        // engine floats are not precise enough for the calculations
        // but precise enough to render visuals :D
        a_transform.translation = a_properties.translation.to_vec3();
        b_transform.translation = b_properties.translation.to_vec3();
    }
}

fn rotation_step(mut rot_query: Query<(&Rotating, &mut Transform), With<Rotating>>) {
    for (rot, mut transform) in rot_query.iter_mut() {
        let mut euler_rot = transform.rotation.to_euler(EulerRot::ZXY);
        euler_rot.2 += rot.degrees_per_second.to_f32();
        transform.rotation = Quat::from_euler(EulerRot::ZXY, euler_rot.0, euler_rot.1, euler_rot.2)
    }
}
