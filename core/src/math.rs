use nalgebra::{Matrix4, UnitQuaternion, Vector2, Vector3};
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub pos: Vector3<f32>,
    pub uv: Vector2<f32>,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub scale: Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector3::default(),
            rotation: UnitQuaternion::default(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Transform {
    pub fn get_matrix(&self) -> Matrix4<f32> {
        let translation = Matrix4::new_translation(&self.position);
        let rotation = self.rotation.to_homogeneous();
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);

        translation * rotation * scale
    }
}
