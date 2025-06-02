// Imports
use egui::CentralPanel;
use eframe::*;
use std::time::Duration;
use std::time::Instant;

// Resource files
mod image_handler;
mod animations;

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
        CentralPanel::default().show(ctx, |ui| {

            // Intro
            if !self.running {
                ctx.request_repaint();
            }

            ui.heading("GWEATHER");
        });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    // Toggles off intro sequence

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

