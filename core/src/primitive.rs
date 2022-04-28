use crate::math::Vertex;
use image::GenericImageView;
use miniquad::{Buffer, BufferType, Context, FilterMode, Texture};
use nalgebra::{Vector2, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Model {
    pub texture: Option<String>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i16>,
}

impl Model {
    pub fn quad(texture: Option<String>) -> Self {
        Self {
            texture,
            vertices: vec![
                Vertex {
                    pos: Vector3::new(-0.5, -0.5, 0.0),
                    uv: Vector2::new(0., 0.),
                },
                Vertex {
                    pos: Vector3::new(0.5, -0.5, 0.0),
                    uv: Vector2::new(1., 0.),
                },
                Vertex {
                    pos: Vector3::new(0.5, 0.5, 0.0),
                    uv: Vector2::new(1., 1.),
                },
                Vertex {
                    pos: Vector3::new(-0.5, 0.5, 0.0),
                    uv: Vector2::new(0., 1.),
                },
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
        }
    }

    pub fn into_buffers(
        &self,
        ctx: &mut Context,
    ) -> (Buffer, Buffer, Option<Texture>) {
        let vertex_buffer =
            Buffer::immutable(ctx, BufferType::VertexBuffer, &self.vertices);
        let index_buffer =
            Buffer::immutable(ctx, BufferType::IndexBuffer, &self.indices);

        let texture: Option<Texture> = None;

        if let Some(texture_file) = &self.texture {
            let image = image::open(texture_file).unwrap();
            let texture = Texture::from_rgba8(
                ctx,
                (image.dimensions().0) as u16,
                (image.dimensions().1) as u16,
                image.to_rgba8().as_raw(),
            );
            texture.set_filter(ctx, FilterMode::Nearest);
        }

        (vertex_buffer, index_buffer, texture)
    }
}
