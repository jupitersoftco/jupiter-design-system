//! State styling utilities for the Jupiter Design System
//!
//! Provides a chainable API for building state CSS classes and configuration
//! that can be used with any component library or framework.

use crate::core::color::ColorProvider;
use crate::patterns::{
    LoadingVariant, StateActionRequirement, StateAlignment, StateIntent, StateProminence, StateSize,
};

/// State styling utility builder
///
/// This is a pure styling utility that generates CSS classes for state components.
/// It can be used with any component library or framework that supports Tailwind CSS.
///
/// # Examples
///
/// ```rust
/// use jupiter_design_system::builders::state::StateStyles;
/// use jupiter_design_system::core::color::VibeColors;
///
/// let classes = StateStyles::new(VibeColors::default())
///     .loading()
///     .standard()
///     .center_aligned()
///     .spinner()
///     .md()
///     .classes();
/// ```
#[derive(Debug, Clone)]
pub struct StateStyles<C: ColorProvider> {
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

impl<C: ColorProvider> StateStyles<C> {
    /// Create a new state styling utility
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

    /// Enable fullscreen layout
    pub fn is_fullscreen(mut self) -> Self {
        self.fullscreen = true;
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

    // === String Convenience Methods ===

    /// Set intent from string
    pub fn intent_str(mut self, intent: &str) -> Self {
        self.intent = match intent {
            "informational" | "info" => StateIntent::Informational,
            "loading" => StateIntent::Loading,
            "success" => StateIntent::Success,
            "warning" | "warn" => StateIntent::Warning,
            "error" => StateIntent::Error,
            "empty" => StateIntent::Empty,
            _ => StateIntent::Informational, // fallback
        };
        self
    }

    /// Set prominence from string
    pub fn prominence_str(mut self, prominence: &str) -> Self {
        self.prominence = match prominence {
            "subtle" => StateProminence::Subtle,
            "standard" => StateProminence::Standard,
            "prominent" => StateProminence::Prominent,
            _ => StateProminence::Standard, // fallback
        };
        self
    }

    /// Set size from string
    pub fn size_str(mut self, size: &str) -> Self {
        self.size = match size {
            "xs" => StateSize::XS,
            "sm" => StateSize::SM,
            "md" => StateSize::MD,
            "lg" => StateSize::LG,
            "xl" => StateSize::XL,
            _ => StateSize::MD, // fallback
        };
        self
    }

    /// Set alignment from string
    pub fn alignment_str(mut self, alignment: &str) -> Self {
        self.alignment = match alignment {
            "left" => StateAlignment::Left,
            "center" => StateAlignment::Center,
            "right" => StateAlignment::Right,
            _ => StateAlignment::Center, // fallback
        };
        self
    }

    /// Set loading variant from string
    pub fn loading_variant_str(mut self, variant: &str) -> Self {
        self.loading_variant = match variant {
            "spinner" => Some(LoadingVariant::Spinner),
            "dots" => Some(LoadingVariant::Dots),
            "pulse" => Some(LoadingVariant::Pulse),
            "bars" => Some(LoadingVariant::Bars),
            "skeleton" => Some(LoadingVariant::Skeleton),
            _ => None, // fallback
        };
        self
    }

    // === Build Methods ===

    /// Build the final CSS classes string
    pub fn classes(self) -> String {
        self.build()
    }

    /// Build the final CSS classes string (alias for classes)
    pub fn build(self) -> String {
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

    /// Get size classes for content elements
    pub fn content_size_classes(&self) -> String {
        match self.size {
            StateSize::XS => "text-lg",
            StateSize::SM => "text-xl",
            StateSize::MD => "text-2xl",
            StateSize::LG => "text-3xl",
            StateSize::XL => "text-4xl",
        }
        .to_string()
    }

    /// Get description size classes
    pub fn description_size_classes(&self) -> String {
        match self.size {
            StateSize::XS => "text-sm",
            StateSize::SM => "text-base",
            StateSize::MD => "text-lg",
            StateSize::LG => "text-xl",
            StateSize::XL => "text-2xl",
        }
        .to_string()
    }

    /// Get icon size classes
    pub fn icon_size_classes(&self) -> String {
        match self.size {
            StateSize::XS => "w-8 h-8",
            StateSize::SM => "w-12 h-12",
            StateSize::MD => "w-16 h-16",
            StateSize::LG => "w-20 h-20",
            StateSize::XL => "w-24 h-24",
        }
        .to_string()
    }

    /// Get loading animation size classes
    pub fn loading_size_classes(&self) -> String {
        match self.loading_variant {
            Some(LoadingVariant::Spinner) => match self.size {
                StateSize::XS => "w-6 h-6",
                StateSize::SM => "w-8 h-8",
                StateSize::MD => "w-12 h-12",
                StateSize::LG => "w-16 h-16",
                StateSize::XL => "w-20 h-20",
            },
            Some(LoadingVariant::Dots) => match self.size {
                StateSize::XS => "w-2 h-2",
                StateSize::SM => "w-3 h-3",
                StateSize::MD => "w-4 h-4",
                StateSize::LG => "w-5 h-5",
                StateSize::XL => "w-6 h-6",
            },
            _ => "w-8 h-8",
        }
        .to_string()
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
            LoadingVariant::Spinner => {
                "animate-spin border-4 border-t-transparent rounded-full".to_string()
            }
            LoadingVariant::Dots => "animate-bounce rounded-full".to_string(),
            LoadingVariant::Pulse => "animate-pulse rounded-full".to_string(),
            LoadingVariant::Bars => "animate-pulse rounded-sm".to_string(),
            LoadingVariant::Skeleton => "animate-pulse rounded".to_string(),
        }
    }
}

/// Convenience function to create state styles
pub fn state_styles<C: ColorProvider>(color_provider: C) -> StateStyles<C> {
    StateStyles::new(color_provider)
}

/// Convenience function to create loading state styles
pub fn loading_state_styles<C: ColorProvider>(color_provider: C) -> StateStyles<C> {
    StateStyles::new(color_provider)
        .loading()
        .standard()
        .center_aligned()
        .spinner()
        .no_action()
}

/// Convenience function to create empty state styles
pub fn empty_state_styles<C: ColorProvider>(color_provider: C) -> StateStyles<C> {
    StateStyles::new(color_provider)
        .empty()
        .standard()
        .center_aligned()
        .optional_action()
}

/// Convenience function to create error state styles
pub fn error_state_styles<C: ColorProvider>(color_provider: C) -> StateStyles<C> {
    StateStyles::new(color_provider)
        .error()
        .prominent()
        .center_aligned()
        .recommended_action()
}

/// Convenience function to create success state styles
pub fn success_state_styles<C: ColorProvider>(color_provider: C) -> StateStyles<C> {
    StateStyles::new(color_provider)
        .success()
        .standard()
        .center_aligned()
        .no_action()
}

/// One-shot convenience function to create state classes from strings
pub fn state_classes_from_strings<C: ColorProvider>(
    color_provider: C,
    intent: &str,
    prominence: &str,
    size: &str,
    alignment: &str,
    loading_variant: Option<&str>,
    fullscreen: bool,
) -> String {
    let mut builder = StateStyles::new(color_provider)
        .intent_str(intent)
        .prominence_str(prominence)
        .size_str(size)
        .alignment_str(alignment)
        .fullscreen(fullscreen);

    if let Some(variant) = loading_variant {
        builder = builder.loading_variant_str(variant);
    }

    builder.classes()
}

#[cfg(test)]
#[path = "state_test.rs"]
mod state_test;
