//! Color system for the design system
//!
//! Provides trait-based color management with semantic color naming
//! and theme-aware color resolution.

use serde::{Deserialize, Serialize};

/// Semantic color tokens for consistent theming
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    // Brand colors
    Primary,
    Secondary,

    // Semantic colors
    Success,
    Warning,
    Error,
    Info,

    // Neutral colors
    Surface,
    Background,
    Foreground,
    Border,

    // Text colors
    TextPrimary,
    TextSecondary,
    TextTertiary,
    TextInverse,

    // Interactive states
    Interactive,
    InteractiveHover,
    InteractiveActive,
    InteractiveDisabled,
}

/// Color palette containing all color values for a theme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorPalette {
    // Brand colors
    pub primary: String,
    pub secondary: String,

    // Semantic colors
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,

    // Neutral colors
    pub surface: String,
    pub background: String,
    pub foreground: String,
    pub border: String,

    // Text colors
    pub text_primary: String,
    pub text_secondary: String,
    pub text_tertiary: String,
    pub text_inverse: String,

    // Interactive states
    pub interactive: String,
    pub interactive_hover: String,
    pub interactive_active: String,
    pub interactive_disabled: String,
}

/// Trait for providing color values from a color palette
pub trait ColorProvider {
    /// Get the color palette for this provider
    fn palette(&self) -> &ColorPalette;

    /// Resolve a semantic color to its CSS class or value
    fn resolve_color(&self, color: Color) -> &str {
        let palette = self.palette();
        match color {
            Color::Primary => &palette.primary,
            Color::Secondary => &palette.secondary,
            Color::Success => &palette.success,
            Color::Warning => &palette.warning,
            Color::Error => &palette.error,
            Color::Info => &palette.info,
            Color::Surface => &palette.surface,
            Color::Background => &palette.background,
            Color::Foreground => &palette.foreground,
            Color::Border => &palette.border,
            Color::TextPrimary => &palette.text_primary,
            Color::TextSecondary => &palette.text_secondary,
            Color::TextTertiary => &palette.text_tertiary,
            Color::TextInverse => &palette.text_inverse,
            Color::Interactive => &palette.interactive,
            Color::InteractiveHover => &palette.interactive_hover,
            Color::InteractiveActive => &palette.interactive_active,
            Color::InteractiveDisabled => &palette.interactive_disabled,
        }
    }

    /// Get a Tailwind CSS class for text color
    fn text_class(&self, color: Color) -> String {
        format!("text-{}", self.resolve_color(color))
    }

    /// Get a Tailwind CSS class for background color
    fn bg_class(&self, color: Color) -> String {
        format!("bg-{}", self.resolve_color(color))
    }

    /// Get a Tailwind CSS class for border color
    fn border_class(&self, color: Color) -> String {
        format!("border-{}", self.resolve_color(color))
    }
}

/// Default Water & Wellness color palette
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaterWellnessColors {
    palette: ColorPalette,
}

impl Default for WaterWellnessColors {
    fn default() -> Self {
        Self {
            palette: ColorPalette {
                // Brand colors - Water & Wellness theme
                primary: "water-blue-500".to_string(),
                secondary: "water-green-500".to_string(),

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
                interactive: "water-blue-500".to_string(),
                interactive_hover: "water-blue-600".to_string(),
                interactive_active: "water-blue-700".to_string(),
                interactive_disabled: "gray-300".to_string(),
            },
        }
    }
}

impl ColorProvider for WaterWellnessColors {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

impl WaterWellnessColors {
    /// Create a new Water & Wellness color provider
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a Water & Wellness color provider with custom overrides
    pub fn with_overrides(overrides: impl Fn(&mut ColorPalette)) -> Self {
        let mut palette = ColorPalette::default();
        overrides(&mut palette);
        Self { palette }
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        WaterWellnessColors::default().palette
    }
}

#[cfg(test)]
#[path = "color_test.rs"]
mod color_test;
