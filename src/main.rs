// Imports
use eframe::*;

// Resource files
mod image_handler;
mod app_elements;

struct GWeather {}

impl eframe::App for GWeather {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        app_elements::intro(ctx);
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    // App options (size, shape, icon, etc.)
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(image_handler::get_ico()), // Icon
        ..Default::default()
    };

    // Runs here with options
    run_native(
        "GW ANALYZER",
        options,
        Box::new(|_cc|{
            Ok(Box::new(GWeather {}))
        })
    )
}
