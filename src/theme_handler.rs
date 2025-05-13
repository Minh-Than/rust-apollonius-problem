use egui::{
    Color32,
    ahash::{HashMap, HashMapExt},
};
use once_cell::sync::Lazy;

use crate::enums::{color_item_names::ColorItemNames, theme_mode::ThemeMode};

static COLORS_MAP: Lazy<HashMap<ColorItemNames, (Color32, Color32)>> = Lazy::new(|| {
    let mut colors_mapping = HashMap::new();

    colors_mapping.insert(
        ColorItemNames::InitialCircles,
        (
            Color32::BLACK.gamma_multiply(0.2),
            Color32::WHITE.gamma_multiply(0.3),
        ),
    );
    colors_mapping.insert(
        ColorItemNames::HomotheticCenters,
        (Color32::GRAY, Color32::GRAY),
    );
    colors_mapping.insert(ColorItemNames::Radical, (Color32::BLACK, Color32::WHITE));
    colors_mapping.insert(
        ColorItemNames::InversePoles1,
        (
            Color32::from_rgb(220, 170, 45),
            Color32::from_rgb(195, 146, 9),
        ),
    );
    colors_mapping.insert(
        ColorItemNames::InversePoles2,
        (
            Color32::from_rgb(208, 94, 216),
            Color32::from_rgb(189, 15, 216),
        ),
    );
    colors_mapping.insert(
        ColorItemNames::InversePoles3,
        (
            Color32::from_rgb(107, 206, 52),
            Color32::from_rgb(61, 168, 38),
        ),
    );
    colors_mapping.insert(
        ColorItemNames::InversePoles4,
        (
            Color32::from_rgb(23, 155, 246),
            Color32::from_rgb(6, 126, 202),
        ),
    );

    colors_mapping
});

pub fn get_color(key: ColorItemNames, theme_mode: &ThemeMode) -> Color32 {
    match COLORS_MAP.get(&key) {
        Some((light, dark)) => match theme_mode {
            ThemeMode::Light => *light,
            ThemeMode::Dark => *dark,
        },
        None => Color32::PLACEHOLDER,
    }
}
