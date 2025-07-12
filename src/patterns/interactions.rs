//! Interactive element patterns
//!
//! Defines abstract interaction states and behaviors that can be applied
//! to any clickable element (buttons, links, cards, menu items, etc.)

use crate::core::color::ColorProvider;
use crate::core::Color;
use serde::{Deserialize, Serialize};

/// Interactive element states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractiveState {
    /// Default state - ready for interaction
    Default,
    /// Hover state - mouse over or touch hover
    Hover,
    /// Active state - being pressed/clicked
    Active,
    /// Focused state - keyboard focus or programmatic focus
    Focused,
    /// Disabled state - cannot be interacted with
    Disabled,
    /// Loading state - processing an action
    Loading,
}

/// Intensity of interactive effects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionIntensity {
    /// Subtle effects - for cards, menu items
    Gentle,
    /// Standard effects - for most buttons
    Standard,
    /// Strong effects - for primary actions
    Prominent,
}

/// Interactive element builder for creating consistent interactive behaviors
#[derive(Debug, Clone)]
pub struct InteractiveElement<C: ColorProvider> {
    state: InteractiveState,
    is_hoverable: bool,
    is_focusable: bool,
    is_pressable: bool,
    interaction_intensity: InteractionIntensity,
    custom_classes: Vec<String>,
    color_provider: C,
}

impl<C: ColorProvider> InteractiveElement<C> {
    /// Create a new interactive element
    pub fn new(color_provider: C) -> Self {
        Self {
            state: InteractiveState::Default,
            is_hoverable: false,
            is_focusable: false,
            is_pressable: false,
            interaction_intensity: InteractionIntensity::Standard,
            custom_classes: Vec::new(),
            color_provider,
        }
    }

    /// Enable hover effects
    pub fn hoverable(mut self) -> Self {
        self.is_hoverable = true;
        self
    }

    /// Enable focus effects (accessibility)
    pub fn focusable(mut self) -> Self {
        self.is_focusable = true;
        self
    }

    /// Enable press/active effects
    pub fn pressable(mut self) -> Self {
        self.is_pressable = true;
        self
    }

    /// Set to gentle interaction (subtle effects)
    pub fn gentle_interaction(mut self) -> Self {
        self.interaction_intensity = InteractionIntensity::Gentle;
        self
    }

    /// Set to standard interaction (normal effects)
    pub fn standard_interaction(mut self) -> Self {
        self.interaction_intensity = InteractionIntensity::Standard;
        self
    }

    /// Set to prominent interaction (strong effects)
    pub fn prominent_interaction(mut self) -> Self {
        self.interaction_intensity = InteractionIntensity::Prominent;
        self
    }

    /// Set the current state
    pub fn state(mut self, state: InteractiveState) -> Self {
        self.state = state;
        self
    }

    /// Add custom CSS classes
    pub fn custom(mut self, class: impl Into<String>) -> Self {
        self.custom_classes.push(class.into());
        self
    }

    /// Set hover state (shorthand)
    pub fn hover(mut self) -> Self {
        self.state = InteractiveState::Hover;
        self
    }

    /// Set active state (shorthand)
    pub fn active(mut self) -> Self {
        self.state = InteractiveState::Active;
        self
    }

    /// Set focused state (shorthand)
    pub fn focused(mut self) -> Self {
        self.state = InteractiveState::Focused;
        self
    }

    /// Set disabled state (shorthand)
    pub fn disabled(mut self) -> Self {
        self.state = InteractiveState::Disabled;
        self
    }

    /// Set loading state (shorthand)
    pub fn loading(mut self) -> Self {
        self.state = InteractiveState::Loading;
        self
    }

    /// Build the interactive classes
    pub fn classes(self) -> String {
        let mut classes = Vec::new();

        // Base interactive classes
        if self.is_hoverable || self.is_focusable || self.is_pressable {
            classes.push("transition-all duration-200 ease-in-out".to_string());
        }

        // Cursor styles
        match self.state {
            InteractiveState::Disabled => classes.push("cursor-not-allowed".to_string()),
            InteractiveState::Loading => classes.push("cursor-wait".to_string()),
            _ if self.is_hoverable || self.is_pressable => {
                classes.push("cursor-pointer".to_string())
            }
            _ => {}
        }

        // Hover effects
        if self.is_hoverable && self.state != InteractiveState::Disabled {
            let hover_classes = match self.interaction_intensity {
                InteractionIntensity::Gentle => "hover:scale-101 hover:shadow-sm",
                InteractionIntensity::Standard => "hover:scale-105 hover:shadow-md",
                InteractionIntensity::Prominent => "hover:scale-110 hover:shadow-lg",
            };
            classes.push(hover_classes.to_string());
        }

        // Press/Active effects
        if self.is_pressable && self.state != InteractiveState::Disabled {
            let active_classes = match self.interaction_intensity {
                InteractionIntensity::Gentle => "active:scale-100",
                InteractionIntensity::Standard => "active:scale-95",
                InteractionIntensity::Prominent => "active:scale-95",
            };
            classes.push(active_classes.to_string());
        }

        // Focus effects (accessibility)
        if self.is_focusable {
            classes.push("focus:outline-none focus:ring-2 focus:ring-offset-2".to_string());
            classes.push(format!(
                "focus:ring-{}",
                self.color_provider
                    .resolve_color(Color::Primary)
                    .replace("bg-", "")
                    .replace("-500", "-300")
            ));
        }

        // State-specific effects
        match self.state {
            InteractiveState::Hover if self.is_hoverable => {
                // Hover classes already applied above
            }
            InteractiveState::Active if self.is_pressable => {
                // Active classes already applied above
            }
            InteractiveState::Focused if self.is_focusable => {
                classes.push("ring-2 ring-offset-2".to_string());
                classes.push(format!(
                    "ring-{}",
                    self.color_provider
                        .resolve_color(Color::Primary)
                        .replace("bg-", "")
                        .replace("-500", "-300")
                ));
            }
            InteractiveState::Disabled => {
                classes.push("opacity-50 pointer-events-none".to_string());
            }
            InteractiveState::Loading => {
                classes.push("opacity-75".to_string());
            }
            _ => {}
        }

        // Add custom classes
        classes.extend(self.custom_classes);

        classes.join(" ")
    }
}

/// Convenience function to create interactive element
pub fn interactive_element<C: ColorProvider>(color_provider: C) -> InteractiveElement<C> {
    InteractiveElement::new(color_provider)
}
