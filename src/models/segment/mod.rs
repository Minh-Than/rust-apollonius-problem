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

    /// Find and return `Some(Segment)` instance comprising of the first-encountered pair of adjacent `Some(Point)` in vector `points`, otherwise return `None`.
    ///
    /// Disclaimer: This is a very specific method which assumes that the points are parts of a 1-dimensional span
    /// (all lying on one straight line).
    pub fn get_any_valid_segment(points: &Vec<Option<Pos2>>) -> Option<Self> {
        points.windows(2).find_map(|w| match (w[0], w[1]) {
            (Some(a), Some(b)) => Some(Self(a, b)),
            _ => None,
        })
    }
}
