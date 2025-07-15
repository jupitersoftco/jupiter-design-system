//! Theme system for the design system

use crate::core::color::{ColorPalette, ColorProvider};

/// Trait for theme providers
pub trait Theme {
    /// Get theme name
    fn name(&self) -> &str;
}

/// Default Jupiter Design System color palette with vibrant psychedelic colors
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VibeColors {
    palette: ColorPalette,
}

impl Default for VibeColors {
    fn default() -> Self {
        Self {
            palette: ColorPalette {
                // Brand colors - Jupiter Design System
                primary: "jupiter-blue-500".to_string(),
                secondary: "jupiter-green-500".to_string(),
                accent: "jupiter-orange-500".to_string(),

                // Semantic colors
                success: "green-500".to_string(),
                warning: "amber-500".to_string(),
                error: "red-500".to_string(),
                info: "blue-500".to_string(),

                // Neutral colors
                surface: "white".to_string(),
                background: "gray-50".to_string(),
                foreground: "gray-900".to_string(),
                border: "gray-200".to_string(),

                // Text colors
                text_primary: "gray-900".to_string(),
                text_secondary: "gray-600".to_string(),
                text_tertiary: "gray-400".to_string(),
                text_inverse: "white".to_string(),

                // Interactive states
                interactive: "jupiter-blue-500".to_string(),
                interactive_hover: "jupiter-blue-600".to_string(),
                interactive_active: "jupiter-blue-700".to_string(),
                interactive_disabled: "gray-300".to_string(),
            },
        }
    }
}

impl ColorProvider for VibeColors {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

impl VibeColors {
    /// Create a new Jupiter color provider
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a Jupiter color provider with custom overrides
    pub fn with_overrides(overrides: impl Fn(&mut ColorPalette)) -> Self {
        let mut palette = ColorPalette::default();
        overrides(&mut palette);
        Self { palette }
    }
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
        vec!["jupiter"] // Only Jupiter theme available now
    }

    /// Get theme description
    pub fn theme_description(theme: &str) -> &'static str {
        match theme {
            "jupiter" => "Jupiter Design System with vibrant psychedelic colors",
            _ => "Unknown theme",
        }
    }
}
