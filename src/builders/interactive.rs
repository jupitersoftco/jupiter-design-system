//! Interactive component builders with fluent pseudo-class API
//!
//! This module provides a clean, type-safe way to build interactive components
//! with hover, focus, active, and other pseudo-class states.
//!
//! # Examples
//!
//! ```rust
//! use jupiter_design_system::builders::interactive::*;
//! use jupiter_design_system::core::color::WaterWellnessColors;
//!
//! let colors = WaterWellnessColors::default();
//! 
//! // Clean input with interactive states
//! let input_classes = interactive_input(colors.clone())
//!     .base_style()
//!     .hover().border_primary().shadow_md()
//!     .focus().border_primary().ring_primary().outline_none()
//!     .disabled().opacity_50()
//!     .build();
//!
//! // Clean button with interactive states
//! let button_classes = interactive_button(colors)
//!     .primary()
//!     .hover().darken().scale_105()
//!     .focus().ring_primary()
//!     .active().scale_95()
//!     .build();
//! ```

use crate::core::color::ColorProvider;
use crate::core::Color;

/// Base interactive component that can be specialized
#[derive(Debug, Clone)]
pub struct InteractiveBase<C: ColorProvider> {
    base_classes: Vec<String>,
    hover_classes: Vec<String>,
    focus_classes: Vec<String>,
    active_classes: Vec<String>,
    disabled_classes: Vec<String>,
    color_provider: C,
}

impl<C: ColorProvider> InteractiveBase<C> {
    pub fn new(color_provider: C) -> Self {
        Self {
            base_classes: Vec::new(),
            hover_classes: Vec::new(),
            focus_classes: Vec::new(),
            active_classes: Vec::new(),
            disabled_classes: Vec::new(),
            color_provider,
        }
    }

    /// Add base classes that always apply
    pub fn base(mut self, classes: &str) -> Self {
        self.base_classes.extend(classes.split_whitespace().map(|s| s.to_string()));
        self
    }

    /// Enter hover state builder
    pub fn hover(self) -> HoverBuilder<C> {
        HoverBuilder::new(self)
    }

    /// Enter focus state builder
    pub fn focus(self) -> FocusBuilder<C> {
        FocusBuilder::new(self)
    }

    /// Enter active state builder
    pub fn active(self) -> ActiveBuilder<C> {
        ActiveBuilder::new(self)
    }

    /// Enter disabled state builder
    pub fn disabled(self) -> DisabledBuilder<C> {
        DisabledBuilder::new(self)
    }

    /// Build the final CSS classes string
    pub fn build(self) -> String {
        let mut all_classes = Vec::new();

        // Base classes
        all_classes.extend(self.base_classes);

        // Hover classes
        if !self.hover_classes.is_empty() {
            all_classes.push(format!("hover:({})", self.hover_classes.join(" ")));
        }

        // Focus classes
        if !self.focus_classes.is_empty() {
            all_classes.push(format!("focus:({})", self.focus_classes.join(" ")));
        }

        // Active classes
        if !self.active_classes.is_empty() {
            all_classes.push(format!("active:({})", self.active_classes.join(" ")));
        }

        // Disabled classes
        if !self.disabled_classes.is_empty() {
            all_classes.push(format!("disabled:({})", self.disabled_classes.join(" ")));
        }

        all_classes.join(" ")
    }
}

/// Builder for hover states
pub struct HoverBuilder<C: ColorProvider> {
    base: InteractiveBase<C>,
}

impl<C: ColorProvider> HoverBuilder<C> {
    fn new(base: InteractiveBase<C>) -> Self {
        Self { base }
    }

    /// Add arbitrary hover classes
    pub fn classes(mut self, classes: &str) -> Self {
        self.base.hover_classes.extend(classes.split_whitespace().map(|s| s.to_string()));
        self
    }

    /// Set border to primary color
    pub fn border_primary(mut self) -> Self {
        self.base.hover_classes.push(
            self.base.color_provider.border_class(Color::Primary).replace("border-", "")
        );
        self
    }

    /// Set background to primary color
    pub fn bg_primary(mut self) -> Self {
        self.base.hover_classes.push(
            self.base.color_provider.bg_class(Color::Primary).replace("bg-", "")
        );
        self
    }

    /// Darken the background
    pub fn darken(mut self) -> Self {
        self.base.hover_classes.push(
            self.base.color_provider.bg_class(Color::InteractiveHover).replace("bg-", "")
        );
        self
    }

    /// Scale transform
    pub fn scale_105(mut self) -> Self {
        self.base.hover_classes.push("scale-105".to_string());
        self
    }

    /// Add shadow
    pub fn shadow_md(mut self) -> Self {
        self.base.hover_classes.push("shadow-md".to_string());
        self
    }

    /// Add shadow
    pub fn shadow_lg(mut self) -> Self {
        self.base.hover_classes.push("shadow-lg".to_string());
        self
    }

    /// Continue building other states
    pub fn focus(self) -> FocusBuilder<C> {
        FocusBuilder::new(self.base)
    }

    /// Continue building other states
    pub fn active(self) -> ActiveBuilder<C> {
        ActiveBuilder::new(self.base)
    }

    /// Continue building other states
    pub fn disabled(self) -> DisabledBuilder<C> {
        DisabledBuilder::new(self.base)
    }

    /// Build the final CSS classes string
    pub fn build(self) -> String {
        self.base.build()
    }
}

/// Builder for focus states
pub struct FocusBuilder<C: ColorProvider> {
    base: InteractiveBase<C>,
}

impl<C: ColorProvider> FocusBuilder<C> {
    fn new(base: InteractiveBase<C>) -> Self {
        Self { base }
    }

    /// Add arbitrary focus classes
    pub fn classes(mut self, classes: &str) -> Self {
        self.base.focus_classes.extend(classes.split_whitespace().map(|s| s.to_string()));
        self
    }

    /// Set border to primary color
    pub fn border_primary(mut self) -> Self {
        self.base.focus_classes.push(
            self.base.color_provider.border_class(Color::Primary).replace("border-", "")
        );
        self
    }

    /// Remove outline
    pub fn outline_none(mut self) -> Self {
        self.base.focus_classes.push("outline-none".to_string());
        self
    }

    /// Add ring with primary color
    pub fn ring_primary(mut self) -> Self {
        self.base.focus_classes.push("ring-2".to_string());
        self.base.focus_classes.push("ring-offset-2".to_string());
        let ring_color = self.base.color_provider
            .resolve_color(Color::Primary)
            .replace("bg-", "ring-")
            .replace("-500", "-300");
        self.base.focus_classes.push(ring_color);
        self
    }

    /// Continue building other states
    pub fn hover(self) -> HoverBuilder<C> {
        HoverBuilder::new(self.base)
    }

    /// Continue building other states
    pub fn active(self) -> ActiveBuilder<C> {
        ActiveBuilder::new(self.base)
    }

    /// Continue building other states
    pub fn disabled(self) -> DisabledBuilder<C> {
        DisabledBuilder::new(self.base)
    }

    /// Build the final CSS classes string
    pub fn build(self) -> String {
        self.base.build()
    }
}

/// Builder for active states
pub struct ActiveBuilder<C: ColorProvider> {
    base: InteractiveBase<C>,
}

impl<C: ColorProvider> ActiveBuilder<C> {
    fn new(base: InteractiveBase<C>) -> Self {
        Self { base }
    }

    /// Add arbitrary active classes
    pub fn classes(mut self, classes: &str) -> Self {
        self.base.active_classes.extend(classes.split_whitespace().map(|s| s.to_string()));
        self
    }

    /// Scale transform
    pub fn scale_95(mut self) -> Self {
        self.base.active_classes.push("scale-95".to_string());
        self
    }

    /// Continue building other states
    pub fn hover(self) -> HoverBuilder<C> {
        HoverBuilder::new(self.base)
    }

    /// Continue building other states
    pub fn focus(self) -> FocusBuilder<C> {
        FocusBuilder::new(self.base)
    }

    /// Continue building other states
    pub fn disabled(self) -> DisabledBuilder<C> {
        DisabledBuilder::new(self.base)
    }

    /// Build the final CSS classes string
    pub fn build(self) -> String {
        self.base.build()
    }
}

/// Builder for disabled states
pub struct DisabledBuilder<C: ColorProvider> {
    base: InteractiveBase<C>,
}

impl<C: ColorProvider> DisabledBuilder<C> {
    fn new(base: InteractiveBase<C>) -> Self {
        Self { base }
    }

    /// Add arbitrary disabled classes
    pub fn classes(mut self, classes: &str) -> Self {
        self.base.disabled_classes.extend(classes.split_whitespace().map(|s| s.to_string()));
        self
    }

    /// Set opacity
    pub fn opacity_50(mut self) -> Self {
        self.base.disabled_classes.push("opacity-50".to_string());
        self
    }

    /// Set cursor
    pub fn cursor_not_allowed(mut self) -> Self {
        self.base.disabled_classes.push("cursor-not-allowed".to_string());
        self
    }

    /// Build the final CSS classes string
    pub fn build(self) -> String {
        self.base.build()
    }
}

/// Specialized input builder
pub struct InputBuilder<C: ColorProvider> {
    base: InteractiveBase<C>,
}

impl<C: ColorProvider> InputBuilder<C> {
    pub fn new(color_provider: C) -> Self {
        Self {
            base: InteractiveBase::new(color_provider),
        }
    }

    /// Apply base input styles
    pub fn base_style(self) -> Self {
        self.base_classes("w-full px-4 py-3 border rounded-md transition-colors focus:outline-none")
    }

    /// Apply standard input styles with theme colors
    pub fn standard_style(mut self) -> Self {
        let base_classes = format!(
            "w-full px-4 py-3 rounded-md transition-colors focus:outline-none {} {}",
            self.base.color_provider.border_class(Color::Border),
            self.base.color_provider.bg_class(Color::Surface)
        );
        self.base = self.base.base(&base_classes);
        self
    }

    /// Add base classes
    pub fn base_classes(mut self, classes: &str) -> Self {
        self.base = self.base.base(classes);
        self
    }

    /// Enter hover state builder
    pub fn hover(self) -> HoverBuilder<C> {
        self.base.hover()
    }

    /// Enter focus state builder
    pub fn focus(self) -> FocusBuilder<C> {
        self.base.focus()
    }

    /// Enter disabled state builder
    pub fn disabled(self) -> DisabledBuilder<C> {
        self.base.disabled()
    }

    /// Build the final CSS classes string
    pub fn build(self) -> String {
        self.base.build()
    }
}

/// Specialized button builder
pub struct ButtonBuilder<C: ColorProvider> {
    base: InteractiveBase<C>,
    variant: ButtonVariant,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
}

impl<C: ColorProvider> ButtonBuilder<C> {
    pub fn new(color_provider: C) -> Self {
        Self {
            base: InteractiveBase::new(color_provider),
            variant: ButtonVariant::Primary,
        }
    }

    /// Set primary variant
    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        let base_classes = format!(
            "inline-flex items-center justify-center px-4 py-2 font-medium rounded-md transition-colors {} {}",
            self.base.color_provider.bg_class(Color::Primary),
            self.base.color_provider.text_class(Color::TextInverse)
        );
        self.base = self.base.base(&base_classes);
        self
    }

    /// Set secondary variant
    pub fn secondary(mut self) -> Self {
        self.variant = ButtonVariant::Secondary;
        let base_classes = format!(
            "inline-flex items-center justify-center px-4 py-2 font-medium rounded-md transition-colors border {} {} {}",
            self.base.color_provider.bg_class(Color::Surface),
            self.base.color_provider.text_class(Color::TextPrimary),
            self.base.color_provider.border_class(Color::Border)
        );
        self.base = self.base.base(&base_classes);
        self
    }

    /// Set ghost variant
    pub fn ghost(mut self) -> Self {
        self.variant = ButtonVariant::Ghost;
        let base_classes = format!(
            "inline-flex items-center justify-center px-4 py-2 font-medium rounded-md transition-colors bg-transparent {}",
            self.base.color_provider.text_class(Color::TextPrimary)
        );
        self.base = self.base.base(&base_classes);
        self
    }

    /// Add base classes
    pub fn base_classes(mut self, classes: &str) -> Self {
        self.base = self.base.base(classes);
        self
    }

    /// Enter hover state builder
    pub fn hover(self) -> HoverBuilder<C> {
        self.base.hover()
    }

    /// Enter focus state builder
    pub fn focus(self) -> FocusBuilder<C> {
        self.base.focus()
    }

    /// Enter active state builder
    pub fn active(self) -> ActiveBuilder<C> {
        self.base.active()
    }

    /// Enter disabled state builder
    pub fn disabled(self) -> DisabledBuilder<C> {
        self.base.disabled()
    }

    /// Build the final CSS classes string
    pub fn build(self) -> String {
        self.base.build()
    }
}

/// Convenience function to create an interactive input
pub fn interactive_input<C: ColorProvider>(color_provider: C) -> InputBuilder<C> {
    InputBuilder::new(color_provider)
}

/// Convenience function to create an interactive button
pub fn interactive_button<C: ColorProvider>(color_provider: C) -> ButtonBuilder<C> {
    ButtonBuilder::new(color_provider)
}

/// Convenience function to create a generic interactive element
pub fn interactive_element<C: ColorProvider>(color_provider: C) -> InteractiveBase<C> {
    InteractiveBase::new(color_provider)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::color::WaterWellnessColors;

    #[test]
    fn test_interactive_input() {
        let colors = WaterWellnessColors::default();
        let classes = interactive_input(colors)
            .base_style()
            .hover().border_primary().shadow_md()
            .focus().border_primary().ring_primary().outline_none()
            .disabled().opacity_50()
            .build();
        
        assert!(classes.contains("w-full"));
        assert!(classes.contains("hover:"));
        assert!(classes.contains("focus:"));
        assert!(classes.contains("disabled:"));
    }

    #[test]
    fn test_interactive_button() {
        let colors = WaterWellnessColors::default();
        let classes = interactive_button(colors)
            .primary()
            .hover().darken().scale_105()
            .focus().ring_primary()
            .active().scale_95()
            .build();
        
        assert!(classes.contains("inline-flex"));
        assert!(classes.contains("hover:"));
        assert!(classes.contains("focus:"));
        assert!(classes.contains("active:"));
    }

    #[test]
    fn test_chaining_order_independence() {
        let colors = WaterWellnessColors::default();
        
        // Test different chaining orders produce same result
        let classes1 = interactive_input(colors.clone())
            .base_style()
            .hover().border_primary()
            .focus().ring_primary()
            .build();
        
        let classes2 = interactive_input(colors)
            .base_style()
            .focus().ring_primary()
            .hover().border_primary()
            .build();
        
        // Both should contain the same pseudo-class groups
        assert!(classes1.contains("hover:"));
        assert!(classes1.contains("focus:"));
        assert!(classes2.contains("hover:"));
        assert!(classes2.contains("focus:"));
    }
}