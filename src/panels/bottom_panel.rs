use egui::{Context, Slider};

use crate::models::app::MyApp;

pub fn bottom_panel(app: &mut MyApp, ctx: &Context) {
    egui::TopBottomPanel::bottom("bottom_control_panel").show(ctx, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| {
                ui.label("Circle 1");
                ui.add(Slider::new(&mut app.circle_1.radius, 1.0..=200.0));
            });
            ui.vertical(|ui| {
                ui.label("Circle 2");
                ui.add(Slider::new(&mut app.circle_2.radius, 1.0..=200.0));
            });
            ui.vertical(|ui| {
                ui.label("Circle 3");
                ui.add(Slider::new(&mut app.circle_3.radius, 1.0..=200.0));
            });
        });
    });
}
