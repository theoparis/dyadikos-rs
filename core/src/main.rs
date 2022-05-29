use dyadikos::{load, stage::Stage};
use miniquad::UserData;

fn main() {
    miniquad::start(miniquad::conf::Conf::default(), |mut ctx| {
        let mut stage = Stage::new(&mut ctx);

        stage.world = load("scene.json").unwrap();

        UserData::owning(stage, ctx)
    });
}
