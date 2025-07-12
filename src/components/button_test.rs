//! Tests for the button component

#[cfg(test)]
mod tests {
    use crate::components::button::{
        Button, ButtonBuilder, ButtonState, ButtonStyle, ButtonVariant,
    };
    use crate::core::color::{ColorProvider, WaterWellnessColors};
    use crate::core::Size;

    fn create_test_colors() -> WaterWellnessColors {
        WaterWellnessColors::new()
    }

    #[test]
    fn test_button_default() {
        let colors = create_test_colors();
        let button = Button::new(colors);
        let classes = button.build();

        assert!(classes.contains("inline-flex"));
        assert!(classes.contains("items-center"));
        assert!(classes.contains("justify-center"));
        assert!(classes.contains("font-medium"));
        assert!(classes.contains("transition-colors"));
        assert!(classes.contains("px-4 py-2")); // medium size
        assert!(classes.contains("bg-water-blue-500")); // primary variant
    }

    #[test]
    fn test_button_variants() {
        let colors = create_test_colors();

        // Primary
        let primary = Button::primary(colors.clone()).build();
        assert!(primary.contains("bg-water-blue-500"));
        assert!(primary.contains("text-white"));

        // Secondary
        let secondary = Button::secondary(colors.clone()).build();
        assert!(secondary.contains("bg-white"));
        assert!(secondary.contains("text-gray-900"));
        assert!(secondary.contains("border"));

        // Success
        let success = Button::success(colors.clone()).build();
        assert!(success.contains("bg-green-500"));
        assert!(success.contains("text-white"));

        // Warning
        let warning = Button::warning(colors.clone()).build();
        assert!(warning.contains("bg-amber-500"));
        assert!(warning.contains("text-white"));

        // Error
        let error = Button::error(colors.clone()).build();
        assert!(error.contains("bg-red-500"));
        assert!(error.contains("text-white"));

        // Ghost
        let ghost = Button::ghost(colors.clone()).build();
        assert!(ghost.contains("bg-transparent"));
        assert!(ghost.contains("text-gray-900"));

        // Link
        let link = Button::link(colors.clone()).build();
        assert!(link.contains("bg-transparent"));
        assert!(link.contains("text-water-blue-500"));
        assert!(link.contains("hover:underline"));
    }

    #[test]
    fn test_button_sizes() {
        let colors = create_test_colors();

        // XSmall
        let xs = Button::new(colors.clone()).size(Size::XSmall).build();
        assert!(xs.contains("px-2 py-1"));
        assert!(xs.contains("text-xs"));
        assert!(xs.contains("rounded"));

        // Small
        let sm = Button::new(colors.clone()).size(Size::Small).build();
        assert!(sm.contains("px-3 py-1.5"));
        assert!(sm.contains("text-sm"));
        assert!(sm.contains("rounded"));

        // Medium
        let md = Button::new(colors.clone()).size(Size::Medium).build();
        assert!(md.contains("px-4 py-2"));
        assert!(md.contains("text-sm"));
        assert!(md.contains("rounded-md"));

        // Large
        let lg = Button::new(colors.clone()).size(Size::Large).build();
        assert!(lg.contains("px-6 py-3"));
        assert!(lg.contains("text-base"));
        assert!(lg.contains("rounded-md"));

        // XLarge
        let xl = Button::new(colors.clone()).size(Size::XLarge).build();
        assert!(xl.contains("px-8 py-4"));
        assert!(xl.contains("text-lg"));
        assert!(xl.contains("rounded-lg"));
    }

    #[test]
    fn test_button_states() {
        let colors = create_test_colors();

        // Default
        let default = Button::new(colors.clone())
            .state(ButtonState::Default)
            .build();
        assert!(!default.contains("hover:scale"));

        // Hover
        let hover = Button::new(colors.clone())
            .state(ButtonState::Hover)
            .build();
        assert!(hover.contains("hover:scale-105"));

        // Active
        let active = Button::new(colors.clone())
            .state(ButtonState::Active)
            .build();
        assert!(active.contains("active:scale-95"));

        // Disabled
        let disabled = Button::new(colors.clone())
            .state(ButtonState::Disabled)
            .build();
        assert!(disabled.contains("opacity-50"));
        assert!(disabled.contains("cursor-not-allowed"));

        // Loading
        let loading = Button::new(colors.clone())
            .state(ButtonState::Loading)
            .build();
        assert!(loading.contains("cursor-wait"));
    }

    #[test]
    fn test_button_full_width() {
        let colors = create_test_colors();

        // Not full width
        let not_full = Button::new(colors.clone()).full_width(false).build();
        assert!(!not_full.contains("w-full"));

        // Full width
        let full = Button::new(colors.clone()).full_width(true).build();
        assert!(full.contains("w-full"));
    }

    #[test]
    fn test_button_with_icon() {
        let colors = create_test_colors();

        // Without icon
        let no_icon = Button::new(colors.clone()).with_icon(false).build();
        assert!(!no_icon.contains("space-x-2"));

        // With icon
        let with_icon = Button::new(colors.clone()).with_icon(true).build();
        assert!(with_icon.contains("space-x-2"));
    }

    #[test]
    fn test_button_fluent_api() {
        let colors = create_test_colors();

        let button = Button::new(colors)
            .variant(ButtonVariant::Success)
            .size(Size::Large)
            .state(ButtonState::Hover)
            .full_width(true)
            .with_icon(true)
            .build();

        assert!(button.contains("bg-green-500")); // success variant
        assert!(button.contains("px-6 py-3")); // large size
        assert!(button.contains("hover:scale-105")); // hover state
        assert!(button.contains("w-full")); // full width
        assert!(button.contains("space-x-2")); // with icon
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
    fn test_button_style_default() {
        let style = ButtonStyle::default();
        assert_eq!(style.variant, ButtonVariant::Primary);
        assert_eq!(style.size, Size::Medium);
        assert_eq!(style.state, ButtonState::Default);
        assert!(!style.full_width);
        assert!(!style.with_icon);
    }

    #[test]
    fn test_button_custom_colors() {
        let custom_colors = WaterWellnessColors::with_overrides(|palette| {
            palette.primary = "custom-blue-500".to_string();
        });

        let button = Button::primary(custom_colors).build();
        assert!(button.contains("bg-custom-blue-500"));
    }
}
