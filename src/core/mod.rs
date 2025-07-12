//! Core design system primitives and traits
//!
//! This module provides the foundational building blocks for the design system,
//! including colors, spacing, typography, and sizing systems.

pub mod color;
pub mod sizing;
pub mod spacing;
pub mod typography;

// Re-export main types
pub use color::{Color, ColorPalette, ColorProvider};
pub use sizing::{Breakpoint, Size, SizeProvider};
pub use spacing::{Spacing, SpacingProvider};
pub use typography::{FontFamily, FontWeight, Typography, TypographyProvider};
