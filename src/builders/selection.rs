//! Selection styling utilities for the Jupiter Design System
//!
//! Provides a chainable API for building selection CSS classes that can be used
//! with any component library or framework.

use crate::core::color::ColorProvider;
use crate::patterns::{
    SelectionBehavior, SelectionDisplay, SelectionInteraction, SelectionLayout, SelectionSize,
    SelectionState,
};

/// Selection styling utility builder
///
/// This is a pure styling utility that generates CSS classes for selection components.
/// It can be used with any component library or framework that supports Tailwind CSS.
///
/// # Examples
///
/// ```rust
/// use jupiter_design_system::builders::selection::SelectionStyles;
/// use jupiter_design_system::themes::VibeColors;
///
/// let container_classes = SelectionStyles::new(VibeColors::default())
///     .single_selection()
///     .button_display()
///     .horizontal_layout()
///     .standard_interaction()
///     .container_classes();
///
/// let item_classes = SelectionStyles::new(VibeColors::default())
///     .selected()
///     .button_display()
///     .md()
///     .item_classes();
/// ```
#[derive(Debug, Clone)]
pub struct SelectionStyles<C: ColorProvider> {
    behavior: SelectionBehavior,
    state: SelectionState,
    display: SelectionDisplay,
    layout: SelectionLayout,
    size: SelectionSize,
    interaction: SelectionInteraction,
    show_counts: bool,
    show_clear_all: bool,
    custom_classes: Vec<String>,
    color_provider: C,
}

impl<C: ColorProvider> SelectionStyles<C> {
    /// Create a new selection styling utility
    pub fn new(color_provider: C) -> Self {
        Self {
            behavior: SelectionBehavior::Single,
            state: SelectionState::Unselected,
            display: SelectionDisplay::Button,
            layout: SelectionLayout::Horizontal,
            size: SelectionSize::MD,
            interaction: SelectionInteraction::Standard,
            show_counts: false,
            show_clear_all: false,
            custom_classes: Vec::new(),
            color_provider,
        }
    }

    // === Behavior Methods ===

    /// Set no selection behavior (display only)
    pub fn no_selection(mut self) -> Self {
        self.behavior = SelectionBehavior::None;
        self
    }

    /// Set single selection behavior
    pub fn single_selection(mut self) -> Self {
        self.behavior = SelectionBehavior::Single;
        self
    }

    /// Set multiple selection behavior
    pub fn multiple_selection(mut self) -> Self {
        self.behavior = SelectionBehavior::Multiple;
        self
    }

    /// Set toggle selection behavior
    pub fn toggle_selection(mut self) -> Self {
        self.behavior = SelectionBehavior::Toggle;
        self
    }

    // === State Methods ===

    /// Set unselected state
    pub fn unselected(mut self) -> Self {
        self.state = SelectionState::Unselected;
        self
    }

    /// Set selected state
    pub fn selected(mut self) -> Self {
        self.state = SelectionState::Selected;
        self
    }

    /// Set partially selected state
    pub fn partially_selected(mut self) -> Self {
        self.state = SelectionState::PartiallySelected;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self) -> Self {
        self.state = SelectionState::Disabled;
        self
    }

    // === Display Methods ===

    /// Set button display style
    pub fn button_display(mut self) -> Self {
        self.display = SelectionDisplay::Button;
        self
    }

    /// Set chip display style
    pub fn chip_display(mut self) -> Self {
        self.display = SelectionDisplay::Chip;
        self
    }

    /// Set list item display style
    pub fn list_item_display(mut self) -> Self {
        self.display = SelectionDisplay::ListItem;
        self
    }

    /// Set card display style
    pub fn card_display(mut self) -> Self {
        self.display = SelectionDisplay::Card;
        self
    }

    /// Set tab display style
    pub fn tab_display(mut self) -> Self {
        self.display = SelectionDisplay::Tab;
        self
    }

    // === Layout Methods ===

    /// Set horizontal layout
    pub fn horizontal_layout(mut self) -> Self {
        self.layout = SelectionLayout::Horizontal;
        self
    }

    /// Set vertical layout
    pub fn vertical_layout(mut self) -> Self {
        self.layout = SelectionLayout::Vertical;
        self
    }

    /// Set grid layout
    pub fn grid_layout(mut self) -> Self {
        self.layout = SelectionLayout::Grid;
        self
    }

    /// Set dropdown layout
    pub fn dropdown_layout(mut self) -> Self {
        self.layout = SelectionLayout::Dropdown;
        self
    }

    /// Set inline layout
    pub fn inline_layout(mut self) -> Self {
        self.layout = SelectionLayout::Inline;
        self
    }

    // === Size Methods ===

    /// Set extra small size
    pub fn xs(mut self) -> Self {
        self.size = SelectionSize::XS;
        self
    }

    /// Set small size
    pub fn sm(mut self) -> Self {
        self.size = SelectionSize::SM;
        self
    }

    /// Set medium size
    pub fn md(mut self) -> Self {
        self.size = SelectionSize::MD;
        self
    }

    /// Set large size
    pub fn lg(mut self) -> Self {
        self.size = SelectionSize::LG;
        self
    }

    /// Set extra large size
    pub fn xl(mut self) -> Self {
        self.size = SelectionSize::XL;
        self
    }

    // === Interaction Methods ===

    /// Set subtle interaction
    pub fn subtle_interaction(mut self) -> Self {
        self.interaction = SelectionInteraction::Subtle;
        self
    }

    /// Set standard interaction
    pub fn standard_interaction(mut self) -> Self {
        self.interaction = SelectionInteraction::Standard;
        self
    }

    /// Set prominent interaction
    pub fn prominent_interaction(mut self) -> Self {
        self.interaction = SelectionInteraction::Prominent;
        self
    }

    // === Feature Methods ===

    /// Show counts for selection items
    pub fn with_counts(mut self, show_counts: bool) -> Self {
        self.show_counts = show_counts;
        self
    }

    /// Show clear all option for multiple selections
    pub fn with_clear_all(mut self, show_clear_all: bool) -> Self {
        self.show_clear_all = show_clear_all;
        self
    }

    // === String Convenience Methods ===

    /// Set behavior from string
    pub fn behavior_str(mut self, behavior: &str) -> Self {
        self.behavior = match behavior {
            "none" => SelectionBehavior::None,
            "single" => SelectionBehavior::Single,
            "multiple" => SelectionBehavior::Multiple,
            "toggle" => SelectionBehavior::Toggle,
            _ => SelectionBehavior::Single, // fallback
        };
        self
    }

    /// Set state from string
    pub fn state_str(mut self, state: &str) -> Self {
        self.state = match state {
            "unselected" | "inactive" => SelectionState::Unselected,
            "selected" | "active" => SelectionState::Selected,
            "partial" => SelectionState::PartiallySelected,
            "disabled" => SelectionState::Disabled,
            _ => SelectionState::Unselected, // fallback
        };
        self
    }

    /// Set display from string
    pub fn display_str(mut self, display: &str) -> Self {
        self.display = match display {
            "button" => SelectionDisplay::Button,
            "chip" => SelectionDisplay::Chip,
            "list" | "list-item" => SelectionDisplay::ListItem,
            "card" => SelectionDisplay::Card,
            "tab" => SelectionDisplay::Tab,
            _ => SelectionDisplay::Button, // fallback
        };
        self
    }

    /// Set layout from string
    pub fn layout_str(mut self, layout: &str) -> Self {
        self.layout = match layout {
            "horizontal" => SelectionLayout::Horizontal,
            "vertical" => SelectionLayout::Vertical,
            "grid" => SelectionLayout::Grid,
            "dropdown" => SelectionLayout::Dropdown,
            "inline" => SelectionLayout::Inline,
            _ => SelectionLayout::Horizontal, // fallback
        };
        self
    }

    /// Set size from string
    pub fn size_str(mut self, size: &str) -> Self {
        self.size = match size {
            "xs" => SelectionSize::XS,
            "sm" => SelectionSize::SM,
            "md" => SelectionSize::MD,
            "lg" => SelectionSize::LG,
            "xl" => SelectionSize::XL,
            _ => SelectionSize::MD, // fallback
        };
        self
    }

    /// Set interaction from string
    pub fn interaction_str(mut self, interaction: &str) -> Self {
        self.interaction = match interaction {
            "subtle" => SelectionInteraction::Subtle,
            "standard" => SelectionInteraction::Standard,
            "prominent" => SelectionInteraction::Prominent,
            _ => SelectionInteraction::Standard, // fallback
        };
        self
    }

    // === Custom Methods ===

    /// Add a custom CSS class
    pub fn custom(mut self, class: impl Into<String>) -> Self {
        self.custom_classes.push(class.into());
        self
    }

    /// Add multiple custom CSS classes at once
    pub fn custom_classes(mut self, classes: impl Into<String>) -> Self {
        let classes_str = classes.into();
        for class in classes_str.split_whitespace() {
            if !class.is_empty() {
                self.custom_classes.push(class.to_string());
            }
        }
        self
    }

    // === Build Methods ===

    /// Build container classes for the selection group
    pub fn container_classes(self) -> String {
        let mut all_classes = Vec::new();

        // Base selection classes
        all_classes.push("selection-pattern".to_string());

        // Layout classes
        let layout_classes = match self.layout {
            SelectionLayout::Horizontal => "flex flex-row gap-2 items-center",
            SelectionLayout::Vertical => "flex flex-col gap-2",
            SelectionLayout::Grid => "grid grid-cols-auto gap-2",
            SelectionLayout::Dropdown => "relative",
            SelectionLayout::Inline => "flex flex-wrap gap-2 items-center",
        };
        all_classes.push(layout_classes.to_string());

        // Size-based spacing
        let spacing_classes = match self.size {
            SelectionSize::XS => "gap-1",
            SelectionSize::SM => "gap-1.5",
            SelectionSize::MD => "gap-2",
            SelectionSize::LG => "gap-3",
            SelectionSize::XL => "gap-4",
        };
        all_classes.push(spacing_classes.to_string());

        // Custom classes
        let custom_classes = self.custom_classes.join(" ");
        if !custom_classes.is_empty() {
            all_classes.push(custom_classes);
        }

        // Join and clean up
        let mut classes: Vec<String> = all_classes
            .join(" ")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        classes.sort();
        classes.dedup();
        classes.join(" ")
    }

    /// Build item classes for individual selection items
    pub fn item_classes(&self) -> String {
        let mut all_classes = Vec::new();

        // Base item classes
        all_classes.push("selection-item".to_string());

        // Display style classes
        let display_classes = match self.display {
            SelectionDisplay::Button => "inline-flex items-center justify-center font-medium rounded-md transition-all duration-200",
            SelectionDisplay::Chip => "inline-flex items-center rounded-full transition-all duration-200",
            SelectionDisplay::ListItem => "flex items-center w-full px-3 py-2 transition-all duration-200",
            SelectionDisplay::Card => "flex flex-col items-center p-4 rounded-lg border transition-all duration-200",
            SelectionDisplay::Tab => "flex items-center px-4 py-2 border-b-2 transition-all duration-200",
        };
        all_classes.push(display_classes.to_string());

        // Size classes
        let size_classes = match (self.display, self.size) {
            (SelectionDisplay::Button, SelectionSize::XS) => "px-2 py-1 text-xs",
            (SelectionDisplay::Button, SelectionSize::SM) => "px-3 py-1.5 text-sm",
            (SelectionDisplay::Button, SelectionSize::MD) => "px-4 py-2 text-base",
            (SelectionDisplay::Button, SelectionSize::LG) => "px-6 py-3 text-lg",
            (SelectionDisplay::Button, SelectionSize::XL) => "px-8 py-4 text-xl",
            (SelectionDisplay::Chip, SelectionSize::XS) => "px-2 py-0.5 text-xs",
            (SelectionDisplay::Chip, SelectionSize::SM) => "px-3 py-1 text-sm",
            (SelectionDisplay::Chip, SelectionSize::MD) => "px-3 py-1.5 text-base",
            (SelectionDisplay::Chip, SelectionSize::LG) => "px-4 py-2 text-lg",
            (SelectionDisplay::Chip, SelectionSize::XL) => "px-6 py-3 text-xl",
            _ => "px-4 py-2 text-base", // fallback
        };
        all_classes.push(size_classes.to_string());

        // State classes
        let state_classes = self.get_state_classes();
        if !state_classes.is_empty() {
            all_classes.push(state_classes);
        }

        // Interaction classes
        let interaction_classes = self.get_interaction_classes();
        if !interaction_classes.is_empty() {
            all_classes.push(interaction_classes);
        }

        // Join and clean up
        let mut classes: Vec<String> = all_classes
            .join(" ")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        classes.sort();
        classes.dedup();
        classes.join(" ")
    }

    /// Get count badge classes
    pub fn count_classes(&self) -> String {
        if !self.show_counts {
            return String::new();
        }

        let mut classes = Vec::new();
        classes.push("ml-2 px-2 py-0.5 text-xs rounded-full".to_string());

        // Color based on state
        match self.state {
            SelectionState::Selected => {
                classes.push(format!(
                    "{} {}",
                    self.color_provider.bg_class(crate::core::Color::Primary),
                    self.color_provider
                        .text_class(crate::core::Color::TextInverse)
                ));
            }
            _ => {
                classes.push(format!(
                    "{} {}",
                    self.color_provider.bg_class(crate::core::Color::Background),
                    self.color_provider
                        .text_class(crate::core::Color::TextSecondary)
                ));
            }
        }

        classes.join(" ")
    }

    fn get_state_classes(&self) -> String {
        match self.state {
            SelectionState::Unselected => format!(
                "{} {} {}",
                self.color_provider.bg_class(crate::core::Color::Surface),
                self.color_provider
                    .text_class(crate::core::Color::TextPrimary),
                self.color_provider.border_class(crate::core::Color::Border)
            ),
            SelectionState::Selected => format!(
                "{} {} {}",
                self.color_provider.bg_class(crate::core::Color::Primary),
                self.color_provider
                    .text_class(crate::core::Color::TextInverse),
                self.color_provider
                    .border_class(crate::core::Color::Primary)
            ),
            SelectionState::PartiallySelected => format!(
                "{} {} {}",
                self.color_provider.bg_class(crate::core::Color::Background),
                self.color_provider.text_class(crate::core::Color::Primary),
                self.color_provider
                    .border_class(crate::core::Color::Primary)
            ),
            SelectionState::Disabled => format!(
                "{} {} {}",
                self.color_provider
                    .bg_class(crate::core::Color::InteractiveDisabled),
                self.color_provider
                    .text_class(crate::core::Color::TextTertiary),
                self.color_provider
                    .border_class(crate::core::Color::InteractiveDisabled)
            ),
        }
    }

    fn get_interaction_classes(&self) -> String {
        if matches!(self.state, SelectionState::Disabled) {
            return "cursor-not-allowed".to_string();
        }

        let mut classes = Vec::new();
        classes.push("cursor-pointer".to_string());

        match self.interaction {
            SelectionInteraction::Subtle => {
                classes.push("hover:opacity-80".to_string());
            }
            SelectionInteraction::Standard => {
                if matches!(self.state, SelectionState::Unselected) {
                    classes.push(format!(
                        "hover:{} hover:{}",
                        self.color_provider.bg_class(crate::core::Color::Background),
                        self.color_provider
                            .border_class(crate::core::Color::Interactive)
                    ));
                }
                classes.push("hover:scale-105 active:scale-95".to_string());
            }
            SelectionInteraction::Prominent => {
                if matches!(self.state, SelectionState::Unselected) {
                    classes.push(format!(
                        "hover:{} hover:{}",
                        self.color_provider
                            .bg_class(crate::core::Color::Interactive),
                        self.color_provider
                            .text_class(crate::core::Color::TextInverse)
                    ));
                }
                classes
                    .push("hover:scale-110 active:scale-90 shadow-lg hover:shadow-xl".to_string());
            }
        }

        classes.join(" ")
    }
}

/// Convenience function to create selection styles
pub fn selection_styles<C: ColorProvider>(color_provider: C) -> SelectionStyles<C> {
    SelectionStyles::new(color_provider)
}

/// Convenience function to create filter selection styles
pub fn filter_selection_styles<C: ColorProvider>(color_provider: C) -> SelectionStyles<C> {
    SelectionStyles::new(color_provider)
        .single_selection()
        .button_display()
        .horizontal_layout()
        .standard_interaction()
        .with_counts(true)
}

/// Convenience function to create chip selection styles
pub fn chip_selection_styles<C: ColorProvider>(color_provider: C) -> SelectionStyles<C> {
    SelectionStyles::new(color_provider)
        .multiple_selection()
        .chip_display()
        .inline_layout()
        .subtle_interaction()
        .with_clear_all(true)
}

/// Convenience function to create tab selection styles
pub fn tab_selection_styles<C: ColorProvider>(color_provider: C) -> SelectionStyles<C> {
    SelectionStyles::new(color_provider)
        .single_selection()
        .tab_display()
        .horizontal_layout()
        .standard_interaction()
}

/// One-shot convenience function to create selection classes from strings
pub fn selection_classes_from_strings<C: ColorProvider + Clone>(
    color_provider: C,
    behavior: &str,
    state: &str,
    display: &str,
    layout: &str,
    size: &str,
    interaction: &str,
    show_counts: bool,
) -> (String, String) {
    let builder = SelectionStyles::new(color_provider)
        .behavior_str(behavior)
        .state_str(state)
        .display_str(display)
        .layout_str(layout)
        .size_str(size)
        .interaction_str(interaction)
        .with_counts(show_counts);

    (builder.clone().container_classes(), builder.item_classes())
}

#[cfg(test)]
#[path = "selection_test.rs"]
mod selection_test;
