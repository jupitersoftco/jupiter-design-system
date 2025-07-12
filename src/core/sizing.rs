//! Sizing system for the design system

use serde::{Deserialize, Serialize};

/// Size tokens for consistent component sizing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Size {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
}

/// Breakpoint tokens for responsive design
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Breakpoint {
    Mobile,
    Tablet,
    Desktop,
    Large,
}

/// Trait for providing size values
pub trait SizeProvider {
    /// Resolve size to CSS class value
    fn resolve_size(&self, size: Size) -> &str;

    /// Get width class
    fn width_class(&self, size: Size) -> String {
        format!("w-{}", self.resolve_size(size))
    }

    /// Get height class
    fn height_class(&self, size: Size) -> String {
        format!("h-{}", self.resolve_size(size))
    }
}
