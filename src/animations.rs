#[warn(unused_imports)]
use egui::Ui;
use std::time::Instant;

pub fn invis_flip(switch: &mut bool) {
    println!("Flipping boolean.");
    *switch = !*switch;
}
