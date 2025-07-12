//! Comprehensive Card Pattern
//!
//! This demonstrates how abstract design concepts compose together to create
//! a complete "card" experience - containers that group related content with
//! appropriate elevation, interactivity, and visual hierarchy.

use crate::core::color::ColorProvider;
use crate::patterns::{FocusManagement, InteractiveElement};
use serde::{Deserialize, Serialize};

/// Card elevation levels representing visual hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardElevation {
    /// Flat - no shadow, minimal elevation
    Flat,
    /// Subtle - minimal shadow for slight separation
    Subtle,
    /// Raised - standard elevation for cards
    Raised,
    /// Floating - higher elevation for overlays
    Floating,
    /// Modal - highest elevation for modals/dialogs
    Modal,
}

/// Card surface variants representing different visual treatments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardSurface {
    /// Standard white/light surface
    Standard,
    /// Elevated surface with subtle background
    Elevated,
    /// Branded surface using theme colors
    Branded,
    /// Glass morphism effect
    Glass,
    /// Dark surface for dark themes
    Dark,
    /// Transparent surface
    Transparent,
}

/// Card layout spacing for consistent internal padding
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardSpacing {
    /// No internal padding
    None,
    /// Compact spacing for dense layouts
    Compact,
    /// Standard spacing for most cards
    Standard,
    /// Comfortable spacing for content-heavy cards
    Comfortable,
    /// Spacious spacing for minimal content
    Spacious,
}

/// Card interaction patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardInteraction {
    /// Static card with no interactions
    Static,
    /// Hoverable card with subtle effects
    Hoverable,
    /// Clickable card that acts like a button
    Clickable,
    /// Selectable card that can be selected/deselected
    Selectable,
    /// Draggable card for reordering
    Draggable,
}

/// Complete card pattern combining all abstract concepts
///
/// This represents the full abstract concept of a "card" - a container that
/// groups related content with appropriate visual hierarchy, interactivity,
/// and accessibility features.
///
/// # Examples
///
/// ```rust
/// use jupiter_design_system::patterns::CardPattern;
/// use jupiter_design_system::core::color::WaterWellnessColors;
///
/// // Basic content card
/// let content_card = CardPattern::new(WaterWellnessColors::default())
///     .standard_surface()
///     .raised_elevation()
///     .standard_spacing()
///     .hoverable_interaction()
///     .classes();
///
/// // Interactive clickable card
/// let clickable_card = CardPattern::new(WaterWellnessColors::default())
///     .elevated_surface()
///     .floating_elevation()
///     .clickable_interaction()
///     .comfortable_spacing()
///     .classes();
///
/// // Branded hero card
/// let hero_card = CardPattern::new(WaterWellnessColors::default())
///     .branded_surface()
///     .modal_elevation()
///     .spacious_spacing()
///     .static_interaction()
///     .classes();
///
/// // Glass morphism card
/// let glass_card = CardPattern::new(WaterWellnessColors::default())
///     .glass_surface()
///     .floating_elevation()
///     .hoverable_interaction()
///     .classes();
/// ```
#[derive(Debug, Clone)]
pub struct CardPattern<C: ColorProvider + Clone> {
    // Core properties
    elevation: CardElevation,
    surface: CardSurface,
    spacing: CardSpacing,
    interaction: CardInteraction,

    // State
    selected: bool,

    // Abstract patterns
    interactive_element: InteractiveElement<C>,
    focus_management: FocusManagement<C>,

    // Custom overrides
    custom_classes: Vec<String>,
    color_provider: C,
}

impl<C: ColorProvider + Clone> CardPattern<C> {
    /// Create a new card pattern with sensible defaults
    pub fn new(color_provider: C) -> Self {
        Self {
            elevation: CardElevation::Subtle,
            surface: CardSurface::Standard,
            spacing: CardSpacing::Standard,
            interaction: CardInteraction::Static,
            selected: false,

            interactive_element: InteractiveElement::new(color_provider.clone()),
            focus_management: FocusManagement::new(color_provider.clone()),

            custom_classes: Vec::new(),
            color_provider,
        }
    }

    // === Elevation Methods ===

    /// Flat elevation (no shadow)
    pub fn flat_elevation(mut self) -> Self {
        self.elevation = CardElevation::Flat;
        self
    }

    /// Subtle elevation (minimal shadow)
    pub fn subtle_elevation(mut self) -> Self {
        self.elevation = CardElevation::Subtle;
        self
    }

    /// Raised elevation (standard card shadow)
    pub fn raised_elevation(mut self) -> Self {
        self.elevation = CardElevation::Raised;
        self
    }

    /// Floating elevation (higher shadow)
    pub fn floating_elevation(mut self) -> Self {
        self.elevation = CardElevation::Floating;
        self
    }

    /// Modal elevation (highest shadow)
    pub fn modal_elevation(mut self) -> Self {
        self.elevation = CardElevation::Modal;
        self
    }

    // === Surface Methods ===

    /// Standard surface (white/light background)
    pub fn standard_surface(mut self) -> Self {
        self.surface = CardSurface::Standard;
        self
    }

    /// Elevated surface (subtle background tint)
    pub fn elevated_surface(mut self) -> Self {
        self.surface = CardSurface::Elevated;
        self
    }

    /// Branded surface (theme colors)
    pub fn branded_surface(mut self) -> Self {
        self.surface = CardSurface::Branded;
        self
    }

    /// Glass morphism surface
    pub fn glass_surface(mut self) -> Self {
        self.surface = CardSurface::Glass;
        self
    }

    /// Dark surface
    pub fn dark_surface(mut self) -> Self {
        self.surface = CardSurface::Dark;
        self
    }

    /// Transparent surface
    pub fn transparent_surface(mut self) -> Self {
        self.surface = CardSurface::Transparent;
        self
    }

    // === Spacing Methods ===

    /// No internal padding
    pub fn no_spacing(mut self) -> Self {
        self.spacing = CardSpacing::None;
        self
    }

    /// Compact spacing
    pub fn compact_spacing(mut self) -> Self {
        self.spacing = CardSpacing::Compact;
        self
    }

    /// Standard spacing
    pub fn standard_spacing(mut self) -> Self {
        self.spacing = CardSpacing::Standard;
        self
    }

    /// Comfortable spacing
    pub fn comfortable_spacing(mut self) -> Self {
        self.spacing = CardSpacing::Comfortable;
        self
    }

    /// Spacious spacing
    pub fn spacious_spacing(mut self) -> Self {
        self.spacing = CardSpacing::Spacious;
        self
    }

    // === Interaction Methods ===

    /// Static card (no interactions)
    pub fn static_interaction(mut self) -> Self {
        self.interaction = CardInteraction::Static;
        self.interactive_element = InteractiveElement::new(self.color_provider.clone());
        self
    }

    /// Hoverable card (subtle hover effects)
    pub fn hoverable_interaction(mut self) -> Self {
        self.interaction = CardInteraction::Hoverable;
        self.interactive_element = InteractiveElement::new(self.color_provider.clone())
            .hoverable()
            .gentle_interaction();
        self
    }

    /// Clickable card (button-like behavior)
    pub fn clickable_interaction(mut self) -> Self {
        self.interaction = CardInteraction::Clickable;
        self.interactive_element = InteractiveElement::new(self.color_provider.clone())
            .hoverable()
            .focusable()
            .pressable()
            .standard_interaction();
        self.focus_management = self.focus_management.button();
        self
    }

    /// Selectable card (can be selected/deselected)
    pub fn selectable_interaction(mut self) -> Self {
        self.interaction = CardInteraction::Selectable;
        self.interactive_element = InteractiveElement::new(self.color_provider.clone())
            .hoverable()
            .focusable()
            .pressable()
            .gentle_interaction();
        self.focus_management = self.focus_management.toggle();
        self
    }

    /// Draggable card (for reordering)
    pub fn draggable_interaction(mut self) -> Self {
        self.interaction = CardInteraction::Draggable;
        self.interactive_element = InteractiveElement::new(self.color_provider.clone())
            .hoverable()
            .focusable()
            .standard_interaction();
        self
    }

    // === State Methods ===

    /// Set selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set hover state
    pub fn hover(mut self) -> Self {
        self.interactive_element = self.interactive_element.hover();
        self
    }

    /// Set active state
    pub fn active(mut self) -> Self {
        self.interactive_element = self.interactive_element.active();
        self
    }

    /// Set focused state
    pub fn focused(mut self) -> Self {
        self.interactive_element = self.interactive_element.focused();
        self
    }

    // === Custom Methods ===

    /// Add custom classes
    pub fn custom(mut self, classes: impl Into<String>) -> Self {
        self.custom_classes.push(classes.into());
        self
    }

    // === Build Methods ===

    /// Build complete card classes
    pub fn classes(self) -> String {
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

        // Interactive classes
        let interactive_classes = self.interactive_element.classes();
        if !interactive_classes.is_empty() {
            all_classes.push(interactive_classes);
        }

        // Focus classes
        let focus_classes = self.focus_management.classes();
        if !focus_classes.is_empty() {
            all_classes.push(focus_classes);
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
        all_classes.extend(self.custom_classes);

        // Remove duplicates and join
        let mut classes: Vec<String> = all_classes
            .join(" ")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        classes.dedup();
        classes.join(" ")
    }

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
            CardSurface::Branded => "bg-gradient-to-br from-water-navy-900/80 to-water-blue-900/80 border-white/10 text-white".to_string(),
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

    /// Get accessibility attributes
    pub fn accessibility_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = self.focus_management.data_attributes();

        // Add card-specific attributes
        if self.selected {
            attrs.push(("aria-selected", "true".to_string()));
        }

        // Add role based on interaction type
        match self.interaction {
            CardInteraction::Clickable => {
                attrs.push(("role", "button".to_string()));
            }
            CardInteraction::Selectable => {
                attrs.push(("role", "option".to_string()));
            }
            _ => {}
        }

        attrs
    }

    /// Get semantic information about this card
    pub fn semantic_info(&self) -> CardSemanticInfo {
        CardSemanticInfo {
            elevation: self.elevation,
            surface: self.surface,
            spacing: self.spacing,
            interaction: self.interaction,
            is_selected: self.selected,
            is_interactive: !matches!(self.interaction, CardInteraction::Static),
        }
    }
}

/// Semantic information about a card pattern
#[derive(Debug, Clone)]
pub struct CardSemanticInfo {
    pub elevation: CardElevation,
    pub surface: CardSurface,
    pub spacing: CardSpacing,
    pub interaction: CardInteraction,
    pub is_selected: bool,
    pub is_interactive: bool,
}

// === Convenience Functions ===

/// Create a card pattern
pub fn card_pattern<C: ColorProvider + Clone>(color_provider: C) -> CardPattern<C> {
    CardPattern::new(color_provider)
}

/// Create a standard content card
pub fn content_card<C: ColorProvider + Clone>(color_provider: C) -> CardPattern<C> {
    CardPattern::new(color_provider)
        .standard_surface()
        .raised_elevation()
        .standard_spacing()
        .static_interaction()
}

/// Create an interactive card
pub fn interactive_card<C: ColorProvider + Clone>(color_provider: C) -> CardPattern<C> {
    CardPattern::new(color_provider)
        .elevated_surface()
        .floating_elevation()
        .clickable_interaction()
        .comfortable_spacing()
}

/// Create a hero/featured card
pub fn hero_card<C: ColorProvider + Clone>(color_provider: C) -> CardPattern<C> {
    CardPattern::new(color_provider)
        .branded_surface()
        .modal_elevation()
        .spacious_spacing()
        .hoverable_interaction()
}

/// Create a glass morphism card
pub fn glass_card<C: ColorProvider + Clone>(color_provider: C) -> CardPattern<C> {
    CardPattern::new(color_provider)
        .glass_surface()
        .floating_elevation()
        .hoverable_interaction()
        .standard_spacing()
}

/// Create a minimal card
pub fn minimal_card<C: ColorProvider + Clone>(color_provider: C) -> CardPattern<C> {
    CardPattern::new(color_provider)
        .standard_surface()
        .flat_elevation()
        .compact_spacing()
        .static_interaction()
}
