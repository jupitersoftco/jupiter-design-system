//! Action patterns and semantic intent
//!
//! Defines abstract action semantics that represent the meaning and importance
//! of user actions, independent of visual implementation.

use crate::core::color::ColorProvider;
use crate::core::Color;
use serde::{Deserialize, Serialize};

/// Semantic action types that represent the intent and importance of actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionIntent {
    /// Primary action - the main thing user should do
    Primary,
    /// Secondary action - alternative or supplementary action  
    Secondary,
    /// Constructive action - creates, adds, confirms
    Constructive,
    /// Destructive action - deletes, removes, cancels
    Destructive,
    /// Navigation action - moves between contexts
    Navigation,
    /// Informational action - shows details, help, etc.
    Informational,
}

/// Action hierarchy - how prominent should this action be?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionHierarchy {
    /// Hero action - the most important action on the page
    Hero,
    /// Primary action - important action in a section  
    Primary,
    /// Secondary action - less important but still visible
    Secondary,
    /// Tertiary action - subtle, supporting action
    Tertiary,
    /// Minimal action - very subtle, almost invisible
    Minimal,
}

/// Action context - where/how is this action being used?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionContext {
    /// Standalone action (normal buttons)
    Standalone,
    /// Part of a form (submit, cancel buttons)
    Form,
    /// Part of navigation (menu items, tabs)
    Navigation,
    /// Part of content (inline links, embedded actions)
    Inline,
    /// Part of a toolbar or action bar
    Toolbar,
    /// Floating action (FAB, overlays)
    Floating,
}

/// Action semantics builder for creating consistent action meaning across components
#[derive(Debug, Clone)]
pub struct ActionSemantics<C: ColorProvider> {
    pub intent: ActionIntent,
    hierarchy: ActionHierarchy,
    context: ActionContext,
    is_urgent: bool,
    custom_classes: Vec<String>,
    color_provider: C,
}

impl<C: ColorProvider> ActionSemantics<C> {
    /// Create new action semantics
    pub fn new(color_provider: C) -> Self {
        Self {
            intent: ActionIntent::Secondary,
            hierarchy: ActionHierarchy::Secondary,
            context: ActionContext::Standalone,
            is_urgent: false,
            custom_classes: Vec::new(),
            color_provider,
        }
    }

    /// Set the action intent
    pub fn intent(mut self, intent: ActionIntent) -> Self {
        self.intent = intent;
        self
    }

    /// Set the action hierarchy
    pub fn hierarchy(mut self, hierarchy: ActionHierarchy) -> Self {
        self.hierarchy = hierarchy;
        self
    }

    /// Set the action context
    pub fn context(mut self, context: ActionContext) -> Self {
        self.context = context;
        self
    }

    /// Mark as urgent (time-sensitive or critical)
    pub fn urgent(mut self) -> Self {
        self.is_urgent = true;
        self
    }

    /// Add custom classes
    pub fn custom(mut self, class: impl Into<String>) -> Self {
        self.custom_classes.push(class.into());
        self
    }

    // Semantic shortcuts

    /// Primary action (shorthand)
    pub fn primary(mut self) -> Self {
        self.intent = ActionIntent::Primary;
        self.hierarchy = ActionHierarchy::Primary;
        self
    }

    /// Secondary action (shorthand)
    pub fn secondary(mut self) -> Self {
        self.intent = ActionIntent::Secondary;
        self.hierarchy = ActionHierarchy::Secondary;
        self
    }

    /// Destructive action (shorthand)
    pub fn destructive(mut self) -> Self {
        self.intent = ActionIntent::Destructive;
        self
    }

    /// Hero call-to-action (shorthand)
    pub fn hero(mut self) -> Self {
        self.intent = ActionIntent::Primary;
        self.hierarchy = ActionHierarchy::Hero;
        self
    }

    /// Navigation action (shorthand)
    pub fn navigation(mut self) -> Self {
        self.intent = ActionIntent::Navigation;
        self.context = ActionContext::Navigation;
        self
    }

    /// Build semantic color and visual weight classes
    pub fn classes(self) -> String {
        let mut classes = Vec::new();

        // Color based on intent
        let color_classes = self.get_intent_colors();
        classes.push(color_classes);

        // Visual weight based on hierarchy
        let weight_classes = self.get_hierarchy_weight();
        classes.push(weight_classes);

        // Context-specific adjustments
        let context_classes = self.get_context_adjustments();
        if !context_classes.is_empty() {
            classes.push(context_classes);
        }

        // Urgency indicators
        if self.is_urgent {
            classes.push("animate-pulse".to_string());
        }

        // Add custom classes
        classes.extend(self.custom_classes);

        classes.join(" ")
    }

    fn get_intent_colors(&self) -> String {
        match self.intent {
            ActionIntent::Primary => {
                let hover_bg = format!(
                    "hover:{}",
                    self.color_provider.bg_class(Color::InteractiveHover)
                );
                format!(
                    "{} {} {}",
                    self.color_provider.bg_class(Color::Primary),
                    self.color_provider.text_class(Color::TextInverse),
                    hover_bg
                )
            }
            ActionIntent::Secondary => format!(
                "{} {} {} {}",
                self.color_provider.bg_class(Color::Surface),
                self.color_provider.text_class(Color::TextPrimary),
                self.color_provider.border_class(Color::Border),
                "border"
            ),
            ActionIntent::Constructive => format!(
                "{} {} {}",
                self.color_provider.bg_class(Color::Success),
                self.color_provider.text_class(Color::TextInverse),
                "hover:bg-green-600"
            ),
            ActionIntent::Destructive => format!(
                "{} {} {}",
                self.color_provider.bg_class(Color::Error),
                self.color_provider.text_class(Color::TextInverse),
                "hover:bg-red-600"
            ),
            ActionIntent::Navigation => {
                let hover_bg = format!("hover:{}", self.color_provider.bg_class(Color::Background));
                format!(
                    "{} {} {}",
                    "bg-transparent",
                    self.color_provider.text_class(Color::TextPrimary),
                    hover_bg
                )
            }
            ActionIntent::Informational => format!(
                "{} {} {}",
                "bg-transparent",
                self.color_provider.text_class(Color::TextSecondary),
                "hover:underline"
            ),
        }
    }

    fn get_hierarchy_weight(&self) -> String {
        match self.hierarchy {
            ActionHierarchy::Hero => "text-xl font-bold px-8 py-4 rounded-lg shadow-lg".to_string(),
            ActionHierarchy::Primary => {
                "text-base font-semibold px-6 py-3 rounded-md shadow-md".to_string()
            }
            ActionHierarchy::Secondary => {
                "text-sm font-medium px-4 py-2 rounded-md shadow-sm".to_string()
            }
            ActionHierarchy::Tertiary => "text-sm font-normal px-3 py-1.5 rounded".to_string(),
            ActionHierarchy::Minimal => "text-xs font-normal px-2 py-1 rounded".to_string(),
        }
    }

    fn get_context_adjustments(&self) -> String {
        match self.context {
            ActionContext::Standalone => "".to_string(),
            ActionContext::Form => "min-w-24".to_string(),
            ActionContext::Navigation => "w-full justify-start".to_string(),
            ActionContext::Inline => "inline underline-offset-2".to_string(),
            ActionContext::Toolbar => "h-8 px-2 text-xs".to_string(),
            ActionContext::Floating => "rounded-full w-14 h-14 shadow-xl".to_string(),
        }
    }
}

/// Convenience function to create action semantics
pub fn action_semantics<C: ColorProvider>(color_provider: C) -> ActionSemantics<C> {
    ActionSemantics::new(color_provider)
}
