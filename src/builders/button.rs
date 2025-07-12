//! Button styling utilities for the Jupiter Design System
//!
//! Provides a chainable API for building button CSS classes that can be used
//! with any component library or framework.

use crate::core::color::ColorProvider;
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

/// Button styling utility builder
///
/// This is a pure styling utility that generates CSS classes for buttons.
/// It can be used with any component library or framework that supports Tailwind CSS.
///
/// # Examples
///
/// ```rust
/// use jupiter_design_system::builders::button::ButtonStyles;
/// use jupiter_design_system::core::color::WaterWellnessColors;
///
/// let classes = ButtonStyles::new(WaterWellnessColors::default())
///     .primary()
///     .large()
///     .full_width()
///     .classes();
/// ```
#[derive(Debug, Clone)]
pub struct ButtonStyles<C: ColorProvider> {
    variant: ButtonVariant,
    size: Size,
    state: ButtonState,
    full_width: bool,
    with_icon: bool,
    custom_classes: Vec<String>,
    color_provider: C,
}

impl<C: ColorProvider> ButtonStyles<C> {
    /// Create a new button styling utility
    pub fn new(color_provider: C) -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size: Size::Medium,
            state: ButtonState::Default,
            full_width: false,
            with_icon: false,
            custom_classes: Vec::new(),
            color_provider,
        }
    }

    /// Set primary variant (shorthand)
    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }

    /// Set secondary variant (shorthand)
    pub fn secondary(mut self) -> Self {
        self.variant = ButtonVariant::Secondary;
        self
    }

    /// Set success variant (shorthand)
    pub fn success(mut self) -> Self {
        self.variant = ButtonVariant::Success;
        self
    }

    /// Set warning variant (shorthand)
    pub fn warning(mut self) -> Self {
        self.variant = ButtonVariant::Warning;
        self
    }

    /// Set error variant (shorthand)
    pub fn error(mut self) -> Self {
        self.variant = ButtonVariant::Error;
        self
    }

    /// Set ghost variant (shorthand)
    pub fn ghost(mut self) -> Self {
        self.variant = ButtonVariant::Ghost;
        self
    }

    /// Set link variant (shorthand)
    pub fn link(mut self) -> Self {
        self.variant = ButtonVariant::Link;
        self
    }

    /// Set variant explicitly
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set variant from string (convenience method)
    ///
    /// Maps common string variants to ButtonVariant enum.
    /// Supports: "primary", "secondary", "success", "warning", "error", "ghost", "link"
    /// Also supports aliases: "outline" -> Secondary, "danger" -> Error, "water" -> Primary
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jupiter_design_system::builders::button::ButtonStyles;
    /// use jupiter_design_system::core::color::WaterWellnessColors;
    ///
    /// let colors = WaterWellnessColors::default();
    /// let classes = ButtonStyles::new(colors)
    ///     .variant_str("primary")
    ///     .size_str("lg")
    ///     .classes();
    /// ```
    pub fn variant_str(mut self, variant: &str) -> Self {
        self.variant = match variant {
            "primary" => ButtonVariant::Primary,
            "secondary" => ButtonVariant::Secondary,
            "outline" => ButtonVariant::Secondary, // alias
            "success" => ButtonVariant::Success,
            "warning" => ButtonVariant::Warning,
            "error" => ButtonVariant::Error,
            "danger" => ButtonVariant::Error, // alias
            "ghost" => ButtonVariant::Ghost,
            "link" => ButtonVariant::Link,
            "water" => ButtonVariant::Primary, // Water & Wellness specific alias
            _ => ButtonVariant::Primary,       // fallback to primary
        };
        self
    }

    /// Set extra small size (shorthand)
    pub fn extra_small(mut self) -> Self {
        self.size = Size::XSmall;
        self
    }

    /// Set small size (shorthand)
    pub fn small(mut self) -> Self {
        self.size = Size::Small;
        self
    }

    /// Set medium size (shorthand)
    pub fn medium(mut self) -> Self {
        self.size = Size::Medium;
        self
    }

    /// Set large size (shorthand)
    pub fn large(mut self) -> Self {
        self.size = Size::Large;
        self
    }

    /// Set extra large size (shorthand)
    pub fn extra_large(mut self) -> Self {
        self.size = Size::XLarge;
        self
    }

    /// Set size explicitly
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Set size from string (convenience method)
    ///
    /// Maps common string sizes to Size enum.
    /// Supports: "xs", "sm", "md", "lg", "xl"
    /// Also supports aliases: "extra_small" -> XSmall, "small" -> Small, etc.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jupiter_design_system::builders::button::ButtonStyles;
    /// use jupiter_design_system::core::color::WaterWellnessColors;
    ///
    /// let colors = WaterWellnessColors::default();
    /// let classes = ButtonStyles::new(colors)
    ///     .variant_str("primary")
    ///     .size_str("lg")
    ///     .classes();
    /// ```
    pub fn size_str(mut self, size: &str) -> Self {
        self.size = match size {
            "xs" | "extra_small" => Size::XSmall,
            "sm" | "small" => Size::Small,
            "md" | "medium" => Size::Medium,
            "lg" | "large" => Size::Large,
            "xl" | "extra_large" => Size::XLarge,
            _ => Size::Medium, // fallback to medium
        };
        self
    }

    /// Set disabled state (shorthand)
    pub fn disabled(mut self) -> Self {
        self.state = ButtonState::Disabled;
        self
    }

    /// Set loading state (shorthand)
    pub fn loading(mut self) -> Self {
        self.state = ButtonState::Loading;
        self
    }

    /// Set hover state (shorthand)
    pub fn hover(mut self) -> Self {
        self.state = ButtonState::Hover;
        self
    }

    /// Set active state (shorthand)
    pub fn active(mut self) -> Self {
        self.state = ButtonState::Active;
        self
    }

    /// Set state explicitly
    pub fn state(mut self, state: ButtonState) -> Self {
        self.state = state;
        self
    }

    /// Set state from string (convenience method)
    ///
    /// Maps common string states to ButtonState enum.
    /// Supports: "default", "hover", "active", "disabled", "loading"
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jupiter_design_system::builders::button::ButtonStyles;
    /// use jupiter_design_system::core::color::WaterWellnessColors;
    ///
    /// let colors = WaterWellnessColors::default();
    /// let classes = ButtonStyles::new(colors)
    ///     .variant_str("primary")
    ///     .state_str("loading")
    ///     .classes();
    /// ```
    pub fn state_str(mut self, state: &str) -> Self {
        self.state = match state {
            "default" => ButtonState::Default,
            "hover" => ButtonState::Hover,
            "active" => ButtonState::Active,
            "disabled" => ButtonState::Disabled,
            "loading" => ButtonState::Loading,
            _ => ButtonState::Default, // fallback to default
        };
        self
    }

    /// Set full width
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    /// Set whether the button has an icon
    pub fn with_icon(mut self) -> Self {
        self.with_icon = true;
        self
    }

    /// Add a custom CSS class
    ///
    /// This allows you to add any custom Tailwind classes or other CSS classes
    /// that aren't covered by the design system.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jupiter_design_system::builders::button::ButtonStyles;
    /// use jupiter_design_system::core::color::WaterWellnessColors;
    ///
    /// let colors = WaterWellnessColors::default();
    /// let classes = ButtonStyles::new(colors)
    ///     .primary()
    ///     .custom("shadow-xl")
    ///     .custom("transform")
    ///     .classes();
    /// ```
    pub fn custom(mut self, class: impl Into<String>) -> Self {
        self.custom_classes.push(class.into());
        self
    }

    /// Add multiple custom CSS classes at once
    ///
    /// Accepts a space-separated string of classes or a vector of class strings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jupiter_design_system::builders::button::ButtonStyles;
    /// use jupiter_design_system::core::color::WaterWellnessColors;
    ///
    /// let colors = WaterWellnessColors::default();
    /// let classes = ButtonStyles::new(colors)
    ///     .primary()
    ///     .custom_classes("shadow-xl transform hover:rotate-1")
    ///     .classes();
    /// ```
    pub fn custom_classes(mut self, classes: impl Into<String>) -> Self {
        let classes_str = classes.into();
        for class in classes_str.split_whitespace() {
            if !class.is_empty() {
                self.custom_classes.push(class.to_string());
            }
        }
        self
    }

    /// Add a vector of custom CSS classes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jupiter_design_system::builders::button::ButtonStyles;
    /// use jupiter_design_system::core::color::WaterWellnessColors;
    ///
    /// let colors = WaterWellnessColors::default();
    /// let classes = ButtonStyles::new(colors)
    ///     .primary()
    ///     .custom_vec(vec!["shadow-xl", "transform", "hover:rotate-1"])
    ///     .classes();
    /// ```
    pub fn custom_vec(mut self, classes: Vec<impl Into<String>>) -> Self {
        for class in classes {
            self.custom_classes.push(class.into());
        }
        self
    }

    /// Build the final CSS classes string
    pub fn classes(self) -> String {
        self.build()
    }

    /// Build the final CSS classes string (alias for classes)
    pub fn build(self) -> String {
        let base_classes = self.get_base_classes();
        let size_classes = self.get_size_classes();
        let variant_classes = self.get_variant_classes();
        let state_classes = self.get_state_classes();
        let width_classes = if self.full_width { "w-full" } else { "" };
        let icon_classes = if self.with_icon { "space-x-2" } else { "" };
        let custom_classes = self.custom_classes.join(" ");

        format!(
            "{base_classes} {size_classes} {variant_classes} {state_classes} {width_classes} {icon_classes} {custom_classes}"
        )
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
    }

    /// Get base button classes
    fn get_base_classes(&self) -> String {
        "inline-flex items-center justify-center font-medium transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed".to_string()
    }

    /// Get size-specific classes
    fn get_size_classes(&self) -> String {
        match self.size {
            Size::XSmall => "px-2 py-1 text-xs rounded",
            Size::Small => "px-3 py-1.5 text-sm rounded",
            Size::Medium => "px-4 py-2 text-sm rounded-md",
            Size::Large => "px-6 py-3 text-base rounded-md",
            Size::XLarge => "px-8 py-4 text-lg rounded-lg",
        }
        .to_string()
    }

    /// Get variant-specific classes
    fn get_variant_classes(&self) -> String {
        match self.variant {
            ButtonVariant::Primary => {
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
            ButtonVariant::Ghost => {
                let hover_bg = format!("hover:{}", self.color_provider.bg_class(Color::Background));
                format!(
                    "{} {} {}",
                    "bg-transparent",
                    self.color_provider.text_class(Color::TextPrimary),
                    hover_bg
                )
            }
            ButtonVariant::Link => format!(
                "{} {} {}",
                "bg-transparent",
                self.color_provider.text_class(Color::Primary),
                "hover:underline"
            ),
        }
    }

    /// Get state-specific classes
    fn get_state_classes(&self) -> String {
        match self.state {
            ButtonState::Default => "".to_string(),
            ButtonState::Hover => "hover:scale-105".to_string(),
            ButtonState::Active => "active:scale-95".to_string(),
            ButtonState::Disabled => "opacity-50 cursor-not-allowed".to_string(),
            ButtonState::Loading => "cursor-wait".to_string(),
        }
    }
}

/// Convenience function to create button styles
pub fn button_styles<C: ColorProvider>(color_provider: C) -> ButtonStyles<C> {
    ButtonStyles::new(color_provider)
}

/// One-shot convenience function to create button classes from strings
///
/// This completely replaces the need for ButtonUtils::classes() and similar utility functions.
/// Perfect for component libraries that need to map string props to CSS classes.
///
/// # Examples
///
/// ```rust
/// use jupiter_design_system::builders::button::button_classes_from_strings;
/// use jupiter_design_system::core::color::WaterWellnessColors;
///
/// let colors = WaterWellnessColors::default();
/// let classes = button_classes_from_strings(
///     colors,
///     "primary",
///     "lg",
///     false,     // disabled
///     true,      // loading
///     false,     // full_width
/// );
/// ```
pub fn button_classes_from_strings<C: ColorProvider>(
    color_provider: C,
    variant: &str,
    size: &str,
    disabled: bool,
    loading: bool,
    full_width: bool,
) -> String {
    let mut builder = ButtonStyles::new(color_provider)
        .variant_str(variant)
        .size_str(size);

    // Set state based on disabled/loading flags
    if loading {
        builder = builder.loading();
    } else if disabled {
        builder = builder.disabled();
    }

    // Apply conditional modifiers
    if full_width {
        builder = builder.full_width();
    }

    builder.classes()
}

#[cfg(test)]
#[path = "button_test.rs"]
mod button_test;
