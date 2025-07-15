//! Tests for the card styling utilities

#[cfg(test)]
mod tests {
    use crate::builders::card::{card_classes_from_strings, card_styles, CardStyles};
    use crate::core::color::VibeColors;

    fn create_test_colors() -> VibeColors {
        VibeColors::new()
    }

    #[test]
    fn test_card_styles_default() {
        let colors = create_test_colors();
        let classes = CardStyles::new(colors).classes();

        assert!(classes.contains("rounded-lg"));
        assert!(classes.contains("border"));
        assert!(classes.contains("transition-all"));
        assert!(classes.contains("duration-300"));
        assert!(classes.contains("shadow-sm")); // subtle elevation
        assert!(classes.contains("p-5")); // standard spacing
        assert!(classes.contains("bg-white")); // standard surface
    }

    #[test]
    fn test_card_styles_chainable_api() {
        let colors = create_test_colors();
        let classes = CardStyles::new(colors)
            .raised_elevation()
            .elevated_surface()
            .comfortable_spacing()
            .clickable_interaction()
            .is_selected()
            .classes();

        assert!(classes.contains("shadow-md")); // raised elevation
        assert!(classes.contains("p-6")); // comfortable spacing
        assert!(classes.contains("cursor-pointer")); // clickable interaction
        assert!(classes.contains("ring-2")); // selected state
    }

    #[test]
    fn test_card_elevations() {
        let colors = create_test_colors();

        // Flat
        let flat = CardStyles::new(colors.clone()).flat_elevation().classes();
        assert!(flat.contains("shadow-none"));

        // Subtle
        let subtle = CardStyles::new(colors.clone()).subtle_elevation().classes();
        assert!(subtle.contains("shadow-sm"));

        // Raised
        let raised = CardStyles::new(colors.clone()).raised_elevation().classes();
        assert!(raised.contains("shadow-md"));

        // Floating
        let floating = CardStyles::new(colors.clone())
            .floating_elevation()
            .classes();
        assert!(floating.contains("shadow-lg"));

        // Modal
        let modal = CardStyles::new(colors.clone()).modal_elevation().classes();
        assert!(modal.contains("shadow-2xl"));
    }

    #[test]
    fn test_card_surfaces() {
        let colors = create_test_colors();

        // Standard
        let standard = CardStyles::new(colors.clone()).standard_surface().classes();
        assert!(standard.contains("bg-white"));
        assert!(standard.contains("text-gray-900"));
        assert!(standard.contains("border-gray-200"));

        // Elevated
        let elevated = CardStyles::new(colors.clone()).elevated_surface().classes();
        assert!(elevated.contains("bg-gray-50"));
        assert!(elevated.contains("text-gray-900"));

        // Branded
        let branded = CardStyles::new(colors.clone()).branded_surface().classes();
        assert!(branded.contains("bg-gradient-to-br"));
        assert!(branded.contains("from-jupiter-navy-900/80"));
        assert!(branded.contains("text-white"));

        // Glass
        let glass = CardStyles::new(colors.clone()).glass_surface().classes();
        assert!(glass.contains("bg-white/10"));
        assert!(glass.contains("backdrop-blur-md"));
        assert!(glass.contains("border-white/20"));

        // Dark
        let dark = CardStyles::new(colors.clone()).dark_surface().classes();
        assert!(dark.contains("bg-gray-900"));
        assert!(dark.contains("border-gray-700"));
        assert!(dark.contains("text-white"));

        // Transparent
        let transparent = CardStyles::new(colors.clone())
            .transparent_surface()
            .classes();
        assert!(transparent.contains("bg-transparent"));
        assert!(transparent.contains("border-transparent"));
    }

    #[test]
    fn test_card_spacings() {
        let colors = create_test_colors();

        // None
        let none = CardStyles::new(colors.clone()).no_spacing().classes();
        assert!(none.contains("p-0"));

        // Compact
        let compact = CardStyles::new(colors.clone()).compact_spacing().classes();
        assert!(compact.contains("p-3"));

        // Standard
        let standard = CardStyles::new(colors.clone()).standard_spacing().classes();
        assert!(standard.contains("p-5"));

        // Comfortable
        let comfortable = CardStyles::new(colors.clone())
            .comfortable_spacing()
            .classes();
        assert!(comfortable.contains("p-6"));

        // Spacious
        let spacious = CardStyles::new(colors.clone()).spacious_spacing().classes();
        assert!(spacious.contains("p-8"));
    }

    #[test]
    fn test_card_interactions() {
        let colors = create_test_colors();

        // Static
        let static_card = CardStyles::new(colors.clone())
            .static_interaction()
            .classes();
        assert!(!static_card.contains("cursor-pointer"));
        assert!(!static_card.contains("hover:scale"));

        // Hoverable
        let hoverable = CardStyles::new(colors.clone())
            .hoverable_interaction()
            .classes();
        assert!(hoverable.contains("hover:scale-101"));

        // Clickable
        let clickable = CardStyles::new(colors.clone())
            .clickable_interaction()
            .classes();
        assert!(clickable.contains("cursor-pointer"));
        assert!(clickable.contains("hover:scale-105"));
        assert!(clickable.contains("active:scale-95"));

        // Selectable
        let selectable = CardStyles::new(colors.clone())
            .selectable_interaction()
            .classes();
        assert!(selectable.contains("cursor-pointer"));
        assert!(selectable.contains("hover:scale-101"));

        // Draggable
        let draggable = CardStyles::new(colors.clone())
            .draggable_interaction()
            .classes();
        assert!(draggable.contains("cursor-move"));
        assert!(draggable.contains("hover:scale-105"));
    }

    #[test]
    fn test_card_selected_state() {
        let colors = create_test_colors();

        // Not selected
        let not_selected = CardStyles::new(colors.clone()).selected(false).classes();
        assert!(!not_selected.contains("ring-2"));

        // Selected
        let selected = CardStyles::new(colors.clone()).selected(true).classes();
        assert!(selected.contains("ring-2"));
        assert!(selected.contains("ring-offset-2"));
        assert!(selected.contains("ring-jupiter-blue-300"));

        // Using shorthand
        let shorthand = CardStyles::new(colors.clone()).is_selected().classes();
        assert!(shorthand.contains("ring-2"));
    }

    #[test]
    fn test_card_hover_elevation_effects() {
        let colors = create_test_colors();

        // Hoverable card with subtle elevation should get hover:shadow-md
        let hoverable_subtle = CardStyles::new(colors.clone())
            .subtle_elevation()
            .hoverable_interaction()
            .classes();
        assert!(hoverable_subtle.contains("hover:shadow-md"));

        // Clickable card with raised elevation should get hover:shadow-lg
        let clickable_raised = CardStyles::new(colors.clone())
            .raised_elevation()
            .clickable_interaction()
            .classes();
        assert!(clickable_raised.contains("hover:shadow-lg"));

        // Static card should not get hover effects
        let static_raised = CardStyles::new(colors.clone())
            .raised_elevation()
            .static_interaction()
            .classes();
        assert!(!static_raised.contains("hover:shadow-lg"));
    }

    #[test]
    fn test_card_custom_classes() {
        let colors = create_test_colors();

        // Single custom class
        let single = CardStyles::new(colors.clone())
            .custom("animate-pulse")
            .classes();
        assert!(single.contains("animate-pulse"));

        // Multiple custom classes
        let multiple = CardStyles::new(colors.clone())
            .custom("animate-pulse")
            .custom("border-dashed")
            .classes();
        assert!(multiple.contains("animate-pulse"));
        assert!(multiple.contains("border-dashed"));

        // Custom classes string
        let string = CardStyles::new(colors.clone())
            .custom_classes("animate-pulse border-dashed transform")
            .classes();
        assert!(string.contains("animate-pulse"));
        assert!(string.contains("border-dashed"));
        assert!(string.contains("transform"));

        // Custom classes vec
        let vec = CardStyles::new(colors.clone())
            .custom_vec(vec!["animate-pulse", "border-dashed"])
            .classes();
        assert!(vec.contains("animate-pulse"));
        assert!(vec.contains("border-dashed"));
    }

    #[test]
    fn test_card_string_methods() {
        let colors = create_test_colors();

        // Elevation from string
        let elevation = CardStyles::new(colors.clone())
            .elevation_str("raised")
            .classes();
        assert!(elevation.contains("shadow-md"));

        // Surface from string
        let surface = CardStyles::new(colors.clone())
            .surface_str("branded")
            .classes();
        assert!(surface.contains("bg-gradient-to-br"));

        // Spacing from string
        let spacing = CardStyles::new(colors.clone())
            .spacing_str("comfortable")
            .classes();
        assert!(spacing.contains("p-6"));

        // Interaction from string
        let interaction = CardStyles::new(colors.clone())
            .interaction_str("clickable")
            .classes();
        assert!(interaction.contains("cursor-pointer"));
    }

    #[test]
    fn test_card_string_aliases() {
        let colors = create_test_colors();

        // Elevation aliases
        let none_alias = CardStyles::new(colors.clone())
            .elevation_str("none")
            .classes();
        assert!(none_alias.contains("shadow-none"));

        let high_alias = CardStyles::new(colors.clone())
            .elevation_str("high")
            .classes();
        assert!(high_alias.contains("shadow-lg"));

        // Surface aliases
        let white_alias = CardStyles::new(colors.clone())
            .surface_str("white")
            .classes();
        assert!(white_alias.contains("bg-white"));

        let theme_alias = CardStyles::new(colors.clone())
            .surface_str("theme")
            .classes();
        assert!(theme_alias.contains("bg-gradient-to-br"));

        // Spacing aliases
        let sm_alias = CardStyles::new(colors.clone()).spacing_str("sm").classes();
        assert!(sm_alias.contains("p-3"));

        let lg_alias = CardStyles::new(colors.clone()).spacing_str("lg").classes();
        assert!(lg_alias.contains("p-6"));

        // Interaction aliases
        let hover_alias = CardStyles::new(colors.clone())
            .interaction_str("hover")
            .classes();
        assert!(hover_alias.contains("hover:scale-101"));

        let click_alias = CardStyles::new(colors.clone())
            .interaction_str("click")
            .classes();
        assert!(click_alias.contains("cursor-pointer"));
    }

    #[test]
    fn test_card_string_fallbacks() {
        let colors = create_test_colors();

        // Unknown elevation should fallback to subtle
        let unknown_elevation = CardStyles::new(colors.clone())
            .elevation_str("unknown")
            .classes();
        assert!(unknown_elevation.contains("shadow-sm"));

        // Unknown surface should fallback to standard
        let unknown_surface = CardStyles::new(colors.clone())
            .surface_str("unknown")
            .classes();
        assert!(unknown_surface.contains("bg-white"));

        // Unknown spacing should fallback to standard
        let unknown_spacing = CardStyles::new(colors.clone())
            .spacing_str("unknown")
            .classes();
        assert!(unknown_spacing.contains("p-5"));

        // Unknown interaction should fallback to static
        let unknown_interaction = CardStyles::new(colors.clone())
            .interaction_str("unknown")
            .classes();
        assert!(!unknown_interaction.contains("cursor-pointer"));
    }

    #[test]
    fn test_card_convenience_function() {
        let colors = create_test_colors();
        let classes = card_styles(colors)
            .raised_elevation()
            .elevated_surface()
            .comfortable_spacing()
            .clickable_interaction()
            .classes();

        assert!(classes.contains("shadow-md"));
        assert!(classes.contains("p-6"));
        assert!(classes.contains("cursor-pointer"));
    }

    #[test]
    fn test_card_build_alias() {
        let colors = create_test_colors();
        let classes1 = CardStyles::new(colors.clone()).raised_elevation().classes();
        let classes2 = CardStyles::new(colors).raised_elevation().build();

        assert_eq!(classes1, classes2);
    }

    #[test]
    fn test_card_classes_from_strings() {
        let colors = create_test_colors();

        // Basic usage
        let classes = card_classes_from_strings(
            colors.clone(),
            "elevated",    // surface
            "raised",      // elevation
            "comfortable", // spacing
            "clickable",   // interaction
            false,         // selected
        );

        assert!(classes.contains("shadow-md")); // raised elevation
        assert!(classes.contains("p-6")); // comfortable spacing
        assert!(classes.contains("cursor-pointer")); // clickable interaction
        assert!(classes.contains("focus:ring-2")); // clickable cards should have focus rings
        assert!(!classes.contains("ring-jupiter-blue-300")); // but not selection rings when not selected

        // With selection
        let selected_classes = card_classes_from_strings(
            colors.clone(),
            "standard",
            "subtle",
            "standard",
            "hoverable",
            true, // selected
        );

        assert!(selected_classes.contains("shadow-sm")); // subtle elevation
        assert!(selected_classes.contains("p-5")); // standard spacing
        assert!(selected_classes.contains("hover:scale-101")); // hoverable interaction
        assert!(selected_classes.contains("ring-jupiter-blue-300")); // selected ring
    }

    #[test]
    fn test_card_styles_consistency() {
        let colors = create_test_colors();

        // All cards should have consistent base classes
        let variants = [
            CardStyles::new(colors.clone()).standard_surface().classes(),
            CardStyles::new(colors.clone()).elevated_surface().classes(),
            CardStyles::new(colors.clone()).branded_surface().classes(),
            CardStyles::new(colors.clone()).glass_surface().classes(),
            CardStyles::new(colors.clone()).dark_surface().classes(),
            CardStyles::new(colors.clone())
                .transparent_surface()
                .classes(),
        ];

        for variant_classes in variants {
            assert!(variant_classes.contains("rounded-lg"));
            assert!(variant_classes.contains("border"));
            assert!(variant_classes.contains("transition-all"));
            assert!(variant_classes.contains("duration-300"));
        }
    }

    #[test]
    fn test_card_no_duplicate_classes() {
        let colors = create_test_colors();
        let classes = CardStyles::new(colors)
            .raised_elevation()
            .standard_surface()
            .standard_spacing()
            .custom("rounded-lg") // duplicate of base class
            .custom("border") // duplicate of base class
            .classes();

        // Check for exact class duplicates by splitting into tokens
        let class_tokens: Vec<&str> = classes.split_whitespace().collect();

        // Count occurrences of specific classes
        let rounded_count = class_tokens.iter().filter(|&&c| c == "rounded-lg").count();
        let border_count = class_tokens.iter().filter(|&&c| c == "border").count();

        assert_eq!(rounded_count, 1, "rounded-lg should appear only once");
        assert_eq!(border_count, 1, "border should appear only once");
    }

    #[test]
    fn test_card_empty_custom_classes() {
        let colors = create_test_colors();
        let classes = CardStyles::new(colors)
            .custom_classes("") // empty string
            .custom_classes("   ") // whitespace only
            .classes();

        // Should not contain extra spaces
        assert!(!classes.contains("  "));
        assert!(classes.contains("rounded-lg")); // base classes should still be there
    }
}
