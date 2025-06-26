use std::iter::zip;

use crate::{models::circle::Circle, models::segment::Segment, services};
use egui::Pos2;

#[derive(Clone)]
pub struct PointSegmentPair {
    pub point: Option<Pos2>,
    pub segment: Option<Segment>,
}

pub struct InversePoleSet {
    pub point_segment_pairs: Vec<PointSegmentPair>,
}
impl InversePoleSet {
    pub fn new(line: Option<Segment>, circles: &[Circle], radical_center: Pos2) -> Option<Self> {
        let p1 = services::calc::get_inverse_pole(&line, circles[0]);
        let p2 = services::calc::get_inverse_pole(&line, circles[1]);
        let p3 = services::calc::get_inverse_pole(&line, circles[2]);

        let mut point_segment_pairs: Vec<PointSegmentPair> = Vec::new();
        for (circle, point) in zip(circles, [p1, p2, p3]) {
            let segment = services::calc::get_circle_straight_line_intersection(
                &Some(Segment(point?, radical_center).as_straight_line()),
                circle,
            );
            point_segment_pairs.push(PointSegmentPair { point, segment });
        }

        Some(InversePoleSet {
            point_segment_pairs,
        })
    }

    pub fn new_special(circles: &[Circle], radical_center: Pos2) -> Option<Self> {
        let mut point_segment_pairs: Vec<PointSegmentPair> = Vec::new();
        for circle in circles {
            point_segment_pairs.push(PointSegmentPair {
                point: Some(circle.center),
                segment: services::calc::get_circle_straight_line_intersection(
                    &Some(Segment(circle.center, radical_center).as_straight_line()),
                    circle,
                ),
            });
        }

        Some(InversePoleSet {
            point_segment_pairs,
        })
    }

    pub fn get_segment(&self, idx: usize) -> Option<Segment> {
        self.point_segment_pairs[idx].segment
    }
}
