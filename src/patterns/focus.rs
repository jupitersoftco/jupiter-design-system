//! Focus and accessibility patterns

use crate::core::color::ColorProvider;
use crate::core::Color;
use serde::{Deserialize, Serialize};

/// Focus behavior types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FocusBehavior {
    /// Standard focus ring
    Standard,
    /// Subtle focus indication
    Subtle,
    /// Prominent focus for critical actions
    Prominent,
    /// No visible focus (use carefully, ensure other indicators)
    None,
    /// Custom focus behavior
    Custom,
}

/// Keyboard navigation patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyboardPattern {
    /// Simple button - Enter/Space activates
    Button,
    /// Link - Enter activates, context menu on context key
    Link,
    /// Menu item - Enter/Space activates, Arrow keys navigate
    MenuItem,
    /// Tab - Enter/Space activates, Arrow keys navigate between tabs
    Tab,
    /// Toggle - Enter/Space toggles state
    Toggle,
    /// Expandable - Enter/Space expands/collapses
    Expandable,
}

/// Screen reader patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScreenReaderPattern {
    /// Button element
    Button,
    /// Link element  
    Link,
    /// Menu item
    MenuItem,
    /// Tab
    Tab,
    /// Toggle button
    ToggleButton,
    /// Expandable section
    Expandable,
}

/// Focus management builder for consistent accessibility
#[derive(Debug, Clone)]
pub struct FocusManagement<C: ColorProvider> {
    focus_behavior: FocusBehavior,
    keyboard_pattern: Option<KeyboardPattern>,
    screen_reader_pattern: Option<ScreenReaderPattern>,
    is_focusable: bool,
    tab_index: Option<i32>,
    custom_classes: Vec<String>,
    color_provider: C,
}

impl<C: ColorProvider> FocusManagement<C> {
    /// Create new focus management
    pub fn new(color_provider: C) -> Self {
        Self {
            focus_behavior: FocusBehavior::Standard,
            keyboard_pattern: None,
            screen_reader_pattern: None,
            is_focusable: true,
            tab_index: None,
            custom_classes: Vec::new(),
            color_provider,
        }
    }

    /// Set focus behavior
    pub fn focus_behavior(mut self, behavior: FocusBehavior) -> Self {
        self.focus_behavior = behavior;
        self
    }

    /// Standard button accessibility
    pub fn button(mut self) -> Self {
        self.keyboard_pattern = Some(KeyboardPattern::Button);
        self.screen_reader_pattern = Some(ScreenReaderPattern::Button);
        self.focus_behavior = FocusBehavior::Standard;
        self
    }

    /// Link accessibility
    pub fn link(mut self) -> Self {
        self.keyboard_pattern = Some(KeyboardPattern::Link);
        self.screen_reader_pattern = Some(ScreenReaderPattern::Link);
        self.focus_behavior = FocusBehavior::Standard;
        self
    }

    /// Menu item accessibility
    pub fn menu_item(mut self) -> Self {
        self.keyboard_pattern = Some(KeyboardPattern::MenuItem);
        self.screen_reader_pattern = Some(ScreenReaderPattern::MenuItem);
        self.focus_behavior = FocusBehavior::Subtle;
        self
    }

    /// Tab accessibility
    pub fn tab(mut self) -> Self {
        self.keyboard_pattern = Some(KeyboardPattern::Tab);
        self.screen_reader_pattern = Some(ScreenReaderPattern::Tab);
        self.focus_behavior = FocusBehavior::Standard;
        self
    }

    /// Toggle button accessibility
    pub fn toggle(mut self) -> Self {
        self.keyboard_pattern = Some(KeyboardPattern::Toggle);
        self.screen_reader_pattern = Some(ScreenReaderPattern::ToggleButton);
        self.focus_behavior = FocusBehavior::Standard;
        self
    }

    /// Build focus and accessibility classes
    pub fn classes(self) -> String {
        let mut classes = Vec::new();

        // Base focus classes
        if self.is_focusable {
            classes.push("focus:outline-none".to_string());

            // Focus ring based on behavior
            let focus_ring = match self.focus_behavior {
                FocusBehavior::Standard => format!(
                    "focus:ring-2 focus:ring-offset-2 focus:ring-{}",
                    self.color_provider
                        .resolve_color(Color::Primary)
                        .replace("bg-", "")
                        .replace("-500", "-300")
                ),
                FocusBehavior::Subtle => format!(
                    "focus:ring-1 focus:ring-offset-1 focus:ring-{}",
                    self.color_provider
                        .resolve_color(Color::Border)
                        .replace("border-", "")
                ),
                FocusBehavior::Prominent => format!(
                    "focus:ring-4 focus:ring-offset-2 focus:ring-{}",
                    self.color_provider
                        .resolve_color(Color::Primary)
                        .replace("bg-", "")
                        .replace("-500", "-300")
                ),
                FocusBehavior::None => "focus:ring-0".to_string(),
                FocusBehavior::Custom => "".to_string(),
            };

            if !focus_ring.is_empty() {
                classes.push(focus_ring);
            }
        }

        // Add custom classes
        classes.extend(self.custom_classes);

        classes.join(" ")
    }

    /// Get data attributes for proper semantic markup
    pub fn data_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        // Tab index
        if let Some(index) = self.tab_index {
            attrs.push(("tabindex", index.to_string()));
        } else if self.is_focusable {
            attrs.push(("tabindex", "0".to_string()));
        }

        // Role based on screen reader pattern
        if let Some(pattern) = self.screen_reader_pattern {
            let role = match pattern {
                ScreenReaderPattern::Button => "button",
                ScreenReaderPattern::Link => "link",
                ScreenReaderPattern::MenuItem => "menuitem",
                ScreenReaderPattern::Tab => "tab",
                ScreenReaderPattern::ToggleButton => "button",
                ScreenReaderPattern::Expandable => "button",
            };
            attrs.push(("role", role.to_string()));
        }

        // Additional ARIA attributes based on patterns
        match self.screen_reader_pattern {
            Some(ScreenReaderPattern::ToggleButton) => {
                attrs.push(("aria-pressed", "false".to_string()));
            }
            Some(ScreenReaderPattern::Expandable) => {
                attrs.push(("aria-expanded", "false".to_string()));
            }
            _ => {}
        }

        attrs
    }
}

/// Convenience function to create focus management
pub fn focus_management<C: ColorProvider>(color_provider: C) -> FocusManagement<C> {
    FocusManagement::new(color_provider)
}
