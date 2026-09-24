use eframe::egui;

#[derive(Default)]
struct BrowserApp {
    address: String,
}

impl eframe::App for BrowserApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Address:");

                ui.add(
                    egui::TextEdit::singleline(&mut self.address)
                        .hint_text("https://example.com")
                        .desired_width(f32::INFINITY),
                );
            });

            ui.separator();
            ui.label("Web page area");
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Dark Horse Browser",
        options,
        Box::new(|_creation_context| Ok(Box::<BrowserApp>::default())),
    )
}