#[cfg(test)]
mod tests {
    use crate::builders::state::{
        empty_state_styles, error_state_styles, loading_state_styles, state_classes_from_strings,
        state_styles, success_state_styles, StateStyles,
    };
    use crate::themes::VibeColors;

    #[test]
    fn test_state_styles_new() {
        let styles = StateStyles::new(VibeColors::default());
        let classes = styles.classes();

        // Default should be informational, standard, center aligned, medium size
        assert!(classes.contains("state-pattern"));
        assert!(classes.contains("flex"));
        assert!(classes.contains("flex-col"));
        assert!(classes.contains("items-center"));
        assert!(classes.contains("text-center"));
        assert!(classes.contains("px-8"));
        assert!(classes.contains("py-16"));
    }

    #[test]
    fn test_intent_methods() {
        let colors = VibeColors::default();

        // Test informational intent
        let classes = StateStyles::new(colors.clone()).informational().classes();
        assert!(classes.contains("bg-gray-50"));
        assert!(classes.contains("text-gray-900"));

        // Test loading intent
        let classes = StateStyles::new(colors.clone()).loading().classes();
        assert!(classes.contains("text-jupiter-blue-500"));

        // Test success intent
        let classes = StateStyles::new(colors.clone()).success().classes();
        assert!(classes.contains("text-green-600"));
        assert!(classes.contains("bg-green-50"));

        // Test warning intent
        let classes = StateStyles::new(colors.clone()).warning().classes();
        assert!(classes.contains("text-orange-600"));
        assert!(classes.contains("bg-orange-50"));

        // Test error intent
        let classes = StateStyles::new(colors.clone()).error().classes();
        assert!(classes.contains("text-red-600"));
        assert!(classes.contains("bg-red-50"));

        // Test empty intent
        let classes = StateStyles::new(colors.clone()).empty().classes();
        assert!(classes.contains("text-gray-600"));
    }

    #[test]
    fn test_prominence_methods() {
        let colors = VibeColors::default();

        // All prominence levels should have the same layout classes
        let subtle = StateStyles::new(colors.clone()).subtle().classes();
        let standard = StateStyles::new(colors.clone()).standard().classes();
        let prominent = StateStyles::new(colors.clone()).prominent().classes();

        // They should all have the same base layout
        assert!(subtle.contains("flex"));
        assert!(standard.contains("flex"));
        assert!(prominent.contains("flex"));
    }

    #[test]
    fn test_size_methods() {
        let colors = VibeColors::default();

        // Test XS size
        let classes = StateStyles::new(colors.clone()).xs().classes();
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-8"));

        // Test SM size
        let classes = StateStyles::new(colors.clone()).sm().classes();
        assert!(classes.contains("px-6"));
        assert!(classes.contains("py-12"));

        // Test MD size (default)
        let classes = StateStyles::new(colors.clone()).md().classes();
        assert!(classes.contains("px-8"));
        assert!(classes.contains("py-16"));

        // Test LG size
        let classes = StateStyles::new(colors.clone()).lg().classes();
        assert!(classes.contains("px-12"));
        assert!(classes.contains("py-20"));

        // Test XL size
        let classes = StateStyles::new(colors.clone()).xl().classes();
        assert!(classes.contains("px-16"));
        assert!(classes.contains("py-24"));
    }

    #[test]
    fn test_alignment_methods() {
        let colors = VibeColors::default();

        // Test left alignment
        let classes = StateStyles::new(colors.clone()).left_aligned().classes();
        assert!(classes.contains("items-start"));
        assert!(classes.contains("text-left"));

        // Test center alignment
        let classes = StateStyles::new(colors.clone()).center_aligned().classes();
        assert!(classes.contains("items-center"));
        assert!(classes.contains("text-center"));

        // Test right alignment
        let classes = StateStyles::new(colors.clone()).right_aligned().classes();
        assert!(classes.contains("items-end"));
        assert!(classes.contains("text-right"));
    }

    #[test]
    fn test_loading_variant_methods() {
        let colors = VibeColors::default();

        // Test spinner variant
        let classes = StateStyles::new(colors.clone()).spinner().classes();
        assert!(classes.contains("animate-spin"));
        assert!(classes.contains("border-4"));
        assert!(classes.contains("border-t-transparent"));
        assert!(classes.contains("rounded-full"));

        // Test dots variant
        let classes = StateStyles::new(colors.clone()).dots().classes();
        assert!(classes.contains("animate-bounce"));
        assert!(classes.contains("rounded-full"));

        // Test pulse variant
        let classes = StateStyles::new(colors.clone()).pulse().classes();
        assert!(classes.contains("animate-pulse"));
        assert!(classes.contains("rounded-full"));

        // Test bars variant
        let classes = StateStyles::new(colors.clone()).bars().classes();
        assert!(classes.contains("animate-pulse"));
        assert!(classes.contains("rounded-sm"));

        // Test skeleton variant
        let classes = StateStyles::new(colors.clone()).skeleton().classes();
        assert!(classes.contains("animate-pulse"));
        assert!(classes.contains("rounded"));
    }

    #[test]
    fn test_fullscreen_methods() {
        let colors = VibeColors::default();

        // Test fullscreen enabled
        let classes = StateStyles::new(colors.clone()).fullscreen(true).classes();
        assert!(classes.contains("min-h-screen"));
        assert!(classes.contains("justify-center"));

        // Test fullscreen disabled
        let classes = StateStyles::new(colors.clone()).fullscreen(false).classes();
        assert!(!classes.contains("min-h-screen"));

        // Test is_fullscreen shorthand
        let classes = StateStyles::new(colors.clone()).is_fullscreen().classes();
        assert!(classes.contains("min-h-screen"));
        assert!(classes.contains("justify-center"));
    }

    #[test]
    fn test_custom_classes() {
        let colors = VibeColors::default();

        // Test single custom class
        let classes = StateStyles::new(colors.clone())
            .custom("custom-class")
            .classes();
        assert!(classes.contains("custom-class"));

        // Test multiple custom classes
        let classes = StateStyles::new(colors.clone())
            .custom_classes("class1 class2 class3")
            .classes();
        assert!(classes.contains("class1"));
        assert!(classes.contains("class2"));
        assert!(classes.contains("class3"));

        // Test chained custom classes
        let classes = StateStyles::new(colors.clone())
            .custom("first")
            .custom("second")
            .classes();
        assert!(classes.contains("first"));
        assert!(classes.contains("second"));
    }

    #[test]
    fn test_string_convenience_methods() {
        let colors = VibeColors::default();

        // Test intent_str
        let classes = StateStyles::new(colors.clone())
            .intent_str("error")
            .classes();
        assert!(classes.contains("text-red-600"));

        // Test prominence_str
        let _classes = StateStyles::new(colors.clone())
            .prominence_str("prominent")
            .classes();
        // Prominence affects visual weight but not basic layout classes

        // Test size_str
        let classes = StateStyles::new(colors.clone()).size_str("lg").classes();
        assert!(classes.contains("px-12"));
        assert!(classes.contains("py-20"));

        // Test alignment_str
        let classes = StateStyles::new(colors.clone())
            .alignment_str("left")
            .classes();
        assert!(classes.contains("items-start"));
        assert!(classes.contains("text-left"));

        // Test loading_variant_str
        let classes = StateStyles::new(colors.clone())
            .loading_variant_str("spinner")
            .classes();
        assert!(classes.contains("animate-spin"));
    }

    #[test]
    fn test_suggested_icon() {
        let colors = VibeColors::default();

        // Test different intents return appropriate icons
        let loading_style = StateStyles::new(colors.clone()).loading();
        assert_eq!(loading_style.suggested_icon(), "loader");

        let error_style = StateStyles::new(colors.clone()).error();
        assert_eq!(error_style.suggested_icon(), "alert-circle");

        let success_style = StateStyles::new(colors.clone()).success();
        assert_eq!(success_style.suggested_icon(), "check-circle");

        let empty_style = StateStyles::new(colors.clone()).empty();
        assert_eq!(empty_style.suggested_icon(), "inbox");
    }

    #[test]
    fn test_suggested_action_text() {
        let colors = VibeColors::default();

        // Test error with recommended action
        let error_style = StateStyles::new(colors.clone())
            .error()
            .recommended_action();
        assert_eq!(
            error_style.suggested_action_text(),
            Some("Try Again".to_string())
        );

        // Test empty with optional action
        let empty_style = StateStyles::new(colors.clone()).empty().optional_action();
        assert_eq!(
            empty_style.suggested_action_text(),
            Some("Refresh".to_string())
        );

        // Test loading with no action
        let loading_style = StateStyles::new(colors.clone()).loading().no_action();
        assert_eq!(loading_style.suggested_action_text(), None);
    }

    #[test]
    fn test_size_helper_methods() {
        let colors = VibeColors::default();

        // Test content size classes
        let md_style = StateStyles::new(colors.clone()).md();
        assert_eq!(md_style.content_size_classes(), "text-2xl");

        let lg_style = StateStyles::new(colors.clone()).lg();
        assert_eq!(lg_style.content_size_classes(), "text-3xl");

        // Test description size classes
        let md_style = StateStyles::new(colors.clone()).md();
        assert_eq!(md_style.description_size_classes(), "text-lg");

        // Test icon size classes
        let md_style = StateStyles::new(colors.clone()).md();
        assert_eq!(md_style.icon_size_classes(), "w-16 h-16");

        // Test loading size classes
        let md_spinner = StateStyles::new(colors.clone()).md().spinner();
        assert_eq!(md_spinner.loading_size_classes(), "w-12 h-12");

        let md_dots = StateStyles::new(colors.clone()).md().dots();
        assert_eq!(md_dots.loading_size_classes(), "w-4 h-4");
    }

    #[test]
    fn test_convenience_functions() {
        let colors = VibeColors::default();

        // Test state_styles function
        let styles = state_styles(colors.clone());
        let classes = styles.classes();
        assert!(classes.contains("state-pattern"));

        // Test loading_state_styles function
        let classes = loading_state_styles(colors.clone()).classes();
        assert!(classes.contains("animate-spin"));
        assert!(classes.contains("border-4"));

        // Test empty_state_styles function
        let classes = empty_state_styles(colors.clone()).classes();
        assert!(classes.contains("text-gray-600"));

        // Test error_state_styles function
        let classes = error_state_styles(colors.clone()).classes();
        assert!(classes.contains("text-red-600"));
        assert!(classes.contains("bg-red-50"));

        // Test success_state_styles function
        let classes = success_state_styles(colors.clone()).classes();
        assert!(classes.contains("text-green-600"));
        assert!(classes.contains("bg-green-50"));
    }

    #[test]
    fn test_state_classes_from_strings() {
        let colors = VibeColors::default();

        let classes = state_classes_from_strings(
            colors,
            "error",
            "prominent",
            "lg",
            "center",
            Some("spinner"),
            true,
        );

        assert!(classes.contains("text-red-600"));
        assert!(classes.contains("bg-red-50"));
        assert!(classes.contains("px-12"));
        assert!(classes.contains("py-20"));
        assert!(classes.contains("items-center"));
        assert!(classes.contains("text-center"));
        assert!(classes.contains("animate-spin"));
        assert!(classes.contains("min-h-screen"));
    }

    #[test]
    fn test_complex_state_composition() {
        let colors = VibeColors::default();

        let classes = StateStyles::new(colors)
            .error()
            .prominent()
            .lg()
            .center_aligned()
            .recommended_action()
            .is_fullscreen()
            .custom("rounded-lg")
            .custom_classes("shadow-lg border")
            .classes();

        assert!(classes.contains("state-pattern"));
        assert!(classes.contains("text-red-600"));
        assert!(classes.contains("bg-red-50"));
        assert!(classes.contains("px-12"));
        assert!(classes.contains("py-20"));
        assert!(classes.contains("items-center"));
        assert!(classes.contains("text-center"));
        assert!(classes.contains("min-h-screen"));
        assert!(classes.contains("rounded-lg"));
        assert!(classes.contains("shadow-lg"));
        assert!(classes.contains("border"));
    }

    #[test]
    fn test_class_deduplication() {
        let colors = VibeColors::default();

        let classes = StateStyles::new(colors)
            .custom("flex")
            .center_aligned() // Also adds flex
            .classes();

        // Should not have duplicate flex
        let class_count = classes.split_whitespace().filter(|&c| c == "flex").count();
        assert_eq!(class_count, 1);
    }

    #[test]
    fn test_fallback_values() {
        let colors = VibeColors::default();

        // Test invalid string values fallback to defaults
        let classes = StateStyles::new(colors)
            .intent_str("invalid")
            .prominence_str("invalid")
            .size_str("invalid")
            .alignment_str("invalid")
            .loading_variant_str("invalid")
            .classes();

        // Should fallback to informational intent
        assert!(classes.contains("text-gray-900"));
        // Should fallback to md size
        assert!(classes.contains("px-8"));
        assert!(classes.contains("py-16"));
        // Should fallback to center alignment
        assert!(classes.contains("items-center"));
        // Should not have loading animation for invalid variant
        assert!(!classes.contains("animate-spin"));
    }

    #[test]
    fn test_build_alias() {
        let colors = VibeColors::default();

        let styles = StateStyles::new(colors).error().prominent().lg();

        let classes1 = styles.clone().classes();
        let classes2 = styles.build();

        assert_eq!(classes1, classes2);
    }
}
