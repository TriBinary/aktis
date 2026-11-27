//! Aktis desktop application entry point.

use aktis_pt::PathTracer;
use aktis_render::RenderEngine;
use aktis_scene::Scene;
use aktis_sp::ShaderPackEngine;

fn main() -> eframe::Result {
    env_logger::init();
    log::info!("starting Aktis {}", env!("CARGO_PKG_VERSION"));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Aktis")
            .with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Aktis",
        options,
        Box::new(|_cc| Ok(Box::new(AktisApp::default()))),
    )
}

/// Top-level application state.
struct AktisApp {
    scene: Scene,
    engines: Vec<Box<dyn RenderEngine>>,
    active_engine: usize,
}

impl Default for AktisApp {
    fn default() -> Self {
        Self {
            scene: Scene::default(),
            engines: vec![
                Box::new(PathTracer::default()),
                Box::new(ShaderPackEngine::default()),
            ],
            active_engine: 0,
        }
    }
}

impl eframe::App for AktisApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::right("settings").show(ui, |ui| {
            ui.heading("Render");
            for (i, engine) in self.engines.iter().enumerate() {
                ui.radio_value(&mut self.active_engine, i, engine.name());
            }
            ui.separator();
            ui.heading("Sun");
            ui.add(egui::Slider::new(&mut self.scene.sun.altitude, -10.0..=90.0).text("Altitude"));
            ui.add(egui::Slider::new(&mut self.scene.sun.azimuth, 0.0..=360.0).text("Azimuth"));
        });
        egui::CentralPanel::default().show(ui, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label("Aktis — open a world to begin (milestone M1)");
            });
        });
    }
}
