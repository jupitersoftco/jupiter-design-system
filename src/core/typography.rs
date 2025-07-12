//! Typography system for the design system

use serde::{Deserialize, Serialize};

/// Typography tokens for consistent text styling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Typography {
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    Body,
    BodySmall,
    Caption,
    Label,
}

/// Font weight tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FontWeight {
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
}

/// Font family tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FontFamily {
    Sans,
    Serif,
    Mono,
}

/// Trait for providing typography values
pub trait TypographyProvider {
    /// Resolve typography to CSS class
    fn resolve_typography(&self, typography: Typography) -> &str;

    /// Get typography class
    fn typography_class(&self, typography: Typography) -> String {
        format!("text-{}", self.resolve_typography(typography))
    }

    /// Get font weight class
    fn font_weight_class(&self, weight: FontWeight) -> String {
        match weight {
            FontWeight::Light => "font-light".to_string(),
            FontWeight::Normal => "font-normal".to_string(),
            FontWeight::Medium => "font-medium".to_string(),
            FontWeight::SemiBold => "font-semibold".to_string(),
            FontWeight::Bold => "font-bold".to_string(),
        }
    }

    /// Get font family class
    fn font_family_class(&self, family: FontFamily) -> String {
        match family {
            FontFamily::Sans => "font-sans".to_string(),
            FontFamily::Serif => "font-serif".to_string(),
            FontFamily::Mono => "font-mono".to_string(),
        }
    }
}
