//! Styling builders for Jupiter Design System
//!
//! This module provides chainable APIs for building CSS classes that can be used
//! with any component library or framework that supports Tailwind CSS.
//!
//! The builders are pure styling utilities that generate CSS classes without
//! being tied to any specific component implementation.

pub mod button;
pub mod card;
pub mod text;

#[cfg(test)]
mod text_test;

// Re-export commonly used items
pub use button::{
    button_classes_from_strings, button_styles, ButtonState, ButtonStyles, ButtonVariant,
};
pub use card::{card_classes_from_strings, card_styles, CardStyles};
pub use text::{
    text_clamp_style, text_classes_from_strings, text_element_from_hierarchy, text_styles,
    TextStyles,
};
