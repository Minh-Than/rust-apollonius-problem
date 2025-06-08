use eframe::egui;
use models::app::MyApp;
use panels::{bottom_panel, central_panel, top_panel};

mod enums;
mod models;
mod panels;
mod services;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Apollonius' Circles",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.theme_mode {
            enums::theme_mode::ThemeMode::Light => {
                let mut style: egui::Style = (*ctx.style()).clone();
                style.visuals = egui::Visuals::light();
                ctx.set_style(style);
            }
            enums::theme_mode::ThemeMode::Dark => {
                let mut style: egui::Style = (*ctx.style()).clone();
                style.visuals = egui::Visuals::dark();
                ctx.set_style(style);
            }
        };

        top_panel::get(self, ctx);
        bottom_panel::get(self, ctx);
        central_panel::get(self, ctx);
    }
}
