use dyadikos::stage::Stage;
use egui_miniquad::EguiMq;
use egui_nodes::{AttributeFlags, LinkArgs, NodeConstructor};
use miniquad as mq;

struct Editor {
    stage: Stage,
    gui_ctx: EguiMq,
    node_ctx: egui_nodes::Context,
    links: Vec<(usize, usize)>,
}

impl Editor {
    pub fn new(ctx: &mut mq::Context) -> Self {
        let stage = Stage::new(ctx);
        let gui_ctx = EguiMq::new(ctx);
        let node_ctx = egui_nodes::Context::default();

        Self {
            stage,
            gui_ctx,
            node_ctx,
            links: vec![],
        }
    }
}

impl miniquad::EventHandler for Editor {
    fn draw(&mut self, ctx: &mut mq::Context) {
        self.stage.draw(ctx);

        self.gui_ctx.run(ctx, |egui_ctx| {
            let nodes = vec![
                NodeConstructor::new(0, Default::default())
                    .with_title(|ui| ui.label("Example Node A"))
                    .with_input_attribute(
                        0,
                        egui_nodes::PinArgs {
                            flags: Some(
                                AttributeFlags::EnableLinkDetachWithDragClick
                                    as usize,
                            ),
                            ..Default::default()
                        },
                        |ui| ui.label("Input"),
                    )
                    .with_output_attribute(
                        2,
                        egui_nodes::PinArgs {
                            flags: Some(
                                AttributeFlags::EnableLinkDetachWithDragClick
                                    as usize,
                            ),
                            ..Default::default()
                        },
                        |ui| ui.label("Output"),
                    ),
                NodeConstructor::new(1, Default::default())
                    .with_title(|ui| ui.label("Example Node B"))
                    .with_output_attribute(
                        3,
                        egui_nodes::PinArgs {
                            flags: Some(
                                AttributeFlags::EnableLinkDetachWithDragClick
                                    as usize,
                            ),
                            ..Default::default()
                        },
                        |ui| ui.label("Output"),
                    )
                    .with_input_attribute(
                        4,
                        egui_nodes::PinArgs {
                            flags: Some(
                                AttributeFlags::EnableLinkDetachWithDragClick
                                    as usize,
                            ),
                            ..Default::default()
                        },
                        |ui| ui.label("Input"),
                    ),
            ];

            egui::Window::new("Entities").show(egui_ctx, |ui| {
                self.node_ctx.show(
                    nodes,
                    self.links.iter().enumerate().map(|(i, (start, end))| {
                        (i, *start, *end, LinkArgs::default())
                    }),
                    ui,
                );
            });

            // remove destroyed links
            if let Some(idx) = self.node_ctx.link_destroyed() {
                self.links.remove(idx);
            }

            // add created links
            if let Some((start, end, _)) = self.node_ctx.link_created() {
                self.links.push((start, end))
            }
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
    mq::start(mq::conf::Conf::default(), |mut ctx| {
        let editor = Editor::new(&mut ctx);

        mq::UserData::owning(editor, ctx)
    });
}
