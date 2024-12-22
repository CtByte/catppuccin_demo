use eframe::egui::CentralPanel;
use eframe::egui::Context;
use eframe::egui::ViewportBuilder;

pub struct App {}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for App {
    fn clear_color(&self, _visuals: &eframe::egui::Visuals) -> [f32; 4] {
        eframe::egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default()
            .frame(eframe::egui::Frame::none())
            .show(ctx, |ui| {
                ui.label("The background should be transparent");
            });
    }
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = ViewportBuilder::default()
        .with_transparent(true)
        .with_always_on_top();

    let _ = eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}
