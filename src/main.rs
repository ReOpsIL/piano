use eframe::egui;

mod app;
mod midi;
mod notation;
mod game;
mod music;
mod ui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Piano Sight Reading")
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Piano Sight Reading",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(app::PianoApp::new(cc)))
        }),
    )
}