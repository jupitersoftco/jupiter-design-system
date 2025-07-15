//! Typography patterns for Jupiter Design System
//!
//! This module provides abstract typography patterns that can be used across different
//! component implementations. These patterns define semantic text hierarchy, sizing systems,
//! weight progression, and color semantics for consistent typography.

use crate::core::color::ColorProvider;

/// Typography hierarchy levels following semantic design principles
#[derive(Debug, Clone, PartialEq)]
pub enum TypographyHierarchy {
    /// Main page title (h1 equivalent)
    Title,
    /// Section heading (h2 equivalent)
    Heading,
    /// Sub-section heading (h3 equivalent)
    Subheading,
    /// Minor heading (h4 equivalent)
    H4,
    /// Regular body text (p equivalent)
    Body,
    /// Larger body text for emphasis
    BodyLarge,
    /// Smaller body text for details
    BodySmall,
    /// Caption text for images/descriptions
    Caption,
    /// Overline text for categories/labels
    Overline,
    /// Code/monospace text
    Code,
}

/// Typography size system following design scale principles
#[derive(Debug, Clone, PartialEq)]
pub enum TypographySize {
    /// Extra small (0.75rem)
    XS,
    /// Small (0.875rem)
    SM,
    /// Medium/base (1rem)
    MD,
    /// Large (1.125rem)
    LG,
    /// Extra large (1.25rem)
    XL,
    /// 2X large (1.5rem)
    XL2,
    /// 3X large (1.875rem)
    XL3,
    /// 4X large (2.25rem)
    XL4,
}

/// Typography weight system following font weight progression
#[derive(Debug, Clone, PartialEq)]
pub enum TypographyWeight {
    /// Light (300)
    Light,
    /// Normal/Regular (400)
    Normal,
    /// Medium (500)
    Medium,
    /// Semibold (600)
    Semibold,
    /// Bold (700)
    Bold,
    /// Extra bold (800)
    ExtraBold,
}

/// Typography color semantics for consistent meaning
#[derive(Debug, Clone, PartialEq)]
pub enum TypographyColor {
    /// Primary brand color
    Primary,
    /// Secondary brand color
    Secondary,
    /// Accent color
    Accent,
    /// Muted/subdued text
    Muted,
    /// Disabled text
    Disabled,
    /// White text
    White,
    /// Black text
    Black,
    /// Success state
    Success,
    /// Warning state
    Warning,
    /// Error state
    Error,
    /// Info state
    Info,
    /// Auto-selected based on hierarchy
    Auto,
}

/// Text alignment options
#[derive(Debug, Clone, PartialEq)]
pub enum TypographyAlignment {
    /// Left aligned
    Left,
    /// Center aligned
    Center,
    /// Right aligned
    Right,
    /// Justified
    Justify,
}

/// Text overflow behavior
#[derive(Debug, Clone, PartialEq)]
pub enum TypographyOverflow {
    /// No overflow handling
    Normal,
    /// Truncate with ellipsis
    Truncate,
    /// Clamp to specific number of lines
    Clamp(u32),
}

/// HTML element semantics for accessibility
#[derive(Debug, Clone, PartialEq)]
pub enum TypographyElement {
    /// Auto-select based on hierarchy
    Auto,
    /// Heading level 1
    H1,
    /// Heading level 2
    H2,
    /// Heading level 3
    H3,
    /// Heading level 4
    H4,
    /// Heading level 5
    H5,
    /// Heading level 6
    H6,
    /// Paragraph
    P,
    /// Inline span
    Span,
    /// Generic div
    Div,
}

/// Typography pattern configuration
#[derive(Debug, Clone)]
pub struct TypographyPattern<T: ColorProvider> {
    pub hierarchy: TypographyHierarchy,
    pub size: Option<TypographySize>,
    pub weight: Option<TypographyWeight>,
    pub color: TypographyColor,
    pub alignment: Option<TypographyAlignment>,
    pub overflow: TypographyOverflow,
    pub element: TypographyElement,
    pub color_provider: T,
}

impl<T: ColorProvider> TypographyPattern<T> {
    /// Create a new typography pattern with default values
    pub fn new(color_provider: T) -> Self {
        Self {
            hierarchy: TypographyHierarchy::Body,
            size: None,
            weight: None,
            color: TypographyColor::Auto,
            alignment: None,
            overflow: TypographyOverflow::Normal,
            element: TypographyElement::Auto,
            color_provider,
        }
    }

    /// Set typography hierarchy
    pub fn hierarchy(mut self, hierarchy: TypographyHierarchy) -> Self {
        self.hierarchy = hierarchy;
        self
    }

    /// Set typography size (overrides hierarchy default)
    pub fn size(mut self, size: TypographySize) -> Self {
        self.size = Some(size);
        self
    }

    /// Set typography weight (overrides hierarchy default)
    pub fn weight(mut self, weight: TypographyWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Set typography color
    pub fn color(mut self, color: TypographyColor) -> Self {
        self.color = color;
        self
    }

    /// Set text alignment
    pub fn alignment(mut self, alignment: TypographyAlignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    /// Set overflow behavior
    pub fn overflow(mut self, overflow: TypographyOverflow) -> Self {
        self.overflow = overflow;
        self
    }

    /// Set HTML element
    pub fn element(mut self, element: TypographyElement) -> Self {
        self.element = element;
        self
    }

    /// Generate CSS classes for this typography pattern
    pub fn classes(&self) -> String {
        let mut classes = vec![];

        // Base typography classes
        classes.push("leading-relaxed".to_string());

        // Hierarchy-based classes (size, weight, tracking)
        let hierarchy_classes = self.get_hierarchy_classes();
        if !hierarchy_classes.is_empty() {
            classes.push(hierarchy_classes);
        }

        // Size override
        if let Some(size) = &self.size {
            classes.push(self.get_size_classes(size));
        }

        // Weight override
        if let Some(weight) = &self.weight {
            classes.push(self.get_weight_classes(weight));
        }

        // Color classes
        let color_classes = self.get_color_classes();
        if !color_classes.is_empty() {
            classes.push(color_classes);
        }

        // Alignment classes
        if let Some(alignment) = &self.alignment {
            classes.push(self.get_alignment_classes(alignment));
        }

        // Overflow classes
        let overflow_classes = self.get_overflow_classes();
        if !overflow_classes.is_empty() {
            classes.push(overflow_classes);
        }

        // Join and deduplicate classes
        let mut all_classes: Vec<String> = classes
            .join(" ")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        all_classes.sort();
        all_classes.dedup();
        all_classes.join(" ")
    }

    /// Get CSS classes for hierarchy
    fn get_hierarchy_classes(&self) -> String {
        match self.hierarchy {
            TypographyHierarchy::Title => {
                let mut classes = vec!["tracking-tight"];
                if self.size.is_none() {
                    classes.push("text-4xl");
                }
                if self.weight.is_none() {
                    classes.push("font-bold");
                }
                classes.join(" ")
            }
            TypographyHierarchy::Heading => {
                let mut classes = vec!["tracking-tight"];
                if self.size.is_none() {
                    classes.push("text-3xl");
                }
                if self.weight.is_none() {
                    classes.push("font-bold");
                }
                classes.join(" ")
            }
            TypographyHierarchy::Subheading => {
                let mut classes = vec!["tracking-tight"];
                if self.size.is_none() {
                    classes.push("text-2xl");
                }
                if self.weight.is_none() {
                    classes.push("font-bold");
                }
                classes.join(" ")
            }
            TypographyHierarchy::H4 => {
                let mut classes = vec!["tracking-tight"];
                if self.size.is_none() {
                    classes.push("text-xl");
                }
                if self.weight.is_none() {
                    classes.push("font-bold");
                }
                classes.join(" ")
            }
            TypographyHierarchy::Body => {
                let mut classes = vec![];
                if self.size.is_none() {
                    classes.push("text-base");
                }
                if self.weight.is_none() {
                    classes.push("font-normal");
                }
                classes.join(" ")
            }
            TypographyHierarchy::BodyLarge => {
                let mut classes = vec![];
                if self.size.is_none() {
                    classes.push("text-lg");
                }
                if self.weight.is_none() {
                    classes.push("font-normal");
                }
                classes.join(" ")
            }
            TypographyHierarchy::BodySmall => {
                let mut classes = vec![];
                if self.size.is_none() {
                    classes.push("text-sm");
                }
                if self.weight.is_none() {
                    classes.push("font-normal");
                }
                classes.join(" ")
            }
            TypographyHierarchy::Caption => {
                let mut classes = vec![];
                if self.size.is_none() {
                    classes.push("text-sm");
                }
                if self.weight.is_none() {
                    classes.push("font-medium");
                }
                classes.join(" ")
            }
            TypographyHierarchy::Overline => {
                let mut classes = vec!["uppercase", "tracking-wider"];
                if self.size.is_none() {
                    classes.push("text-xs");
                }
                if self.weight.is_none() {
                    classes.push("font-medium");
                }
                classes.join(" ")
            }
            TypographyHierarchy::Code => {
                let mut classes = vec!["font-mono", "bg-gray-100", "px-1", "py-0.5", "rounded"];
                if self.size.is_none() {
                    classes.push("text-sm");
                }
                classes.join(" ")
            }
        }
    }

    /// Get CSS classes for size
    fn get_size_classes(&self, size: &TypographySize) -> String {
        match size {
            TypographySize::XS => "text-xs",
            TypographySize::SM => "text-sm",
            TypographySize::MD => "text-base",
            TypographySize::LG => "text-lg",
            TypographySize::XL => "text-xl",
            TypographySize::XL2 => "text-2xl",
            TypographySize::XL3 => "text-3xl",
            TypographySize::XL4 => "text-4xl",
        }
        .to_string()
    }

    /// Get CSS classes for weight
    fn get_weight_classes(&self, weight: &TypographyWeight) -> String {
        match weight {
            TypographyWeight::Light => "font-light",
            TypographyWeight::Normal => "font-normal",
            TypographyWeight::Medium => "font-medium",
            TypographyWeight::Semibold => "font-semibold",
            TypographyWeight::Bold => "font-bold",
            TypographyWeight::ExtraBold => "font-extrabold",
        }
        .to_string()
    }

    /// Get CSS classes for color
    fn get_color_classes(&self) -> String {
        use crate::core::Color;
        match self.color {
            TypographyColor::Primary => self.color_provider.text_class(Color::Primary),
            TypographyColor::Secondary => self.color_provider.text_class(Color::Secondary),
            TypographyColor::Accent => self.color_provider.text_class(Color::Accent),
            TypographyColor::Muted => self.color_provider.text_class(Color::TextSecondary),
            TypographyColor::Disabled => self.color_provider.text_class(Color::InteractiveDisabled),
            TypographyColor::White => self.color_provider.text_class(Color::TextInverse),
            TypographyColor::Black => self.color_provider.text_class(Color::Foreground),
            TypographyColor::Success => self.color_provider.text_class(Color::Success),
            TypographyColor::Warning => self.color_provider.text_class(Color::Warning),
            TypographyColor::Error => self.color_provider.text_class(Color::Error),
            TypographyColor::Info => self.color_provider.text_class(Color::Info),
            TypographyColor::Auto => {
                // Auto-select color based on hierarchy
                match self.hierarchy {
                    TypographyHierarchy::Title
                    | TypographyHierarchy::Heading
                    | TypographyHierarchy::Subheading
                    | TypographyHierarchy::H4
                    | TypographyHierarchy::Body
                    | TypographyHierarchy::BodyLarge
                    | TypographyHierarchy::BodySmall => {
                        self.color_provider.text_class(Color::TextPrimary)
                    }
                    TypographyHierarchy::Caption => {
                        self.color_provider.text_class(Color::TextSecondary)
                    }
                    TypographyHierarchy::Overline => {
                        self.color_provider.text_class(Color::TextTertiary)
                    }
                    TypographyHierarchy::Code => self.color_provider.text_class(Color::TextPrimary),
                }
            }
        }
    }

    /// Get CSS classes for alignment
    fn get_alignment_classes(&self, alignment: &TypographyAlignment) -> String {
        match alignment {
            TypographyAlignment::Left => "text-left",
            TypographyAlignment::Center => "text-center",
            TypographyAlignment::Right => "text-right",
            TypographyAlignment::Justify => "text-justify",
        }
        .to_string()
    }

    /// Get CSS classes for overflow
    fn get_overflow_classes(&self) -> String {
        match self.overflow {
            TypographyOverflow::Normal => String::new(),
            TypographyOverflow::Truncate => "truncate".to_string(),
            TypographyOverflow::Clamp(_) => {
                // Line clamping requires CSS style, not class
                // This will be handled by the component implementation
                String::new()
            }
        }
    }

    /// Get appropriate HTML element based on hierarchy
    pub fn get_element(&self) -> String {
        match self.element {
            TypographyElement::Auto => {
                // Auto-select element based on hierarchy
                match self.hierarchy {
                    TypographyHierarchy::Title => "h1".to_string(),
                    TypographyHierarchy::Heading => "h2".to_string(),
                    TypographyHierarchy::Subheading => "h3".to_string(),
                    TypographyHierarchy::H4 => "h4".to_string(),
                    TypographyHierarchy::Caption | TypographyHierarchy::Overline => {
                        "span".to_string()
                    }
                    TypographyHierarchy::Code => "code".to_string(),
                    _ => "p".to_string(),
                }
            }
            TypographyElement::H1 => "h1".to_string(),
            TypographyElement::H2 => "h2".to_string(),
            TypographyElement::H3 => "h3".to_string(),
            TypographyElement::H4 => "h4".to_string(),
            TypographyElement::H5 => "h5".to_string(),
            TypographyElement::H6 => "h6".to_string(),
            TypographyElement::P => "p".to_string(),
            TypographyElement::Span => "span".to_string(),
            TypographyElement::Div => "div".to_string(),
        }
    }

    /// Get CSS style for line clamping (if overflow is Clamp)
    pub fn get_clamp_style(&self) -> String {
        match self.overflow {
            TypographyOverflow::Clamp(lines) => {
                format!(
                    "display: -webkit-box; -webkit-line-clamp: {lines}; -webkit-box-orient: vertical; overflow: hidden;"
                )
            }
            _ => String::new(),
        }
    }
}

/// Convenience function to create a typography pattern
pub fn typography_pattern<T: ColorProvider>(color_provider: T) -> TypographyPattern<T> {
    TypographyPattern::new(color_provider)
}

/// Convenience function to create a title typography pattern
pub fn title_typography<T: ColorProvider>(color_provider: T) -> TypographyPattern<T> {
    TypographyPattern::new(color_provider).hierarchy(TypographyHierarchy::Title)
}

/// Convenience function to create a heading typography pattern
pub fn heading_typography<T: ColorProvider>(color_provider: T) -> TypographyPattern<T> {
    TypographyPattern::new(color_provider).hierarchy(TypographyHierarchy::Heading)
}

/// Convenience function to create a body typography pattern
pub fn body_typography<T: ColorProvider>(color_provider: T) -> TypographyPattern<T> {
    TypographyPattern::new(color_provider).hierarchy(TypographyHierarchy::Body)
}

/// Convenience function to create a caption typography pattern
pub fn caption_typography<T: ColorProvider>(color_provider: T) -> TypographyPattern<T> {
    TypographyPattern::new(color_provider).hierarchy(TypographyHierarchy::Caption)
}

/// Convenience function to create a code typography pattern
pub fn code_typography<T: ColorProvider>(color_provider: T) -> TypographyPattern<T> {
    TypographyPattern::new(color_provider).hierarchy(TypographyHierarchy::Code)
}
