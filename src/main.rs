#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::{Style, Vec2};
use models::app::MyApp;
use panels::{central_panel::central_panel, top_panel::top_panel};

mod calc;
mod enums;
mod models;
mod panels;
mod theme_handler;

fn main() -> eframe::Result {
    let window_size: Vec2 = Vec2 {
        x: 1000.0,
        y: 1000.0,
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_max_inner_size(window_size)
            .with_min_inner_size(window_size),
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
                let mut style: Style = (*ctx.style()).clone();
                style.visuals = egui::Visuals::light();
                ctx.set_style(style);
            }
            enums::theme_mode::ThemeMode::Dark => {
                let mut style: Style = (*ctx.style()).clone();
                style.visuals = egui::Visuals::dark();
                ctx.set_style(style);
            }
        };
        top_panel(self, ctx);
        central_panel(self, ctx);
    }
}
