use dyadikos::{math::Transform, primitive::Model, stage::Stage};
use egui_miniquad::EguiMq;
use legion::IntoQuery;
use miniquad as mq;

struct Editor {
    stage: Stage,
    gui_ctx: EguiMq,
}

impl Editor {
    pub fn new(ctx: &mut mq::Context) -> Self {
        let stage = Stage::new(ctx);
        let gui_ctx = EguiMq::new(ctx);

        Self { stage, gui_ctx }
    }
}

impl miniquad::EventHandler for Editor {
    fn draw(&mut self, ctx: &mut mq::Context) {
        self.stage.draw(ctx);

        self.gui_ctx.run(ctx, |egui_ctx| {
            egui::Window::new("Entities").show(egui_ctx, |ui| {
                if ui.button("+").clicked() {
                    self.stage
                        .world
                        .push((Transform::default(), Model::quad(None)));
                }

                let mut query = <(&mut Transform, &mut Model)>::query();

                for (i, (transform, model)) in
                    query.iter_mut(&mut self.stage.world).enumerate()
                {
                    egui::CollapsingHeader::new(i.to_string())
                        .selectable(true)
                        .show(ui, |ui| {
                            egui::CollapsingHeader::new("Position").show(
                                ui,
                                |ui| {
                                    ui.add(
                                        egui::DragValue::new(
                                            &mut transform.position[0],
                                        )
                                        .speed(0.01),
                                    );
                                    ui.add(
                                        egui::DragValue::new(
                                            &mut transform.position[1],
                                        )
                                        .speed(0.01),
                                    );
                                    ui.add(
                                        egui::DragValue::new(
                                            &mut transform.position[2],
                                        )
                                        .speed(0.01),
                                    );
                                },
                            );
                        });
                }
            });
        });

        self.gui_ctx.draw(ctx);
    }

    fn update(&mut self, ctx: &mut mq::Context) {
        self.stage.update(ctx);
    }

    fn mouse_motion_event(&mut self, ctx: &mut mq::Context, x: f32, y: f32) {
        self.gui_ctx.mouse_motion_event(ctx, x, y);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut mq::Context, dx: f32, dy: f32) {
        self.gui_ctx.mouse_wheel_event(ctx, dx, dy);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.gui_ctx.mouse_button_down_event(ctx, mb, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.gui_ctx.mouse_button_up_event(ctx, mb, x, y);
    }

    fn char_event(
        &mut self,
        _ctx: &mut mq::Context,
        character: char,
        _keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.gui_ctx.char_event(character);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut mq::Context,
        keycode: mq::KeyCode,
        keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.gui_ctx.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut mq::Context,
        keycode: mq::KeyCode,
        keymods: mq::KeyMods,
    ) {
        self.gui_ctx.key_up_event(keycode, keymods);
    }
}

fn main() {
    mq::start(
        mq::conf::Conf {
            window_title: "Dyadikos Editor".to_string(),
            ..Default::default()
        },
        |mut ctx| {
            let editor = Editor::new(&mut ctx);

            mq::UserData::owning(editor, ctx)
        },
    );
}
