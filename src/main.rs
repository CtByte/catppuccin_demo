use eframe::{
    egui::{CentralPanel, Context, Frame, Rgba, ViewportBuilder, Visuals},
    App, CreationContext, NativeOptions,
};

pub struct MyApp {}

impl MyApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {}
    }
}

impl App for MyApp {
    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default()
            .frame(Frame::none())
            .show(ctx, |ui| {
                ui.label("The background should be transparent");
            });
    }
}

fn main() {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_transparent(true)
            .with_always_on_top(),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    );
}
