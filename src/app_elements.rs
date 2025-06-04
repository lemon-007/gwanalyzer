// This file holds each pannel you might see when using the app.
// All funcitons are used in building of main.rs

use egui::*;

// Intro panel. Shows "GW ANALYZER" at the beginning with the logo for 1 second.
pub fn intro(ctx: &egui::Context) -> egui::InnerResponse<()> {
    CentralPanel::default().show(ctx, |ui| {
        //TODO: turn the screen size into a rectangle,
        //half each pos2
        //create new rectangle with positions
        //put widgets in new positions
        //
        //ctx.screen_size
        //ui.allocate_ui_at_rect(Rect, ui)
    })
}

pub fn get_screen_size() -> Rect {}
