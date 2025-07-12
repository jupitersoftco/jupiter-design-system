//! Theme system for the design system

use crate::core::color::WaterWellnessColors;

/// Trait for theme providers
pub trait Theme {
    /// Get theme name
    fn name(&self) -> &str;
}

/// Water & Wellness theme
#[derive(Debug, Clone, Default)]
pub struct WaterWellnessTheme {
    #[allow(dead_code)]
    colors: WaterWellnessColors,
}

impl Theme for WaterWellnessTheme {
    fn name(&self) -> &str {
        "Water & Wellness"
    }
}

impl WaterWellnessTheme {
    pub fn new() -> Self {
        Self::default()
    }
}
