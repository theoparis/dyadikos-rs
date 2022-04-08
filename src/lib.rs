use math::Transform;
use miniquad::{
    Bindings, BufferLayout, Context, EventHandler, Pipeline, Shader, Texture,
    VertexAttribute, VertexFormat,
};
use primitive::Model;

pub mod math;
pub mod primitive;

pub struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    model: Model,
    transform: Transform,
    update_fn: Box<dyn FnMut(&mut Transform)>,
}

impl Stage {
    pub fn new(
        ctx: &mut Context,
        model: Model,
        update_fn: Box<dyn FnMut(&mut Transform)>,
    ) -> Stage {
        let (vertex_buffer, index_buffer, texture) = model.into_buffers(ctx);
        let mut images = vec![];

        if let Some(texture) = texture {
            images.push(texture);
        } else {
            let texture = Texture::from_rgba8(ctx, 1, 1, &[255, 255, 255, 255]);

            images.push(texture);
        }

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images,
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
            model,
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
        ctx.draw(0, self.model.indices.len() as i32, 1);
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
