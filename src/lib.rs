//! # Jupiter Design System
//!
//! A pure styling utility library for building consistent UI components with any framework.
//! Provides chainable APIs for generating Tailwind CSS classes with design system constraints.
//!
//! ## Quick Start
//!
//! ```rust
//! use jupiter_design_system::prelude::*;
//! use jupiter_design_system::core::color::WaterWellnessColors;
//!
//! // Create button styles with chainable API
//! let button_classes = button_styles(WaterWellnessColors::default())
//!     .primary()
//!     .large()
//!     .full_width()
//!     .classes();
//!
//! // Use with any component library (Dioxus example):
//! // rsx! { button { class: "{button_classes}", "Click me" } }
//! ```

pub mod builders;
pub mod core;
pub mod themes;
pub mod utils;

// Re-export commonly used items
pub use crate::builders::*;
pub use crate::core::*;
pub use crate::themes::*;
pub use crate::utils::*;

/// Common imports for everyday usage
pub mod prelude {
    pub use crate::builders::{button_styles, ButtonState, ButtonStyles, ButtonVariant};
    pub use crate::core::color::{ColorProvider, WaterWellnessColors};
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
