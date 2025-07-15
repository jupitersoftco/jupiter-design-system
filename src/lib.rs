//! # Jupiter Design System
//!
//! A pure styling utility library for building consistent UI components with any framework.
//! Provides chainable APIs for generating Tailwind CSS classes with design system constraints.
//!
//! ## Quick Start
//!
//! ```rust
//! use jupiter_design_system::prelude::*;
//! use jupiter_design_system::core::color::VibeColors;
//!
//! // Create button styles with chainable API
//! let button_classes = button_styles(VibeColors::default())
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
    action_semantics, body_typography, button_link, button_pattern, caption_typography,
    card_pattern, code_typography, destructive_button, focus_management, heading_typography,
    hero_button, interactive_element, navigation_button, primary_button, secondary_button,
    title_typography, typography_pattern, ActionContext, ActionHierarchy, ActionIntent,
    ActionSemantics, ButtonPattern, ButtonSemanticInfo, CardElevation, CardInteraction,
    CardPattern, CardSpacing, CardSurface, FocusBehavior, FocusManagement, InteractionIntensity,
    InteractiveElement, InteractiveState, KeyboardPattern, ScreenReaderPattern,
    TypographyAlignment, TypographyColor, TypographyHierarchy, TypographyOverflow,
    TypographyPattern, TypographySize, TypographyWeight,
};
pub use crate::themes::*;
pub use crate::utils::*;

/// Common imports for everyday usage
pub mod prelude {
    pub use crate::builders::{
        button_classes_from_strings, button_styles, card_classes_from_strings, card_styles,
        interactive_button, interactive_element, interactive_input, text_classes_from_strings,
        text_styles, ButtonBuilder, ButtonState, ButtonStyles, ButtonVariant, CardStyles,
        InputBuilder, TextStyles,
    };
    pub use crate::core::color::{ColorProvider, VibeColors};
    pub use crate::core::{Breakpoint, Color, Size, Spacing, Typography};
    pub use crate::patterns::{
        action_semantics, body_typography, caption_typography, card_pattern, code_typography,
        destructive_button, focus_management, heading_typography, hero_button,
        primary_button, secondary_button, title_typography, typography_pattern, ActionIntent,
        ActionSemantics, ButtonPattern, CardElevation, CardInteraction, CardPattern, CardSpacing,
        CardSurface, FocusManagement, InteractiveElement, InteractiveState, TypographyAlignment,
        TypographyColor, TypographyHierarchy, TypographyOverflow, TypographyPattern,
        TypographySize, TypographyWeight,
    };
    pub use crate::themes::{Theme, VibeTheme};
    pub use crate::utils::DesignSystem;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prelude_imports() {
        // Basic smoke test to ensure prelude exports work
        use crate::prelude::*;
        let _theme = VibeTheme::default();
    }
}
