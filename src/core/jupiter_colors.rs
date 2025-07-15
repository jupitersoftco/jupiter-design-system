//! Jupiter Software color system
//!
//! Provides Jupiter-branded color palette with orange/blue color scheme
//! based on the Jupiter planetary theme and technology branding.

use crate::core::color::{ColorPalette, ColorProvider};

/// Jupiter Software color provider implementing planetary orange/tech blue theme
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JupiterColors {
    palette: ColorPalette,
}

impl Default for JupiterColors {
    fn default() -> Self {
        Self {
            palette: ColorPalette {
                // Brand colors - Jupiter theme
                primary: "jupiter-orange-500".to_string(),      // #FF6B35 - Jupiter orange
                secondary: "jupiter-blue-500".to_string(),      // #4A90E2 - Tech blue

                // Semantic colors
                success: "green-500".to_string(),               // #10B981
                warning: "amber-500".to_string(),               // #F59E0B  
                error: "red-500".to_string(),                   // #EF4444
                info: "jupiter-blue-500".to_string(),           // #4A90E2 - Use brand blue

                // Neutral colors
                surface: "jupiter-gray-50".to_string(),         // #F8FAFC
                background: "white".to_string(),                // #FFFFFF
                foreground: "jupiter-gray-900".to_string(),     // #1A202C
                border: "jupiter-gray-200".to_string(),         // #E2E8F0

                // Text colors
                text_primary: "jupiter-gray-900".to_string(),   // #1A202C
                text_secondary: "jupiter-gray-700".to_string(), // #374151
                text_tertiary: "jupiter-gray-500".to_string(),  // #64748B
                text_inverse: "white".to_string(),              // #FFFFFF

                // Interactive states
                interactive: "jupiter-orange-500".to_string(),         // #FF6B35
                interactive_hover: "jupiter-orange-600".to_string(),   // #F49D37
                interactive_active: "jupiter-orange-700".to_string(),  // #E8944A
                interactive_disabled: "jupiter-gray-300".to_string(),  // #CBD5E1
            },
        }
    }
}

impl ColorProvider for JupiterColors {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

impl JupiterColors {
    /// Create a new Jupiter color provider
    pub fn new() -> Self {
        Self::default()
    }

    /// Create Jupiter colors with custom overrides
    pub fn with_overrides(overrides: impl Fn(&mut ColorPalette)) -> Self {
        let mut palette = Self::default().palette;
        overrides(&mut palette);
        Self { palette }
    }

    /// Get hex color value for non-CSS contexts (SVG icons, etc.)
    pub fn hex_color(&self, color: crate::core::color::Color) -> &'static str {
        use crate::core::color::Color;
        
        match color {
            Color::Primary => "#FF6B35",           // jupiter-orange-500
            Color::Secondary => "#4A90E2",         // jupiter-blue-500
            Color::Success => "#10B981",           // green-500
            Color::Warning => "#F59E0B",           // amber-500
            Color::Error => "#EF4444",             // red-500
            Color::Info => "#4A90E2",              // jupiter-blue-500
            Color::Surface => "#F8FAFC",           // jupiter-gray-50
            Color::Background => "#FFFFFF",        // white
            Color::Foreground => "#1A202C",        // jupiter-gray-900
            Color::Border => "#E2E8F0",            // jupiter-gray-200
            Color::TextPrimary => "#1A202C",       // jupiter-gray-900
            Color::TextSecondary => "#374151",     // jupiter-gray-700
            Color::TextTertiary => "#64748B",      // jupiter-gray-500
            Color::TextInverse => "#FFFFFF",       // white
            Color::Interactive => "#FF6B35",       // jupiter-orange-500
            Color::InteractiveHover => "#F49D37",  // jupiter-orange-600
            Color::InteractiveActive => "#E8944A", // jupiter-orange-700
            Color::InteractiveDisabled => "#CBD5E1", // jupiter-gray-300
        }
    }

    /// Generate Jupiter brand gradient classes
    pub fn primary_gradient(&self) -> String {
        "bg-gradient-to-r from-jupiter-orange-500 to-jupiter-orange-600".to_string()
    }

    /// Generate secondary gradient classes
    pub fn secondary_gradient(&self) -> String {
        "bg-gradient-to-r from-jupiter-blue-500 to-jupiter-blue-600".to_string()
    }

    /// Generate hero section gradient
    pub fn hero_gradient(&self) -> String {
        "bg-gradient-to-br from-jupiter-orange-50 via-white to-jupiter-blue-50".to_string()
    }

    /// Generate brand accent gradient (orange to blue)
    pub fn brand_gradient(&self) -> String {
        "bg-gradient-to-r from-jupiter-orange-600 to-jupiter-blue-600".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::color::Color;

    #[test]
    fn test_jupiter_colors_creation() {
        let colors = JupiterColors::new();
        assert_eq!(colors.palette().primary, "jupiter-orange-500");
        assert_eq!(colors.palette().secondary, "jupiter-blue-500");
    }

    #[test]
    fn test_jupiter_hex_colors() {
        let colors = JupiterColors::new();
        assert_eq!(colors.hex_color(Color::Primary), "#FF6B35");
        assert_eq!(colors.hex_color(Color::Secondary), "#4A90E2");
    }

    #[test]
    fn test_jupiter_color_resolution() {
        let colors = JupiterColors::new();
        assert_eq!(colors.resolve_color(Color::Primary), "jupiter-orange-500");
        assert_eq!(colors.resolve_color(Color::Secondary), "jupiter-blue-500");
    }

    #[test]
    fn test_jupiter_gradients() {
        let colors = JupiterColors::new();
        let primary_gradient = colors.primary_gradient();
        assert!(primary_gradient.contains("jupiter-orange"));
        
        let brand_gradient = colors.brand_gradient();
        assert!(brand_gradient.contains("jupiter-orange"));
        assert!(brand_gradient.contains("jupiter-blue"));
    }

    #[test]
    fn test_jupiter_colors_with_overrides() {
        let colors = JupiterColors::with_overrides(|palette| {
            palette.primary = "custom-orange-500".to_string();
        });
        
        assert_eq!(colors.palette().primary, "custom-orange-500");
        assert_eq!(colors.palette().secondary, "jupiter-blue-500"); // Should remain unchanged
    }
}