// Todo:
// - create simple list of all already watched anime
// - add new anime
// - remove anime from list

use eframe::egui;
use egui_extras::{TableBuilder};

fn main() {
    eframe::run_native("Test UI", eframe::NativeOptions::default(), Box::new(|cc| Ok(Box::new(App::new(cc)))));
}

#[derive(Default)]
struct App {}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Side Panel for future features
        egui::SidePanel::left("menu").show(&ctx, |ui| {
            ui.label("Test Menu");
        });

        // Main Panel
        egui::CentralPanel::default().show(&ctx, |ui| {
            let mut watch_list = TableBuilder::new(ui);
        });
    }
}
