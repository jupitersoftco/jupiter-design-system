//! Comprehensive Button Pattern
//!
//! This demonstrates how abstract design concepts compose together to create
//! a complete "button" experience that can be applied to any element.

use crate::core::color::ColorProvider;
use crate::patterns::{
    ActionContext, ActionHierarchy, ActionIntent, ActionSemantics, FocusBehavior, FocusManagement,
    InteractiveElement,
};

/// Complete button pattern combining all abstract concepts
///
/// This represents the full abstract concept of a "button" - not just styling,
/// but the complete interactive, semantic, and accessible experience.
///
/// # Examples
///
/// ```rust
/// use jupiter_design_system::patterns::ButtonPattern;
/// use jupiter_design_system::core::color::WaterWellnessColors;
///
/// // Primary CTA button
/// let cta_button = ButtonPattern::new(WaterWellnessColors::default())
///     .primary_action()
///     .hero_prominence()
///     .prominent_interaction()
///     .classes();
///
/// // Destructive action button
/// let delete_button = ButtonPattern::new(WaterWellnessColors::default())
///     .destructive_action()
///     .standard_prominence()
///     .disabled(true)
///     .classes();
///
/// // Navigation menu item (button-like)
/// let menu_item = ButtonPattern::new(WaterWellnessColors::default())
///     .navigation_action()
///     .tertiary_prominence()
///     .gentle_interaction()
///     .classes();
///
/// // Link that acts like a button
/// let button_link = ButtonPattern::new(WaterWellnessColors::default())
///     .secondary_action()
///     .inline_context()
///     .custom_interaction("underline hover:no-underline")
///     .classes();
/// ```
#[derive(Debug, Clone)]
pub struct ButtonPattern<C: ColorProvider + Clone> {
    // Core state
    disabled: bool,
    loading: bool,
    selected: bool,

    // Abstract patterns
    action_semantics: ActionSemantics<C>,
    interactive_element: InteractiveElement<C>,
    focus_management: FocusManagement<C>,

    // Custom overrides
    custom_classes: Vec<String>,
    #[allow(dead_code)]
    color_provider: C,
}

impl<C: ColorProvider + Clone> ButtonPattern<C> {
    /// Create a new button pattern with sensible defaults
    pub fn new(color_provider: C) -> Self {
        Self {
            disabled: false,
            loading: false,
            selected: false,

            action_semantics: ActionSemantics::new(color_provider.clone())
                .secondary()
                .context(ActionContext::Standalone),

            interactive_element: InteractiveElement::new(color_provider.clone())
                .hoverable()
                .focusable()
                .pressable()
                .standard_interaction(),

            focus_management: FocusManagement::new(color_provider.clone()).button(),

            custom_classes: Vec::new(),
            color_provider,
        }
    }

    // === Action Semantics ===

    /// Primary action button
    pub fn primary_action(mut self) -> Self {
        self.action_semantics = self.action_semantics.primary();
        self
    }

    /// Secondary action button
    pub fn secondary_action(mut self) -> Self {
        self.action_semantics = self.action_semantics.secondary();
        self
    }

    /// Destructive action button
    pub fn destructive_action(mut self) -> Self {
        self.action_semantics = self.action_semantics.destructive();
        self
    }

    /// Navigation action button
    pub fn navigation_action(mut self) -> Self {
        self.action_semantics = self.action_semantics.navigation();
        self
    }

    /// Hero prominence (most important)
    pub fn hero_prominence(mut self) -> Self {
        self.action_semantics = self.action_semantics.hierarchy(ActionHierarchy::Hero);
        self
    }

    /// Primary prominence
    pub fn primary_prominence(mut self) -> Self {
        self.action_semantics = self.action_semantics.hierarchy(ActionHierarchy::Primary);
        self
    }

    /// Secondary prominence
    pub fn standard_prominence(mut self) -> Self {
        self.action_semantics = self.action_semantics.hierarchy(ActionHierarchy::Secondary);
        self
    }

    /// Tertiary prominence
    pub fn tertiary_prominence(mut self) -> Self {
        self.action_semantics = self.action_semantics.hierarchy(ActionHierarchy::Tertiary);
        self
    }

    /// Inline context (within text)
    pub fn inline_context(mut self) -> Self {
        self.action_semantics = self.action_semantics.context(ActionContext::Inline);
        self
    }

    /// Form context
    pub fn form_context(mut self) -> Self {
        self.action_semantics = self.action_semantics.context(ActionContext::Form);
        self
    }

    /// Toolbar context
    pub fn toolbar_context(mut self) -> Self {
        self.action_semantics = self.action_semantics.context(ActionContext::Toolbar);
        self
    }

    // === Interactive Behavior ===

    /// Gentle interaction (subtle effects)
    pub fn gentle_interaction(mut self) -> Self {
        self.interactive_element = self.interactive_element.gentle_interaction();
        self
    }

    /// Standard interaction effects
    pub fn standard_interaction(mut self) -> Self {
        self.interactive_element = self.interactive_element.standard_interaction();
        self
    }

    /// Prominent interaction (strong effects)
    pub fn prominent_interaction(mut self) -> Self {
        self.interactive_element = self.interactive_element.prominent_interaction();
        self
    }

    /// Custom interaction effects
    pub fn custom_interaction(mut self, classes: impl Into<String>) -> Self {
        self.interactive_element = self.interactive_element.custom(classes);
        self
    }

    // === Focus Management ===

    /// Menu item focus behavior
    pub fn menu_item_focus(mut self) -> Self {
        self.focus_management = self.focus_management.menu_item();
        self
    }

    /// Link focus behavior
    pub fn link_focus(mut self) -> Self {
        self.focus_management = self.focus_management.link();
        self
    }

    /// Toggle focus behavior
    pub fn toggle_focus(mut self) -> Self {
        self.focus_management = self.focus_management.toggle();
        self
    }

    /// Subtle focus behavior
    pub fn subtle_focus(mut self) -> Self {
        self.focus_management = self.focus_management.focus_behavior(FocusBehavior::Subtle);
        self
    }

    /// Prominent focus behavior
    pub fn prominent_focus(mut self) -> Self {
        self.focus_management = self
            .focus_management
            .focus_behavior(FocusBehavior::Prominent);
        self
    }

    // === State Management ===

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        if disabled {
            self.interactive_element = self.interactive_element.disabled();
        }
        self
    }

    /// Set loading state
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        if loading {
            self.interactive_element = self.interactive_element.loading();
        }
        self
    }

    /// Set selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set hover state
    pub fn hover(mut self) -> Self {
        self.interactive_element = self.interactive_element.hover();
        self
    }

    /// Set active state
    pub fn active(mut self) -> Self {
        self.interactive_element = self.interactive_element.active();
        self
    }

    /// Set focused state
    pub fn focused(mut self) -> Self {
        self.interactive_element = self.interactive_element.focused();
        self
    }

    // === Custom Overrides ===

    /// Add custom classes
    pub fn custom(mut self, classes: impl Into<String>) -> Self {
        self.custom_classes.push(classes.into());
        self
    }

    /// Mark as urgent
    pub fn urgent(mut self) -> Self {
        self.action_semantics = self.action_semantics.urgent();
        self
    }

    // === Build Methods ===

    /// Build complete button classes
    pub fn classes(self) -> String {
        let mut all_classes = Vec::new();

        // Get classes from each pattern
        let action_classes = self.action_semantics.classes();
        let interactive_classes = self.interactive_element.classes();
        let focus_classes = self.focus_management.classes();

        // Combine all classes
        if !action_classes.is_empty() {
            all_classes.push(action_classes);
        }
        if !interactive_classes.is_empty() {
            all_classes.push(interactive_classes);
        }
        if !focus_classes.is_empty() {
            all_classes.push(focus_classes);
        }

        // State-specific classes
        if self.selected {
            all_classes.push("bg-opacity-80".to_string());
        }

        // Custom classes
        all_classes.extend(self.custom_classes);

        // Remove duplicates and join
        let mut classes: Vec<String> = all_classes
            .join(" ")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        classes.dedup();
        classes.join(" ")
    }

    /// Get accessibility attributes
    pub fn accessibility_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = self.focus_management.data_attributes();

        // Add button-specific attributes
        if self.disabled {
            attrs.push(("aria-disabled", "true".to_string()));
        }
        if self.loading {
            attrs.push(("aria-busy", "true".to_string()));
        }
        if self.selected {
            attrs.push(("aria-pressed", "true".to_string()));
        }

        attrs
    }

    /// Get semantic information about this button
    pub fn semantic_info(&self) -> ButtonSemanticInfo {
        ButtonSemanticInfo {
            action_intent: self.action_semantics.intent,
            is_primary: matches!(self.action_semantics.intent, ActionIntent::Primary),
            is_destructive: matches!(self.action_semantics.intent, ActionIntent::Destructive),
            is_disabled: self.disabled,
            is_loading: self.loading,
            is_selected: self.selected,
        }
    }
}

/// Semantic information about a button pattern
#[derive(Debug, Clone)]
pub struct ButtonSemanticInfo {
    pub action_intent: ActionIntent,
    pub is_primary: bool,
    pub is_destructive: bool,
    pub is_disabled: bool,
    pub is_loading: bool,
    pub is_selected: bool,
}

// === Convenience Functions ===

/// Create a button pattern
pub fn button_pattern<C: ColorProvider + Clone>(color_provider: C) -> ButtonPattern<C> {
    ButtonPattern::new(color_provider)
}

/// Create a primary button pattern
pub fn primary_button<C: ColorProvider + Clone>(color_provider: C) -> ButtonPattern<C> {
    ButtonPattern::new(color_provider)
        .primary_action()
        .primary_prominence()
        .standard_interaction()
}

/// Create a secondary button pattern
pub fn secondary_button<C: ColorProvider + Clone>(color_provider: C) -> ButtonPattern<C> {
    ButtonPattern::new(color_provider)
        .secondary_action()
        .standard_prominence()
        .standard_interaction()
}

/// Create a destructive button pattern
pub fn destructive_button<C: ColorProvider + Clone>(color_provider: C) -> ButtonPattern<C> {
    ButtonPattern::new(color_provider)
        .destructive_action()
        .standard_prominence()
        .standard_interaction()
}

/// Create a hero CTA button pattern
pub fn hero_button<C: ColorProvider + Clone>(color_provider: C) -> ButtonPattern<C> {
    ButtonPattern::new(color_provider)
        .primary_action()
        .hero_prominence()
        .prominent_interaction()
        .prominent_focus()
}

/// Create a navigation button pattern
pub fn navigation_button<C: ColorProvider + Clone>(color_provider: C) -> ButtonPattern<C> {
    ButtonPattern::new(color_provider)
        .navigation_action()
        .tertiary_prominence()
        .gentle_interaction()
        .menu_item_focus()
}

/// Create a link that acts like a button
pub fn button_link<C: ColorProvider + Clone>(color_provider: C) -> ButtonPattern<C> {
    ButtonPattern::new(color_provider)
        .secondary_action()
        .inline_context()
        .gentle_interaction()
        .link_focus()
}
