use egui::{epaint::CircleShape, Color32, Pos2, Rect, Stroke};

use crate::enums::dragging::Dragging;

#[derive(Clone)]
pub struct MyApp {
    pub scene_rect: egui::Rect,
    pub circle_1: CircleShape,
    pub circle_2: CircleShape,
    pub circle_3: CircleShape,
    pub is_dragging: Dragging,
    pub show_homothetic: bool,
    pub show_radical: bool,
    pub show_inverse_poles: bool,
    pub show_connectors: bool,
    pub show_apollonius_circle_1: bool,
    pub show_apollonius_circle_2: bool,
    pub show_apollonius_circle_3: bool,
    pub show_apollonius_circle_4: bool,
}
impl MyApp {
    pub fn reset() -> MyApp {
        MyApp {
            scene_rect: Rect::ZERO,
            circle_1: CircleShape { center: Pos2 { x: 50.0, y: 50.0 }, radius: 35.0, fill: Color32::from_white_alpha(50), stroke: Stroke::NONE },
            circle_2: CircleShape { center: Pos2 { x: -60.0, y: 10.0 }, radius: 50.0, fill: Color32::from_white_alpha(50), stroke: Stroke::NONE },
            circle_3: CircleShape { center: Pos2 { x: 40.0, y: -30.0 }, radius: 20.0, fill: Color32::from_white_alpha(50), stroke: Stroke::NONE },
            is_dragging: Dragging::None,
            show_homothetic: false,
            show_radical: true,
            show_inverse_poles: false,
            show_connectors: false,
            show_apollonius_circle_1: true,
            show_apollonius_circle_2: true,
            show_apollonius_circle_3: true,
            show_apollonius_circle_4: true,
        }
    }
}
impl Default for MyApp {
    fn default() -> Self { Self::reset() }
}

