use egui::Pos2;

use crate::{
    models::{circle::Circle, segment::Segment},
    services,
};

#[derive(Clone)]
pub struct HomotheticPair {
    pub ex: Option<Pos2>,
    pub ir: Option<Pos2>,
}
impl HomotheticPair {
    pub fn new(circle_1: Circle, circle_2: Circle) -> Self {
        Self {
            ex: services::calc::get_external_homothetic_center(circle_1, circle_2),
            ir: services::calc::get_internal_homothetic_center(circle_1, circle_2),
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
}
