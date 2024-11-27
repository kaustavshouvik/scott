#![windows_subsystem = "windows"]

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_simple_native("Scott", options, |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File manager");
        });
    })
}
