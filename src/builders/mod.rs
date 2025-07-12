//! Styling builders for Jupiter Design System
//!
//! This module provides chainable APIs for building CSS classes that can be used
//! with any component library or framework that supports Tailwind CSS.
//!
//! The builders are pure styling utilities that generate CSS classes without
//! being tied to any specific component implementation.

pub mod button;

// Re-export commonly used items
pub use button::{
    button_classes_from_strings, button_styles, ButtonState, ButtonStyles, ButtonVariant,
};
