//! Layout patterns for Jupiter Design System
//!
//! This module provides abstract layout patterns for structural components
//! like card sub-components, dividers, and container elements.

use crate::core::color::ColorProvider;
use serde::{Deserialize, Serialize};

/// Layout spacing types for consistent spacing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayoutSpacing {
    /// No spacing
    None,
    /// Extra small spacing (0.25rem)
    XS,
    /// Small spacing (0.5rem)
    SM,
    /// Medium spacing (1rem)
    MD,
    /// Large spacing (1.5rem)
    LG,
    /// Extra large spacing (2rem)
    XL,
    /// 2X large spacing (3rem)
    XL2,
}

/// Layout divider types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayoutDivider {
    /// No divider
    None,
    /// Top border divider
    Top,
    /// Bottom border divider
    Bottom,
    /// Left border divider
    Left,
    /// Right border divider
    Right,
}

/// Layout alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayoutAlignment {
    /// Start aligned
    Start,
    /// Center aligned
    Center,
    /// End aligned
    End,
    /// Space between
    Between,
    /// Space around
    Around,
    /// Space evenly
    Evenly,
}

/// Layout direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayoutDirection {
    /// Vertical layout
    Vertical,
    /// Horizontal layout
    Horizontal,
}

/// Card section layout pattern for headers, content, and footers
#[derive(Debug, Clone)]
pub struct CardSectionLayout<C: ColorProvider> {
    divider: LayoutDivider,
    spacing: LayoutSpacing,
    alignment: Option<LayoutAlignment>,
    direction: Option<LayoutDirection>,
    custom_classes: Vec<String>,
    color_provider: C,
}

impl<C: ColorProvider> CardSectionLayout<C> {
    /// Create a new card section layout
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

    /// Set divider type
    pub fn divider(mut self, divider: LayoutDivider) -> Self {
        self.divider = divider;
        self
    }

    /// Set spacing
    pub fn spacing(mut self, spacing: LayoutSpacing) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set alignment
    pub fn alignment(mut self, alignment: LayoutAlignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    /// Set direction
    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    /// Add custom classes
    pub fn custom(mut self, classes: impl Into<String>) -> Self {
        self.custom_classes.push(classes.into());
        self
    }

    /// Build the CSS classes
    pub fn classes(self) -> String {
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
}

/// Convenience function for card header layout
pub fn card_header_layout<C: ColorProvider>(color_provider: C) -> CardSectionLayout<C> {
    CardSectionLayout::new(color_provider)
        .divider(LayoutDivider::Bottom)
        .spacing(LayoutSpacing::MD)
}

/// Convenience function for card content layout
pub fn card_content_layout<C: ColorProvider>(color_provider: C) -> CardSectionLayout<C> {
    CardSectionLayout::new(color_provider)
        .spacing(LayoutSpacing::MD)
        .custom("space-y-4")
}

/// Convenience function for card footer layout
pub fn card_footer_layout<C: ColorProvider>(color_provider: C) -> CardSectionLayout<C> {
    CardSectionLayout::new(color_provider)
        .divider(LayoutDivider::Top)
        .spacing(LayoutSpacing::MD)
        .direction(LayoutDirection::Horizontal)
        .alignment(LayoutAlignment::Between)
}

/// Generic layout builder for various layout needs
#[derive(Debug, Clone)]
pub struct LayoutBuilder<C: ColorProvider> {
    color_provider: C,
}

impl<C: ColorProvider> LayoutBuilder<C> {
    /// Create a new layout builder
    pub fn new(color_provider: C) -> Self {
        Self { color_provider }
    }

    /// Create a card section layout
    pub fn card_section(self) -> CardSectionLayout<C> {
        CardSectionLayout::new(self.color_provider)
    }

    /// Create a card header layout
    pub fn card_header(self) -> CardSectionLayout<C> {
        card_header_layout(self.color_provider)
    }

    /// Create a card content layout
    pub fn card_content(self) -> CardSectionLayout<C> {
        card_content_layout(self.color_provider)
    }

    /// Create a card footer layout
    pub fn card_footer(self) -> CardSectionLayout<C> {
        card_footer_layout(self.color_provider)
    }
}

/// Convenience function to create a layout builder
pub fn layout<C: ColorProvider>(color_provider: C) -> LayoutBuilder<C> {
    LayoutBuilder::new(color_provider)
}
