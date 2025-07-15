#[cfg(test)]
mod tests {
    use crate::builders::selection::{
        chip_selection_styles, filter_selection_styles, selection_classes_from_strings,
        selection_styles, tab_selection_styles, SelectionStyles,
    };
    use crate::themes::VibeColors;

    #[test]
    fn test_selection_styles_new() {
        let styles = SelectionStyles::new(VibeColors::default());
        let container_classes = styles.clone().container_classes();
        let item_classes = styles.item_classes();

        // Default should be single selection, button display, horizontal layout
        assert!(container_classes.contains("selection-pattern"));
        assert!(container_classes.contains("flex"));
        assert!(container_classes.contains("flex-row"));
        assert!(container_classes.contains("gap-2"));

        assert!(item_classes.contains("selection-item"));
        assert!(item_classes.contains("inline-flex"));
        assert!(item_classes.contains("rounded-md"));
        assert!(item_classes.contains("px-4"));
        assert!(item_classes.contains("py-2"));
    }

    #[test]
    fn test_behavior_methods() {
        let colors = VibeColors::default();

        // Test no selection
        let _styles = SelectionStyles::new(colors.clone()).no_selection();
        // Behavior affects semantic info more than classes

        // Test single selection
        let styles = SelectionStyles::new(colors.clone()).single_selection();
        let container_classes = styles.container_classes();
        assert!(container_classes.contains("selection-pattern"));

        // Test multiple selection
        let styles = SelectionStyles::new(colors.clone()).multiple_selection();
        let container_classes = styles.container_classes();
        assert!(container_classes.contains("selection-pattern"));

        // Test toggle selection
        let styles = SelectionStyles::new(colors.clone()).toggle_selection();
        let container_classes = styles.container_classes();
        assert!(container_classes.contains("selection-pattern"));
    }

    #[test]
    fn test_state_methods() {
        let colors = VibeColors::default();

        // Test unselected state
        let item_classes = SelectionStyles::new(colors.clone())
            .unselected()
            .item_classes();
        assert!(item_classes.contains("bg-white"));
        assert!(item_classes.contains("text-gray-900"));
        assert!(item_classes.contains("border-gray-200"));

        // Test selected state
        let item_classes = SelectionStyles::new(colors.clone())
            .selected()
            .item_classes();
        assert!(item_classes.contains("bg-jupiter-blue-500"));
        assert!(item_classes.contains("text-white"));
        assert!(item_classes.contains("border-jupiter-blue-500"));

        // Test partially selected state
        let item_classes = SelectionStyles::new(colors.clone())
            .partially_selected()
            .item_classes();
        assert!(item_classes.contains("bg-gray-50"));
        assert!(item_classes.contains("text-jupiter-blue-500"));
        assert!(item_classes.contains("border-jupiter-blue-500"));

        // Test disabled state
        let item_classes = SelectionStyles::new(colors.clone())
            .disabled()
            .item_classes();
        assert!(item_classes.contains("bg-gray-300"));
        assert!(item_classes.contains("text-gray-400"));
        assert!(item_classes.contains("cursor-not-allowed"));
    }

    #[test]
    fn test_display_methods() {
        let colors = VibeColors::default();

        // Test button display
        let item_classes = SelectionStyles::new(colors.clone())
            .button_display()
            .item_classes();
        assert!(item_classes.contains("inline-flex"));
        assert!(item_classes.contains("rounded-md"));

        // Test chip display
        let item_classes = SelectionStyles::new(colors.clone())
            .chip_display()
            .item_classes();
        assert!(item_classes.contains("inline-flex"));
        assert!(item_classes.contains("rounded-full"));

        // Test list item display
        let item_classes = SelectionStyles::new(colors.clone())
            .list_item_display()
            .item_classes();
        assert!(item_classes.contains("flex"));
        assert!(item_classes.contains("w-full"));

        // Test card display
        let item_classes = SelectionStyles::new(colors.clone())
            .card_display()
            .item_classes();
        assert!(item_classes.contains("flex"));
        assert!(item_classes.contains("flex-col"));
        assert!(item_classes.contains("rounded-lg"));
        assert!(item_classes.contains("border"));

        // Test tab display
        let item_classes = SelectionStyles::new(colors.clone())
            .tab_display()
            .item_classes();
        assert!(item_classes.contains("flex"));
        assert!(item_classes.contains("border-b-2"));
    }

    #[test]
    fn test_layout_methods() {
        let colors = VibeColors::default();

        // Test horizontal layout
        let container_classes = SelectionStyles::new(colors.clone())
            .horizontal_layout()
            .container_classes();
        assert!(container_classes.contains("flex-row"));

        // Test vertical layout
        let container_classes = SelectionStyles::new(colors.clone())
            .vertical_layout()
            .container_classes();
        assert!(container_classes.contains("flex-col"));

        // Test grid layout
        let container_classes = SelectionStyles::new(colors.clone())
            .grid_layout()
            .container_classes();
        assert!(container_classes.contains("grid"));

        // Test dropdown layout
        let container_classes = SelectionStyles::new(colors.clone())
            .dropdown_layout()
            .container_classes();
        assert!(container_classes.contains("relative"));

        // Test inline layout
        let container_classes = SelectionStyles::new(colors.clone())
            .inline_layout()
            .container_classes();
        assert!(container_classes.contains("flex-wrap"));
    }

    #[test]
    fn test_size_methods() {
        let colors = VibeColors::default();

        // Test XS size
        let classes = SelectionStyles::new(colors.clone()).xs();
        let container_classes = classes.clone().container_classes();
        let item_classes = classes.item_classes();
        assert!(container_classes.contains("gap-1"));
        assert!(item_classes.contains("px-2"));
        assert!(item_classes.contains("py-1"));
        assert!(item_classes.contains("text-xs"));

        // Test SM size
        let classes = SelectionStyles::new(colors.clone()).sm();
        let item_classes = classes.item_classes();
        assert!(item_classes.contains("px-3"));
        assert!(item_classes.contains("py-1.5"));
        assert!(item_classes.contains("text-sm"));

        // Test MD size (default)
        let classes = SelectionStyles::new(colors.clone()).md();
        let container_classes = classes.clone().container_classes();
        let item_classes = classes.item_classes();
        assert!(container_classes.contains("gap-2"));
        assert!(item_classes.contains("px-4"));
        assert!(item_classes.contains("py-2"));
        assert!(item_classes.contains("text-base"));

        // Test LG size
        let classes = SelectionStyles::new(colors.clone()).lg();
        let container_classes = classes.clone().container_classes();
        let item_classes = classes.item_classes();
        assert!(container_classes.contains("gap-3"));
        assert!(item_classes.contains("px-6"));
        assert!(item_classes.contains("py-3"));
        assert!(item_classes.contains("text-lg"));

        // Test XL size
        let classes = SelectionStyles::new(colors.clone()).xl();
        let container_classes = classes.clone().container_classes();
        let item_classes = classes.item_classes();
        assert!(container_classes.contains("gap-4"));
        assert!(item_classes.contains("px-8"));
        assert!(item_classes.contains("py-4"));
        assert!(item_classes.contains("text-xl"));
    }

    #[test]
    fn test_interaction_methods() {
        let colors = VibeColors::default();

        // Test subtle interaction
        let item_classes = SelectionStyles::new(colors.clone())
            .subtle_interaction()
            .unselected()
            .item_classes();
        assert!(item_classes.contains("cursor-pointer"));
        assert!(item_classes.contains("hover:opacity-80"));

        // Test standard interaction
        let item_classes = SelectionStyles::new(colors.clone())
            .standard_interaction()
            .unselected()
            .item_classes();
        assert!(item_classes.contains("cursor-pointer"));
        assert!(item_classes.contains("hover:scale-105"));
        assert!(item_classes.contains("active:scale-95"));

        // Test prominent interaction
        let item_classes = SelectionStyles::new(colors.clone())
            .prominent_interaction()
            .unselected()
            .item_classes();
        assert!(item_classes.contains("cursor-pointer"));
        assert!(item_classes.contains("hover:scale-110"));
        assert!(item_classes.contains("active:scale-90"));
        assert!(item_classes.contains("shadow-lg"));
    }

    #[test]
    fn test_feature_methods() {
        let colors = VibeColors::default();

        // Test with counts enabled
        let styles = SelectionStyles::new(colors.clone()).with_counts(true);
        let count_classes = styles.count_classes();
        assert!(!count_classes.is_empty());
        assert!(count_classes.contains("ml-2"));
        assert!(count_classes.contains("px-2"));
        assert!(count_classes.contains("rounded-full"));

        // Test with counts disabled
        let styles = SelectionStyles::new(colors.clone()).with_counts(false);
        let count_classes = styles.count_classes();
        assert!(count_classes.is_empty());

        // Test with clear all enabled
        let _styles = SelectionStyles::new(colors.clone()).with_clear_all(true);
        // Clear all affects semantic info more than classes

        // Test with clear all disabled
        let _styles = SelectionStyles::new(colors.clone()).with_clear_all(false);
        // Clear all affects semantic info more than classes
    }

    #[test]
    fn test_string_convenience_methods() {
        let colors = VibeColors::default();

        // Test behavior_str
        let container_classes = SelectionStyles::new(colors.clone())
            .behavior_str("multiple")
            .container_classes();
        assert!(container_classes.contains("selection-pattern"));

        // Test state_str
        let item_classes = SelectionStyles::new(colors.clone())
            .state_str("selected")
            .item_classes();
        assert!(item_classes.contains("bg-jupiter-blue-500"));

        // Test display_str
        let item_classes = SelectionStyles::new(colors.clone())
            .display_str("chip")
            .item_classes();
        assert!(item_classes.contains("rounded-full"));

        // Test layout_str
        let container_classes = SelectionStyles::new(colors.clone())
            .layout_str("vertical")
            .container_classes();
        assert!(container_classes.contains("flex-col"));

        // Test size_str
        let item_classes = SelectionStyles::new(colors.clone())
            .size_str("lg")
            .item_classes();
        assert!(item_classes.contains("px-6"));
        assert!(item_classes.contains("text-lg"));

        // Test interaction_str
        let item_classes = SelectionStyles::new(colors.clone())
            .interaction_str("prominent")
            .unselected()
            .item_classes();
        assert!(item_classes.contains("hover:scale-110"));
    }

    #[test]
    fn test_custom_classes() {
        let colors = VibeColors::default();

        // Test single custom class
        let container_classes = SelectionStyles::new(colors.clone())
            .custom("custom-class")
            .container_classes();
        assert!(container_classes.contains("custom-class"));

        // Test multiple custom classes
        let container_classes = SelectionStyles::new(colors.clone())
            .custom_classes("class1 class2 class3")
            .container_classes();
        assert!(container_classes.contains("class1"));
        assert!(container_classes.contains("class2"));
        assert!(container_classes.contains("class3"));
    }

    #[test]
    fn test_convenience_functions() {
        let colors = VibeColors::default();

        // Test selection_styles function
        let styles = selection_styles(colors.clone());
        let container_classes = styles.container_classes();
        assert!(container_classes.contains("selection-pattern"));

        // Test filter_selection_styles function
        let classes = filter_selection_styles(colors.clone());
        let container_classes = classes.clone().container_classes();
        let item_classes = classes.item_classes();
        assert!(container_classes.contains("flex-row"));
        assert!(item_classes.contains("rounded-md"));

        // Test chip_selection_styles function
        let classes = chip_selection_styles(colors.clone());
        let container_classes = classes.clone().container_classes();
        let item_classes = classes.item_classes();
        assert!(container_classes.contains("flex-wrap"));
        assert!(item_classes.contains("rounded-full"));

        // Test tab_selection_styles function
        let classes = tab_selection_styles(colors.clone());
        let container_classes = classes.clone().container_classes();
        let item_classes = classes.item_classes();
        assert!(container_classes.contains("flex-row"));
        assert!(item_classes.contains("border-b-2"));
    }

    #[test]
    fn test_selection_classes_from_strings() {
        let colors = VibeColors::default();

        let (container_classes, item_classes) = selection_classes_from_strings(
            colors,
            "single",
            "selected",
            "button",
            "horizontal",
            "lg",
            "standard",
            true,
        );

        assert!(container_classes.contains("selection-pattern"));
        assert!(container_classes.contains("flex-row"));
        assert!(container_classes.contains("gap-3"));

        assert!(item_classes.contains("selection-item"));
        assert!(item_classes.contains("bg-jupiter-blue-500"));
        assert!(item_classes.contains("text-white"));
        assert!(item_classes.contains("px-6"));
        assert!(item_classes.contains("py-3"));
        assert!(item_classes.contains("text-lg"));
        assert!(item_classes.contains("cursor-pointer"));
    }

    #[test]
    fn test_count_classes_with_states() {
        let colors = VibeColors::default();

        // Test count classes with selected state
        let count_classes = SelectionStyles::new(colors.clone())
            .selected()
            .with_counts(true)
            .count_classes();
        assert!(count_classes.contains("bg-jupiter-blue-500"));
        assert!(count_classes.contains("text-white"));

        // Test count classes with unselected state
        let count_classes = SelectionStyles::new(colors.clone())
            .unselected()
            .with_counts(true)
            .count_classes();
        assert!(count_classes.contains("bg-gray-50"));
        assert!(count_classes.contains("text-gray-600"));
    }

    #[test]
    fn test_chip_display_size_variations() {
        let colors = VibeColors::default();

        // Test chip display with different sizes
        let xs_classes = SelectionStyles::new(colors.clone())
            .chip_display()
            .xs()
            .item_classes();
        assert!(xs_classes.contains("px-2"));
        assert!(xs_classes.contains("py-0.5"));
        assert!(xs_classes.contains("text-xs"));

        let lg_classes = SelectionStyles::new(colors.clone())
            .chip_display()
            .lg()
            .item_classes();
        assert!(lg_classes.contains("px-4"));
        assert!(lg_classes.contains("py-2"));
        assert!(lg_classes.contains("text-lg"));
    }

    #[test]
    fn test_disabled_interaction_override() {
        let colors = VibeColors::default();

        // Test that disabled state overrides interaction styles
        let item_classes = SelectionStyles::new(colors.clone())
            .prominent_interaction()
            .disabled()
            .item_classes();
        assert!(item_classes.contains("cursor-not-allowed"));
        assert!(!item_classes.contains("hover:scale-110"));
    }

    #[test]
    fn test_complex_selection_composition() {
        let colors = VibeColors::default();

        let styles = SelectionStyles::new(colors)
            .multiple_selection()
            .chip_display()
            .inline_layout()
            .lg()
            .prominent_interaction()
            .with_counts(true)
            .with_clear_all(true)
            .custom("rounded-xl")
            .custom_classes("shadow-md border-2");

        let container_classes = styles.clone().container_classes();
        let item_classes = styles.selected().item_classes();

        assert!(container_classes.contains("selection-pattern"));
        assert!(container_classes.contains("flex-wrap"));
        assert!(container_classes.contains("gap-3"));
        assert!(container_classes.contains("rounded-xl"));
        assert!(container_classes.contains("shadow-md"));
        assert!(container_classes.contains("border-2"));

        assert!(item_classes.contains("selection-item"));
        assert!(item_classes.contains("rounded-full"));
        assert!(item_classes.contains("px-4"));
        assert!(item_classes.contains("py-2"));
        assert!(item_classes.contains("text-lg"));
        assert!(item_classes.contains("bg-jupiter-blue-500"));
        assert!(item_classes.contains("cursor-pointer"));
    }

    #[test]
    fn test_class_deduplication() {
        let colors = VibeColors::default();

        let container_classes = SelectionStyles::new(colors)
            .custom("flex")
            .horizontal_layout() // Also adds flex
            .container_classes();

        // Should not have duplicate flex
        let class_count = container_classes
            .split_whitespace()
            .filter(|&c| c == "flex")
            .count();
        assert_eq!(class_count, 1);
    }

    #[test]
    fn test_fallback_values() {
        let colors = VibeColors::default();

        // Test invalid string values fallback to defaults
        let styles = SelectionStyles::new(colors)
            .behavior_str("invalid")
            .state_str("invalid")
            .display_str("invalid")
            .layout_str("invalid")
            .size_str("invalid")
            .interaction_str("invalid");

        let container_classes = styles.clone().container_classes();
        let item_classes = styles.item_classes();

        // Should fallback to single selection, unselected, button, horizontal, md, standard
        assert!(container_classes.contains("flex-row"));
        assert!(container_classes.contains("gap-2"));
        assert!(item_classes.contains("rounded-md"));
        assert!(item_classes.contains("px-4"));
        assert!(item_classes.contains("py-2"));
        assert!(item_classes.contains("bg-white"));
    }
}
