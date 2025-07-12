//! Button usage example for the Jupiter Design System
//!
//! This example demonstrates how to use the trait-based Button component
//! builder to generate CSS classes for different button variants and states.

use jupiter_design_system::components::button::{
    Button, ButtonBuilder, ButtonState, ButtonVariant,
};
use jupiter_design_system::core::color::{ColorProvider, WaterWellnessColors};
use jupiter_design_system::prelude::*;

fn main() {
    println!("🔲 Jupiter Design System - Button Usage Example");
    println!("===============================================");

    // Create a color provider
    let colors = WaterWellnessColors::new();
    println!("🎨 Created Water & Wellness color provider");

    // Example 1: Basic button variants
    println!("\n🎯 Basic Button Variants:");
    let primary = Button::primary(colors.clone()).build();
    println!("  Primary: {}", primary);

    let secondary = Button::secondary(colors.clone()).build();
    println!("  Secondary: {}", secondary);

    let success = Button::success(colors.clone()).build();
    println!("  Success: {}", success);

    let warning = Button::warning(colors.clone()).build();
    println!("  Warning: {}", warning);

    let error = Button::error(colors.clone()).build();
    println!("  Error: {}", error);

    let ghost = Button::ghost(colors.clone()).build();
    println!("  Ghost: {}", ghost);

    let link = Button::link(colors.clone()).build();
    println!("  Link: {}", link);

    // Example 2: Different sizes
    println!("\n📏 Button Sizes:");
    let xs = Button::primary(colors.clone()).size(Size::XSmall).build();
    println!("  XSmall: {}", xs);

    let sm = Button::primary(colors.clone()).size(Size::Small).build();
    println!("  Small: {}", sm);

    let md = Button::primary(colors.clone()).size(Size::Medium).build();
    println!("  Medium: {}", md);

    let lg = Button::primary(colors.clone()).size(Size::Large).build();
    println!("  Large: {}", lg);

    let xl = Button::primary(colors.clone()).size(Size::XLarge).build();
    println!("  XLarge: {}", xl);

    // Example 3: Button states
    println!("\n🎭 Button States:");
    let hover = Button::primary(colors.clone())
        .state(ButtonState::Hover)
        .build();
    println!("  Hover: {}", hover);

    let active = Button::primary(colors.clone())
        .state(ButtonState::Active)
        .build();
    println!("  Active: {}", active);

    let disabled = Button::primary(colors.clone())
        .state(ButtonState::Disabled)
        .build();
    println!("  Disabled: {}", disabled);

    let loading = Button::primary(colors.clone())
        .state(ButtonState::Loading)
        .build();
    println!("  Loading: {}", loading);

    // Example 4: Special configurations
    println!("\n⚙️  Special Configurations:");
    let full_width = Button::primary(colors.clone()).full_width(true).build();
    println!("  Full Width: {}", full_width);

    let with_icon = Button::primary(colors.clone()).with_icon(true).build();
    println!("  With Icon: {}", with_icon);

    // Example 5: Fluent API demonstration
    println!("\n🌊 Fluent API Example:");
    let complex_button = Button::new(colors.clone())
        .variant(ButtonVariant::Success)
        .size(Size::Large)
        .state(ButtonState::Hover)
        .full_width(true)
        .with_icon(true)
        .build();
    println!("  Complex Button: {}", complex_button);

    // Example 6: Custom theme
    println!("\n🎨 Custom Theme Example:");
    let custom_colors = WaterWellnessColors::with_overrides(|palette| {
        palette.primary = "purple-600".to_string();
        palette.secondary = "pink-500".to_string();
    });

    let custom_primary = Button::primary(custom_colors.clone()).build();
    println!("  Custom Primary: {}", custom_primary);

    let custom_secondary = Button::secondary(custom_colors.clone()).build();
    println!("  Custom Secondary: {}", custom_secondary);

    // Example 7: Trait-based approach
    println!("\n🏗️  Trait-based Builder Pattern:");
    let trait_button = Button::new(colors.clone())
        .variant(ButtonVariant::Warning)
        .size(Size::Small)
        .state(ButtonState::Default)
        .full_width(false)
        .with_icon(false)
        .build();
    println!("  Trait Builder: {}", trait_button);

    // Example 8: Component comparison
    println!("\n📊 Component Comparison:");
    println!("  Same variant, different providers:");
    let ww_button = Button::primary(WaterWellnessColors::new()).build();
    println!("    Water & Wellness: {}", ww_button);

    let custom_button = Button::primary(WaterWellnessColors::with_overrides(|p| {
        p.primary = "indigo-600".to_string();
    }))
    .build();
    println!("    Custom Theme: {}", custom_button);

    println!("\n✅ Button usage example completed successfully!");
    println!("💡 Notice how the trait-based approach allows for:");
    println!("   - Type-safe color management");
    println!("   - Consistent styling across themes");
    println!("   - Fluent builder pattern");
    println!("   - Easy customization and extension");
}
