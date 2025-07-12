//! Tests for the color system

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_enum_variants() {
        // Test that all color variants exist
        let colors = vec![
            Color::Primary,
            Color::Secondary,
            Color::Success,
            Color::Warning,
            Color::Error,
            Color::Info,
            Color::Surface,
            Color::Background,
            Color::Foreground,
            Color::Border,
            Color::TextPrimary,
            Color::TextSecondary,
            Color::TextTertiary,
            Color::TextInverse,
            Color::Interactive,
            Color::InteractiveHover,
            Color::InteractiveActive,
            Color::InteractiveDisabled,
        ];

        assert_eq!(colors.len(), 18);
    }

    #[test]
    fn test_color_palette_default() {
        let palette = ColorPalette::default();

        // Test brand colors
        assert_eq!(palette.primary, "water-blue-500");
        assert_eq!(palette.secondary, "water-green-500");

        // Test semantic colors
        assert_eq!(palette.success, "green-500");
        assert_eq!(palette.warning, "amber-500");
        assert_eq!(palette.error, "red-500");
        assert_eq!(palette.info, "blue-500");

        // Test neutral colors
        assert_eq!(palette.surface, "white");
        assert_eq!(palette.background, "gray-50");
        assert_eq!(palette.foreground, "gray-900");
        assert_eq!(palette.border, "gray-200");

        // Test text colors
        assert_eq!(palette.text_primary, "gray-900");
        assert_eq!(palette.text_secondary, "gray-600");
        assert_eq!(palette.text_tertiary, "gray-400");
        assert_eq!(palette.text_inverse, "white");

        // Test interactive states
        assert_eq!(palette.interactive, "water-blue-500");
        assert_eq!(palette.interactive_hover, "water-blue-600");
        assert_eq!(palette.interactive_active, "water-blue-700");
        assert_eq!(palette.interactive_disabled, "gray-300");
    }

    #[test]
    fn test_water_wellness_colors_default() {
        let colors = WaterWellnessColors::default();
        let palette = colors.palette();

        assert_eq!(palette.primary, "water-blue-500");
        assert_eq!(palette.secondary, "water-green-500");
    }

    #[test]
    fn test_water_wellness_colors_new() {
        let colors = WaterWellnessColors::new();
        let palette = colors.palette();

        assert_eq!(palette.primary, "water-blue-500");
        assert_eq!(palette.secondary, "water-green-500");
    }

    #[test]
    fn test_color_provider_resolve_color() {
        let colors = WaterWellnessColors::default();

        // Test brand colors
        assert_eq!(colors.resolve_color(Color::Primary), "water-blue-500");
        assert_eq!(colors.resolve_color(Color::Secondary), "water-green-500");

        // Test semantic colors
        assert_eq!(colors.resolve_color(Color::Success), "green-500");
        assert_eq!(colors.resolve_color(Color::Warning), "amber-500");
        assert_eq!(colors.resolve_color(Color::Error), "red-500");
        assert_eq!(colors.resolve_color(Color::Info), "blue-500");

        // Test neutral colors
        assert_eq!(colors.resolve_color(Color::Surface), "white");
        assert_eq!(colors.resolve_color(Color::Background), "gray-50");
        assert_eq!(colors.resolve_color(Color::Foreground), "gray-900");
        assert_eq!(colors.resolve_color(Color::Border), "gray-200");

        // Test text colors
        assert_eq!(colors.resolve_color(Color::TextPrimary), "gray-900");
        assert_eq!(colors.resolve_color(Color::TextSecondary), "gray-600");
        assert_eq!(colors.resolve_color(Color::TextTertiary), "gray-400");
        assert_eq!(colors.resolve_color(Color::TextInverse), "white");

        // Test interactive states
        assert_eq!(colors.resolve_color(Color::Interactive), "water-blue-500");
        assert_eq!(
            colors.resolve_color(Color::InteractiveHover),
            "water-blue-600"
        );
        assert_eq!(
            colors.resolve_color(Color::InteractiveActive),
            "water-blue-700"
        );
        assert_eq!(colors.resolve_color(Color::InteractiveDisabled), "gray-300");
    }

    #[test]
    fn test_color_provider_text_class() {
        let colors = WaterWellnessColors::default();

        assert_eq!(colors.text_class(Color::Primary), "text-water-blue-500");
        assert_eq!(colors.text_class(Color::Success), "text-green-500");
        assert_eq!(colors.text_class(Color::Error), "text-red-500");
        assert_eq!(colors.text_class(Color::TextPrimary), "text-gray-900");
    }

    #[test]
    fn test_color_provider_bg_class() {
        let colors = WaterWellnessColors::default();

        assert_eq!(colors.bg_class(Color::Primary), "bg-water-blue-500");
        assert_eq!(colors.bg_class(Color::Surface), "bg-white");
        assert_eq!(colors.bg_class(Color::Background), "bg-gray-50");
        assert_eq!(colors.bg_class(Color::Success), "bg-green-500");
    }

    #[test]
    fn test_color_provider_border_class() {
        let colors = WaterWellnessColors::default();

        assert_eq!(colors.border_class(Color::Primary), "border-water-blue-500");
        assert_eq!(colors.border_class(Color::Border), "border-gray-200");
        assert_eq!(colors.border_class(Color::Error), "border-red-500");
    }

    #[test]
    fn test_water_wellness_colors_with_overrides() {
        let colors = WaterWellnessColors::with_overrides(|palette| {
            palette.primary = "custom-blue-500".to_string();
            palette.secondary = "custom-green-500".to_string();
        });

        assert_eq!(colors.resolve_color(Color::Primary), "custom-blue-500");
        assert_eq!(colors.resolve_color(Color::Secondary), "custom-green-500");
        // Other colors should remain default
        assert_eq!(colors.resolve_color(Color::Success), "green-500");
    }

    #[test]
    fn test_color_serialization() {
        let color = Color::Primary;
        let serialized = serde_json::to_string(&color).unwrap();
        let deserialized: Color = serde_json::from_str(&serialized).unwrap();

        assert_eq!(color, deserialized);
    }

    #[test]
    fn test_color_palette_serialization() {
        let palette = ColorPalette::default();
        let serialized = serde_json::to_string(&palette).unwrap();
        let deserialized: ColorPalette = serde_json::from_str(&serialized).unwrap();

        assert_eq!(palette, deserialized);
    }
}
