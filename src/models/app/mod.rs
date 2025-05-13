use egui::{Pos2, Rect};

use crate::enums::{dragging::Dragging, theme_mode::ThemeMode};

use super::circle::Circle;

#[derive(Clone)]
pub struct MyApp {
    pub scene_rect: egui::Rect,
    pub circle_1: Circle,
    pub circle_2: Circle,
    pub circle_3: Circle,
    pub is_dragging: Dragging,
    pub show_homothetic: bool,
    pub show_radical: bool,
    pub show_inverse_poles: bool,
    pub show_connectors: bool,
    pub show_apollonius_circle_1: bool,
    pub show_apollonius_circle_2: bool,
    pub show_apollonius_circle_3: bool,
    pub show_apollonius_circle_4: bool,
    pub theme_mode: ThemeMode,
}
impl MyApp {
    pub fn reset() -> MyApp {
        MyApp {
            scene_rect: Rect::ZERO,
            circle_1: Circle {
                center: Pos2 { x: 50.0, y: 50.0 },
                radius: 35.0,
            },
            circle_2: Circle {
                center: Pos2 { x: -60.0, y: 10.0 },
                radius: 50.0,
            },
            circle_3: Circle {
                center: Pos2 { x: 40.0, y: -30.0 },
                radius: 20.0,
            },
            is_dragging: Dragging::None,
            show_homothetic: false,
            show_radical: true,
            show_inverse_poles: false,
            show_connectors: false,
            show_apollonius_circle_1: true,
            show_apollonius_circle_2: true,
            show_apollonius_circle_3: true,
            show_apollonius_circle_4: true,
            theme_mode: ThemeMode::Dark,
        }
    }
}
impl Default for MyApp {
    fn default() -> Self {
        Self::reset()
    }
}
