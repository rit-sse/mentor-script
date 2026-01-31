use eframe::egui::ViewportBuilder;
use eframe::HardwareAcceleration;
use crate::app::MentorApp;
use crate::config::Config;

mod scheduler;
mod config;
mod app;
mod sound;

fn main() -> ! {
    let config = Config::load().expect("Failed to load config");

    eprintln!("Starting Mentor Script GUI!");

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("Mentor Reminder")
            .with_inner_size([640.0, 420.0])
            .with_resizable(false)
            .with_always_on_top()
            .with_decorations(true),
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: HardwareAcceleration::Preferred,
        renderer: Default::default(),
        run_and_return: true,
        event_loop_builder: None,
        window_builder: None,
        shader_version: None,
        centered: true,
        persist_window: false,
        persistence_path: None,
        dithering: false,
    };

    if let Err(e) = eframe::run_native(
        "Mentor Reminder",
        options,
        Box::new(|_cc| Ok(Box::new(MentorApp::new(config)))),
    ) {
        eprintln!("eframe failed: {e}");
    }

    eprintln!("Mentor Script Stopped!");

    std::process::exit(0);
}