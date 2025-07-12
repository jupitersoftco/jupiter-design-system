//! # Jupiter Design System
//!
//! A trait-based design system for Jupiter Startups applications that provides
//! type-safe design tokens, component builders, and theme management.
//!
//! ## Quick Start
//!
//! ```rust
//! use jupiter_design_system::prelude::*;
//!
//! // Create a button with design system
//! let button = Button::new()
//!     .variant(ButtonVariant::Primary)
//!     .size(Size::Medium)
//!     .theme(WaterWellnessTheme::default())
//!     .build();
//! ```

pub mod components;
pub mod core;
pub mod themes;
pub mod utils;

// Re-export commonly used items
pub use crate::components::*;
pub use crate::core::*;
pub use crate::themes::*;
pub use crate::utils::*;

/// Common imports for everyday usage
pub mod prelude {
    pub use crate::components::*;
    pub use crate::core::{Breakpoint, Color, Size, Spacing, Typography};
    pub use crate::themes::{Theme, WaterWellnessTheme};
    pub use crate::utils::DesignSystem;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prelude_imports() {
        // Basic smoke test to ensure prelude exports work
        use crate::prelude::*;
        let _theme = WaterWellnessTheme::default();
    }
}
