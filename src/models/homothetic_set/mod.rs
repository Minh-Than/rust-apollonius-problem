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

#[derive(Clone)]
pub struct HomotheticSet {
    pub pairs: [HomotheticPair; 3],
    pub lines: [Option<Segment>; 4],
}
impl HomotheticSet {
    pub fn new(circles: &[Circle]) -> Self {
        let ex_12 = services::calc::get_external_homothetic_center(circles[0], circles[1]);
        let ir_12 = services::calc::get_internal_homothetic_center(circles[0], circles[1]);
        let ex_23 = services::calc::get_external_homothetic_center(circles[1], circles[2]);
        let ir_23 = services::calc::get_internal_homothetic_center(circles[1], circles[2]);
        let ex_31 = services::calc::get_external_homothetic_center(circles[2], circles[0]);
        let ir_31 = services::calc::get_internal_homothetic_center(circles[2], circles[0]);

        let line_1 = Segment::get_any_valid_segment(vec![ex_31, ex_23, ex_12]);
        let line_2 = Segment::get_any_valid_segment(vec![ex_12, ir_31, ir_23]);
        let line_3 = Segment::get_any_valid_segment(vec![ex_31, ir_23, ir_12]);
        let line_4 = Segment::get_any_valid_segment(vec![ex_23, ir_31, ir_12]);

        Self {
            pairs: [
                HomotheticPair {
                    ex: ex_12,
                    ir: ir_12,
                },
                HomotheticPair {
                    ex: ex_23,
                    ir: ir_23,
                },
                HomotheticPair {
                    ex: ex_31,
                    ir: ir_31,
                },
            ],
            lines: [line_1, line_2, line_3, line_4],
        }
    }
}

