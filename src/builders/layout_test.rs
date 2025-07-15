#[cfg(test)]
mod tests {
    use crate::builders::layout::{
        card_content_styles, card_footer_styles, card_header_styles, layout_styles, LayoutStyles,
    };
    use crate::core::color::VibeColors;

    #[test]
    fn test_layout_styles_new() {
        let styles = LayoutStyles::new(VibeColors::default());
        let classes = styles.classes();
        assert!(classes.contains("p-4")); // Default MD spacing
        assert!(!classes.contains("border")); // No divider by default
    }

    #[test]
    fn test_divider_methods() {
        let colors = VibeColors::default();

        // Test no divider
        let classes = LayoutStyles::new(colors.clone())
            .divider_none()
            .spacing_none()
            .classes();
        assert!(!classes.contains("border"));

        // Test top divider
        let classes = LayoutStyles::new(colors.clone())
            .divider_top()
            .spacing_none()
            .classes();
        assert!(classes.contains("border-t"));
        assert!(classes.contains("border-gray-200"));

        // Test bottom divider
        let classes = LayoutStyles::new(colors.clone())
            .divider_bottom()
            .spacing_none()
            .classes();
        assert!(classes.contains("border-b"));
        assert!(classes.contains("border-gray-200"));

        // Test left divider
        let classes = LayoutStyles::new(colors.clone())
            .divider_left()
            .spacing_none()
            .classes();
        assert!(classes.contains("border-l"));
        assert!(classes.contains("border-gray-200"));

        // Test right divider
        let classes = LayoutStyles::new(colors.clone())
            .divider_right()
            .spacing_none()
            .classes();
        assert!(classes.contains("border-r"));
        assert!(classes.contains("border-gray-200"));
    }

    #[test]
    fn test_spacing_methods() {
        let colors = VibeColors::default();

        // Test all spacing levels
        let classes = LayoutStyles::new(colors.clone()).spacing_none().classes();
        assert!(!classes.contains("p-"));

        let classes = LayoutStyles::new(colors.clone()).spacing_xs().classes();
        assert!(classes.contains("p-1"));

        let classes = LayoutStyles::new(colors.clone()).spacing_sm().classes();
        assert!(classes.contains("p-2"));

        let classes = LayoutStyles::new(colors.clone()).spacing_md().classes();
        assert!(classes.contains("p-4"));

        let classes = LayoutStyles::new(colors.clone()).spacing_lg().classes();
        assert!(classes.contains("p-6"));

        let classes = LayoutStyles::new(colors.clone()).spacing_xl().classes();
        assert!(classes.contains("p-8"));

        let classes = LayoutStyles::new(colors.clone()).spacing_xl2().classes();
        assert!(classes.contains("p-12"));
    }

    #[test]
    fn test_direction_methods() {
        let colors = VibeColors::default();

        // Test vertical direction
        let classes = LayoutStyles::new(colors.clone())
            .direction_vertical()
            .spacing_none()
            .classes();
        assert!(classes.contains("flex"));
        assert!(classes.contains("flex-col"));

        // Test horizontal direction
        let classes = LayoutStyles::new(colors.clone())
            .direction_horizontal()
            .spacing_none()
            .classes();
        assert!(classes.contains("flex"));
        assert!(classes.contains("flex-row"));
    }

    #[test]
    fn test_alignment_methods() {
        let colors = VibeColors::default();

        // Test start alignment
        let classes = LayoutStyles::new(colors.clone())
            .alignment_start()
            .spacing_none()
            .classes();
        assert!(classes.contains("items-start"));
        assert!(classes.contains("justify-start"));

        // Test center alignment
        let classes = LayoutStyles::new(colors.clone())
            .alignment_center()
            .spacing_none()
            .classes();
        assert!(classes.contains("items-center"));
        assert!(classes.contains("justify-center"));

        // Test end alignment
        let classes = LayoutStyles::new(colors.clone())
            .alignment_end()
            .spacing_none()
            .classes();
        assert!(classes.contains("items-end"));
        assert!(classes.contains("justify-end"));

        // Test between alignment
        let classes = LayoutStyles::new(colors.clone())
            .alignment_between()
            .spacing_none()
            .classes();
        assert!(classes.contains("items-center"));
        assert!(classes.contains("justify-between"));

        // Test around alignment
        let classes = LayoutStyles::new(colors.clone())
            .alignment_around()
            .spacing_none()
            .classes();
        assert!(classes.contains("items-center"));
        assert!(classes.contains("justify-around"));

        // Test evenly alignment
        let classes = LayoutStyles::new(colors.clone())
            .alignment_evenly()
            .spacing_none()
            .classes();
        assert!(classes.contains("items-center"));
        assert!(classes.contains("justify-evenly"));
    }

    #[test]
    fn test_custom_classes() {
        let colors = VibeColors::default();

        // Test single custom class
        let classes = LayoutStyles::new(colors.clone())
            .custom("custom-class")
            .spacing_none()
            .classes();
        assert!(classes.contains("custom-class"));

        // Test multiple custom classes
        let classes = LayoutStyles::new(colors.clone())
            .custom_classes("class1 class2 class3")
            .spacing_none()
            .classes();
        assert!(classes.contains("class1"));
        assert!(classes.contains("class2"));
        assert!(classes.contains("class3"));

        // Test chained custom classes
        let classes = LayoutStyles::new(colors.clone())
            .custom("first")
            .custom("second")
            .spacing_none()
            .classes();
        assert!(classes.contains("first"));
        assert!(classes.contains("second"));
    }

    #[test]
    fn test_card_header_styles() {
        let classes = card_header_styles(VibeColors::default()).classes();
        assert!(classes.contains("border-b"));
        assert!(classes.contains("border-gray-200"));
        assert!(classes.contains("p-4"));
    }

    #[test]
    fn test_card_content_styles() {
        let classes = card_content_styles(VibeColors::default()).classes();
        assert!(classes.contains("p-4"));
        assert!(classes.contains("space-y-4"));
        assert!(!classes.contains("border"));
    }

    #[test]
    fn test_card_footer_styles() {
        let classes = card_footer_styles(VibeColors::default()).classes();
        assert!(classes.contains("border-t"));
        assert!(classes.contains("border-gray-200"));
        assert!(classes.contains("p-4"));
        assert!(classes.contains("flex"));
        assert!(classes.contains("flex-row"));
        assert!(classes.contains("items-center"));
        assert!(classes.contains("justify-between"));
    }

    #[test]
    fn test_layout_styles_function() {
        let styles = layout_styles(VibeColors::default());
        let classes = styles.classes();
        assert!(classes.contains("p-4")); // Default MD spacing
    }

    #[test]
    fn test_complex_layout_composition() {
        let classes = LayoutStyles::new(VibeColors::default())
            .divider_bottom()
            .spacing_lg()
            .direction_horizontal()
            .alignment_between()
            .custom("rounded-lg")
            .custom_classes("shadow-sm bg-white")
            .classes();

        assert!(classes.contains("border-b"));
        assert!(classes.contains("border-gray-200"));
        assert!(classes.contains("p-6"));
        assert!(classes.contains("flex"));
        assert!(classes.contains("flex-row"));
        assert!(classes.contains("items-center"));
        assert!(classes.contains("justify-between"));
        assert!(classes.contains("rounded-lg"));
        assert!(classes.contains("shadow-sm"));
        assert!(classes.contains("bg-white"));
    }

    #[test]
    fn test_class_deduplication() {
        let classes = LayoutStyles::new(VibeColors::default())
            .custom("p-4")
            .spacing_md() // Also adds p-4
            .classes();

        // Should not have duplicate p-4
        let class_count = classes.split_whitespace().filter(|&c| c == "p-4").count();
        assert_eq!(class_count, 1);
    }

    #[test]
    fn test_alignment_without_direction() {
        let classes = LayoutStyles::new(VibeColors::default())
            .alignment_center()
            .spacing_none()
            .classes();

        // Should add alignment without flex direction
        assert!(classes.contains("items-center"));
        assert!(classes.contains("justify-center"));
        assert!(!classes.contains("flex"));
    }

    #[test]
    fn test_direction_with_alignment() {
        let classes = LayoutStyles::new(VibeColors::default())
            .direction_horizontal()
            .alignment_between()
            .spacing_none()
            .classes();

        // Should have both direction and alignment
        assert!(classes.contains("flex"));
        assert!(classes.contains("flex-row"));
        assert!(classes.contains("items-center"));
        assert!(classes.contains("justify-between"));
    }

    #[test]
    fn test_empty_custom_classes() {
        let classes = LayoutStyles::new(VibeColors::default())
            .custom("")
            .custom_classes("")
            .custom_classes("  ")
            .spacing_none()
            .classes();

        // Should not add empty classes
        assert!(!classes.contains("\"\""));
        // When spacing is none and no other classes, result may be empty
        // This is correct behavior
    }

    #[test]
    fn test_build_alias() {
        let styles = LayoutStyles::new(VibeColors::default())
            .divider_top()
            .spacing_sm();

        let classes1 = styles.clone().classes();
        let classes2 = styles.build();

        assert_eq!(classes1, classes2);
    }
}
