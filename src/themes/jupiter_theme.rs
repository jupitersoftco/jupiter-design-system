//! Jupiter Software theme for the design system
//!
//! Provides a complete theme based on Jupiter's planetary orange and tech blue
//! color scheme, designed for AI consulting and enterprise software applications.

use crate::core::jupiter_colors::JupiterColors;
use crate::themes::Theme;

/// Jupiter Software theme implementing planetary orange/tech blue branding
#[derive(Debug, Clone, Default)]
pub struct JupiterTheme {
    colors: JupiterColors,
}

impl Theme for JupiterTheme {
    fn name(&self) -> &str {
        "Jupiter Software"
    }
}

impl JupiterTheme {
    /// Create a new Jupiter theme
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the color provider for this theme
    pub fn colors(&self) -> &JupiterColors {
        &self.colors
    }

    /// Get a mutable reference to colors for customization
    pub fn colors_mut(&mut self) -> &mut JupiterColors {
        &mut self.colors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::color::ColorProvider;

    #[test]
    fn test_jupiter_theme_creation() {
        let theme = JupiterTheme::new();
        assert_eq!(theme.name(), "Jupiter Software");
    }

    #[test]
    fn test_jupiter_theme_colors() {
        let theme = JupiterTheme::new();
        let colors = theme.colors();
        
        assert_eq!(colors.palette().primary, "jupiter-orange-500");
        assert_eq!(colors.palette().secondary, "jupiter-blue-500");
    }

    #[test]
    fn test_jupiter_theme_color_resolution() {
        let theme = JupiterTheme::new();
        let colors = theme.colors();
        
        // Test CSS class generation
        let primary_bg = colors.bg_class(crate::core::color::Color::Primary);
        assert_eq!(primary_bg, "bg-jupiter-orange-500");
        
        let secondary_text = colors.text_class(crate::core::color::Color::Secondary);
        assert_eq!(secondary_text, "text-jupiter-blue-500");
    }

    #[test]
    fn test_jupiter_theme_hex_colors() {
        let theme = JupiterTheme::new();
        let colors = theme.colors();
        
        // Test hex color extraction for icons
        assert_eq!(colors.hex_color(crate::core::color::Color::Primary), "#FF6B35");
        assert_eq!(colors.hex_color(crate::core::color::Color::Secondary), "#4A90E2");
    }
}