use dyadikos::Stage;
use miniquad::UserData;

fn main() {
    miniquad::start(miniquad::conf::Conf::default(), |mut ctx| {
        UserData::owning(
            Stage::new(
                &mut ctx,
                128,
                // Perform updates here
                Box::new(move |transform| {
                    transform.position.y -= 0.001;
                }),
            ),
            ctx,
        )
    });
}
