//! Button component builder for the design system
//!
//! Provides a trait-based approach to building buttons with consistent styling
//! and semantic variants.

use crate::core::color::ColorProvider;
use crate::core::sizing::SizeProvider;
use crate::core::{Color, Size};
use serde::{Deserialize, Serialize};

/// Button variant types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
    Ghost,
    Link,
}

/// Button state types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ButtonState {
    Default,
    Hover,
    Active,
    Disabled,
    Loading,
}

/// Trait for building button styles
pub trait ButtonBuilder {
    /// Set the button variant
    fn variant(self, variant: ButtonVariant) -> Self;

    /// Set the button size
    fn size(self, size: Size) -> Self;

    /// Set the button state
    fn state(self, state: ButtonState) -> Self;

    /// Set whether the button is full width
    fn full_width(self, full_width: bool) -> Self;

    /// Set whether the button has an icon
    fn with_icon(self, has_icon: bool) -> Self;

    /// Build the final CSS classes
    fn build(self) -> String;
}

/// Button style configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonStyle {
    pub variant: ButtonVariant,
    pub size: Size,
    pub state: ButtonState,
    pub full_width: bool,
    pub with_icon: bool,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size: Size::Medium,
            state: ButtonState::Default,
            full_width: false,
            with_icon: false,
        }
    }
}

/// Button class builder with theme support
pub struct Button<C: ColorProvider> {
    style: ButtonStyle,
    color_provider: C,
}

impl<C: ColorProvider> Button<C> {
    /// Create a new button with a color provider
    pub fn new(color_provider: C) -> Self {
        Self {
            style: ButtonStyle::default(),
            color_provider,
        }
    }

    /// Create a primary button
    pub fn primary(color_provider: C) -> Self {
        Self::new(color_provider).variant(ButtonVariant::Primary)
    }

    /// Create a secondary button
    pub fn secondary(color_provider: C) -> Self {
        Self::new(color_provider).variant(ButtonVariant::Secondary)
    }

    /// Create a success button
    pub fn success(color_provider: C) -> Self {
        Self::new(color_provider).variant(ButtonVariant::Success)
    }

    /// Create a warning button
    pub fn warning(color_provider: C) -> Self {
        Self::new(color_provider).variant(ButtonVariant::Warning)
    }

    /// Create an error button
    pub fn error(color_provider: C) -> Self {
        Self::new(color_provider).variant(ButtonVariant::Error)
    }

    /// Create a ghost button
    pub fn ghost(color_provider: C) -> Self {
        Self::new(color_provider).variant(ButtonVariant::Ghost)
    }

    /// Create a link button
    pub fn link(color_provider: C) -> Self {
        Self::new(color_provider).variant(ButtonVariant::Link)
    }

    /// Get the base button classes
    fn base_classes(&self) -> String {
        let base = "inline-flex items-center justify-center font-medium transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed";

        let size_classes = self.size_classes();
        let variant_classes = self.variant_classes();
        let state_classes = self.state_classes();
        let width_classes = if self.style.full_width { "w-full" } else { "" };
        let icon_classes = if self.style.with_icon {
            "space-x-2"
        } else {
            ""
        };

        format!(
            "{} {} {} {} {} {}",
            base, size_classes, variant_classes, state_classes, width_classes, icon_classes
        )
        .trim()
        .to_string()
    }

    /// Get size-specific classes
    fn size_classes(&self) -> String {
        match self.style.size {
            Size::XSmall => "px-2 py-1 text-xs rounded",
            Size::Small => "px-3 py-1.5 text-sm rounded",
            Size::Medium => "px-4 py-2 text-sm rounded-md",
            Size::Large => "px-6 py-3 text-base rounded-md",
            Size::XLarge => "px-8 py-4 text-lg rounded-lg",
        }
        .to_string()
    }

    /// Get variant-specific classes
    fn variant_classes(&self) -> String {
        match self.style.variant {
            ButtonVariant::Primary => format!(
                "{} {} {}",
                self.color_provider.bg_class(Color::Primary),
                self.color_provider.text_class(Color::TextInverse),
                format!(
                    "hover:{}",
                    self.color_provider.bg_class(Color::InteractiveHover)
                )
            ),
            ButtonVariant::Secondary => format!(
                "{} {} {} {}",
                self.color_provider.bg_class(Color::Surface),
                self.color_provider.text_class(Color::TextPrimary),
                self.color_provider.border_class(Color::Border),
                "border"
            ),
            ButtonVariant::Success => format!(
                "{} {} {}",
                self.color_provider.bg_class(Color::Success),
                self.color_provider.text_class(Color::TextInverse),
                "hover:bg-green-600"
            ),
            ButtonVariant::Warning => format!(
                "{} {} {}",
                self.color_provider.bg_class(Color::Warning),
                self.color_provider.text_class(Color::TextInverse),
                "hover:bg-amber-600"
            ),
            ButtonVariant::Error => format!(
                "{} {} {}",
                self.color_provider.bg_class(Color::Error),
                self.color_provider.text_class(Color::TextInverse),
                "hover:bg-red-600"
            ),
            ButtonVariant::Ghost => format!(
                "{} {} {}",
                "bg-transparent",
                self.color_provider.text_class(Color::TextPrimary),
                format!("hover:{}", self.color_provider.bg_class(Color::Background))
            ),
            ButtonVariant::Link => format!(
                "{} {} {}",
                "bg-transparent",
                self.color_provider.text_class(Color::Primary),
                "hover:underline"
            ),
        }
    }

    /// Get state-specific classes
    fn state_classes(&self) -> String {
        match self.style.state {
            ButtonState::Default => "".to_string(),
            ButtonState::Hover => "hover:scale-105".to_string(),
            ButtonState::Active => "active:scale-95".to_string(),
            ButtonState::Disabled => "opacity-50 cursor-not-allowed".to_string(),
            ButtonState::Loading => "cursor-wait".to_string(),
        }
    }
}

impl<C: ColorProvider> ButtonBuilder for Button<C> {
    fn variant(mut self, variant: ButtonVariant) -> Self {
        self.style.variant = variant;
        self
    }

    fn size(mut self, size: Size) -> Self {
        self.style.size = size;
        self
    }

    fn state(mut self, state: ButtonState) -> Self {
        self.style.state = state;
        self
    }

    fn full_width(mut self, full_width: bool) -> Self {
        self.style.full_width = full_width;
        self
    }

    fn with_icon(mut self, has_icon: bool) -> Self {
        self.style.with_icon = has_icon;
        self
    }

    fn build(self) -> String {
        self.base_classes()
    }
}

#[cfg(test)]
#[path = "button_test.rs"]
mod button_test;
