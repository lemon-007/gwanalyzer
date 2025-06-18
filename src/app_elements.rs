// This file holds each pannel you might see when using the app.
// All funcitons are used in building of main.rs

use egui::*;

// Intro panel. Shows "GW ANALYZER" at the beginning with the logo for 1 second.
pub fn intro(ctx: &egui::Context) -> egui::InnerResponse<()> {
    CentralPanel::default().show(ctx, |ui| {
            ui.allocate_ui_at_rect(rect_center_screen(ctx, 2.0), |ui| {
                ui.vertical_centered(|ui| {
                    ui.image(egui::include_image!("assets/globe.png"));
                    ui.heading("GW Analyzer");
                });
        });
    })
}

pub fn dev(ctx: &egui::Context) -> egui::InnerResponse<()> {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Transition");
    })
}

// Creates a rectangle (screen size / size_cut) big
pub fn rect_center_screen(ctx: &egui::Context, size_cut: f32) -> Rect {
    let screen = ctx.screen_rect();
    let size = screen.size();

    let rect = egui::Rect::from_center_size(
        screen.center(),

        Vec2::new(
            size.x / size_cut,
            size.y / size_cut
        ));

    rect
}

//pub fn run(ctx: &egui::Context, stage: &mut runtime) -> egui::InnerResponse<()> {
//    if 
//}
