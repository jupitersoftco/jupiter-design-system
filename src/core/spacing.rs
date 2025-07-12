//! Spacing system for the design system

use serde::{Deserialize, Serialize};

/// Spacing tokens for consistent spacing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Spacing {
    None,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
}

/// Trait for providing spacing values
pub trait SpacingProvider {
    /// Resolve spacing to CSS class
    fn resolve_spacing(&self, spacing: Spacing) -> &str;

    /// Get padding class
    fn padding_class(&self, spacing: Spacing) -> String {
        format!("p-{}", self.resolve_spacing(spacing))
    }

    /// Get margin class
    fn margin_class(&self, spacing: Spacing) -> String {
        format!("m-{}", self.resolve_spacing(spacing))
    }
}
