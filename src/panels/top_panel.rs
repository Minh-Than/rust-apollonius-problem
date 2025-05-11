use egui::Context;

use crate::models::app::MyApp;

pub fn top_panel(ctx: &Context, app: &mut MyApp) {
    egui::TopBottomPanel::top("control_panel").show(ctx, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut app.show_homothetic, "Homothetic centers");
            ui.checkbox(&mut app.show_radical, "Radical center");
            ui.checkbox(&mut app.show_inverse_poles, "Inverse poles");
            ui.checkbox(&mut app.show_connectors, "Connectors");
            ui.checkbox(&mut app.show_apollonius_circle_1, "A1");
            ui.checkbox(&mut app.show_apollonius_circle_2, "A2");
            ui.checkbox(&mut app.show_apollonius_circle_3, "A3");
            ui.checkbox(&mut app.show_apollonius_circle_4, "A4");
            if ui.button("Reset").clicked() { 
                *app = MyApp::reset();
                ctx.request_repaint();
            }
            let mut theme =
                egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
            ui.collapsing("Theme", |ui| {
                theme.ui(ui);
                theme.store_in_memory(ui.ctx());
            });
        })
    });
}