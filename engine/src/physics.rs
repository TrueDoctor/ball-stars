use std::time::Instant;

use rapier3d::{
    dynamics::{
        CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet,
        RigidBodySet,
    },
    geometry::{ColliderSet, DefaultBroadPhase, NarrowPhase},
    math::Vec3,
    pipeline::PhysicsPipeline,
};

pub struct Physics {
    pub colliders: ColliderSet,
    pub rigid_bodies: RigidBodySet,
    gravity: Vec3,
    integration_parameters: IntegrationParameters,
    accumulator: f32,
    last_eval: Instant,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
}

impl Default for Physics {
    fn default() -> Self {
        let colliders = ColliderSet::new();
        let rigid_bodies = RigidBodySet::new();
        // let gravity = Vec3::new(0.0, -9.81, 0.0);
        let gravity = Vec3::new(0.0, -9.81, 0.0);
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();

        Self {
            colliders,
            rigid_bodies,
            gravity,
            integration_parameters,
            accumulator: 0.0,
            physics_pipeline,
            island_manager,
            last_eval: Instant::now(),
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
        }
    }
}

impl Physics {
    /// The fixed size of a single physics step, in seconds.
    pub fn timestep(&self) -> f32 {
        self.integration_parameters.dt
    }

    pub fn advance(&mut self) {
        let frame_time = self.last_eval.elapsed();
        self.last_eval = Instant::now();
        self.accumulator += frame_time.as_secs_f32();
        let times = self.accumulator.div_euclid(self.timestep()) as u32;
        self.accumulator = self.accumulator.rem_euclid(self.timestep());
        for _ in 0..times {
            self.simulate_step();
        }
    }

    fn simulate_step(&mut self) {
        self.physics_pipeline.step(
            self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_bodies,
            &mut self.colliders,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &(),
            &(),
        );
    }
}
