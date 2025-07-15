//! Card styling utilities for the Jupiter Design System
//!
//! Provides a chainable API for building card CSS classes that can be used
//! with any component library or framework.

use crate::core::color::ColorProvider;
use crate::patterns::{CardElevation, CardInteraction, CardSpacing, CardSurface};

/// Card styling utility builder
///
/// This is a pure styling utility that generates CSS classes for cards.
/// It can be used with any component library or framework that supports Tailwind CSS.
///
/// # Examples
///
/// ```rust
/// use jupiter_design_system::builders::card::CardStyles;
/// use jupiter_design_system::themes::VibeColors;
///
/// let classes = CardStyles::new(VibeColors::default())
///     .elevated_surface()
///     .raised_elevation()
///     .comfortable_spacing()
///     .clickable_interaction()
///     .classes();
/// ```
#[derive(Debug, Clone)]
pub struct CardStyles<C: ColorProvider> {
    elevation: CardElevation,
    surface: CardSurface,
    spacing: CardSpacing,
    interaction: CardInteraction,
    selected: bool,
    custom_classes: Vec<String>,
    color_provider: C,
}

impl<C: ColorProvider> CardStyles<C> {
    /// Create a new card styling utility
    pub fn new(color_provider: C) -> Self {
        Self {
            elevation: CardElevation::Subtle,
            surface: CardSurface::Standard,
            spacing: CardSpacing::Standard,
            interaction: CardInteraction::Static,
            selected: false,
            custom_classes: Vec::new(),
            color_provider,
        }
    }

    // === Elevation Methods ===

    /// Set flat elevation (no shadow)
    pub fn flat_elevation(mut self) -> Self {
        self.elevation = CardElevation::Flat;
        self
    }

    /// Set subtle elevation (minimal shadow)
    pub fn subtle_elevation(mut self) -> Self {
        self.elevation = CardElevation::Subtle;
        self
    }

    /// Set raised elevation (standard card shadow)
    pub fn raised_elevation(mut self) -> Self {
        self.elevation = CardElevation::Raised;
        self
    }

    /// Set floating elevation (higher shadow)
    pub fn floating_elevation(mut self) -> Self {
        self.elevation = CardElevation::Floating;
        self
    }

    /// Set modal elevation (highest shadow)
    pub fn modal_elevation(mut self) -> Self {
        self.elevation = CardElevation::Modal;
        self
    }

    /// Set elevation from string (convenience method)
    ///
    /// Maps common string elevations to CardElevation enum.
    /// Supports: "flat", "subtle", "raised", "floating", "modal"
    /// Also supports aliases: "none" -> Flat, "low" -> Subtle, "high" -> Floating
    pub fn elevation_str(mut self, elevation: &str) -> Self {
        self.elevation = match elevation {
            "flat" | "none" => CardElevation::Flat,
            "subtle" | "low" => CardElevation::Subtle,
            "raised" | "standard" => CardElevation::Raised,
            "floating" | "high" => CardElevation::Floating,
            "modal" | "highest" => CardElevation::Modal,
            _ => CardElevation::Subtle, // fallback
        };
        self
    }

    // === Surface Methods ===

    /// Set standard surface (white/light background)
    pub fn standard_surface(mut self) -> Self {
        self.surface = CardSurface::Standard;
        self
    }

    /// Set elevated surface (subtle background tint)
    pub fn elevated_surface(mut self) -> Self {
        self.surface = CardSurface::Elevated;
        self
    }

    /// Set branded surface (theme colors)
    pub fn branded_surface(mut self) -> Self {
        self.surface = CardSurface::Branded;
        self
    }

    /// Set glass morphism surface
    pub fn glass_surface(mut self) -> Self {
        self.surface = CardSurface::Glass;
        self
    }

    /// Set dark surface
    pub fn dark_surface(mut self) -> Self {
        self.surface = CardSurface::Dark;
        self
    }

    /// Set transparent surface
    pub fn transparent_surface(mut self) -> Self {
        self.surface = CardSurface::Transparent;
        self
    }

    /// Set surface from string (convenience method)
    ///
    /// Maps common string surfaces to CardSurface enum.
    /// Supports: "standard", "elevated", "branded", "glass", "dark", "transparent"
    /// Also supports aliases: "white" -> Standard, "theme" -> Branded, "clear" -> Transparent
    pub fn surface_str(mut self, surface: &str) -> Self {
        self.surface = match surface {
            "standard" | "white" => CardSurface::Standard,
            "elevated" => CardSurface::Elevated,
            "branded" | "theme" => CardSurface::Branded,
            "glass" => CardSurface::Glass,
            "dark" => CardSurface::Dark,
            "transparent" | "clear" => CardSurface::Transparent,
            _ => CardSurface::Standard, // fallback
        };
        self
    }

    // === Spacing Methods ===

    /// Set no internal padding
    pub fn no_spacing(mut self) -> Self {
        self.spacing = CardSpacing::None;
        self
    }

    /// Set compact spacing
    pub fn compact_spacing(mut self) -> Self {
        self.spacing = CardSpacing::Compact;
        self
    }

    /// Set standard spacing
    pub fn standard_spacing(mut self) -> Self {
        self.spacing = CardSpacing::Standard;
        self
    }

    /// Set comfortable spacing
    pub fn comfortable_spacing(mut self) -> Self {
        self.spacing = CardSpacing::Comfortable;
        self
    }

    /// Set spacious spacing
    pub fn spacious_spacing(mut self) -> Self {
        self.spacing = CardSpacing::Spacious;
        self
    }

    /// Set spacing from string (convenience method)
    ///
    /// Maps common string spacings to CardSpacing enum.
    /// Supports: "none", "compact", "standard", "comfortable", "spacious"
    /// Also supports aliases: "sm" -> Compact, "md" -> Standard, "lg" -> Comfortable, "xl" -> Spacious
    pub fn spacing_str(mut self, spacing: &str) -> Self {
        self.spacing = match spacing {
            "none" => CardSpacing::None,
            "compact" | "sm" => CardSpacing::Compact,
            "standard" | "md" => CardSpacing::Standard,
            "comfortable" | "lg" => CardSpacing::Comfortable,
            "spacious" | "xl" => CardSpacing::Spacious,
            _ => CardSpacing::Standard, // fallback
        };
        self
    }

    // === Interaction Methods ===

    /// Set static interaction (no interactions)
    pub fn static_interaction(mut self) -> Self {
        self.interaction = CardInteraction::Static;
        self
    }

    /// Set hoverable interaction (subtle hover effects)
    pub fn hoverable_interaction(mut self) -> Self {
        self.interaction = CardInteraction::Hoverable;
        self
    }

    /// Set clickable interaction (button-like behavior)
    pub fn clickable_interaction(mut self) -> Self {
        self.interaction = CardInteraction::Clickable;
        self
    }

    /// Set selectable interaction (can be selected/deselected)
    pub fn selectable_interaction(mut self) -> Self {
        self.interaction = CardInteraction::Selectable;
        self
    }

    /// Set draggable interaction (for reordering)
    pub fn draggable_interaction(mut self) -> Self {
        self.interaction = CardInteraction::Draggable;
        self
    }

    /// Set interaction from string (convenience method)
    ///
    /// Maps common string interactions to CardInteraction enum.
    /// Supports: "static", "hoverable", "clickable", "selectable", "draggable"
    /// Also supports aliases: "none" -> Static, "hover" -> Hoverable, "click" -> Clickable
    pub fn interaction_str(mut self, interaction: &str) -> Self {
        self.interaction = match interaction {
            "static" | "none" => CardInteraction::Static,
            "hoverable" | "hover" => CardInteraction::Hoverable,
            "clickable" | "click" => CardInteraction::Clickable,
            "selectable" | "select" => CardInteraction::Selectable,
            "draggable" | "drag" => CardInteraction::Draggable,
            _ => CardInteraction::Static, // fallback
        };
        self
    }

    // === State Methods ===

    /// Set selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set selected state (shorthand)
    pub fn is_selected(mut self) -> Self {
        self.selected = true;
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

    /// Add a vector of custom CSS classes
    pub fn custom_vec(mut self, classes: Vec<impl Into<String>>) -> Self {
        for class in classes {
            self.custom_classes.push(class.into());
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

        // Base classes
        all_classes.push("rounded-lg border transition-all duration-300".to_string());

        // Elevation classes
        let elevation_classes = match self.elevation {
            CardElevation::Flat => "shadow-none",
            CardElevation::Subtle => "shadow-sm",
            CardElevation::Raised => "shadow-md",
            CardElevation::Floating => "shadow-lg",
            CardElevation::Modal => "shadow-2xl",
        };
        all_classes.push(elevation_classes.to_string());

        // Surface classes
        let surface_classes = self.get_surface_classes();
        if !surface_classes.is_empty() {
            all_classes.push(surface_classes);
        }

        // Spacing classes
        let spacing_classes = match self.spacing {
            CardSpacing::None => "p-0",
            CardSpacing::Compact => "p-3",
            CardSpacing::Standard => "p-5",
            CardSpacing::Comfortable => "p-6",
            CardSpacing::Spacious => "p-8",
        };
        all_classes.push(spacing_classes.to_string());

        // Interaction classes
        let interaction_classes = self.get_interaction_classes();
        if !interaction_classes.is_empty() {
            all_classes.push(interaction_classes);
        }

        // Selection state
        if self.selected {
            all_classes.push("ring-2 ring-offset-2".to_string());
            all_classes.push(format!(
                "ring-{}",
                self.color_provider
                    .resolve_color(crate::core::Color::Primary)
                    .replace("bg-", "")
                    .replace("-500", "-300")
            ));
        }

        // Hover elevation effects for interactive cards
        if matches!(
            self.interaction,
            CardInteraction::Hoverable | CardInteraction::Clickable
        ) {
            match self.elevation {
                CardElevation::Subtle => all_classes.push("hover:shadow-md".to_string()),
                CardElevation::Raised => all_classes.push("hover:shadow-lg".to_string()),
                CardElevation::Floating => all_classes.push("hover:shadow-xl".to_string()),
                _ => {}
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

    /// Get surface-specific classes
    fn get_surface_classes(&self) -> String {
        match self.surface {
            CardSurface::Standard => format!(
                "{} {} {}",
                self.color_provider.bg_class(crate::core::Color::Surface),
                self.color_provider.text_class(crate::core::Color::TextPrimary),
                self.color_provider.border_class(crate::core::Color::Border)
            ),
            CardSurface::Elevated => format!(
                "{} {} {}",
                self.color_provider.bg_class(crate::core::Color::Background),
                self.color_provider.text_class(crate::core::Color::TextPrimary),
                self.color_provider.border_class(crate::core::Color::Border)
            ),
            CardSurface::Branded => {
                "bg-gradient-to-br from-jupiter-navy-900/80 to-jupiter-blue-900/80 border-white/10 text-white".to_string()
            },
            CardSurface::Glass => {
                "bg-white/10 backdrop-blur-md border-white/20 text-white".to_string()
            },
            CardSurface::Dark => {
                "bg-gray-900 border-gray-700 text-white".to_string()
            },
            CardSurface::Transparent => {
                "bg-transparent border-transparent".to_string()
            },
        }
    }

    /// Get interaction-specific classes
    fn get_interaction_classes(&self) -> String {
        match self.interaction {
            CardInteraction::Static => "".to_string(),
            CardInteraction::Hoverable => "hover:scale-101 hover:shadow-sm".to_string(),
            CardInteraction::Clickable => {
                "cursor-pointer hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 focus:ring-offset-2".to_string()
            },
            CardInteraction::Selectable => {
                "cursor-pointer hover:scale-101 focus:outline-none focus:ring-2 focus:ring-offset-2".to_string()
            },
            CardInteraction::Draggable => {
                "cursor-move hover:scale-105 active:scale-95".to_string()
            },
        }
    }
}

/// Convenience function to create card styles
pub fn card_styles<C: ColorProvider>(color_provider: C) -> CardStyles<C> {
    CardStyles::new(color_provider)
}

/// One-shot convenience function to create card classes from strings
///
/// Perfect for component libraries that need to map string props to CSS classes.
///
/// # Examples
///
/// ```rust
/// use jupiter_design_system::builders::card::card_classes_from_strings;
/// use jupiter_design_system::themes::VibeColors;
///
/// let colors = VibeColors::default();
/// let classes = card_classes_from_strings(
///     colors,
///     "elevated",     // surface
///     "raised",       // elevation
///     "comfortable",  // spacing
///     "clickable",    // interaction
///     false,          // selected
/// );
/// ```
pub fn card_classes_from_strings<C: ColorProvider>(
    color_provider: C,
    surface: &str,
    elevation: &str,
    spacing: &str,
    interaction: &str,
    selected: bool,
) -> String {
    let mut builder = CardStyles::new(color_provider)
        .surface_str(surface)
        .elevation_str(elevation)
        .spacing_str(spacing)
        .interaction_str(interaction);

    if selected {
        builder = builder.selected(true);
    }

    builder.classes()
}

#[cfg(test)]
#[path = "card_test.rs"]
mod card_test;
