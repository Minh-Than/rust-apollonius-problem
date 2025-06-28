use egui::Pos2;

use crate::models::{circle::Circle, segment::Segment};

#[derive(Clone)]
pub struct HomotheticPair {
    pub ex: Option<Pos2>,
    pub ir: Option<Pos2>,
}
impl HomotheticPair {
    pub fn new(circle_1: Circle, circle_2: Circle) -> Self {
        Self {
            ex: HomotheticSet::get_external_homothetic_center(circle_1, circle_2),
            ir: HomotheticSet::get_internal_homothetic_center(circle_1, circle_2),
        }
    }
}

#[derive(Clone)]
pub struct HomotheticSet {
    pub pairs: [HomotheticPair; 3],
    pub lines: [Option<Segment>; 4],
}
impl HomotheticSet {
    pub fn new(circles: &[Circle]) -> Self {
        let pairs: [HomotheticPair; 3] = [
            HomotheticPair::new(circles[0], circles[1]),
            HomotheticPair::new(circles[1], circles[2]),
            HomotheticPair::new(circles[2], circles[0]),
        ];

        let lines: [Option<Segment>; 4] = [
            Segment::get_any_valid_segment(vec![pairs[2].ex, pairs[1].ex, pairs[0].ex]),
            Segment::get_any_valid_segment(vec![pairs[0].ex, pairs[2].ir, pairs[1].ir]),
            Segment::get_any_valid_segment(vec![pairs[2].ex, pairs[1].ir, pairs[0].ir]),
            Segment::get_any_valid_segment(vec![pairs[1].ex, pairs[2].ir, pairs[0].ir]),
        ];

        Self { pairs, lines }
    }

    pub fn get_external_homothetic_center(c1: Circle, c2: Circle) -> Option<Pos2> {
        let denominator = c2.radius - c1.radius;
        if denominator.abs() < 10e-6 {
            return None;
        }

        let x = (c2.radius * c1.center.x - c1.radius * c2.center.x) / denominator;
        let y = (c2.radius * c1.center.y - c1.radius * c2.center.y) / denominator;

        Some(Pos2 { x, y })
    }

    // Unlike the beta external, internal is unfuckable
    pub fn get_internal_homothetic_center(c1: Circle, c2: Circle) -> Option<Pos2> {
        let denominator = c2.radius + c1.radius;
        let x = (c2.radius * c1.center.x + c1.radius * c2.center.x) / denominator;
        let y = (c2.radius * c1.center.y + c1.radius * c2.center.y) / denominator;

        Some(Pos2 { x, y })
    }
}
