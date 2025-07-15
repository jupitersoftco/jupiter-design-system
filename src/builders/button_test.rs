//! Tests for the button styling utilities

#[cfg(test)]
mod tests {
    use crate::builders::button::{
        button_classes_from_strings, button_styles, ButtonState, ButtonStyles, ButtonVariant,
    };
    use crate::core::color::VibeColors;

    fn create_test_colors() -> VibeColors {
        VibeColors::new()
    }

    #[test]
    fn test_button_styles_default() {
        let colors = create_test_colors();
        let classes = ButtonStyles::new(colors).classes();

        assert!(classes.contains("inline-flex"));
        assert!(classes.contains("items-center"));
        assert!(classes.contains("justify-center"));
        assert!(classes.contains("font-medium"));
        assert!(classes.contains("transition-colors"));
        assert!(classes.contains("px-4 py-2")); // medium size
        assert!(classes.contains("bg-water-blue-500")); // primary variant
    }

    #[test]
    fn test_button_styles_chainable_api() {
        let colors = create_test_colors();
        let classes = ButtonStyles::new(colors)
            .success()
            .large()
            .full_width()
            .with_icon()
            .classes();

        assert!(classes.contains("bg-green-500")); // success variant
        assert!(classes.contains("px-6 py-3")); // large size
        assert!(classes.contains("w-full")); // full width
        assert!(classes.contains("space-x-2")); // with icon
    }

    #[test]
    fn test_button_styles_variants() {
        let colors = create_test_colors();

        // Primary
        let primary = ButtonStyles::new(colors.clone()).primary().classes();
        assert!(primary.contains("bg-water-blue-500"));
        assert!(primary.contains("text-white"));

        // Secondary
        let secondary = ButtonStyles::new(colors.clone()).secondary().classes();
        assert!(secondary.contains("bg-white"));
        assert!(secondary.contains("text-gray-900"));
        assert!(secondary.contains("border"));

        // Success
        let success = ButtonStyles::new(colors.clone()).success().classes();
        assert!(success.contains("bg-green-500"));
        assert!(success.contains("text-white"));

        // Warning
        let warning = ButtonStyles::new(colors.clone()).warning().classes();
        assert!(warning.contains("bg-amber-500"));
        assert!(warning.contains("text-white"));

        // Error
        let error = ButtonStyles::new(colors.clone()).error().classes();
        assert!(error.contains("bg-red-500"));
        assert!(error.contains("text-white"));

        // Ghost
        let ghost = ButtonStyles::new(colors.clone()).ghost().classes();
        assert!(ghost.contains("bg-transparent"));
        assert!(ghost.contains("text-gray-900"));

        // Link
        let link = ButtonStyles::new(colors.clone()).link().classes();
        assert!(link.contains("bg-transparent"));
        assert!(link.contains("text-water-blue-500"));
        assert!(link.contains("hover:underline"));
    }

    #[test]
    fn test_button_styles_sizes() {
        let colors = create_test_colors();

        // Extra small
        let xs = ButtonStyles::new(colors.clone()).extra_small().classes();
        assert!(xs.contains("px-2 py-1"));
        assert!(xs.contains("text-xs"));
        assert!(xs.contains("rounded"));

        // Small
        let sm = ButtonStyles::new(colors.clone()).small().classes();
        assert!(sm.contains("px-3 py-1.5"));
        assert!(sm.contains("text-sm"));
        assert!(sm.contains("rounded"));

        // Medium
        let md = ButtonStyles::new(colors.clone()).medium().classes();
        assert!(md.contains("px-4 py-2"));
        assert!(md.contains("text-sm"));
        assert!(md.contains("rounded-md"));

        // Large
        let lg = ButtonStyles::new(colors.clone()).large().classes();
        assert!(lg.contains("px-6 py-3"));
        assert!(lg.contains("text-base"));
        assert!(lg.contains("rounded-md"));

        // Extra large
        let xl = ButtonStyles::new(colors.clone()).extra_large().classes();
        assert!(xl.contains("px-8 py-4"));
        assert!(xl.contains("text-lg"));
        assert!(xl.contains("rounded-lg"));
    }

    #[test]
    fn test_button_styles_states() {
        let colors = create_test_colors();

        // Disabled
        let disabled = ButtonStyles::new(colors.clone()).disabled().classes();
        assert!(disabled.contains("opacity-50"));
        assert!(disabled.contains("cursor-not-allowed"));

        // Loading
        let loading = ButtonStyles::new(colors.clone()).loading().classes();
        assert!(loading.contains("cursor-wait"));

        // Hover
        let hover = ButtonStyles::new(colors.clone()).hover().classes();
        assert!(hover.contains("hover:scale-105"));

        // Active
        let active = ButtonStyles::new(colors.clone()).active().classes();
        assert!(active.contains("active:scale-95"));
    }

    #[test]
    fn test_button_styles_convenience_function() {
        let colors = create_test_colors();
        let classes = button_styles(colors)
            .primary()
            .large()
            .full_width()
            .classes();

        assert!(classes.contains("bg-water-blue-500"));
        assert!(classes.contains("px-6 py-3"));
        assert!(classes.contains("w-full"));
    }

    #[test]
    fn test_button_styles_build_alias() {
        let colors = create_test_colors();
        let classes1 = ButtonStyles::new(colors.clone()).primary().classes();
        let classes2 = ButtonStyles::new(colors).primary().build();

        assert_eq!(classes1, classes2);
    }

    #[test]
    fn test_button_styles_consistency() {
        let colors = create_test_colors();

        // All variants should have consistent base classes
        let variants = [
            ButtonStyles::new(colors.clone()).primary().classes(),
            ButtonStyles::new(colors.clone()).secondary().classes(),
            ButtonStyles::new(colors.clone()).success().classes(),
            ButtonStyles::new(colors.clone()).warning().classes(),
            ButtonStyles::new(colors.clone()).error().classes(),
            ButtonStyles::new(colors.clone()).ghost().classes(),
            ButtonStyles::new(colors.clone()).link().classes(),
        ];

        for variant_classes in variants {
            assert!(variant_classes.contains("inline-flex"));
            assert!(variant_classes.contains("items-center"));
            assert!(variant_classes.contains("justify-center"));
            assert!(variant_classes.contains("font-medium"));
            assert!(variant_classes.contains("transition-colors"));
            assert!(variant_classes.contains("duration-200"));
            assert!(variant_classes.contains("disabled:opacity-50"));
            assert!(variant_classes.contains("disabled:cursor-not-allowed"));
        }
    }

    #[test]
    fn test_button_variant_serialization() {
        let variant = ButtonVariant::Primary;
        let serialized = serde_json::to_string(&variant).unwrap();
        let deserialized: ButtonVariant = serde_json::from_str(&serialized).unwrap();
        assert_eq!(variant, deserialized);
    }

    #[test]
    fn test_button_state_serialization() {
        let state = ButtonState::Loading;
        let serialized = serde_json::to_string(&state).unwrap();
        let deserialized: ButtonState = serde_json::from_str(&serialized).unwrap();
        assert_eq!(state, deserialized);
    }

    #[test]
    fn test_button_styles_custom_single() {
        let colors = create_test_colors();
        let classes = ButtonStyles::new(colors)
            .primary()
            .custom("shadow-xl")
            .classes();

        assert!(classes.contains("bg-water-blue-500")); // primary variant
        assert!(classes.contains("shadow-xl")); // custom class
    }

    #[test]
    fn test_button_styles_custom_multiple() {
        let colors = create_test_colors();
        let classes = ButtonStyles::new(colors)
            .primary()
            .custom("shadow-xl")
            .custom("transform")
            .custom("hover:rotate-1")
            .classes();

        assert!(classes.contains("shadow-xl"));
        assert!(classes.contains("transform"));
        assert!(classes.contains("hover:rotate-1"));
    }

    #[test]
    fn test_button_styles_custom_classes_string() {
        let colors = create_test_colors();
        let classes = ButtonStyles::new(colors)
            .primary()
            .custom_classes("shadow-xl transform hover:rotate-1")
            .classes();

        assert!(classes.contains("shadow-xl"));
        assert!(classes.contains("transform"));
        assert!(classes.contains("hover:rotate-1"));
    }

    #[test]
    fn test_button_styles_custom_vec() {
        let colors = create_test_colors();
        let classes = ButtonStyles::new(colors)
            .primary()
            .custom_vec(vec!["shadow-xl", "transform", "hover:rotate-1"])
            .classes();

        assert!(classes.contains("shadow-xl"));
        assert!(classes.contains("transform"));
        assert!(classes.contains("hover:rotate-1"));
    }

    #[test]
    fn test_button_styles_mixed_custom() {
        let colors = create_test_colors();
        let classes = ButtonStyles::new(colors)
            .primary()
            .large()
            .custom("shadow-xl")
            .custom_classes("transform hover:rotate-1")
            .full_width()
            .custom_vec(vec!["animate-pulse", "duration-300"])
            .classes();

        // Design system classes
        assert!(classes.contains("bg-water-blue-500")); // primary
        assert!(classes.contains("px-6 py-3")); // large
        assert!(classes.contains("w-full")); // full width

        // Custom classes
        assert!(classes.contains("shadow-xl"));
        assert!(classes.contains("transform"));
        assert!(classes.contains("hover:rotate-1"));
        assert!(classes.contains("animate-pulse"));
        assert!(classes.contains("duration-300"));
    }

    #[test]
    fn test_button_styles_convenience_with_custom() {
        let colors = create_test_colors();
        let classes = button_styles(colors)
            .custom_classes("shadow-xl transform")
            .primary()
            .large()
            .classes();

        assert!(classes.contains("bg-water-blue-500"));
        assert!(classes.contains("px-6 py-3"));
        assert!(classes.contains("shadow-xl"));
        assert!(classes.contains("transform"));
    }

    #[test]
    fn test_button_styles_custom_empty_string() {
        let colors = create_test_colors();
        let classes = ButtonStyles::new(colors)
            .primary()
            .custom_classes("") // empty string should be handled gracefully
            .classes();

        assert!(classes.contains("bg-water-blue-500"));
        assert!(!classes.contains("  ")); // no extra spaces
    }

    #[test]
    fn test_button_styles_custom_whitespace_handling() {
        let colors = create_test_colors();
        let classes = ButtonStyles::new(colors)
            .primary()
            .custom_classes("  shadow-xl   transform  hover:rotate-1  ") // extra whitespace
            .classes();

        assert!(classes.contains("shadow-xl"));
        assert!(classes.contains("transform"));
        assert!(classes.contains("hover:rotate-1"));

        // Check that there are no extra spaces in the final string
        let class_parts: Vec<&str> = classes.split_whitespace().collect();
        assert_eq!(classes, class_parts.join(" "));
    }

    // String-based convenience method tests
    #[test]
    fn test_button_styles_variant_str() {
        let colors = create_test_colors();

        // Test standard variants
        let primary = ButtonStyles::new(colors.clone())
            .variant_str("primary")
            .classes();
        assert!(primary.contains("bg-water-blue-500"));

        let secondary = ButtonStyles::new(colors.clone())
            .variant_str("secondary")
            .classes();
        assert!(secondary.contains("bg-white"));

        let success = ButtonStyles::new(colors.clone())
            .variant_str("success")
            .classes();
        assert!(success.contains("bg-green-500"));

        // Test aliases
        let outline = ButtonStyles::new(colors.clone())
            .variant_str("outline")
            .classes();
        assert!(outline.contains("bg-white")); // maps to secondary

        let danger = ButtonStyles::new(colors.clone())
            .variant_str("danger")
            .classes();
        assert!(danger.contains("bg-red-500")); // maps to error

        let water = ButtonStyles::new(colors.clone())
            .variant_str("water")
            .classes();
        assert!(water.contains("bg-water-blue-500")); // maps to primary

        // Test fallback
        let unknown = ButtonStyles::new(colors.clone())
            .variant_str("unknown")
            .classes();
        assert!(unknown.contains("bg-water-blue-500")); // fallback to primary
    }

    #[test]
    fn test_button_styles_size_str() {
        let colors = create_test_colors();

        // Test standard sizes
        let xs = ButtonStyles::new(colors.clone()).size_str("xs").classes();
        assert!(xs.contains("px-2 py-1"));

        let sm = ButtonStyles::new(colors.clone()).size_str("sm").classes();
        assert!(sm.contains("px-3 py-1.5"));

        let md = ButtonStyles::new(colors.clone()).size_str("md").classes();
        assert!(md.contains("px-4 py-2"));

        let lg = ButtonStyles::new(colors.clone()).size_str("lg").classes();
        assert!(lg.contains("px-6 py-3"));

        let xl = ButtonStyles::new(colors.clone()).size_str("xl").classes();
        assert!(xl.contains("px-8 py-4"));

        // Test aliases
        let small = ButtonStyles::new(colors.clone())
            .size_str("small")
            .classes();
        assert!(small.contains("px-3 py-1.5")); // maps to sm

        let large = ButtonStyles::new(colors.clone())
            .size_str("large")
            .classes();
        assert!(large.contains("px-6 py-3")); // maps to lg

        // Test fallback
        let unknown = ButtonStyles::new(colors.clone())
            .size_str("unknown")
            .classes();
        assert!(unknown.contains("px-4 py-2")); // fallback to medium
    }

    #[test]
    fn test_button_styles_state_str() {
        let colors = create_test_colors();

        // Test standard states
        let default = ButtonStyles::new(colors.clone())
            .state_str("default")
            .classes();
        // Note: Base classes include "disabled:opacity-50" for all buttons
        assert!(default.contains("disabled:opacity-50")); // base class always present

        let disabled = ButtonStyles::new(colors.clone())
            .state_str("disabled")
            .classes();
        assert!(disabled.contains("opacity-50"));
        assert!(disabled.contains("cursor-not-allowed"));

        let loading = ButtonStyles::new(colors.clone())
            .state_str("loading")
            .classes();
        assert!(loading.contains("cursor-wait"));

        let hover = ButtonStyles::new(colors.clone())
            .state_str("hover")
            .classes();
        assert!(hover.contains("hover:scale-105"));

        let active = ButtonStyles::new(colors.clone())
            .state_str("active")
            .classes();
        assert!(active.contains("active:scale-95"));

        // Test fallback
        let unknown = ButtonStyles::new(colors.clone())
            .state_str("unknown")
            .classes();
        assert!(unknown.contains("disabled:opacity-50")); // base class always present
    }

    #[test]
    fn test_button_classes_from_strings() {
        let colors = create_test_colors();

        // Test basic usage
        let classes = button_classes_from_strings(
            colors.clone(),
            "primary",
            "lg",
            false, // disabled
            false, // loading
            true,  // full_width
        );

        assert!(classes.contains("bg-water-blue-500")); // primary variant
        assert!(classes.contains("px-6 py-3")); // large size
        assert!(classes.contains("w-full")); // full width
        assert!(classes.contains("disabled:opacity-50")); // base class always present
        assert!(!classes.contains("cursor-wait")); // not loading

        // Test with loading state
        let loading_classes = button_classes_from_strings(
            colors.clone(),
            "secondary",
            "sm",
            false, // disabled
            true,  // loading
            false, // full_width
        );

        assert!(loading_classes.contains("bg-white")); // secondary variant
        assert!(loading_classes.contains("px-3 py-1.5")); // small size
        assert!(loading_classes.contains("cursor-wait")); // loading state
        assert!(!loading_classes.contains("w-full")); // not full width

        // Test with disabled state
        let disabled_classes = button_classes_from_strings(
            colors.clone(),
            "success",
            "md",
            true,  // disabled
            false, // loading
            false, // full_width
        );

        assert!(disabled_classes.contains("bg-green-500")); // success variant
        assert!(disabled_classes.contains("px-4 py-2")); // medium size
        assert!(disabled_classes.contains("opacity-50")); // disabled state
        assert!(disabled_classes.contains("cursor-not-allowed")); // disabled cursor

        // Test with aliases
        let alias_classes = button_classes_from_strings(
            colors.clone(),
            "outline", // maps to secondary
            "xl",
            false, // disabled
            false, // loading
            false, // full_width
        );

        assert!(alias_classes.contains("bg-white")); // secondary variant (from outline alias)
        assert!(alias_classes.contains("px-8 py-4")); // extra large size
    }

    #[test]
    fn test_button_string_methods_chaining() {
        let colors = create_test_colors();

        // Test that string methods can be chained with other methods
        let classes = ButtonStyles::new(colors)
            .variant_str("primary")
            .size_str("lg")
            .state_str("hover")
            .full_width()
            .custom("shadow-xl")
            .classes();

        assert!(classes.contains("bg-water-blue-500")); // primary variant
        assert!(classes.contains("px-6 py-3")); // large size
        assert!(classes.contains("hover:scale-105")); // hover state
        assert!(classes.contains("w-full")); // full width
        assert!(classes.contains("shadow-xl")); // custom class
    }
}
