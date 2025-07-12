//! Button usage example for the Jupiter Design System
//!
//! This example demonstrates how to use the styling utility ButtonStyles
//! to generate CSS classes for different button variants and states.

use jupiter_design_system::builders::button::{
    button_styles, button_styles_with_custom, ButtonState, ButtonStyles, ButtonVariant,
};
use jupiter_design_system::core::color::WaterWellnessColors;
use jupiter_design_system::prelude::*;

fn main() {
    println!("🔲 Jupiter Design System - Button Styling Example");
    println!("================================================");

    // Create a color provider
    let colors = WaterWellnessColors::new();
    println!("🎨 Created Water & Wellness color provider");

    // Example 1: Basic button variants
    println!("\n🎯 Basic Button Variants:");
    let primary = ButtonStyles::new(colors.clone()).primary().classes();
    println!("  Primary: {}", primary);

    let secondary = ButtonStyles::new(colors.clone()).secondary().classes();
    println!("  Secondary: {}", secondary);

    let success = ButtonStyles::new(colors.clone()).success().classes();
    println!("  Success: {}", success);

    let warning = ButtonStyles::new(colors.clone()).warning().classes();
    println!("  Warning: {}", warning);

    let error = ButtonStyles::new(colors.clone()).error().classes();
    println!("  Error: {}", error);

    let ghost = ButtonStyles::new(colors.clone()).ghost().classes();
    println!("  Ghost: {}", ghost);

    let link = ButtonStyles::new(colors.clone()).link().classes();
    println!("  Link: {}", link);

    // Example 2: Different sizes
    println!("\n📏 Button Sizes:");
    let xs = ButtonStyles::new(colors.clone())
        .primary()
        .extra_small()
        .classes();
    println!("  XSmall: {}", xs);

    let sm = ButtonStyles::new(colors.clone())
        .primary()
        .small()
        .classes();
    println!("  Small: {}", sm);

    let md = ButtonStyles::new(colors.clone())
        .primary()
        .medium()
        .classes();
    println!("  Medium: {}", md);

    let lg = ButtonStyles::new(colors.clone())
        .primary()
        .large()
        .classes();
    println!("  Large: {}", lg);

    let xl = ButtonStyles::new(colors.clone())
        .primary()
        .extra_large()
        .classes();
    println!("  XLarge: {}", xl);

    // Example 3: Button states
    println!("\n🎭 Button States:");
    let hover = ButtonStyles::new(colors.clone())
        .primary()
        .hover()
        .classes();
    println!("  Hover: {}", hover);

    let active = ButtonStyles::new(colors.clone())
        .primary()
        .active()
        .classes();
    println!("  Active: {}", active);

    let disabled = ButtonStyles::new(colors.clone())
        .primary()
        .disabled()
        .classes();
    println!("  Disabled: {}", disabled);

    let loading = ButtonStyles::new(colors.clone())
        .primary()
        .loading()
        .classes();
    println!("  Loading: {}", loading);

    // Example 4: Special configurations
    println!("\n⚙️  Special Configurations:");
    let full_width = ButtonStyles::new(colors.clone())
        .primary()
        .full_width()
        .classes();
    println!("  Full Width: {}", full_width);

    let with_icon = ButtonStyles::new(colors.clone())
        .primary()
        .with_icon()
        .classes();
    println!("  With Icon: {}", with_icon);

    // Example 5: Fluent API demonstration
    println!("\n🌊 Fluent API Example:");
    let complex_button = ButtonStyles::new(colors.clone())
        .success()
        .large()
        .hover()
        .full_width()
        .with_icon()
        .classes();
    println!("  Complex Button: {}", complex_button);

    // Example 6: Custom theme
    println!("\n🎨 Custom Theme Example:");
    let custom_colors = WaterWellnessColors::with_overrides(|palette| {
        palette.primary = "purple-600".to_string();
        palette.secondary = "pink-500".to_string();
    });

    let custom_primary = ButtonStyles::new(custom_colors.clone()).primary().classes();
    println!("  Custom Primary: {}", custom_primary);

    let custom_secondary = ButtonStyles::new(custom_colors.clone())
        .secondary()
        .classes();
    println!("  Custom Secondary: {}", custom_secondary);

    // Example 7: Custom CSS classes
    println!("\n🎨 Custom CSS Classes Example:");
    let custom_button = ButtonStyles::new(colors.clone())
        .primary()
        .large()
        .custom("shadow-xl")
        .custom("transform")
        .custom_classes("hover:scale-110 transition-transform")
        .classes();
    println!("  With Custom Classes: {}", custom_button);

    // Example 8: Convenience functions
    println!("\n🚀 Convenience Functions:");
    let quick_button = button_styles(colors.clone())
        .warning()
        .small()
        .full_width()
        .classes();
    println!("  Quick Button: {}", quick_button);

    let button_with_custom = button_styles_with_custom(colors.clone(), "animate-pulse")
        .success()
        .medium()
        .classes();
    println!("  Button with Custom: {}", button_with_custom);

    // Example 9: Component comparison
    println!("\n📊 Component Comparison:");
    println!("  Same variant, different providers:");
    let ww_button = ButtonStyles::new(WaterWellnessColors::new())
        .primary()
        .classes();
    println!("    Water & Wellness: {}", ww_button);

    let custom_button = ButtonStyles::new(WaterWellnessColors::with_overrides(|p| {
        p.primary = "indigo-600".to_string();
    }))
    .primary()
    .classes();
    println!("    Custom Theme: {}", custom_button);

    println!("\n✅ Button styling example completed successfully!");
    println!("💡 Notice how the styling utility approach allows for:");
    println!("   - Pure CSS class generation");
    println!("   - Framework-agnostic usage");
    println!("   - Type-safe color management");
    println!("   - Consistent styling across themes");
    println!("   - Fluent builder pattern");
    println!("   - Custom CSS class injection");
    println!("   - Easy customization and extension");
}
