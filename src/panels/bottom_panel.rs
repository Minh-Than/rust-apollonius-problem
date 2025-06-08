use std::ops::RangeInclusive;

use egui::{Context, Slider};

use crate::models::app::MyApp;

pub fn get(app: &mut MyApp, ctx: &Context) {
    egui::TopBottomPanel::bottom("bottom_control_panel")
        .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(8.0))
        .show(ctx, |ui| {
            const RADIUS_RANGE: RangeInclusive<f32> = 1.0..=200.0;
            ui.horizontal_wrapped(|ui| {
                ui.vertical(|ui| {
                    ui.label("Circle 1");
                    ui.add(Slider::new(&mut app.circle_1.radius, RADIUS_RANGE));
                });
                ui.vertical(|ui| {
                    ui.label("Circle 2");
                    ui.add(Slider::new(&mut app.circle_2.radius, RADIUS_RANGE));
                });
                ui.vertical(|ui| {
                    ui.label("Circle 3");
                    ui.add(Slider::new(&mut app.circle_3.radius, RADIUS_RANGE));
                });

                ui.separator();

                if ui.add(egui::Button::new("Reset camera")).clicked() {
                    *app = MyApp::reset_scene(app);
                    ctx.request_repaint();
                }
                if ui.add(egui::Button::new("Reset circles")).clicked() {
                    *app = MyApp::reset_circles(app);
                    ctx.request_repaint();
                }
                ui.separator();
            });
        });
}
