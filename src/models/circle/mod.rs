use core::f32;

use egui::Pos2;

use crate::models::segment::Segment;
use crate::models::straightline::StraightLine;
use crate::services;

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub center: Pos2,
    pub radius: f32,
}
impl Circle {
    pub fn get_circle_clipping_rect(self) -> egui::Rect {
        egui::Rect {
            min: egui::Pos2 {
                x: self.center.x - self.radius,
                y: self.center.y - self.radius,
            },
            max: egui::Pos2 {
                x: self.center.x + self.radius,
                y: self.center.y + self.radius,
            },
        }
    }

    pub fn get_circle_3_points(
        a: &Option<Pos2>,
        b: &Option<Pos2>,
        c: &Option<Pos2>,
    ) -> Option<Self> {
        match (*a, *b, *c) {
            (Some(p1), Some(p2), Some(p3)) => {
                let s1: Segment = Segment(p1, p2);
                let s2: Segment = Segment(p2, p3);
                let s3: Segment = Segment(p3, p1);

                if services::calc::check_if_flat_angle(&s1, &s2)
                    || services::calc::check_if_flat_angle(&s2, &s3)
                    || services::calc::check_if_flat_angle(&s3, &s1)
                {
                    return None;
                }

                fn get_orthoganal_point(s: Segment) -> Pos2 {
                    let mut result_point: Pos2 = Pos2 {
                        x: f32::NEG_INFINITY,
                        y: f32::NEG_INFINITY,
                    };

                    let p1: Pos2 = Pos2 { x: s.0.x, y: s.0.y };
                    let p2: Pos2 = Pos2 { x: s.1.x, y: s.1.y };
                    if let Some(point) = services::calc::internal_division_ratio(p1, p2, 1.0, 1.0) {
                        result_point = point;
                    }

                    result_point
                }

                let orth_1 = get_orthoganal_point(s1);
                let orth_2 = get_orthoganal_point(s2);

                let l1: StraightLine =
                    services::calc::orthoganalize(&s1.as_straight_line(), orth_1);
                let l2: StraightLine =
                    services::calc::orthoganalize(&s2.as_straight_line(), orth_2);
                let intersection: Pos2 = services::calc::find_intersection(&l1, &l2);
                Some(Circle {
                    center: intersection,
                    radius: p1.distance(intersection),
                })
            }
            _ => None,
        }
    }
}
