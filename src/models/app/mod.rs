use egui::{Pos2, Rect};

use crate::enums::{dragging::Dragging, theme_mode::ThemeMode};

use super::circle::Circle;

#[derive(Clone, Copy)]
pub struct InitialCircles {
    pub circle_1: Circle,
    pub circle_2: Circle,
    pub circle_3: Circle,
}
impl InitialCircles {
    pub fn as_array(self) -> [Circle; 3] {
        [self.circle_1, self.circle_2, self.circle_3]
    }

    pub fn same_radius(self) -> bool {
        self.circle_1.radius == self.circle_2.radius && self.circle_2.radius == self.circle_3.radius
    }
}
impl Default for InitialCircles {
    fn default() -> Self {
        Self {
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
        }
    }
}

#[derive(Clone, Copy)]
pub struct DisplayOptions {
    pub show_homothetic: bool,
    pub show_radical: bool,
    pub show_inverse_poles: bool,
    pub show_apollonius_circle_1: bool,
    pub show_apollonius_circle_2: bool,
    pub show_apollonius_circle_3: bool,
    pub show_apollonius_circle_4: bool,
}
impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            show_homothetic: false,
            show_radical: true,
            show_inverse_poles: false,
            show_apollonius_circle_1: true,
            show_apollonius_circle_2: true,
            show_apollonius_circle_3: true,
            show_apollonius_circle_4: true,
        }
    }
}

#[derive(Clone)]
pub struct MyApp {
    pub initial_circles: InitialCircles,
    pub display_options: DisplayOptions,
    pub scene_rect: egui::Rect,
    pub is_dragging: Dragging,
    pub theme_mode: ThemeMode,
}
impl MyApp {
    fn reset() -> Self {
        Self {
            initial_circles: InitialCircles::default(),
            display_options: DisplayOptions::default(),
            scene_rect: Rect::ZERO,
            is_dragging: Dragging::None,
            theme_mode: ThemeMode::Dark,
        }
    }

    pub fn reset_scene(&self) -> Self {
        Self {
            scene_rect: Rect::ZERO,
            ..*self
        }
    }
    pub fn reset_circles(&self) -> Self {
        Self {
            initial_circles: InitialCircles::default(),
            ..*self
        }
    }
}
impl Default for MyApp {
    fn default() -> Self {
        Self::reset()
    }
}
