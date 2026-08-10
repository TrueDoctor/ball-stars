use std::error::Error;

use rapier3d::{
    dynamics::{
        CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet,
        RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
    },
    geometry::{ColliderBuilder, ColliderSet, DefaultBroadPhase, NarrowPhase},
    math::Vec3,
    pipeline::PhysicsPipeline,
};
use rapier3d_meshloader::LoadedShape;

pub struct World {
    level: Mesh,
    player: Player,
    physics: Physics,
}

struct Physics {
    colliders: ColliderSet,
    rigid_bodies: RigidBodySet,
    gravity: Vec3,
    integration_parameters: IntegrationParameters,
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
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
        }
    }
}

impl Physics {
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

impl World {
    pub fn new(path: &str) -> Self {
        let mut physics = Physics::default();
        let mesh = load_geometry(path).unwrap();

        for shape in &mesh.shapes {
            let collider = ColliderBuilder::new(shape.shape.clone());
            physics.colliders.insert(collider);
        }

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(Vec3::new(0.0, 10.0, 0.0))
            .build();

        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let ball_body_handle = physics.rigid_bodies.insert(rigid_body);
        physics
            .colliders
            .insert_with_parent(collider, ball_body_handle, &mut physics.rigid_bodies);

        World {
            level: mesh,
            player: Player {
                handle: ball_body_handle,
            },
            physics,
        }
    }

    pub fn player_position(&mut self) -> Vec3 {
        let ball = &self.physics.rigid_bodies[self.player.handle];
        ball.translation()
    }
    pub fn set_player_position(&mut self, pos: Vec3) {
        let ball = &mut self.physics.rigid_bodies[self.player.handle];
        ball.set_translation(pos, true);
    }
    pub fn apply_impulse(&mut self, impulse: Vec3) {
        let ball = &mut self.physics.rigid_bodies[self.player.handle];
        ball.apply_impulse(impulse, true);
    }
    pub fn simulate_step(&mut self) {
        self.physics.simulate_step();
    }
}

pub struct Player {
    handle: RigidBodyHandle,
}

pub fn load_geometry(path: &str) -> Result<Mesh, Box<dyn Error>> {
    let mut shapes = vec![];
    for mesh in rapier3d_meshloader::load_from_path(
        path,
        &rapier3d::geometry::MeshConverter::TriMesh,
        Vec3::ONE,
    )? {
        shapes.push(mesh?);
    }
    Ok(Mesh { shapes })
}

pub struct Mesh {
    shapes: Vec<LoadedShape>,
}
