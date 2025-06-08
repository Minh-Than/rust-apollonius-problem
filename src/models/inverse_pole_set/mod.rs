use crate::{models::circle::Circle, services};
use egui::Pos2;

use crate::models::segment::Segment;

#[derive(Clone)]
pub struct InversePoleSet {
    pub p1: Option<Pos2>,
    pub p2: Option<Pos2>,
    pub p3: Option<Pos2>,
    pub s1: Option<Segment>,
    pub s2: Option<Segment>,
    pub s3: Option<Segment>,
}
impl InversePoleSet {
    pub fn new(line: Option<Segment>, circles: &[Circle], radical_center: Pos2) -> Option<Self> {
        let p1 = services::calc::get_inverse_pole(&line, circles[0]);
        let p2 = services::calc::get_inverse_pole(&line, circles[1]);
        let p3 = services::calc::get_inverse_pole(&line, circles[2]);

        Some(Self {
            p1,
            p2,
            p3,
            s1: services::calc::get_circle_straight_line_intersection(
                &Some(Segment(p1?, radical_center).as_straight_line()),
                &circles[0],
            ),
            s2: services::calc::get_circle_straight_line_intersection(
                &Some(Segment(p2?, radical_center).as_straight_line()),
                &circles[1],
            ),
            s3: services::calc::get_circle_straight_line_intersection(
                &Some(Segment(p3?, radical_center).as_straight_line()),
                &circles[2],
            ),
        })
    }
}
