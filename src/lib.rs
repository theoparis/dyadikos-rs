use glam::{vec2, vec3};
use math::{Transform, Vertex};
use miniquad::{
    Bindings, Buffer, BufferLayout, BufferType, Context, EventHandler,
    FilterMode, Pipeline, Shader, Texture, VertexAttribute, VertexFormat,
};
use rand::random;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub mod math;

pub struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    transform: Transform,
    update_fn: Box<dyn FnMut(&mut Transform)>,
}

impl Stage {
    pub fn new(
        ctx: &mut Context,
        texture_size: u32,
        update_fn: Box<dyn FnMut(&mut Transform)>,
    ) -> Stage {
        let vertices = vec![
            Vertex {
                pos: vec3(-0.5, -0.5, 0.0),
                uv: vec2(0., 0.),
            },
            Vertex {
                pos: vec3(0.5, -0.5, 0.0),
                uv: vec2(1., 0.),
            },
            Vertex {
                pos: vec3(0.5, 0.5, 0.0),
                uv: vec2(1., 1.),
            },
            Vertex {
                pos: vec3(-0.5, 0.5, 0.0),
                uv: vec2(0., 1.),
            },
        ];
        let vertex_buffer =
            Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices = vec![0, 1, 2, 0, 2, 3];
        let index_buffer =
            Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let pixels: Vec<u8> = (0..(texture_size.pow(2)))
            .into_par_iter()
            .map(|_| random::<u8>())
            .collect();

        let texture = Texture::from_rgba8(
            ctx,
            (texture_size / 2) as u16,
            (texture_size / 2) as u16,
            &pixels,
        );
        texture.set_filter(ctx, FilterMode::Nearest);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader =
            Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta());

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        let transform = Transform::default();

        Stage {
            pipeline,
            bindings,
            vertices,
            indices,
            transform,
            update_fn,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {
        (self.update_fn)(&mut self.transform);
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&shader::Uniforms {
            transform: self.transform.get_matrix(),
        });
        ctx.draw(0, self.indices.len() as i32, 1);
        ctx.end_render_pass();

        ctx.commit_frame();
    }
}

mod shader {
    use glam::Mat4;
    use miniquad::{ShaderMeta, UniformBlockLayout, UniformType};

    pub const VERTEX: &str = r#"#version 330 core
    layout(location = 0) in vec3 pos;
    layout(location = 1) in vec2 uv;

    uniform mat4 transform;

    out vec2 texcoord;

    void main() {
        gl_Position = transform * vec4(pos, 1);
        //gl_Position = vec4(pos, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 330 core
    in vec2 texcoord;

    uniform sampler2D tex;

    out vec4 fragColor;

    void main() {
        fragColor = texture2D(tex, texcoord);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: &["tex"],
            uniforms: UniformBlockLayout {
                uniforms: &[("transform", UniformType::Mat4)],
            },
        }
    }
    pub struct Uniforms {
        pub transform: Mat4,
    }
}
