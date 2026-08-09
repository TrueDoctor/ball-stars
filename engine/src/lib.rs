use std::error::Error;

use rapier3d::{dynamics::RigidBodySet, geometry::ColliderSet, math::Vec3};
use rapier3d_meshloader::LoadedShape;

struct World {
    level: Mesh,
    player: Player,
    colliders: ColliderSet,
    rigid_bodies: RigidBodySet,
}

impl World {
    fn new(path: &str) -> Self {
        let mesh = load_geometry(path).unwrap();

        let mut colliders = ColliderSet::new();

        // for shape in mesh.shapes {
        //     colliders.insert(shape.shape);
        // }
        todo!()
    }
}

struct Player {
    pos: Vec3,
    acc: Vec3,
    vel: Vec3,
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
