use egui::{Color32, epaint::CircleShape};

use crate::models::{
    apollonius_pair::ApolloniusPair, circle::Circle, homothetic_set::HomotheticSet,
    inverse_pole_set::InversePoleSet,
};

pub fn draw_three_circles(ui: &mut egui::Ui, circles: [Circle; 3], fill: Color32) {
    for circle in circles {
        ui.painter().add(egui::Shape::Circle(CircleShape {
            center: circle.center,
            radius: circle.radius,
            fill,
            stroke: egui::Stroke::NONE,
        }));
    }
}

pub fn draw_homothetic_centers(
    ui: &mut egui::Ui,
    homothetic_set: &HomotheticSet,
    condition: bool,
    fill: Color32,
) {
    if !condition {
        return;
    }
    for pair in homothetic_set.pairs.clone().into_iter() {
        if let Some(external) = pair.ex {
            ui.painter().add(egui::Shape::Circle(CircleShape {
                center: external,
                radius: 2.0,
                fill,
                stroke: egui::Stroke::NONE,
            }));
        }
        if let Some(internal) = pair.ir {
            ui.painter().add(egui::Shape::Circle(CircleShape {
                center: internal,
                radius: 2.0,
                fill,
                stroke: egui::Stroke::NONE,
            }));
        }
    }
}

pub fn draw_radical_center(
    ui: &mut egui::Ui,
    radical_center: egui::Pos2,
    condition: bool,
    fill: Color32,
) {
    if !condition {
        return;
    }
    ui.painter().add(egui::Shape::Circle(CircleShape {
        center: radical_center,
        radius: 4.0,
        fill,
        stroke: egui::Stroke::NONE,
    }));
}

pub fn draw_inverse_poles(
    ui: &mut egui::Ui,
    poles_set: &Option<InversePoleSet>,
    condition: bool,
    fill: egui::Color32,
) {
    if !condition {
        return;
    }

    if let Some(set) = poles_set {
        for pair in set.point_segment_pairs.clone() {
            if let Some(point) = pair.point {
                ui.painter().add(egui::Shape::Circle(CircleShape {
                    center: point,
                    radius: 2.0,
                    fill,
                    stroke: egui::Stroke::NONE,
                }));
            }
            if let Some(segment) = pair.segment {
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
    condition: bool,
    stroke: egui::Color32,
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
