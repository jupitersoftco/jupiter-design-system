//! Theme system for the design system

use crate::core::color::VibeColors;

/// Trait for theme providers
pub trait Theme {
    /// Get theme name
    fn name(&self) -> &str;
}

/// Water & Wellness theme
#[derive(Debug, Clone, Default)]
pub struct VibeTheme {
    #[allow(dead_code)]
    colors: VibeColors,
}

impl Theme for VibeTheme {
    fn name(&self) -> &str {
        "Water & Wellness"
    }
}

impl VibeTheme {
    pub fn new() -> Self {
        Self::default()
    }
}
