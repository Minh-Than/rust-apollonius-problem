use egui::Pos2;

use super::straightline::StraightLine;

#[derive(Clone, Copy)]
pub struct Segment(pub Pos2, pub Pos2);
impl Segment {
    pub fn as_straight_line(&self) -> StraightLine {
        StraightLine {
            a: self.1.y - self.0.y,
            b: self.0.x - self.1.x,
            c: self.0.y * self.1.x - self.0.x * self.1.y,
        }
    }

    pub fn get_any_valid_segment(points: Vec<Option<Pos2>>) -> Option<Self> {
        let filtered_points: Vec<Pos2> = points.into_iter().flatten().collect();
        if filtered_points.len() < 2 {
            return None;
        }

        Some(Segment(filtered_points[0], filtered_points[1]))
    }
}
