// Imports
use egui::CentralPanel;
use eframe::*;

// Resource files
mod image_handler;
mod app_elements;

struct GWeather {
    running: bool,
}

impl GWeather {
    fn default() -> Self {
        Self {
            running: false,
        }
    }
}

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
        "GWeather",
        options,
        Box::new(|_cc|{
            Ok(Box::new(GWeather::default()))
        })
    )
}

