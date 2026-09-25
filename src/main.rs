mod browser_app;
mod icons;
mod user_data;

use eframe::egui;

use browser_app::BrowserApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1800.0, 1100.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Dark Horse Browser",
        options,
        Box::new(|_creation_context| Ok(Box::<BrowserApp>::default())),
    )
}
