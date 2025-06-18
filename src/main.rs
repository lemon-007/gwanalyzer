// Imports
use eframe::*;
use egui_extras::*;
use std::time::Duration;
use std::thread;

// Resource files
mod image_handler;
mod app_elements;

struct GWeather {
    pub status: String
}

impl GWeather {
    fn default() -> Self {
        Self {
            status: String::from("Intro_reading"),
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

    // Sets timed intro, that's about it. Figuring things out.
    let gw = GWeather::default();
    thread::spawn(move || {
        println!("GW stage: {}", gw.status.clone());
    });

    // Runs here with options
    run_native(
        "GW ANALYZER",
        options,
        Box::new(|cc|{
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(gw))
        })
    )
}
