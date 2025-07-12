#[cfg(test)]
mod tests {
    use crate::builders::text::{
        text_clamp_style, text_classes_from_strings, text_element_from_hierarchy, text_styles,
        TextStyles,
    };
    use crate::core::color::WaterWellnessColors;

    // Helper function to create text styles with default color provider
    fn create_text_styles() -> TextStyles<WaterWellnessColors> {
        text_styles(WaterWellnessColors::default())
    }

    #[test]
    fn test_text_hierarchy_str() {
        // Test all hierarchy string mappings
        let hierarchies = vec![
            ("title", "font-bold text-4xl text-gray-900 tracking-tight"),
            ("heading", "font-bold text-3xl text-gray-900 tracking-tight"),
            (
                "subheading",
                "font-bold text-2xl text-gray-900 tracking-tight",
            ),
            ("h4", "font-bold text-xl text-gray-900 tracking-tight"),
            ("body", "font-normal text-base text-gray-900"),
            ("body-large", "font-normal text-lg text-gray-900"),
            ("body-small", "font-normal text-sm text-gray-900"),
            ("caption", "font-medium text-sm text-gray-600"),
            (
                "overline",
                "font-medium text-xs text-gray-500 tracking-wider uppercase",
            ),
            ("code", "bg-gray-100 font-mono px-1 py-0.5 rounded text-sm"),
        ];

        for (hierarchy, expected_content) in hierarchies {
            let classes = create_text_styles().hierarchy_str(hierarchy).classes();
            assert!(
                classes.contains("leading-relaxed"),
                "All text should have leading-relaxed"
            );

            // Check that all expected classes are present
            for expected_class in expected_content.split_whitespace() {
                assert!(
                    classes.contains(expected_class),
                    "Hierarchy '{}' should contain '{}' but got '{}'",
                    hierarchy,
                    expected_class,
                    classes
                );
            }
        }
    }

    #[test]
    fn test_text_convenience_methods() {
        // Test hierarchy convenience methods
        let title_classes = create_text_styles().title().classes();
        assert!(title_classes.contains("text-4xl"));
        assert!(title_classes.contains("font-bold"));
        assert!(title_classes.contains("tracking-tight"));

        let heading_classes = create_text_styles().heading().classes();
        assert!(heading_classes.contains("text-3xl"));
        assert!(heading_classes.contains("font-bold"));

        let body_classes = create_text_styles().body().classes();
        assert!(body_classes.contains("text-base"));
        assert!(body_classes.contains("font-normal"));

        let caption_classes = create_text_styles().caption().classes();
        assert!(caption_classes.contains("text-sm"));
        assert!(caption_classes.contains("font-medium"));
        assert!(caption_classes.contains("text-gray-600"));
    }

    #[test]
    fn test_text_size_override() {
        // Test that size override works
        let classes = create_text_styles().title().size_str("sm").classes();
        assert!(classes.contains("text-sm"));
        assert!(!classes.contains("text-4xl")); // Should not contain default title size

        let classes = create_text_styles().body().extra_large().classes();
        assert!(classes.contains("text-xl"));
        assert!(!classes.contains("text-base")); // Should not contain default body size
    }

    #[test]
    fn test_text_weight_override() {
        // Test that weight override works
        let classes = create_text_styles().body().bold().classes();
        assert!(classes.contains("font-bold"));
        assert!(!classes.contains("font-normal")); // Should not contain default body weight

        let classes = create_text_styles().title().light().classes();
        assert!(classes.contains("font-light"));
        assert!(!classes.contains("font-bold")); // Should not contain default title weight
    }

    #[test]
    fn test_text_color_methods() {
        // Test color convenience methods
        let primary_classes = create_text_styles().body().primary().classes();
        assert!(primary_classes.contains("text-water-blue-500"));

        let secondary_classes = create_text_styles().body().secondary().classes();
        assert!(secondary_classes.contains("text-water-green-500"));

        let muted_classes = create_text_styles().body().muted().classes();
        assert!(muted_classes.contains("text-gray-500"));

        let error_classes = create_text_styles().body().error().classes();
        assert!(error_classes.contains("text-red-600"));
    }

    #[test]
    fn test_text_alignment_methods() {
        // Test alignment convenience methods
        let left_classes = create_text_styles().body().left().classes();
        assert!(left_classes.contains("text-left"));

        let center_classes = create_text_styles().body().center().classes();
        assert!(center_classes.contains("text-center"));

        let right_classes = create_text_styles().body().right().classes();
        assert!(right_classes.contains("text-right"));

        let justify_classes = create_text_styles().body().justify().classes();
        assert!(justify_classes.contains("text-justify"));
    }

    #[test]
    fn test_text_truncation() {
        let truncate_classes = create_text_styles().body().truncate().classes();
        assert!(truncate_classes.contains("truncate"));
    }

    #[test]
    fn test_text_clamp_lines() {
        let styles = create_text_styles().body().clamp_lines(3);
        let clamp_style = styles.clamp_style();
        assert!(clamp_style.contains("webkit-line-clamp: 3"));
        assert!(clamp_style.contains("overflow: hidden"));
    }

    #[test]
    fn test_text_custom_classes() {
        let classes = create_text_styles()
            .body()
            .custom_classes("custom-class another-class")
            .classes();
        assert!(classes.contains("custom-class"));
        assert!(classes.contains("another-class"));
    }

    #[test]
    fn test_text_classes_from_strings() {
        // Test with truncation (no line clamping)
        let classes = text_classes_from_strings(
            WaterWellnessColors::default(),
            "title",
            Some("lg"),
            Some("medium"),
            Some("primary"),
            Some("center"),
            true,
            None, // No line clamping
            Some("custom-class"),
        );

        assert!(classes.contains("text-lg")); // size override
        assert!(classes.contains("font-medium")); // weight override
        assert!(classes.contains("text-water-blue-500")); // primary color
        assert!(classes.contains("text-center")); // center alignment
        assert!(classes.contains("truncate")); // truncation
        assert!(classes.contains("custom-class")); // custom class
        assert!(classes.contains("tracking-tight")); // from title hierarchy

        // Test with line clamping (no truncation)
        let classes_with_clamp = text_classes_from_strings(
            WaterWellnessColors::default(),
            "title",
            Some("lg"),
            Some("medium"),
            Some("primary"),
            Some("center"),
            false,   // No truncation
            Some(2), // Line clamping
            Some("custom-class"),
        );

        assert!(classes_with_clamp.contains("text-lg")); // size override
        assert!(classes_with_clamp.contains("font-medium")); // weight override
        assert!(classes_with_clamp.contains("text-water-blue-500")); // primary color
        assert!(classes_with_clamp.contains("text-center")); // center alignment
        assert!(!classes_with_clamp.contains("truncate")); // no truncation
        assert!(classes_with_clamp.contains("custom-class")); // custom class
        assert!(classes_with_clamp.contains("tracking-tight")); // from title hierarchy
    }

    #[test]
    fn test_text_element_selection() {
        let title_element = create_text_styles().title().element();
        assert_eq!(title_element, "h1");

        let heading_element = create_text_styles().heading().element();
        assert_eq!(heading_element, "h2");

        let body_element = create_text_styles().body().element();
        assert_eq!(body_element, "p");

        let caption_element = create_text_styles().caption().element();
        assert_eq!(caption_element, "span");
    }

    #[test]
    fn test_text_element_from_hierarchy() {
        assert_eq!(text_element_from_hierarchy("title"), "h1");
        assert_eq!(text_element_from_hierarchy("heading"), "h2");
        assert_eq!(text_element_from_hierarchy("subheading"), "h3");
        assert_eq!(text_element_from_hierarchy("h4"), "h4");
        assert_eq!(text_element_from_hierarchy("body"), "p");
        assert_eq!(text_element_from_hierarchy("caption"), "span");
        assert_eq!(text_element_from_hierarchy("overline"), "span");
        assert_eq!(text_element_from_hierarchy("code"), "code");
        assert_eq!(text_element_from_hierarchy("unknown"), "p");
    }

    #[test]
    fn test_text_clamp_style_utility() {
        let style = text_clamp_style(Some(3));
        assert!(style.contains("webkit-line-clamp: 3"));
        assert!(style.contains("overflow: hidden"));

        let no_style = text_clamp_style(None);
        assert!(no_style.is_empty());
    }

    #[test]
    fn test_text_size_str_all_sizes() {
        let sizes = vec![
            ("xs", "text-xs"),
            ("sm", "text-sm"),
            ("md", "text-base"),
            ("lg", "text-lg"),
            ("xl", "text-xl"),
            ("2xl", "text-2xl"),
            ("3xl", "text-3xl"),
            ("4xl", "text-4xl"),
        ];

        for (size_str, expected_class) in sizes {
            let classes = create_text_styles().body().size_str(size_str).classes();
            assert!(
                classes.contains(expected_class),
                "Size '{}' should contain '{}' but got '{}'",
                size_str,
                expected_class,
                classes
            );
        }
    }

    #[test]
    fn test_text_weight_str_all_weights() {
        let weights = vec![
            ("light", "font-light"),
            ("normal", "font-normal"),
            ("medium", "font-medium"),
            ("semibold", "font-semibold"),
            ("bold", "font-bold"),
            ("extrabold", "font-extrabold"),
        ];

        for (weight_str, expected_class) in weights {
            let classes = create_text_styles().body().weight_str(weight_str).classes();
            assert!(
                classes.contains(expected_class),
                "Weight '{}' should contain '{}' but got '{}'",
                weight_str,
                expected_class,
                classes
            );
        }
    }

    #[test]
    fn test_text_color_str_all_colors() {
        let colors = vec![
            ("primary", "text-water-blue-500"),
            ("secondary", "text-water-green-500"),
            ("accent", "text-water-orange-500"),
            ("muted", "text-gray-500"),
            ("disabled", "text-gray-400"),
            ("white", "text-white"),
            ("black", "text-black"),
            ("success", "text-green-600"),
            ("warning", "text-yellow-600"),
            ("error", "text-red-600"),
            ("info", "text-blue-600"),
        ];

        for (color_str, expected_class) in colors {
            let classes = create_text_styles().body().color_str(color_str).classes();
            assert!(
                classes.contains(expected_class),
                "Color '{}' should contain '{}' but got '{}'",
                color_str,
                expected_class,
                classes
            );
        }
    }

    #[test]
    fn test_text_alignment_str_all_alignments() {
        let alignments = vec![
            ("left", "text-left"),
            ("center", "text-center"),
            ("right", "text-right"),
            ("justify", "text-justify"),
        ];

        for (alignment_str, expected_class) in alignments {
            let classes = create_text_styles()
                .body()
                .alignment_str(alignment_str)
                .classes();
            assert!(
                classes.contains(expected_class),
                "Alignment '{}' should contain '{}' but got '{}'",
                alignment_str,
                expected_class,
                classes
            );
        }
    }

    #[test]
    fn test_text_invalid_strings_ignored() {
        // Test that invalid strings are ignored (builder should not change)
        let base_classes = create_text_styles().body().classes();

        let invalid_size_classes = create_text_styles().body().size_str("invalid").classes();
        assert_eq!(base_classes, invalid_size_classes);

        let invalid_weight_classes = create_text_styles().body().weight_str("invalid").classes();
        assert_eq!(base_classes, invalid_weight_classes);

        let invalid_color_classes = create_text_styles().body().color_str("invalid").classes();
        assert_eq!(base_classes, invalid_color_classes);

        let invalid_alignment_classes = create_text_styles()
            .body()
            .alignment_str("invalid")
            .classes();
        assert_eq!(base_classes, invalid_alignment_classes);
    }

    #[test]
    fn test_text_chaining_api() {
        // Test that chaining works correctly
        let classes = create_text_styles()
            .title()
            .large()
            .bold()
            .primary()
            .center()
            .truncate()
            .custom_classes("custom-class")
            .classes();

        assert!(classes.contains("text-lg")); // size
        assert!(classes.contains("font-bold")); // weight
        assert!(classes.contains("text-water-blue-500")); // color
        assert!(classes.contains("text-center")); // alignment
        assert!(classes.contains("truncate")); // truncation
        assert!(classes.contains("custom-class")); // custom
        assert!(classes.contains("tracking-tight")); // from title hierarchy
        assert!(classes.contains("leading-relaxed")); // base class
    }

    #[test]
    fn test_text_class_deduplication() {
        // Test that duplicate classes are removed
        let classes = create_text_styles()
            .body()
            .custom_classes("text-gray-900 text-gray-900") // duplicate classes
            .classes();

        // Count occurrences of the duplicate class
        let count = classes.matches("text-gray-900").count();
        assert_eq!(count, 1, "Duplicate classes should be deduplicated");
    }

    #[test]
    fn test_text_auto_color_by_hierarchy() {
        // Test that auto color selection works correctly for different hierarchies
        let title_classes = create_text_styles().title().color_str("auto").classes();
        assert!(title_classes.contains("text-gray-900"));

        let caption_classes = create_text_styles().caption().color_str("auto").classes();
        assert!(caption_classes.contains("text-gray-600"));

        let overline_classes = create_text_styles().overline().color_str("auto").classes();
        assert!(overline_classes.contains("text-gray-500"));
    }

    #[test]
    fn test_text_code_hierarchy_special_classes() {
        // Test that code hierarchy includes special styling
        let code_classes = create_text_styles().code().classes();
        assert!(code_classes.contains("font-mono"));
        assert!(code_classes.contains("bg-gray-100"));
        assert!(code_classes.contains("px-1"));
        assert!(code_classes.contains("py-0.5"));
        assert!(code_classes.contains("rounded"));
        assert!(code_classes.contains("text-sm"));
    }

    #[test]
    fn test_text_default_hierarchy() {
        // Test that unknown hierarchy defaults to body
        let classes = create_text_styles().hierarchy_str("unknown").classes();
        let body_classes = create_text_styles().body().classes();
        assert_eq!(classes, body_classes);
    }

    #[test]
    fn test_text_base_classes_always_present() {
        // Test that base classes are always present
        let hierarchies = vec![
            "title",
            "heading",
            "subheading",
            "h4",
            "body",
            "body-large",
            "body-small",
            "caption",
            "overline",
            "code",
        ];

        for hierarchy in hierarchies {
            let classes = create_text_styles().hierarchy_str(hierarchy).classes();
            assert!(
                classes.contains("leading-relaxed"),
                "Hierarchy '{}' should contain base class 'leading-relaxed'",
                hierarchy
            );
        }
    }
}
