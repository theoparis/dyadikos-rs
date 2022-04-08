use dyadikos::stage::{create_registry, Stage};
use legion::serialize::Canon;
use miniquad::{Context, EventHandler, UserData};
use nu_json::Value;
use serde::de::DeserializeSeed;

pub struct App {
    stage: Stage,
}

impl App {
    pub fn new(ctx: &mut Context, scene_file: &str) -> Self {
        let mut stage = Stage::new(ctx);

        let json: Value = nu_json::from_str(
            &std::fs::read_to_string(scene_file)
                .expect("Failed to read scene file"),
        )
        .expect("Failed to parse scene file");

        let registry = create_registry();
        let entity_serializer = Canon::default();
        let scene = registry
            .as_deserialize(&entity_serializer)
            .deserialize(json)
            .expect("Failed to deserialize world!");

        stage.world = scene;

        Self { stage }
    }
}

impl EventHandler for App {
    fn update(&mut self, ctx: &mut miniquad::Context) {
        self.stage.update(ctx);
    }

    fn draw(&mut self, ctx: &mut miniquad::Context) {
        self.stage.draw(ctx);
    }
}

fn main() {
    miniquad::start(miniquad::conf::Conf::default(), |mut ctx| {
        UserData::owning(App::new(&mut ctx, "scene.hjson"), ctx)
    });
}
