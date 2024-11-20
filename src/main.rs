use eframe::egui::Align;
use eframe::egui::CentralPanel;
use eframe::egui::Context;
use eframe::egui::Layout;
use eframe::egui::ViewportBuilder;

pub struct App {}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE);

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    for item in [
                        "First".to_string(),
                        "Second".to_string(),
                        "Third".to_string(),
                    ] {
                        if ui
                            .add(eframe::egui::SelectableLabel::new(
                                "First" == item.to_string(),
                                item.to_string(),
                            ))
                            .on_hover_text(<String as Into<String>>::into(item.clone()))
                            .clicked()
                        {};
                    }
                })
            });
        });
    }
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = ViewportBuilder::default().with_always_on_top();

    let _ = eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}
