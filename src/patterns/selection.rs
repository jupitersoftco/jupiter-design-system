//! Selection patterns for Jupiter Design System
//!
//! This module provides abstract selection patterns for interactive selection
//! interfaces including filters, toggles, single selection, and multi-selection.

use crate::core::color::ColorProvider;
use serde::{Deserialize, Serialize};

/// Selection behavior defining how items can be selected
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SelectionBehavior {
    /// No selection allowed (display only)
    None,
    /// Single item can be selected at a time
    Single,
    /// Multiple items can be selected
    Multiple,
    /// Items can be toggled on/off independently
    Toggle,
}

/// Selection state for individual items
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SelectionState {
    /// Item is not selected
    Unselected,
    /// Item is selected
    Selected,
    /// Item is partially selected (for hierarchical selections)
    PartiallySelected,
    /// Item is disabled and cannot be selected
    Disabled,
}

/// Selection display style affecting visual presentation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SelectionDisplay {
    /// Button-like selection items
    Button,
    /// Chip/tag-like selection items
    Chip,
    /// List item selection
    ListItem,
    /// Card selection
    Card,
    /// Tab-like selection
    Tab,
}

/// Selection layout for organizing multiple selection items
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SelectionLayout {
    /// Horizontal flow layout
    Horizontal,
    /// Vertical stack layout
    Vertical,
    /// Grid layout
    Grid,
    /// Dropdown/menu layout
    Dropdown,
    /// Inline flow layout (wrapping)
    Inline,
}

/// Selection size affecting item dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SelectionSize {
    /// Extra small selection items
    XS,
    /// Small selection items
    SM,
    /// Medium selection items (default)
    MD,
    /// Large selection items
    LG,
    /// Extra large selection items
    XL,
}

/// Selection interaction intensity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SelectionInteraction {
    /// Subtle interaction effects
    Subtle,
    /// Standard interaction effects
    Standard,
    /// Prominent interaction effects
    Prominent,
}

/// Complete selection pattern for interactive selection interfaces
#[derive(Debug, Clone)]
pub struct SelectionPattern<C: ColorProvider> {
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

impl<C: ColorProvider> SelectionPattern<C> {
    /// Create a new selection pattern with default values
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

    /// Add custom classes
    pub fn custom(mut self, classes: impl Into<String>) -> Self {
        self.custom_classes.push(classes.into());
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
        all_classes.extend(self.custom_classes);

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

    /// Get semantic information about this selection
    pub fn semantic_info(&self) -> SelectionSemanticInfo {
        SelectionSemanticInfo {
            behavior: self.behavior,
            state: self.state,
            display: self.display,
            layout: self.layout,
            size: self.size,
            interaction: self.interaction,
            allows_multiple: matches!(
                self.behavior,
                SelectionBehavior::Multiple | SelectionBehavior::Toggle
            ),
            is_interactive: !matches!(self.behavior, SelectionBehavior::None)
                && !matches!(self.state, SelectionState::Disabled),
            has_counts: self.show_counts,
            has_clear_all: self.show_clear_all,
        }
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

/// Semantic information about a selection pattern
#[derive(Debug, Clone)]
pub struct SelectionSemanticInfo {
    pub behavior: SelectionBehavior,
    pub state: SelectionState,
    pub display: SelectionDisplay,
    pub layout: SelectionLayout,
    pub size: SelectionSize,
    pub interaction: SelectionInteraction,
    pub allows_multiple: bool,
    pub is_interactive: bool,
    pub has_counts: bool,
    pub has_clear_all: bool,
}

// === Convenience Functions ===

/// Create a filter selection pattern (single selection with button display)
pub fn filter_selection<C: ColorProvider>(color_provider: C) -> SelectionPattern<C> {
    SelectionPattern::new(color_provider)
        .single_selection()
        .button_display()
        .horizontal_layout()
        .standard_interaction()
        .with_counts(true)
}

/// Create a chip selection pattern (multiple selection with chip display)
pub fn chip_selection<C: ColorProvider>(color_provider: C) -> SelectionPattern<C> {
    SelectionPattern::new(color_provider)
        .multiple_selection()
        .chip_display()
        .inline_layout()
        .subtle_interaction()
        .with_clear_all(true)
}

/// Create a tab selection pattern (single selection with tab display)
pub fn tab_selection<C: ColorProvider>(color_provider: C) -> SelectionPattern<C> {
    SelectionPattern::new(color_provider)
        .single_selection()
        .tab_display()
        .horizontal_layout()
        .standard_interaction()
}

/// Create a list selection pattern (multiple selection with list item display)
pub fn list_selection<C: ColorProvider>(color_provider: C) -> SelectionPattern<C> {
    SelectionPattern::new(color_provider)
        .multiple_selection()
        .list_item_display()
        .vertical_layout()
        .standard_interaction()
        .with_counts(true)
}

/// Create a card selection pattern (single selection with card display)
pub fn card_selection<C: ColorProvider>(color_provider: C) -> SelectionPattern<C> {
    SelectionPattern::new(color_provider)
        .single_selection()
        .card_display()
        .grid_layout()
        .prominent_interaction()
}
