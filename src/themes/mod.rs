//! Theme system for the design system

use crate::core::color::VibeColors;

pub mod psychedelic_theme;
pub use psychedelic_theme::{PsychedelicColors, PsychedelicTheme};

/// Trait for theme providers
pub trait Theme {
    /// Get theme name
    fn name(&self) -> &str;
}

/// Jupiter Design System theme
#[derive(Debug, Clone, Default)]
pub struct VibeTheme {
    #[allow(dead_code)]
    colors: VibeColors,
}

impl Theme for VibeTheme {
    fn name(&self) -> &str {
        "Jupiter"
    }
}

impl VibeTheme {
    pub fn new() -> Self {
        Self::default()
    }

    /// Available themes
    pub fn available_themes() -> Vec<&'static str> {
        vec![
            "jupiter",     // Jupiter Design System
            "llasi",       // Llasi theme
            "psychedelic", // Psychedelic theme
        ]
    }

    /// Get theme description
    pub fn theme_description(theme: &str) -> &'static str {
        match theme {
            "jupiter" => "Jupiter Design System default theme",
            "llasi" => "Llasi theme",
            "psychedelic" => "Vibrant psychedelic theme with electric colors",
            _ => "Unknown theme",
        }
    }
}
