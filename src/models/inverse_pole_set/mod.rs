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
        let p1 = Self::get_inverse_pole(&line, circles[0]);
        let p2 = Self::get_inverse_pole(&line, circles[1]);
        let p3 = Self::get_inverse_pole(&line, circles[2]);

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

    pub fn get_inverse_pole(s: &Option<Segment>, c: Circle) -> Option<Pos2> {
        match *s {
            Some(sgm) => {
                if services::calc::is_segment_intersecting_circle(&sgm, &c) {
                    let intersecting_segment: Option<Segment> =
                        services::calc::get_circle_straight_line_intersection(
                            &Some(sgm.as_straight_line()),
                            &c,
                        );

                    match intersecting_segment {
                        Some(int_sgm) => {
                            let dx = int_sgm.0.x - c.center.x;
                            let dy = int_sgm.0.y - c.center.y;

                            let tangent_x = -dy;
                            let tangent_y = dx;

                            let tangent: Segment = Segment(
                                Pos2 {
                                    x: int_sgm.0.x - tangent_x,
                                    y: int_sgm.0.y - tangent_y,
                                },
                                Pos2 {
                                    x: int_sgm.0.x + tangent_x,
                                    y: int_sgm.0.y + tangent_y,
                                },
                            );

                            let projection_segment: Segment = Segment(
                                c.center,
                                services::calc::find_projection(&sgm.as_straight_line(), c.center),
                            );
                            Some(services::calc::find_intersection(
                                &tangent.as_straight_line(),
                                &projection_segment.as_straight_line(),
                            ))
                        }
                        None => None,
                    }
                } else {
                    let polar_projection_segment: Segment = Segment(
                        c.center,
                        services::calc::find_projection(&sgm.as_straight_line(), c.center),
                    );
                    let projection_midpoint: Pos2 =
                        services::calc::mid_point(&polar_projection_segment);
                    let compass_circle: Circle = Circle {
                        center: projection_midpoint,
                        radius: projection_midpoint.distance(c.center),
                    };
                    let intersection =
                        services::calc::get_circles_intersection(&compass_circle, &c);

                    Some(services::calc::find_intersection(
                        &polar_projection_segment.as_straight_line(),
                        &intersection.as_straight_line(),
                    ))
                }
            }
            None => None,
        }
    }
}
