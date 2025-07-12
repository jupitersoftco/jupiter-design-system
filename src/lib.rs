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
pub mod patterns;
pub mod themes;
pub mod utils;

// Re-export commonly used items
pub use crate::builders::*;
pub use crate::core::*;
pub use crate::patterns::{
    action_semantics, button_link, button_pattern, card_pattern, destructive_button,
    focus_management, hero_button, interactive_element, navigation_button, primary_button,
    secondary_button, ActionContext, ActionHierarchy, ActionIntent, ActionSemantics, ButtonPattern,
    ButtonSemanticInfo, CardElevation, CardInteraction, CardPattern, CardSpacing, CardSurface,
    FocusBehavior, FocusManagement, InteractionIntensity, InteractiveElement, InteractiveState,
    KeyboardPattern, ScreenReaderPattern,
};
pub use crate::themes::*;
pub use crate::utils::*;

/// Common imports for everyday usage
pub mod prelude {
    pub use crate::builders::{
        button_classes_from_strings, button_styles, card_classes_from_strings, card_styles,
        ButtonState, ButtonStyles, ButtonVariant, CardStyles,
    };
    pub use crate::core::color::{ColorProvider, WaterWellnessColors};
    pub use crate::core::{Breakpoint, Color, Size, Spacing, Typography};
    pub use crate::patterns::{
        action_semantics, card_pattern, destructive_button, focus_management, hero_button,
        interactive_element, primary_button, secondary_button, ActionIntent, ActionSemantics,
        ButtonPattern, CardElevation, CardInteraction, CardPattern, CardSpacing, CardSurface,
        FocusManagement, InteractiveElement, InteractiveState,
    };
    pub use crate::themes::{Theme, WaterWellnessTheme};
    pub use crate::utils::DesignSystem;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prelude_imports() {
        // Basic smoke test to ensure prelude exports work
        use crate::prelude::*;
        let _theme = WaterWellnessTheme::default();
    }
}
