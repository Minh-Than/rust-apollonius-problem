use eframe::egui;
use egui::{epaint::CircleShape, Color32, Pos2, Stroke};

use crate::models::{segment::Segment, straightline::StraightLine};

pub fn get_external_homothetic_center(c1: CircleShape, c2: CircleShape) -> Pos2 {
    let x = (c2.radius * c1.center.x - c1.radius * c2.center.x) / (c2.radius - c1.radius);
    let y = (c2.radius * c1.center.y - c1.radius * c2.center.y) / (c2.radius - c1.radius);

    Pos2 { x, y }
}

pub fn get_internal_homothetic_center(c1: CircleShape, c2: CircleShape) -> Pos2 {
    let x = (c2.radius * c1.center.x + c1.radius * c2.center.x) / (c2.radius + c1.radius);
    let y = (c2.radius * c1.center.y + c1.radius * c2.center.y) / (c2.radius + c1.radius);

    Pos2 { x, y }
}

pub fn get_radical_axis(c1: CircleShape, c2: CircleShape) -> StraightLine {
    let a: f32 = 2.0 * (c2.center.x - c1.center.x);
    let b: f32 = 2.0 * (c2.center.y - c1.center.y);
    let c: f32 = (c1.center.x.powi(2) + c1.center.y.powi(2) - c1.radius.powi(2)) 
        - (c2.center.x.powi(2) + c2.center.y.powi(2) - c2.radius.powi(2));

    StraightLine { a, b, c }
}

pub fn get_inverse_pole(s: &Segment, c: CircleShape) -> Pos2 {
    if is_segment_intersecting_circle(&s, c) {
        let intersecting_segment: Segment = get_circle_straight_line_intersection(&s.as_straight_line(), c);
        let dx = intersecting_segment.0.x - c.center.x;
        let dy = intersecting_segment.0.y - c.center.y;

        let tangent_x = -dy;
        let tangent_y = dx;

        let tangent: Segment = Segment(
            Pos2 {
                x: intersecting_segment.0.x - tangent_x, 
                y: intersecting_segment.0.y - tangent_y
            }, 
            Pos2 { 
                x: intersecting_segment.0.x + tangent_x, 
                y: intersecting_segment.0.y + tangent_y
            }
        );

        let projection_segment: Segment = Segment(c.center, find_projection(&s.as_straight_line(), c.center));
        find_intersection(&tangent.as_straight_line(), &projection_segment.as_straight_line())
    } else {
        let polar_projection_segment: Segment = Segment(
            c.center, 
            find_projection(&s.as_straight_line(), c.center)
        );
        let projection_midpoint: Pos2 = mid_point(&polar_projection_segment);
        let compass_circle: CircleShape = CircleShape { 
            center: projection_midpoint, 
            radius: projection_midpoint.distance(c.center), 
            fill: Color32::PLACEHOLDER, 
            stroke: Stroke::NONE };
            let intersection = get_circles_intersection(compass_circle, c);
            
        find_intersection(&polar_projection_segment.as_straight_line(), &intersection.as_straight_line())
    }
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
        y: (a2 * c1 - a1 * c2) / (a1 * b2 - a2 * b1) 
    }
}

pub fn is_segment_intersecting_circle(s: &Segment, c: CircleShape) -> bool {
    c.radius >= calculate_distance(&s.as_straight_line(), c.center)
}

pub fn calculate_distance(l: &StraightLine, p: Pos2) -> f32 {
    (
        (l.a * p.x + l.b * p.y + l.c) / (l.a.powi(2) + l.b.powi(2)).sqrt()
    ).abs()
}

pub fn get_circle_straight_line_intersection(l: &StraightLine, c: CircleShape) -> Segment {
    let projection: Pos2 = find_projection(l, c.center);
    let length_a = calculate_distance(l, c.center);
    let length_b = (c.radius.powi(2) - length_a.powi(2)).sqrt();
    let denominator = (l.b.powi(2) + l.a.powi(2)).sqrt();

    Segment(
        Pos2 { 
            x: projection.x + l.b * length_b / denominator, 
            y: projection.y - l.a * length_b / denominator
        },
        Pos2 { 
            x: projection.x - l.b * length_b / denominator, 
            y: projection.y + l.a * length_b / denominator
        }
    )
}

pub fn get_circles_intersection(c1: CircleShape, c2: CircleShape) -> Segment {
    let t0: StraightLine = get_circles_intersection_as_straight_line(c1, c2);
    let t1: StraightLine = StraightLine { 
        a: c2.center.y - c1.center.y, 
        b: c1.center.x - c2.center.x, 
        c: c1.center.y * c2.center.x - c1.center.x * c2.center.y 
    };
    let intersection_t0_t1: Pos2 = find_intersection(&t0, &t1);

    let length_a = calculate_distance(&t0, c1.center);
    let length_b = (c1.radius.powi(2) - length_a.powi(2)).sqrt();
    let denominator = (t0.b.powi(2) + t0.a.powi(2)).sqrt();
    
    Segment(
        Pos2 { 
            x: intersection_t0_t1.x + t0.b * length_b / denominator, 
            y: intersection_t0_t1.y - t0.a * length_b / denominator
        },
        Pos2 { 
            x: intersection_t0_t1.x - t0.b * length_b / denominator, 
            y: intersection_t0_t1.y + t0.a * length_b / denominator
        }
    )
}

fn get_circles_intersection_as_straight_line(c1: CircleShape, c2: CircleShape) -> StraightLine {
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