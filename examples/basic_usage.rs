//! Basic usage example for the Jupiter Design System
//!
//! This example demonstrates how to use the trait-based design system
//! to generate CSS classes for styling components.

use jupiter_design_system::core::color::{ColorProvider, WaterWellnessColors};
use jupiter_design_system::prelude::*;

fn main() {
    println!("🎨 Jupiter Design System - Basic Usage Example");
    println!("==============================================");

    // Create a theme
    let theme = WaterWellnessTheme::default();
    println!("📝 Created theme: {}", theme.name());

    // Create a color provider
    let colors = WaterWellnessColors::new();
    println!("🎨 Created Water & Wellness color provider");

    // Generate CSS classes for different colors
    println!("\n🌈 Color Classes:");
    println!("  Primary text: {}", colors.text_class(Color::Primary));
    println!("  Primary background: {}", colors.bg_class(Color::Primary));
    println!("  Primary border: {}", colors.border_class(Color::Primary));

    println!("  Success text: {}", colors.text_class(Color::Success));
    println!("  Success background: {}", colors.bg_class(Color::Success));

    println!("  Error text: {}", colors.text_class(Color::Error));
    println!("  Error background: {}", colors.bg_class(Color::Error));

    // Show semantic colors
    println!("\n📋 Semantic Colors:");
    println!("  Surface: {}", colors.resolve_color(Color::Surface));
    println!("  Background: {}", colors.resolve_color(Color::Background));
    println!(
        "  Text Primary: {}",
        colors.resolve_color(Color::TextPrimary)
    );
    println!(
        "  Text Secondary: {}",
        colors.resolve_color(Color::TextSecondary)
    );

    // Show interactive states
    println!("\n🖱️  Interactive States:");
    println!(
        "  Interactive: {}",
        colors.resolve_color(Color::Interactive)
    );
    println!("  Hover: {}", colors.resolve_color(Color::InteractiveHover));
    println!(
        "  Active: {}",
        colors.resolve_color(Color::InteractiveActive)
    );
    println!(
        "  Disabled: {}",
        colors.resolve_color(Color::InteractiveDisabled)
    );

    // Demonstrate custom color provider
    println!("\n🎯 Custom Color Provider:");
    let custom_colors = WaterWellnessColors::with_overrides(|palette| {
        palette.primary = "custom-blue-600".to_string();
        palette.secondary = "custom-green-600".to_string();
    });

    println!(
        "  Custom primary: {}",
        custom_colors.resolve_color(Color::Primary)
    );
    println!(
        "  Custom secondary: {}",
        custom_colors.resolve_color(Color::Secondary)
    );
    println!(
        "  Default success: {}",
        custom_colors.resolve_color(Color::Success)
    );

    println!("\n✅ Example completed successfully!");
}
