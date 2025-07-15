//! LLASI Brand Color Theme for Jupiter Design System
//!
//! Implementation of LLASI's minimalist luxury color palette
//! following the brand guidelines in their product requirements.

use crate::core::color::{Color, ColorPalette, ColorProvider};

/// LLASI brand color provider implementing the minimalist luxury palette
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlasiColors {
    palette: ColorPalette,
}

impl Default for LlasiColors {
    fn default() -> Self {
        Self {
            palette: ColorPalette {
                // Brand colors - LLASI minimalist palette
                primary: "slate-900".to_string(),      // Deep Charcoal (#212121)
                secondary: "slate-600".to_string(),    // Soft Gray (#6B6B6B)

                // Semantic colors - muted to match brand aesthetic
                success: "emerald-600".to_string(),
                warning: "amber-600".to_string(),
                error: "red-600".to_string(),          // Sale Red (#C60C0C)
                info: "blue-600".to_string(),

                // Neutral colors - high contrast monochromatic
                surface: "white".to_string(),          // Pure White (#FFFFFF)
                background: "amber-50".to_string(),    // Warm Cream (#F4EEDA)
                foreground: "slate-50".to_string(),    // Highlight Gray (#F7F7F7)
                border: "slate-200".to_string(),       // Border Gray (#E6E6E6)

                // Text colors - strong hierarchy
                text_primary: "slate-900".to_string(),   // Deep Charcoal
                text_secondary: "slate-600".to_string(), // Soft Gray
                text_tertiary: "slate-400".to_string(),  // Light Gray
                text_inverse: "white".to_string(),       // Pure White

                // Interactive states - consistent with brand
                interactive: "slate-900".to_string(),         // Deep Charcoal
                interactive_hover: "black".to_string(),       // Active Black (#000000)
                interactive_active: "black".to_string(),      // Active Black
                interactive_disabled: "slate-300".to_string(), // Light Gray (#E5E5E5)
            },
        }
    }
}

impl ColorProvider for LlasiColors {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

impl LlasiColors {
    /// Create a new LLASI color provider
    pub fn new() -> Self {
        Self::default()
    }

    /// Create LLASI colors with custom overrides while maintaining brand consistency
    pub fn with_overrides(overrides: impl Fn(&mut ColorPalette)) -> Self {
        let mut palette = Self::default().palette;
        overrides(&mut palette);
        Self { palette }
    }

    /// Get the exact hex values as defined in LLASI brand guidelines
    pub fn hex_value(&self, color: Color) -> &'static str {
        match color {
            Color::Primary => "#212121",        // Deep Charcoal
            Color::Secondary => "#6B6B6B",      // Soft Gray
            Color::Surface => "#FFFFFF",        // Pure White
            Color::Background => "#F4EEDA",     // Warm Cream
            Color::Border => "#E6E6E6",         // Border Gray
            Color::Error => "#C60C0C",          // Sale Red
            Color::TextPrimary => "#212121",    // Deep Charcoal
            Color::TextSecondary => "#6B6B6B",  // Soft Gray
            Color::TextTertiary => "#E5E5E5",   // Light Gray
            Color::TextInverse => "#FFFFFF",    // Pure White
            Color::Interactive => "#212121",    // Deep Charcoal
            Color::InteractiveHover => "#000000", // Active Black
            Color::InteractiveActive => "#000000", // Active Black
            Color::InteractiveDisabled => "#E5E5E5", // Light Gray
            _ => "#000000", // Fallback
        }
    }

    /// Generate CSS custom properties for the LLASI color system
    pub fn css_custom_properties(&self) -> String {
        format!(
            r#":root {{
  /* LLASI Brand Colors */
  --color-deep-charcoal: #212121;
  --color-pure-white: #ffffff;
  --color-warm-cream: #F4EEDA;
  --color-soft-gray: #6b6b6b;
  --color-border-gray: #e6e6e6;
  
  /* Extended Palette */
  --color-active-black: #000000;
  --color-light-gray: #e5e5e5;
  --color-highlight-gray: #f7f7f7;
  --color-footer-gray: #ebebeb;
  --color-input-gray: #d6d6d6;
  
  /* Accent Colors */
  --color-sale-red: #c60c0c;
  --color-brand-accent: #8e8e8e;

  /* Semantic Mappings */
  --color-text-primary: var(--color-deep-charcoal);
  --color-text-secondary: var(--color-soft-gray);
  --color-text-inactive: var(--color-light-gray);
  --color-bg-primary: var(--color-pure-white);
  --color-bg-secondary: var(--color-warm-cream);
  --color-bg-footer: var(--color-footer-gray);
  --color-button-primary: var(--color-deep-charcoal);
  --color-button-primary-text: var(--color-pure-white);
  --color-button-hover: var(--color-active-black);
  --color-sale: var(--color-sale-red);
  --color-border: var(--color-border-gray);
}}"#
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llasi_colors_creation() {
        let colors = LlasiColors::new();
        assert_eq!(colors.resolve_color(Color::Primary), "slate-900");
        assert_eq!(colors.resolve_color(Color::Secondary), "slate-600");
        assert_eq!(colors.resolve_color(Color::Surface), "white");
        assert_eq!(colors.resolve_color(Color::Background), "amber-50");
    }

    #[test]
    fn test_llasi_hex_values() {
        let colors = LlasiColors::new();
        assert_eq!(colors.hex_value(Color::Primary), "#212121");
        assert_eq!(colors.hex_value(Color::Surface), "#FFFFFF");
        assert_eq!(colors.hex_value(Color::Background), "#F4EEDA");
        assert_eq!(colors.hex_value(Color::Error), "#C60C0C");
    }

    #[test]
    fn test_css_classes() {
        let colors = LlasiColors::new();
        assert_eq!(colors.text_class(Color::Primary), "text-slate-900");
        assert_eq!(colors.bg_class(Color::Surface), "bg-white");
        assert_eq!(colors.border_class(Color::Border), "border-slate-200");
    }

    #[test]
    fn test_brand_consistency() {
        let colors = LlasiColors::new();
        
        // Ensure primary and interactive colors are consistent (brand requirement)
        assert_eq!(
            colors.resolve_color(Color::Primary),
            colors.resolve_color(Color::Interactive)
        );
        
        // Ensure text hierarchy is properly implemented
        assert_ne!(
            colors.resolve_color(Color::TextPrimary),
            colors.resolve_color(Color::TextSecondary)
        );
    }
}