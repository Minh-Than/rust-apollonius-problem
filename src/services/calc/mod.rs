use core::f32;

use eframe::egui;
use egui::Pos2;

use crate::models::{circle::Circle, segment::Segment, straightline::StraightLine};

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

pub fn get_radical_axis(c1: Circle, c2: Circle) -> StraightLine {
    let a: f32 = 2.0 * (c2.center.x - c1.center.x);
    let b: f32 = 2.0 * (c2.center.y - c1.center.y);
    let c: f32 = (c1.center.x.powi(2) + c1.center.y.powi(2) - c1.radius.powi(2))
        - (c2.center.x.powi(2) + c2.center.y.powi(2) - c2.radius.powi(2));

    StraightLine { a, b, c }
}

pub fn get_inverse_pole(s: &Option<Segment>, c: Circle) -> Option<Pos2> {
    match *s {
        Some(sgm) => {
            if is_segment_intersecting_circle(&sgm, &c) {
                let intersecting_segment: Option<Segment> =
                    get_circle_straight_line_intersection(&Some(sgm.as_straight_line()), &c);

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

                        let projection_segment: Segment =
                            Segment(c.center, find_projection(&sgm.as_straight_line(), c.center));
                        Some(find_intersection(
                            &tangent.as_straight_line(),
                            &projection_segment.as_straight_line(),
                        ))
                    }
                    None => None,
                }
            } else {
                let polar_projection_segment: Segment =
                    Segment(c.center, find_projection(&sgm.as_straight_line(), c.center));
                let projection_midpoint: Pos2 = mid_point(&polar_projection_segment);
                let compass_circle: Circle = Circle {
                    center: projection_midpoint,
                    radius: projection_midpoint.distance(c.center),
                };
                let intersection = get_circles_intersection(&compass_circle, &c);

                Some(find_intersection(
                    &polar_projection_segment.as_straight_line(),
                    &intersection.as_straight_line(),
                ))
            }
        }
        None => None,
    }
}

pub fn get_circle_3_points(a: &Option<Pos2>, b: &Option<Pos2>, c: &Option<Pos2>) -> Option<Circle> {
    match (*a, *b, *c) {
        (Some(p1), Some(p2), Some(p3)) => {
            let s1: Segment = Segment(p1, p2);
            let s2: Segment = Segment(p2, p3);
            let s3: Segment = Segment(p3, p1);

            if check_if_flat_angle(&s1, &s2)
                || check_if_flat_angle(&s2, &s3)
                || check_if_flat_angle(&s3, &s1)
            {
                return None;
            }

            let orth_1 = match internal_division_ratio(
                Pos2 {
                    x: s1.0.x,
                    y: s1.0.y,
                },
                Pos2 {
                    x: s1.1.x,
                    y: s1.1.y,
                },
                1.0,
                1.0,
            ) {
                Some(point) => point,
                None => Pos2 {
                    x: f32::NEG_INFINITY,
                    y: f32::NEG_INFINITY,
                },
            };
            let orth_2 = match internal_division_ratio(
                Pos2 {
                    x: s2.0.x,
                    y: s2.0.y,
                },
                Pos2 {
                    x: s2.1.x,
                    y: s2.1.y,
                },
                1.0,
                1.0,
            ) {
                Some(point) => point,
                None => Pos2 {
                    x: f32::NEG_INFINITY,
                    y: f32::NEG_INFINITY,
                },
            };

            let l1: StraightLine = orthoganalize(&s1.as_straight_line(), orth_1);
            let l2: StraightLine = orthoganalize(&s2.as_straight_line(), orth_2);
            let intersection: Pos2 = find_intersection(&l1, &l2);
            Some(Circle {
                center: intersection,
                radius: p1.distance(intersection),
            })
        }
        _ => None,
    }
}

pub fn internal_division_ratio(p1: Pos2, p2: Pos2, ratio_s: f32, ratio_t: f32) -> Option<Pos2> {
    if p1.distance(p2) < 1e-6 {
        return None;
    }

    if ratio_s < 1e-6 {
        if ratio_t < 1e-6 { None } else { Some(p1) }
    } else {
        if ratio_t < 1e-6 {
            return Some(p2);
        }
        let s: Segment = Segment(p1, p2);
        let denominator = ratio_s + ratio_t;
        let x = (ratio_t * s.0.x + ratio_s * s.1.x) / denominator;
        let y = (ratio_t * s.0.y + ratio_s * s.1.y) / denominator;
        Some(Pos2 { x, y })
    }
}

pub fn check_if_flat_angle(s1: &Segment, s2: &Segment) -> bool {
    let angle_1: f32 = angle(s1, s2).abs();
    let angle_2: f32 = (angle(s1, s2) - 180.0).abs();
    let angle_3: f32 = (angle(s1, s2) - 360.0).abs();

    angle_1 < 1e-6 || angle_2 < 1e-6 || angle_3 < 1e-6
}

pub fn angle_between_0_360(mut angle: f32) -> f32 {
    while angle < 0.0 {
        angle += 360.0;
    }
    while angle >= 360.0 {
        angle -= 360.0;
    }

    angle
}

pub fn angle_of_vector(p1: Pos2, p2: Pos2) -> Option<f32> {
    let x = p2.x - p1.x;
    let y = p2.y - p1.y;

    if x.abs() < 1e-6 && y.abs() < 1e-6 {
        return None;
    }

    let mut angle = y.atan2(x).to_degrees();
    if angle < 0.0 {
        angle += 360.0;
    }

    Some(angle)
}

pub fn angle(s1: &Segment, s2: &Segment) -> f32 {
    let a: Pos2 = Pos2 {
        x: s1.0.x,
        y: s1.0.y,
    };
    let b: Pos2 = Pos2 {
        x: s1.1.x,
        y: s1.1.y,
    };
    let c: Pos2 = Pos2 {
        x: s2.0.x,
        y: s2.0.y,
    };
    let d: Pos2 = Pos2 {
        x: s2.1.x,
        y: s2.1.y,
    };

    let angle_1: f32 = match angle_of_vector(a, b) {
        Some(value) => value,
        None => -10000.0,
    };

    let angle_2: f32 = match angle_of_vector(c, d) {
        Some(value) => value,
        None => -10000.0,
    };

    angle_between_0_360(angle_2 - angle_1)
}

pub fn find_intersection(l1: &StraightLine, l2: &StraightLine) -> Pos2 {
    let a1 = l1.a;
    let b1 = l1.b;
    let c1 = l1.c;
    let a2 = l2.a;
    let b2 = l2.b;
    let c2 = l2.c;
    Pos2 {
        x: (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1),
        y: (a2 * c1 - a1 * c2) / (a1 * b2 - a2 * b1),
    }
}

pub fn is_segment_intersecting_circle(s: &Segment, c: &Circle) -> bool {
    c.radius >= calculate_distance(&s.as_straight_line(), c.center)
}

pub fn calculate_distance(l: &StraightLine, p: Pos2) -> f32 {
    ((l.a * p.x + l.b * p.y + l.c) / (l.a.powi(2) + l.b.powi(2)).sqrt()).abs()
}

pub fn get_circle_straight_line_intersection(
    l: &Option<StraightLine>,
    c: &Circle,
) -> Option<Segment> {
    match l {
        Some(strl) => {
            let projection: Pos2 = find_projection(strl, c.center);
            let length_a = calculate_distance(strl, c.center);
            let length_b = (c.radius.powi(2) - length_a.powi(2)).sqrt();
            let denominator = (strl.b.powi(2) + strl.a.powi(2)).sqrt();

            Some(Segment(
                Pos2 {
                    x: projection.x + strl.b * length_b / denominator,
                    y: projection.y - strl.a * length_b / denominator,
                },
                Pos2 {
                    x: projection.x - strl.b * length_b / denominator,
                    y: projection.y + strl.a * length_b / denominator,
                },
            ))
        }
        None => None,
    }
}

pub fn get_circles_intersection(c1: &Circle, c2: &Circle) -> Segment {
    let t0: StraightLine = get_circles_intersection_as_straight_line(c1, c2);
    let t1: StraightLine = StraightLine {
        a: c2.center.y - c1.center.y,
        b: c1.center.x - c2.center.x,
        c: c1.center.y * c2.center.x - c1.center.x * c2.center.y,
    };
    let intersection_t0_t1: Pos2 = find_intersection(&t0, &t1);

    let length_a = calculate_distance(&t0, c1.center);
    let length_b = (c1.radius.powi(2) - length_a.powi(2)).sqrt();
    let denominator = (t0.b.powi(2) + t0.a.powi(2)).sqrt();

    Segment(
        Pos2 {
            x: intersection_t0_t1.x + t0.b * length_b / denominator,
            y: intersection_t0_t1.y - t0.a * length_b / denominator,
        },
        Pos2 {
            x: intersection_t0_t1.x - t0.b * length_b / denominator,
            y: intersection_t0_t1.y + t0.a * length_b / denominator,
        },
    )
}

fn get_circles_intersection_as_straight_line(c1: &Circle, c2: &Circle) -> StraightLine {
    let x1 = c1.center.x;
    let y1 = c1.center.y;
    let r1 = c1.radius;
    let x2 = c2.center.x;
    let y2 = c2.center.y;
    let r2 = c2.radius;

    let a = 2.0 * x1 - 2.0 * x2;
    let b = 2.0 * y1 - 2.0 * y2;
    let c = x2.powi(2) - x1.powi(2) + y2.powi(2) - y1.powi(2) + r1.powi(2) - r2.powi(2);

    StraightLine { a, b, c }
}

fn find_projection(l: &StraightLine, p: Pos2) -> Pos2 {
    let ort_l: StraightLine = orthoganalize(l, p);
    find_intersection(l, &ort_l)
}

fn orthoganalize(l: &StraightLine, p: Pos2) -> StraightLine {
    let a = l.b;
    let b = -l.a;
    let c = -l.b * p.x + l.a * p.y;

    StraightLine { a, b, c }
}

fn mid_point(s: &Segment) -> Pos2 {
    let x = (s.1.x + s.0.x) / 2.0;
    let y = (s.1.y + s.0.y) / 2.0;

    Pos2 { x, y }
}
