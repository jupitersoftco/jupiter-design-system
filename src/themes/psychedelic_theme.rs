//! Psychedelic theme for Jupiter Design System
//!
//! A vibrant, high-energy theme with bold colors and psychedelic vibes.

use crate::core::color::{ColorPalette, ColorProvider};

/// Psychedelic color palette with vibrant, bold colors
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PsychedelicColors {
    palette: ColorPalette,
}

impl Default for PsychedelicColors {
    fn default() -> Self {
        Self {
            palette: ColorPalette {
                // Brand colors - Psychedelic theme with vibrant colors
                primary: "fuchsia-500".to_string(), // Electric magenta
                secondary: "lime-400".to_string(),  // Electric lime
                accent: "cyan-400".to_string(),     // Electric cyan

                // Semantic colors - Bold and vibrant
                success: "emerald-400".to_string(), // Bright emerald
                warning: "orange-400".to_string(),  // Electric orange
                error: "rose-400".to_string(),      // Hot pink error
                info: "violet-400".to_string(),     // Electric violet

                // Neutral colors - High contrast
                surface: "slate-900".to_string(), // Dark surface for contrast
                background: "black".to_string(),  // Pure black background
                foreground: "white".to_string(),  // Pure white text
                border: "purple-500".to_string(), // Purple borders

                // Text colors - High contrast for readability
                text_primary: "white".to_string(),
                text_secondary: "gray-200".to_string(),
                text_tertiary: "gray-400".to_string(),
                text_inverse: "black".to_string(),

                // Interactive states - Electric variations
                interactive: "fuchsia-500".to_string(),
                interactive_hover: "fuchsia-400".to_string(),
                interactive_active: "fuchsia-600".to_string(),
                interactive_disabled: "gray-600".to_string(),
            },
        }
    }
}

impl ColorProvider for PsychedelicColors {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

impl PsychedelicColors {
    /// Create a new psychedelic color provider
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a psychedelic color provider with custom overrides
    pub fn with_overrides(overrides: impl Fn(&mut ColorPalette)) -> Self {
        let mut palette = ColorPalette::default();
        overrides(&mut palette);
        Self { palette }
    }
}

/// Psychedelic theme configuration
#[derive(Debug, Clone, Default)]
pub struct PsychedelicTheme {
    colors: PsychedelicColors,
}

impl crate::themes::Theme for PsychedelicTheme {
    fn name(&self) -> &str {
        "Psychedelic"
    }
}

impl PsychedelicTheme {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn colors(&self) -> &PsychedelicColors {
        &self.colors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::themes::Theme;

    #[test]
    fn test_psychedelic_colors_default() {
        let colors = PsychedelicColors::default();
        let palette = colors.palette();

        // Test brand colors are vibrant
        assert_eq!(palette.primary, "fuchsia-500");
        assert_eq!(palette.secondary, "lime-400");
        assert_eq!(palette.accent, "cyan-400");
    }

    #[test]
    fn test_psychedelic_theme() {
        let theme = PsychedelicTheme::new();
        assert_eq!(theme.name(), "Psychedelic");
    }
}
