// This file holds each pannel you might see when using the app.
// All funcitons are used in building of main.rs

use egui::*;

pub fn intro(ctx: &egui::Context) -> egui::InnerResponse<()> {
    CentralPanel::default().show(ctx, |ui| {
        ui.centered_and_justified(|uix| {
            uix.heading("GWeather");
        });
    })
}
