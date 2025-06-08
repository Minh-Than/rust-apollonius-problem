use egui::epaint::CircleShape;

use crate::{
    enums::{color_item_names::ColorItemNames, theme_mode::ThemeMode},
    models::{
        apollonius_pair::ApolloniusPair, circle::Circle, homothetic_centers::HomotheticCenters,
        inverse_pole_set::InversePoleSet,
    },
    services,
};

pub fn draw_three_circles(ui: &mut egui::Ui, circles: [Circle; 3], theme_mode: &ThemeMode) {
    for circle in circles {
        ui.painter().add(egui::Shape::Circle(CircleShape {
            center: circle.center,
            radius: circle.radius,
            fill: services::theme::get_color(ColorItemNames::InitialCircles, theme_mode),
            stroke: egui::Stroke::NONE,
        }));
    }
}

pub fn draw_homothetis_centers(
    ui: &mut egui::Ui,
    homothetic_centers: &HomotheticCenters,
    condition: bool,
    theme_mode: &ThemeMode,
) {
    if !condition {
        return;
    }
    for center in homothetic_centers.into_iter() {
        match center {
            Some(c) => {
                ui.painter().add(egui::Shape::Circle(CircleShape {
                    center: c,
                    radius: 2.0,
                    fill: services::theme::get_color(ColorItemNames::HomotheticCenters, theme_mode),
                    stroke: egui::Stroke::NONE,
                }));
            }
            None => (),
        }
    }
}

pub fn draw_radical_center(
    ui: &mut egui::Ui,
    radical_center: egui::Pos2,
    condition: bool,
    theme_mode: &ThemeMode,
) {
    if !condition {
        return;
    }
    ui.painter().add(egui::Shape::Circle(CircleShape {
        center: radical_center,
        radius: 4.0,
        fill: services::theme::get_color(ColorItemNames::Radical, theme_mode),
        stroke: egui::Stroke::NONE,
    }));
}

pub fn draw_inverse_poles(
    ui: &mut egui::Ui,
    poles_set: &Option<InversePoleSet>,
    fill: egui::Color32,
    condition: bool,
) {
    if !condition {
        return;
    }

    match poles_set {
        Some(set) => {
            for pole in [set.p1, set.p2, set.p3] {
                match pole {
                    Some(p) => {
                        ui.painter().add(egui::Shape::Circle(CircleShape {
                            center: p,
                            radius: 2.0,
                            fill,
                            stroke: egui::Stroke::NONE,
                        }));
                    }
                    None => (),
                }
            }
        }
        None => (),
    }
}

pub fn draw_connectors(
    ui: &mut egui::Ui,
    poles_set: &Option<InversePoleSet>,
    stroke_color: egui::Color32,
    condition: bool,
) {
    if !condition {
        return;
    }

    match poles_set {
        Some(set) => {
            for segment in [set.s1, set.s2, set.s3] {
                match segment {
                    Some(sgm) => {
                        ui.painter().add(egui::Shape::LineSegment {
                            points: [sgm.0, sgm.1],
                            stroke: egui::Stroke {
                                width: 0.7,
                                color: stroke_color,
                            },
                        });
                    }
                    None => (),
                }
            }
        }
        None => (),
    }
}

pub fn draw_apollonius_circles_pair(
    ui: &mut egui::Ui,
    circle_pair: &ApolloniusPair,
    stroke: egui::Color32,
    condition: bool,
) {
    if !condition {
        return;
    }

    for circle in circle_pair.into_iter() {
        if let Some(c) = circle {
            ui.painter().add(egui::Shape::Circle(CircleShape {
                center: c.center,
                radius: c.radius,
                fill: egui::Color32::TRANSPARENT,
                stroke: egui::Stroke::new(0.5, stroke),
            }));
        }
    }
}
