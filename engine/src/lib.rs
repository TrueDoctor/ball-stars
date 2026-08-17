use std::{error::Error, fmt::Debug};
mod physics;

use glam::Quat;
use rapier3d::{
    dynamics::{RigidBodyBuilder, RigidBodyHandle},
    geometry::ColliderBuilder,
    math::Vec3,
};
use rapier3d_meshloader::LoadedShape;

use crate::physics::Physics;

pub struct World {
    level: Mesh,
    player: Player,
    physics: Physics,
}

impl World {
    pub fn new(path: &str) -> Self {
        let mut physics = Physics::default();
        let mesh = load_geometry(path).unwrap();

        for shape in &mesh.shapes {
            let collider = ColliderBuilder::new(shape.shape.clone()).restitution(0.7);
            physics.colliders.insert(collider);
        }

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(Vec3::new(0.0, 10.0, 0.0))
            .build();

        let collider = ColliderBuilder::ball(1.84).restitution(0.7).build();
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

    pub fn player_position(&self) -> Vec3 {
        let ball = &self.physics.rigid_bodies[self.player.handle];
        ball.translation()
    }
    pub fn player_rotation(&self) -> Quat {
        let ball = &self.physics.rigid_bodies[self.player.handle];
        *ball.rotation()
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
        self.physics.advance();
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
