//! Text styling builder for Jupiter Design System
//!
//! This module provides a chainable API for building text CSS classes that can be used
//! with any component library or framework that supports Tailwind CSS. The builder generates
//! typography classes based on semantic hierarchy and design system constraints.

use crate::core::color::ColorProvider;
use crate::patterns::typography::{
    TypographyAlignment, TypographyColor, TypographyHierarchy, TypographyOverflow,
    TypographyPattern, TypographySize, TypographyWeight,
};

/// Text styling builder with chainable API
#[derive(Debug, Clone)]
pub struct TextStyles<T: ColorProvider> {
    pattern: TypographyPattern<T>,
    custom_classes: Vec<String>,
}

impl<T: ColorProvider> TextStyles<T> {
    /// Create a new text styles builder with default values
    pub fn new(color_provider: T) -> Self {
        Self {
            pattern: TypographyPattern::new(color_provider),
            custom_classes: Vec::new(),
        }
    }

    /// Set typography hierarchy
    pub fn hierarchy(mut self, hierarchy: TypographyHierarchy) -> Self {
        self.pattern = self.pattern.hierarchy(hierarchy);
        self
    }

    /// Set typography hierarchy from string
    pub fn hierarchy_str(mut self, hierarchy: &str) -> Self {
        let hierarchy_enum = match hierarchy {
            "title" => TypographyHierarchy::Title,
            "heading" => TypographyHierarchy::Heading,
            "subheading" => TypographyHierarchy::Subheading,
            "h4" => TypographyHierarchy::H4,
            "body" => TypographyHierarchy::Body,
            "body-large" => TypographyHierarchy::BodyLarge,
            "body-small" => TypographyHierarchy::BodySmall,
            "caption" => TypographyHierarchy::Caption,
            "overline" => TypographyHierarchy::Overline,
            "code" => TypographyHierarchy::Code,
            _ => TypographyHierarchy::Body, // default fallback
        };
        self.pattern = self.pattern.hierarchy(hierarchy_enum);
        self
    }

    /// Set typography size (overrides hierarchy default)
    pub fn size(mut self, size: TypographySize) -> Self {
        self.pattern = self.pattern.size(size);
        self
    }

    /// Set typography size from string
    pub fn size_str(mut self, size: &str) -> Self {
        let size_enum = match size {
            "xs" => TypographySize::XS,
            "sm" => TypographySize::SM,
            "md" => TypographySize::MD,
            "lg" => TypographySize::LG,
            "xl" => TypographySize::XL,
            "2xl" => TypographySize::XL2,
            "3xl" => TypographySize::XL3,
            "4xl" => TypographySize::XL4,
            _ => return self, // ignore invalid sizes
        };
        self.pattern = self.pattern.size(size_enum);
        self
    }

    /// Set typography weight (overrides hierarchy default)
    pub fn weight(mut self, weight: TypographyWeight) -> Self {
        self.pattern = self.pattern.weight(weight);
        self
    }

    /// Set typography weight from string
    pub fn weight_str(mut self, weight: &str) -> Self {
        let weight_enum = match weight {
            "light" => TypographyWeight::Light,
            "normal" => TypographyWeight::Normal,
            "medium" => TypographyWeight::Medium,
            "semibold" => TypographyWeight::Semibold,
            "bold" => TypographyWeight::Bold,
            "extrabold" => TypographyWeight::ExtraBold,
            _ => return self, // ignore invalid weights
        };
        self.pattern = self.pattern.weight(weight_enum);
        self
    }

    /// Set typography color
    pub fn color(mut self, color: TypographyColor) -> Self {
        self.pattern = self.pattern.color(color);
        self
    }

    /// Set typography color from string
    pub fn color_str(mut self, color: &str) -> Self {
        let color_enum = match color {
            "primary" => TypographyColor::Primary,
            "secondary" => TypographyColor::Secondary,
            "accent" => TypographyColor::Accent,
            "muted" => TypographyColor::Muted,
            "disabled" => TypographyColor::Disabled,
            "white" => TypographyColor::White,
            "black" => TypographyColor::Black,
            "success" => TypographyColor::Success,
            "warning" => TypographyColor::Warning,
            "error" => TypographyColor::Error,
            "info" => TypographyColor::Info,
            "auto" => TypographyColor::Auto,
            _ => return self, // ignore invalid colors
        };
        self.pattern = self.pattern.color(color_enum);
        self
    }

    /// Set text alignment
    pub fn alignment(mut self, alignment: TypographyAlignment) -> Self {
        self.pattern = self.pattern.alignment(alignment);
        self
    }

    /// Set text alignment from string
    pub fn alignment_str(mut self, alignment: &str) -> Self {
        let alignment_enum = match alignment {
            "left" => TypographyAlignment::Left,
            "center" => TypographyAlignment::Center,
            "right" => TypographyAlignment::Right,
            "justify" => TypographyAlignment::Justify,
            _ => return self, // ignore invalid alignments
        };
        self.pattern = self.pattern.alignment(alignment_enum);
        self
    }

    /// Set text overflow behavior
    pub fn overflow(mut self, overflow: TypographyOverflow) -> Self {
        self.pattern = self.pattern.overflow(overflow);
        self
    }

    /// Enable text truncation
    pub fn truncate(mut self) -> Self {
        self.pattern = self.pattern.overflow(TypographyOverflow::Truncate);
        self
    }

    /// Set line clamping
    pub fn clamp_lines(mut self, lines: u32) -> Self {
        self.pattern = self.pattern.overflow(TypographyOverflow::Clamp(lines));
        self
    }

    /// Add custom CSS classes
    pub fn custom_classes(mut self, classes: &str) -> Self {
        if !classes.is_empty() {
            self.custom_classes.push(classes.to_string());
        }
        self
    }

    /// Convenience methods for common hierarchies
    pub fn title(self) -> Self {
        self.hierarchy(TypographyHierarchy::Title)
    }

    pub fn heading(self) -> Self {
        self.hierarchy(TypographyHierarchy::Heading)
    }

    pub fn subheading(self) -> Self {
        self.hierarchy(TypographyHierarchy::Subheading)
    }

    pub fn h4(self) -> Self {
        self.hierarchy(TypographyHierarchy::H4)
    }

    pub fn body(self) -> Self {
        self.hierarchy(TypographyHierarchy::Body)
    }

    pub fn body_large(self) -> Self {
        self.hierarchy(TypographyHierarchy::BodyLarge)
    }

    pub fn body_small(self) -> Self {
        self.hierarchy(TypographyHierarchy::BodySmall)
    }

    pub fn caption(self) -> Self {
        self.hierarchy(TypographyHierarchy::Caption)
    }

    pub fn overline(self) -> Self {
        self.hierarchy(TypographyHierarchy::Overline)
    }

    pub fn code(self) -> Self {
        self.hierarchy(TypographyHierarchy::Code)
    }

    /// Convenience methods for common sizes
    pub fn extra_small(self) -> Self {
        self.size(TypographySize::XS)
    }

    pub fn small(self) -> Self {
        self.size(TypographySize::SM)
    }

    pub fn medium(self) -> Self {
        self.size(TypographySize::MD)
    }

    pub fn large(self) -> Self {
        self.size(TypographySize::LG)
    }

    pub fn extra_large(self) -> Self {
        self.size(TypographySize::XL)
    }

    /// Convenience methods for common weights
    pub fn light(self) -> Self {
        self.weight(TypographyWeight::Light)
    }

    pub fn normal(self) -> Self {
        self.weight(TypographyWeight::Normal)
    }

    pub fn medium_weight(self) -> Self {
        self.weight(TypographyWeight::Medium)
    }

    pub fn semibold(self) -> Self {
        self.weight(TypographyWeight::Semibold)
    }

    pub fn bold(self) -> Self {
        self.weight(TypographyWeight::Bold)
    }

    /// Convenience methods for common colors
    pub fn primary(self) -> Self {
        self.color(TypographyColor::Primary)
    }

    pub fn secondary(self) -> Self {
        self.color(TypographyColor::Secondary)
    }

    pub fn accent(self) -> Self {
        self.color(TypographyColor::Accent)
    }

    pub fn muted(self) -> Self {
        self.color(TypographyColor::Muted)
    }

    pub fn disabled(self) -> Self {
        self.color(TypographyColor::Disabled)
    }

    pub fn white(self) -> Self {
        self.color(TypographyColor::White)
    }

    pub fn success(self) -> Self {
        self.color(TypographyColor::Success)
    }

    pub fn warning(self) -> Self {
        self.color(TypographyColor::Warning)
    }

    pub fn error(self) -> Self {
        self.color(TypographyColor::Error)
    }

    /// Convenience methods for common alignments
    pub fn left(self) -> Self {
        self.alignment(TypographyAlignment::Left)
    }

    pub fn center(self) -> Self {
        self.alignment(TypographyAlignment::Center)
    }

    pub fn right(self) -> Self {
        self.alignment(TypographyAlignment::Right)
    }

    pub fn justify(self) -> Self {
        self.alignment(TypographyAlignment::Justify)
    }

    /// Generate final CSS classes
    pub fn classes(&self) -> String {
        let mut classes = vec![self.pattern.classes()];

        // Add custom classes
        for custom_class in &self.custom_classes {
            classes.push(custom_class.clone());
        }

        // Join all classes and deduplicate
        let mut all_classes: Vec<String> = classes
            .join(" ")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        all_classes.sort();
        all_classes.dedup();
        all_classes.join(" ")
    }

    /// Get the appropriate HTML element for this text
    pub fn element(&self) -> String {
        self.pattern.get_element()
    }

    /// Get CSS style for line clamping (if applicable)
    pub fn clamp_style(&self) -> String {
        self.pattern.get_clamp_style()
    }
}

/// Create a text styles builder
pub fn text_styles<T: ColorProvider>(color_provider: T) -> TextStyles<T> {
    TextStyles::new(color_provider)
}

/// Utility function to generate text classes from string parameters
pub fn text_classes_from_strings<T: ColorProvider>(
    color_provider: T,
    hierarchy: &str,
    size: Option<&str>,
    weight: Option<&str>,
    color: Option<&str>,
    alignment: Option<&str>,
    truncate: bool,
    clamp_lines: Option<u32>,
    custom_classes: Option<&str>,
) -> String {
    let mut builder = text_styles(color_provider).hierarchy_str(hierarchy);

    if let Some(size) = size {
        builder = builder.size_str(size);
    }

    if let Some(weight) = weight {
        builder = builder.weight_str(weight);
    }

    if let Some(color) = color {
        builder = builder.color_str(color);
    }

    if let Some(alignment) = alignment {
        builder = builder.alignment_str(alignment);
    }

    if truncate {
        builder = builder.truncate();
    }

    if let Some(lines) = clamp_lines {
        builder = builder.clamp_lines(lines);
    }

    if let Some(custom) = custom_classes {
        builder = builder.custom_classes(custom);
    }

    builder.classes()
}

/// Utility function to get HTML element from hierarchy string
pub fn text_element_from_hierarchy(hierarchy: &str) -> String {
    match hierarchy {
        "title" => "h1".to_string(),
        "heading" => "h2".to_string(),
        "subheading" => "h3".to_string(),
        "h4" => "h4".to_string(),
        "caption" | "overline" => "span".to_string(),
        "code" => "code".to_string(),
        _ => "p".to_string(),
    }
}

/// Utility function to get clamp style from clamp_lines
pub fn text_clamp_style(clamp_lines: Option<u32>) -> String {
    if let Some(lines) = clamp_lines {
        format!(
            "display: -webkit-box; -webkit-line-clamp: {lines}; -webkit-box-orient: vertical; overflow: hidden;"
        )
    } else {
        String::new()
    }
}
