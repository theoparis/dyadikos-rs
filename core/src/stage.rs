use crate::math::Transform;
use crate::primitive::Model;
use legion::{IntoQuery, Registry, World};
use miniquad::{
    Bindings, Buffer, BufferLayout, Context, EventHandler, Pipeline, Shader,
    Texture, VertexAttribute, VertexFormat,
};

pub fn create_registry() -> Registry<String> {
    let mut registry = Registry::<String>::default();
    registry.register::<Transform>("transform".to_string());
    registry.register::<Model>("model".to_string());

    registry
}

pub struct Stage {
    pipeline: Pipeline,
    pub world: World,
}

impl Stage {
    pub fn spawn_model(
        ctx: &mut Context,
        model: &Model,
    ) -> (Buffer, Buffer, Vec<Texture>) {
        let (vertex_buffer, index_buffer, texture) = model.into_buffers(ctx);
        let mut images = vec![];

        if let Some(texture) = texture {
            images.push(texture);
        } else {
            let texture = Texture::from_rgba8(ctx, 1, 1, &[255, 255, 255, 255]);

            images.push(texture);
        }

        (vertex_buffer, index_buffer, images)
    }

    pub fn new(ctx: &mut Context) -> Stage {
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

        let world = World::default();

        Stage { pipeline, world }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);

        let mut query = <(&Model, &Transform)>::query();
        for (model, transform) in query.iter_mut(&mut self.world) {
            let (vertex_buffer, index_buffer, images) =
                Self::spawn_model(ctx, model);

            let bindings = Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer,
                images,
            };

            ctx.apply_bindings(&bindings);
            ctx.apply_uniforms(&shader::Uniforms {
                transform: transform.get_matrix(),
            });
            ctx.draw(0, model.indices.len() as i32, 1);
        }
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

//mod test {
//fn test_serialize() {
//use super::create_registry;
//use crate::{math::Transform, primitive::Model};
//use legion::{
//serialize::{set_entity_serializer, Canon},
//World,
//};

//let mut world = World::default();

//world.push((Transform::default(), Model::quad(None)));

//let entity_serializer = Canon::default();
//let registry = create_registry();

//let json = set_entity_serializer(&entity_serializer, || {
//// The guid here will match the guid of the first entity we created
//nu_json::to_value(&world.as_serializable(
//legion::any(),
//&registry,
//&entity_serializer,
//))
//.expect("Failed to serialize entity container!")
//});

//panic!("{:?}", json);
//}
//}
