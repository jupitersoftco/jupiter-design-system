//! # Jupiter Design System - Abstract Design Patterns Example
//!
//! This example demonstrates how abstract design concepts work in the Jupiter Design System.
//!
//! ## Abstract Design Concepts
//!
//! ### 1. Interactive Elements
//! - Abstract interaction behavior that can be applied to any clickable element
//! - Handles hover, focus, press states with configurable intensity
//! - Framework-agnostic - works with buttons, links, cards, menu items, etc.
//!
//! ### 2. Action Semantics  
//! - Abstract action intent and hierarchy
//! - Represents meaning (primary, destructive, navigation) independent of styling
//! - Consistent semantic treatment across all interactive elements
//!
//! ### 3. Focus Management
//! - Abstract accessibility patterns
//! - Ensures proper keyboard navigation and screen reader support
//! - Configurable focus behavior for different interaction patterns
//!
//! ### 4. Button Pattern
//! - Composite pattern combining all three abstract concepts
//! - Represents the complete "button-ness" experience
//! - Can be applied to any element that acts like a button

use jupiter_design_system::core::color::WaterWellnessColors;
use jupiter_design_system::patterns::*;

fn main() {
    let colors = WaterWellnessColors::default();

    println!("=== Jupiter Design System - Abstract Design Patterns ===\n");

    // 1. Interactive Elements - Abstract interaction behavior
    println!("1. INTERACTIVE ELEMENTS");
    println!("   Abstract interaction behavior for any clickable element\n");

    // Button interaction
    let button_interaction = interactive_element(colors.clone())
        .hoverable()
        .focusable()
        .pressable()
        .standard_interaction()
        .classes();
    println!("   Button interaction:   {}", button_interaction);

    // Card interaction (subtle)
    let card_interaction = interactive_element(colors.clone())
        .hoverable()
        .focusable()
        .gentle_interaction()
        .classes();
    println!("   Card interaction:     {}", card_interaction);

    // Menu item interaction
    let menu_interaction = interactive_element(colors.clone())
        .hoverable()
        .focusable()
        .pressable()
        .gentle_interaction()
        .classes();
    println!("   Menu interaction:     {}", menu_interaction);

    // 2. Action Semantics - Abstract action intent and hierarchy
    println!("\n2. ACTION SEMANTICS");
    println!("   Abstract action intent and visual hierarchy\n");

    // Primary action
    let primary_action = action_semantics(colors.clone()).primary().classes();
    println!("   Primary action:       {}", primary_action);

    // Destructive action
    let destructive_action = action_semantics(colors.clone()).destructive().classes();
    println!("   Destructive action:   {}", destructive_action);

    // Navigation action
    let nav_action = action_semantics(colors.clone()).navigation().classes();
    println!("   Navigation action:    {}", nav_action);

    // Hero CTA action
    let hero_action = action_semantics(colors.clone()).hero().urgent().classes();
    println!("   Hero CTA action:      {}", hero_action);

    // 3. Focus Management - Abstract accessibility patterns
    println!("\n3. FOCUS MANAGEMENT");
    println!("   Abstract accessibility and keyboard navigation\n");

    // Button focus
    let button_focus = focus_management(colors.clone()).button().classes();
    println!("   Button focus:         {}", button_focus);

    // Link focus
    let link_focus = focus_management(colors.clone()).link().classes();
    println!("   Link focus:           {}", link_focus);

    // Menu item focus
    let menu_focus = focus_management(colors.clone()).menu_item().classes();
    println!("   Menu item focus:      {}", menu_focus);

    // 4. Button Pattern - Composite pattern combining all concepts
    println!("\n4. BUTTON PATTERN");
    println!("   Complete button abstraction combining all concepts\n");

    // Primary CTA button
    let primary_cta = primary_button(colors.clone())
        .hero_prominence()
        .prominent_interaction()
        .classes();
    println!("   Primary CTA:          {}", primary_cta);

    // Secondary button
    let secondary_btn = secondary_button(colors.clone()).classes();
    println!("   Secondary button:     {}", secondary_btn);

    // Destructive button
    let destructive_btn = destructive_button(colors.clone()).classes();
    println!("   Destructive button:   {}", destructive_btn);

    // Navigation button (menu item style)
    let nav_btn = navigation_button(colors.clone()).classes();
    println!("   Navigation button:    {}", nav_btn);

    // Custom button composition
    let custom_btn = button_pattern(colors.clone())
        .primary_action()
        .tertiary_prominence()
        .gentle_interaction()
        .subtle_focus()
        .disabled(false)
        .classes();
    println!("   Custom composition:   {}", custom_btn);

    // 5. Framework-agnostic usage examples
    println!("\n5. FRAMEWORK-AGNOSTIC USAGE");
    println!("   Same patterns work with any component library\n");

    let hero_classes = hero_button(colors.clone()).classes();

    // Dioxus example
    println!(
        "   Dioxus:    rsx! {{ button {{ class: \"{}\", \"Click me\" }} }}",
        hero_classes
    );

    // React example
    println!(
        "   React:     <button className=\"{}\">Click me</button>",
        hero_classes
    );

    // Vue example
    println!(
        "   Vue:       <button class=\"{}\">Click me</button>",
        hero_classes
    );

    // Svelte example
    println!(
        "   Svelte:    <button class=\"{}\">Click me</button>",
        hero_classes
    );

    // 6. Accessibility attributes
    println!("\n6. ACCESSIBILITY ATTRIBUTES");
    println!("   Abstract patterns provide semantic markup\n");

    let accessible_btn = primary_button(colors.clone()).loading(true);

    let attrs = accessible_btn.accessibility_attributes();
    println!("   Accessibility attributes:");
    for (key, value) in attrs {
        println!("     {}=\"{}\"", key, value);
    }

    // 7. Semantic information
    println!("\n7. SEMANTIC INFORMATION");
    println!("   Extract semantic meaning from button patterns\n");

    let semantic_btn = destructive_button(colors.clone()).disabled(true);

    let semantic_info = semantic_btn.semantic_info();
    println!("   Semantic info:");
    println!("     Action intent: {:?}", semantic_info.action_intent);
    println!("     Is primary: {}", semantic_info.is_primary);
    println!("     Is destructive: {}", semantic_info.is_destructive);
    println!("     Is disabled: {}", semantic_info.is_disabled);

    println!("\n=== Summary ===");
    println!("Abstract design patterns provide:");
    println!("• Consistent interaction behavior across all elements");
    println!("• Semantic meaning independent of visual styling");
    println!("• Built-in accessibility and keyboard navigation");
    println!("• Framework-agnostic implementation");
    println!("• Composable patterns for complex components");
    println!("• Type-safe design system constraints");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interactive_elements() {
        let colors = WaterWellnessColors::default();

        // Test that interactive elements generate classes
        let interaction = interactive_element(colors)
            .hoverable()
            .focusable()
            .pressable()
            .classes();

        assert!(interaction.contains("transition-all"));
        assert!(interaction.contains("cursor-pointer"));
        assert!(interaction.contains("hover:"));
        assert!(interaction.contains("focus:"));
    }

    #[test]
    fn test_action_semantics() {
        let colors = WaterWellnessColors::default();

        // Test different action intents
        let primary = action_semantics(colors.clone()).primary().classes();
        let destructive = action_semantics(colors.clone()).destructive().classes();
        let nav = action_semantics(colors).navigation().classes();

        assert!(primary.contains("bg-water-500"));
        assert!(destructive.contains("bg-red-500"));
        assert!(nav.contains("bg-transparent"));
    }

    #[test]
    fn test_focus_management() {
        let colors = WaterWellnessColors::default();

        // Test focus management
        let button_focus = focus_management(colors).button().classes();

        assert!(button_focus.contains("focus:outline-none"));
        assert!(button_focus.contains("focus:ring-2"));
    }

    #[test]
    fn test_button_pattern_composition() {
        let colors = WaterWellnessColors::default();

        // Test that button patterns combine all concepts
        let primary_btn = primary_button(colors).classes();

        // Should have interaction classes
        assert!(primary_btn.contains("transition-all"));
        assert!(primary_btn.contains("cursor-pointer"));

        // Should have semantic classes
        assert!(primary_btn.contains("bg-water-500"));

        // Should have focus classes
        assert!(primary_btn.contains("focus:outline-none"));
    }

    #[test]
    fn test_accessibility_attributes() {
        let colors = WaterWellnessColors::default();

        // Test accessibility attributes
        let btn = primary_button(colors).loading(true).disabled(false);

        let attrs = btn.accessibility_attributes();

        // Should have proper ARIA attributes
        assert!(attrs.iter().any(|(k, v)| k == &"aria-busy" && v == "true"));
        assert!(attrs.iter().any(|(k, v)| k == &"tabindex" && v == "0"));
    }

    #[test]
    fn test_semantic_info() {
        let colors = WaterWellnessColors::default();

        // Test semantic information extraction
        let btn = destructive_button(colors).disabled(true);
        let info = btn.semantic_info();

        assert!(matches!(info.action_intent, ActionIntent::Destructive));
        assert!(!info.is_primary);
        assert!(info.is_destructive);
        assert!(info.is_disabled);
    }
}
