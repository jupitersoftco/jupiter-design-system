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
    Accent,

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
    pub accent: String,

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
            Color::Accent => &palette.accent,
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

impl Default for ColorPalette {
    fn default() -> Self {
        use crate::core::color::ColorProvider;
        use crate::themes::VibeColors;
        VibeColors::default().palette().clone()
    }
}

#[cfg(test)]
#[path = "color_test.rs"]
mod color_test;
