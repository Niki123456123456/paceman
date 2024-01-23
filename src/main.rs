#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app;
pub mod client;
pub mod models;
pub mod tabcontrol;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..]).unwrap(),),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "paceman",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    );
}
