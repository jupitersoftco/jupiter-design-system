//! Styling builders for Jupiter Design System
//!
//! This module provides chainable APIs for building CSS classes that can be used
//! with any component library or framework that supports Tailwind CSS.
//!
//! The builders are pure styling utilities that generate CSS classes without
//! being tied to any specific component implementation.

pub mod button;
pub mod card;
pub mod layout;
pub mod product;
pub mod selection;
pub mod state;
pub mod text;

#[cfg(test)]
mod text_test;

// Re-export commonly used items
pub use button::{
    button_classes_from_strings, button_styles, ButtonState, ButtonStyles, ButtonVariant,
};
pub use card::{card_classes_from_strings, card_styles, CardStyles};
pub use layout::{
    card_content_styles, card_footer_styles, card_header_styles, layout_styles, LayoutStyles,
};
pub use product::{
    featured_product_styles, product_preview_styles, product_showcase_styles, product_styles,
    product_tile_styles, ProductBuilder,
};
pub use selection::{
    chip_selection_styles, filter_selection_styles, selection_classes_from_strings,
    selection_styles, tab_selection_styles, SelectionStyles,
};
pub use state::{
    empty_state_styles, error_state_styles, loading_state_styles, state_classes_from_strings,
    state_styles, success_state_styles, StateStyles,
};
pub use text::{
    text_clamp_style, text_classes_from_strings, text_element_from_hierarchy, text_styles,
    TextStyles,
};
