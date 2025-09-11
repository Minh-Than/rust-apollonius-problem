#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn value(&self) -> String {
        match *self {
            ThemeMode::Light => String::from("Light"),
            ThemeMode::Dark => String::from("Dark"),
        }
    }

    pub fn as_vec() -> Vec<Self> {
        vec![Self::Light, Self::Dark]
    }
}
