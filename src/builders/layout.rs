//! Layout styling utilities for the Jupiter Design System
//!
//! Provides a chainable API for building layout CSS classes that can be used
//! with any component library or framework.

use crate::core::color::ColorProvider;
use crate::patterns::{LayoutAlignment, LayoutDirection, LayoutDivider, LayoutSpacing};

/// Layout styling utility builder
///
/// This is a pure styling utility that generates CSS classes for layout components.
/// It can be used with any component library or framework that supports Tailwind CSS.
///
/// # Examples
///
/// ```rust
/// use jupiter_design_system::builders::layout::LayoutStyles;
/// use jupiter_design_system::themes::VibeColors;
///
/// let classes = LayoutStyles::new(VibeColors::default())
///     .divider_bottom()
///     .spacing_md()
///     .direction_horizontal()
///     .alignment_between()
///     .classes();
/// ```
#[derive(Debug, Clone)]
pub struct LayoutStyles<C: ColorProvider> {
    divider: LayoutDivider,
    spacing: LayoutSpacing,
    alignment: Option<LayoutAlignment>,
    direction: Option<LayoutDirection>,
    custom_classes: Vec<String>,
    color_provider: C,
}

impl<C: ColorProvider> LayoutStyles<C> {
    /// Create a new layout styling utility
    pub fn new(color_provider: C) -> Self {
        Self {
            divider: LayoutDivider::None,
            spacing: LayoutSpacing::MD,
            alignment: None,
            direction: None,
            custom_classes: Vec::new(),
            color_provider,
        }
    }

    // === Divider Methods ===

    /// Set no divider
    pub fn divider_none(mut self) -> Self {
        self.divider = LayoutDivider::None;
        self
    }

    /// Set top border divider
    pub fn divider_top(mut self) -> Self {
        self.divider = LayoutDivider::Top;
        self
    }

    /// Set bottom border divider
    pub fn divider_bottom(mut self) -> Self {
        self.divider = LayoutDivider::Bottom;
        self
    }

    /// Set left border divider
    pub fn divider_left(mut self) -> Self {
        self.divider = LayoutDivider::Left;
        self
    }

    /// Set right border divider
    pub fn divider_right(mut self) -> Self {
        self.divider = LayoutDivider::Right;
        self
    }

    // === Spacing Methods ===

    /// Set no spacing
    pub fn spacing_none(mut self) -> Self {
        self.spacing = LayoutSpacing::None;
        self
    }

    /// Set extra small spacing
    pub fn spacing_xs(mut self) -> Self {
        self.spacing = LayoutSpacing::XS;
        self
    }

    /// Set small spacing
    pub fn spacing_sm(mut self) -> Self {
        self.spacing = LayoutSpacing::SM;
        self
    }

    /// Set medium spacing
    pub fn spacing_md(mut self) -> Self {
        self.spacing = LayoutSpacing::MD;
        self
    }

    /// Set large spacing
    pub fn spacing_lg(mut self) -> Self {
        self.spacing = LayoutSpacing::LG;
        self
    }

    /// Set extra large spacing
    pub fn spacing_xl(mut self) -> Self {
        self.spacing = LayoutSpacing::XL;
        self
    }

    /// Set 2x large spacing
    pub fn spacing_xl2(mut self) -> Self {
        self.spacing = LayoutSpacing::XL2;
        self
    }

    // === Direction Methods ===

    /// Set vertical direction
    pub fn direction_vertical(mut self) -> Self {
        self.direction = Some(LayoutDirection::Vertical);
        self
    }

    /// Set horizontal direction
    pub fn direction_horizontal(mut self) -> Self {
        self.direction = Some(LayoutDirection::Horizontal);
        self
    }

    // === Alignment Methods ===

    /// Set start alignment
    pub fn alignment_start(mut self) -> Self {
        self.alignment = Some(LayoutAlignment::Start);
        self
    }

    /// Set center alignment
    pub fn alignment_center(mut self) -> Self {
        self.alignment = Some(LayoutAlignment::Center);
        self
    }

    /// Set end alignment
    pub fn alignment_end(mut self) -> Self {
        self.alignment = Some(LayoutAlignment::End);
        self
    }

    /// Set space between alignment
    pub fn alignment_between(mut self) -> Self {
        self.alignment = Some(LayoutAlignment::Between);
        self
    }

    /// Set space around alignment
    pub fn alignment_around(mut self) -> Self {
        self.alignment = Some(LayoutAlignment::Around);
        self
    }

    /// Set space evenly alignment
    pub fn alignment_evenly(mut self) -> Self {
        self.alignment = Some(LayoutAlignment::Evenly);
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

    // === Build Methods ===

    /// Build the final CSS classes string
    pub fn classes(self) -> String {
        self.build()
    }

    /// Build the final CSS classes string (alias for classes)
    pub fn build(self) -> String {
        let mut all_classes = Vec::new();

        // Divider classes
        let divider_classes = match self.divider {
            LayoutDivider::None => "",
            LayoutDivider::Top => {
                let border_class = self.color_provider.border_class(crate::core::Color::Border);
                &format!("border-t {}", border_class)
            }
            LayoutDivider::Bottom => {
                let border_class = self.color_provider.border_class(crate::core::Color::Border);
                &format!("border-b {}", border_class)
            }
            LayoutDivider::Left => {
                let border_class = self.color_provider.border_class(crate::core::Color::Border);
                &format!("border-l {}", border_class)
            }
            LayoutDivider::Right => {
                let border_class = self.color_provider.border_class(crate::core::Color::Border);
                &format!("border-r {}", border_class)
            }
        };

        if !divider_classes.is_empty() {
            all_classes.push(divider_classes.to_string());
        }

        // Spacing classes
        let spacing_classes = match self.spacing {
            LayoutSpacing::None => "",
            LayoutSpacing::XS => "p-1",
            LayoutSpacing::SM => "p-2",
            LayoutSpacing::MD => "p-4",
            LayoutSpacing::LG => "p-6",
            LayoutSpacing::XL => "p-8",
            LayoutSpacing::XL2 => "p-12",
        };

        if !spacing_classes.is_empty() {
            all_classes.push(spacing_classes.to_string());
        }

        // Direction classes
        if let Some(direction) = self.direction {
            let direction_classes = match direction {
                LayoutDirection::Vertical => "flex flex-col",
                LayoutDirection::Horizontal => "flex flex-row",
            };
            all_classes.push(direction_classes.to_string());
        }

        // Alignment classes
        if let Some(alignment) = self.alignment {
            let alignment_classes = match alignment {
                LayoutAlignment::Start => "items-start justify-start",
                LayoutAlignment::Center => "items-center justify-center",
                LayoutAlignment::End => "items-end justify-end",
                LayoutAlignment::Between => "items-center justify-between",
                LayoutAlignment::Around => "items-center justify-around",
                LayoutAlignment::Evenly => "items-center justify-evenly",
            };
            all_classes.push(alignment_classes.to_string());
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
}

/// Convenience function to create layout styles
pub fn layout_styles<C: ColorProvider>(color_provider: C) -> LayoutStyles<C> {
    LayoutStyles::new(color_provider)
}

/// Convenience function to create card header layout styles
pub fn card_header_styles<C: ColorProvider>(color_provider: C) -> LayoutStyles<C> {
    LayoutStyles::new(color_provider)
        .divider_bottom()
        .spacing_md()
}

/// Convenience function to create card content layout styles
pub fn card_content_styles<C: ColorProvider>(color_provider: C) -> LayoutStyles<C> {
    LayoutStyles::new(color_provider)
        .spacing_md()
        .custom("space-y-4")
}

/// Convenience function to create card footer layout styles
pub fn card_footer_styles<C: ColorProvider>(color_provider: C) -> LayoutStyles<C> {
    LayoutStyles::new(color_provider)
        .divider_top()
        .spacing_md()
        .direction_horizontal()
        .alignment_between()
}

#[cfg(test)]
#[path = "layout_test.rs"]
mod layout_test;
