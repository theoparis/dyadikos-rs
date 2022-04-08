use dyadikos::{math::Transform, primitive::Model, stage::Stage};
use miniquad::{Context, EventHandler, UserData};

pub struct App {
    stage: Stage,
}

impl App {
    pub fn new(ctx: &mut Context) -> Self {
        let mut stage = Stage::new(ctx);

        {
            let mut transform = Transform::default();
            transform.position.x = 1.0;

            stage.world.spawn((transform, Model::quad(None)));
        }

        {
            let mut transform = Transform::default();
            transform.position.x = -1.0;

            stage.world.spawn((transform, Model::quad(None)));
        }

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
        UserData::owning(App::new(&mut ctx), ctx)
    });
}
