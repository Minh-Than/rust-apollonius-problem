use crate::{enums::theme_mode::ThemeMode, models::app::MyApp};

pub fn get(app: &mut MyApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_control_panel")
        .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(8.0))
        .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.checkbox(
                    &mut app.display_options.show_homothetic,
                    "Homothetic centers",
                );
                ui.checkbox(&mut app.display_options.show_radical, "Radical center");
                ui.checkbox(&mut app.display_options.show_inverse_poles, "Inverse poles");
                ui.checkbox(&mut app.display_options.show_apollonius_circle_1, "A1");
                ui.checkbox(&mut app.display_options.show_apollonius_circle_2, "A2");
                ui.checkbox(&mut app.display_options.show_apollonius_circle_3, "A3");
                ui.checkbox(&mut app.display_options.show_apollonius_circle_4, "A4");
                ui.separator();
                egui::ComboBox::from_label("Theme")
                    .selected_text(format!("{:?}", app.theme_mode))
                    .show_ui(ui, |ui| {
                        for theme_item in ThemeMode::as_vec() {
                            ui.selectable_value(
                                &mut app.theme_mode,
                                theme_item,
                                theme_item.value(),
                            );
                        }
                    });
            });
        });
}
