//! State patterns for Jupiter Design System
//!
//! This module provides abstract state patterns for communicating application state
//! to users, including empty states, loading states, error states, and success states.

use crate::core::color::ColorProvider;
use serde::{Deserialize, Serialize};

/// State intent representing the semantic meaning of the state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateIntent {
    /// Informational state - neutral information
    Informational,
    /// Loading state - operation in progress
    Loading,
    /// Success state - operation completed successfully
    Success,
    /// Warning state - attention needed but not critical
    Warning,
    /// Error state - something went wrong
    Error,
    /// Empty state - no data available
    Empty,
}

/// State prominence level affecting visual hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateProminence {
    /// Subtle state - minimal visual impact
    Subtle,
    /// Standard state - normal prominence
    Standard,
    /// Prominent state - high visual impact
    Prominent,
}

/// State size affecting spacing and content sizing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateSize {
    /// Extra small state
    XS,
    /// Small state
    SM,
    /// Medium state (default)
    MD,
    /// Large state
    LG,
    /// Extra large state
    XL,
}

/// State layout alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateAlignment {
    /// Left aligned
    Left,
    /// Center aligned
    Center,
    /// Right aligned
    Right,
}

/// State action requirement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateActionRequirement {
    /// No action required or available
    None,
    /// Action is optional
    Optional,
    /// Action is recommended
    Recommended,
    /// Action is required
    Required,
}

/// Loading animation variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LoadingVariant {
    /// Spinning circle
    Spinner,
    /// Bouncing dots
    Dots,
    /// Pulsing circle
    Pulse,
    /// Animated bars
    Bars,
    /// Skeleton placeholder
    Skeleton,
}

/// State pattern configuration
#[derive(Debug, Clone)]
pub struct StatePattern<C: ColorProvider> {
    intent: StateIntent,
    prominence: StateProminence,
    size: StateSize,
    alignment: StateAlignment,
    action_requirement: StateActionRequirement,
    loading_variant: Option<LoadingVariant>,
    fullscreen: bool,
    custom_classes: Vec<String>,
    color_provider: C,
}

impl<C: ColorProvider> StatePattern<C> {
    /// Create a new state pattern with default values
    pub fn new(color_provider: C) -> Self {
        Self {
            intent: StateIntent::Informational,
            prominence: StateProminence::Standard,
            size: StateSize::MD,
            alignment: StateAlignment::Center,
            action_requirement: StateActionRequirement::None,
            loading_variant: None,
            fullscreen: false,
            custom_classes: Vec::new(),
            color_provider,
        }
    }

    // === Intent Methods ===

    /// Set informational intent
    pub fn informational(mut self) -> Self {
        self.intent = StateIntent::Informational;
        self
    }

    /// Set loading intent
    pub fn loading(mut self) -> Self {
        self.intent = StateIntent::Loading;
        self
    }

    /// Set success intent
    pub fn success(mut self) -> Self {
        self.intent = StateIntent::Success;
        self
    }

    /// Set warning intent
    pub fn warning(mut self) -> Self {
        self.intent = StateIntent::Warning;
        self
    }

    /// Set error intent
    pub fn error(mut self) -> Self {
        self.intent = StateIntent::Error;
        self
    }

    /// Set empty intent
    pub fn empty(mut self) -> Self {
        self.intent = StateIntent::Empty;
        self
    }

    // === Prominence Methods ===

    /// Set subtle prominence
    pub fn subtle(mut self) -> Self {
        self.prominence = StateProminence::Subtle;
        self
    }

    /// Set standard prominence
    pub fn standard(mut self) -> Self {
        self.prominence = StateProminence::Standard;
        self
    }

    /// Set prominent prominence
    pub fn prominent(mut self) -> Self {
        self.prominence = StateProminence::Prominent;
        self
    }

    // === Size Methods ===

    /// Set extra small size
    pub fn xs(mut self) -> Self {
        self.size = StateSize::XS;
        self
    }

    /// Set small size
    pub fn sm(mut self) -> Self {
        self.size = StateSize::SM;
        self
    }

    /// Set medium size
    pub fn md(mut self) -> Self {
        self.size = StateSize::MD;
        self
    }

    /// Set large size
    pub fn lg(mut self) -> Self {
        self.size = StateSize::LG;
        self
    }

    /// Set extra large size
    pub fn xl(mut self) -> Self {
        self.size = StateSize::XL;
        self
    }

    // === Alignment Methods ===

    /// Set left alignment
    pub fn left_aligned(mut self) -> Self {
        self.alignment = StateAlignment::Left;
        self
    }

    /// Set center alignment
    pub fn center_aligned(mut self) -> Self {
        self.alignment = StateAlignment::Center;
        self
    }

    /// Set right alignment
    pub fn right_aligned(mut self) -> Self {
        self.alignment = StateAlignment::Right;
        self
    }

    // === Action Methods ===

    /// Set no action requirement
    pub fn no_action(mut self) -> Self {
        self.action_requirement = StateActionRequirement::None;
        self
    }

    /// Set optional action
    pub fn optional_action(mut self) -> Self {
        self.action_requirement = StateActionRequirement::Optional;
        self
    }

    /// Set recommended action
    pub fn recommended_action(mut self) -> Self {
        self.action_requirement = StateActionRequirement::Recommended;
        self
    }

    /// Set required action
    pub fn required_action(mut self) -> Self {
        self.action_requirement = StateActionRequirement::Required;
        self
    }

    // === Loading Variant Methods ===

    /// Set spinner loading variant
    pub fn spinner(mut self) -> Self {
        self.loading_variant = Some(LoadingVariant::Spinner);
        self
    }

    /// Set dots loading variant
    pub fn dots(mut self) -> Self {
        self.loading_variant = Some(LoadingVariant::Dots);
        self
    }

    /// Set pulse loading variant
    pub fn pulse(mut self) -> Self {
        self.loading_variant = Some(LoadingVariant::Pulse);
        self
    }

    /// Set bars loading variant
    pub fn bars(mut self) -> Self {
        self.loading_variant = Some(LoadingVariant::Bars);
        self
    }

    /// Set skeleton loading variant
    pub fn skeleton(mut self) -> Self {
        self.loading_variant = Some(LoadingVariant::Skeleton);
        self
    }

    // === Layout Methods ===

    /// Set fullscreen layout
    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    /// Add custom classes
    pub fn custom(mut self, classes: impl Into<String>) -> Self {
        self.custom_classes.push(classes.into());
        self
    }

    // === Build Methods ===

    /// Build the CSS classes for this state pattern
    pub fn classes(self) -> String {
        let mut all_classes = Vec::new();

        // Base state classes
        all_classes.push("state-pattern".to_string());

        // Layout classes
        let layout_classes = match self.alignment {
            StateAlignment::Left => "flex flex-col items-start text-left",
            StateAlignment::Center => "flex flex-col items-center text-center",
            StateAlignment::Right => "flex flex-col items-end text-right",
        };
        all_classes.push(layout_classes.to_string());

        // Fullscreen classes
        if self.fullscreen {
            all_classes.push("min-h-screen justify-center".to_string());
        }

        // Size-based spacing
        let spacing_classes = match self.size {
            StateSize::XS => "px-4 py-8",
            StateSize::SM => "px-6 py-12",
            StateSize::MD => "px-8 py-16",
            StateSize::LG => "px-12 py-20",
            StateSize::XL => "px-16 py-24",
        };
        all_classes.push(spacing_classes.to_string());

        // Intent-based classes
        let intent_classes = self.get_intent_classes();
        if !intent_classes.is_empty() {
            all_classes.push(intent_classes);
        }

        // Loading variant classes
        if let Some(variant) = self.loading_variant {
            let loading_classes = self.get_loading_classes(variant);
            if !loading_classes.is_empty() {
                all_classes.push(loading_classes);
            }
        }

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

    /// Get suggested icon for this state
    pub fn suggested_icon(&self) -> String {
        match self.intent {
            StateIntent::Informational => "info",
            StateIntent::Loading => "loader",
            StateIntent::Success => "check-circle",
            StateIntent::Warning => "alert-triangle",
            StateIntent::Error => "alert-circle",
            StateIntent::Empty => "inbox",
        }
        .to_string()
    }

    /// Get suggested action text for this state
    pub fn suggested_action_text(&self) -> Option<String> {
        match (self.intent, self.action_requirement) {
            (StateIntent::Error, StateActionRequirement::Recommended) => {
                Some("Try Again".to_string())
            }
            (StateIntent::Empty, StateActionRequirement::Optional) => Some("Refresh".to_string()),
            (StateIntent::Empty, StateActionRequirement::Recommended) => {
                Some("Add Item".to_string())
            }
            (StateIntent::Warning, StateActionRequirement::Required) => {
                Some("Take Action".to_string())
            }
            _ => None,
        }
    }

    /// Get semantic information about this state
    pub fn semantic_info(&self) -> StateSemanticInfo {
        StateSemanticInfo {
            intent: self.intent,
            prominence: self.prominence,
            size: self.size,
            alignment: self.alignment,
            action_requirement: self.action_requirement,
            loading_variant: self.loading_variant,
            requires_action: matches!(self.action_requirement, StateActionRequirement::Required),
            is_interactive: !matches!(self.action_requirement, StateActionRequirement::None),
        }
    }

    fn get_intent_classes(&self) -> String {
        match self.intent {
            StateIntent::Informational => {
                format!(
                    "{} {}",
                    self.color_provider
                        .text_class(crate::core::Color::TextPrimary),
                    self.color_provider.bg_class(crate::core::Color::Background)
                )
            }
            StateIntent::Loading => {
                format!(
                    "{} {}",
                    self.color_provider.text_class(crate::core::Color::Primary),
                    self.color_provider.bg_class(crate::core::Color::Background)
                )
            }
            StateIntent::Success => "text-green-600 bg-green-50".to_string(),
            StateIntent::Warning => "text-orange-600 bg-orange-50".to_string(),
            StateIntent::Error => "text-red-600 bg-red-50".to_string(),
            StateIntent::Empty => {
                format!(
                    "{} {}",
                    self.color_provider
                        .text_class(crate::core::Color::TextSecondary),
                    self.color_provider.bg_class(crate::core::Color::Background)
                )
            }
        }
    }

    fn get_loading_classes(&self, variant: LoadingVariant) -> String {
        match variant {
            LoadingVariant::Spinner => "animate-spin".to_string(),
            LoadingVariant::Dots => "animate-bounce".to_string(),
            LoadingVariant::Pulse => "animate-pulse".to_string(),
            LoadingVariant::Bars => "animate-pulse".to_string(),
            LoadingVariant::Skeleton => "animate-pulse".to_string(),
        }
    }
}

/// Semantic information about a state pattern
#[derive(Debug, Clone)]
pub struct StateSemanticInfo {
    pub intent: StateIntent,
    pub prominence: StateProminence,
    pub size: StateSize,
    pub alignment: StateAlignment,
    pub action_requirement: StateActionRequirement,
    pub loading_variant: Option<LoadingVariant>,
    pub requires_action: bool,
    pub is_interactive: bool,
}

// === Convenience Functions ===

/// Create a basic informational state
pub fn informational_state<C: ColorProvider>(color_provider: C) -> StatePattern<C> {
    StatePattern::new(color_provider)
        .informational()
        .standard()
        .center_aligned()
        .no_action()
}

/// Create a loading state
pub fn loading_state<C: ColorProvider>(color_provider: C) -> StatePattern<C> {
    StatePattern::new(color_provider)
        .loading()
        .standard()
        .center_aligned()
        .spinner()
        .no_action()
}

/// Create an empty state
pub fn empty_state<C: ColorProvider>(color_provider: C) -> StatePattern<C> {
    StatePattern::new(color_provider)
        .empty()
        .standard()
        .center_aligned()
        .optional_action()
}

/// Create an error state
pub fn error_state<C: ColorProvider>(color_provider: C) -> StatePattern<C> {
    StatePattern::new(color_provider)
        .error()
        .prominent()
        .center_aligned()
        .recommended_action()
}

/// Create a success state
pub fn success_state<C: ColorProvider>(color_provider: C) -> StatePattern<C> {
    StatePattern::new(color_provider)
        .success()
        .standard()
        .center_aligned()
        .no_action()
}

/// Create a warning state
pub fn warning_state<C: ColorProvider>(color_provider: C) -> StatePattern<C> {
    StatePattern::new(color_provider)
        .warning()
        .prominent()
        .center_aligned()
        .recommended_action()
}
