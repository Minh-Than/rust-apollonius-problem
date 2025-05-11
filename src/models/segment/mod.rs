use egui::Pos2;

use super::straightline::StraightLine;

#[derive(Clone)]
pub struct Segment(pub Pos2, pub Pos2);
impl Segment {
    pub fn as_straight_line(&self) -> StraightLine {
        StraightLine { 
            a: self.1.y - self.0.y, 
            b: self.0.x - self.1.x, 
            c: self.0.y * self.1.x - self.0.x * self.1.y
        }
    }
}