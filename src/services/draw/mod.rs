use egui::{Color32, Pos2, epaint::CircleShape};

use crate::models::{
    apollonius_pair::ApolloniusPair, circle::Circle, homothetic_set::HomotheticSet,
    inverse_pole_set::InversePoleSet,
};

pub fn draw_circle(
    ui: &mut egui::Ui,
    center: Pos2,
    radius: f32,
    fill: Color32,
    stroke: egui::Stroke,
) {
    ui.painter().add(egui::Shape::Circle(CircleShape {
        center,
        radius,
        fill,
        stroke,
    }));
}

pub fn draw_line(ui: &mut egui::Ui, points: [Pos2; 2], stroke: egui::Stroke) {
    ui.painter()
        .add(egui::Shape::LineSegment { points, stroke });
}

pub fn draw_three_circles(ui: &mut egui::Ui, circles: [Circle; 3], fill: Color32) {
    for c in circles {
        draw_circle(ui, c.center, c.radius, fill, egui::Stroke::NONE)
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
            draw_circle(ui, external, 2.0, fill, egui::Stroke::NONE);
        }
        if let Some(internal) = pair.ir {
            draw_circle(ui, internal, 2.0, fill, egui::Stroke::NONE);
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
    draw_circle(ui, radical_center, 4.0, fill, egui::Stroke::NONE);
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
                draw_circle(ui, point, 2.0, fill, egui::Stroke::NONE);
            }
            if let Some(segment) = pair.segment {
                draw_line(
                    ui,
                    [segment.0, segment.1],
                    egui::Stroke {
                        width: 0.5,
                        color: fill,
                    },
                );
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

    for c in circle_pair.into_iter().flatten() {
        draw_circle(
            ui,
            c.center,
            c.radius,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(0.5, stroke),
        );
    }
}
