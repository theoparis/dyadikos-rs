use glam::{vec3, Mat4, Quat, Vec2, Vec3};
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub pos: Vec3,
    pub uv: Vec2,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::default(),
            rotation: Quat::default(),
            scale: vec3(1.0, 1.0, 1.0),
        }
    }
}

impl Transform {
    pub fn get_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            self.scale,
            self.rotation,
            self.position,
        )
    }
}
