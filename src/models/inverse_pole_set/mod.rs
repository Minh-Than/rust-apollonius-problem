use crate::{models::circle::Circle, models::segment::Segment, services};
use egui::Pos2;

#[derive(Clone)]
pub struct PointSegmentPair(Option<Pos2>, Option<Segment>);
impl PointSegmentPair {
    pub fn get_point(&self) -> Option<Pos2> {
        self.0
    }

    pub fn get_segment(&self) -> Option<Segment> {
        self.1
    }
}

pub struct InversePoleSet {
    pub point_segment_pairs: Vec<PointSegmentPair>,
}
impl InversePoleSet {
    pub fn new(line: Option<Segment>, circles: &[Circle], radical_center: Pos2) -> Option<Self> {
        let p1 = services::calc::get_inverse_pole(&line, circles[0]);
        let p2 = services::calc::get_inverse_pole(&line, circles[1]);
        let p3 = services::calc::get_inverse_pole(&line, circles[2]);

        let point_segment_pairs: Vec<PointSegmentPair> = vec![
            PointSegmentPair(
                p1,
                services::calc::get_circle_straight_line_intersection(
                    &Some(Segment(p1?, radical_center).as_straight_line()),
                    &circles[0],
                ),
            ),
            PointSegmentPair(
                p2,
                services::calc::get_circle_straight_line_intersection(
                    &Some(Segment(p2?, radical_center).as_straight_line()),
                    &circles[1],
                ),
            ),
            PointSegmentPair(
                p3,
                services::calc::get_circle_straight_line_intersection(
                    &Some(Segment(p3?, radical_center).as_straight_line()),
                    &circles[2],
                ),
            ),
        ];

        Some(InversePoleSet {
            point_segment_pairs,
        })
    }

    pub fn get_segment(&self, idx: usize) -> Option<Segment> {
        self.point_segment_pairs[idx].get_segment()
    }
}
