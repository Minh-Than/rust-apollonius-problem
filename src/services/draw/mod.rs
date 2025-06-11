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
    for center in homothetic_centers.into_iter().flatten() {
        ui.painter().add(egui::Shape::Circle(CircleShape {
            center,
            radius: 2.0,
            fill: services::theme::get_color(ColorItemNames::HomotheticCenters, theme_mode),
            stroke: egui::Stroke::NONE,
        }));
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

    if let Some(set) = poles_set {
        for pair in set.point_segment_pairs.clone() {
            if let Some(point) = pair.get_point() {
                ui.painter().add(egui::Shape::Circle(CircleShape {
                    center: point,
                    radius: 2.0,
                    fill,
                    stroke: egui::Stroke::NONE,
                }));
            }
            if let Some(segment) = pair.get_segment() {
                ui.painter().add(egui::Shape::LineSegment {
                    points: [segment.0, segment.1],
                    stroke: egui::Stroke {
                        width: 0.5,
                        color: fill,
                    },
                });
            }
        }
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

    for circle in circle_pair.into_iter().flatten() {
        ui.painter().add(egui::Shape::Circle(CircleShape {
            center: circle.center,
            radius: circle.radius,
            fill: egui::Color32::TRANSPARENT,
            stroke: egui::Stroke::new(0.5, stroke),
        }));
    }
}
