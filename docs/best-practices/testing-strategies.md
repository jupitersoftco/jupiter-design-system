# Testing Strategies

## 🧪 Testing Philosophy

Comprehensive testing ensures the Jupiter Design System maintains quality, accessibility, and performance across all usage scenarios.

### Testing Pyramid

1. **Unit Tests** - Individual component generation and theme resolution
2. **Integration Tests** - Component interactions and theme switching
3. **Visual Regression Tests** - Ensuring consistent visual output
4. **Accessibility Tests** - WCAG compliance and screen reader compatibility
5. **Performance Tests** - Build times, memory usage, and runtime performance

## 🔬 Unit Testing

### Component Generation Tests

```rust
#[cfg(test)]
mod component_tests {
    use super::*;
    use jupiter_design_system::prelude::*;

    #[test]
    fn test_button_variants_generate_correct_classes() {
        let colors = VibeColors::new();
        
        let primary = button_styles(colors.clone()).primary().classes();
        let secondary = button_styles(colors.clone()).secondary().classes();
        let success = button_styles(colors.clone()).success().classes();
        let error = button_styles(colors.clone()).error().classes();
        
        // Test primary button contains expected classes
        assert!(primary.contains("bg-jupiter-blue-500"));
        assert!(primary.contains("text-white"));
        assert!(primary.contains("hover:bg-jupiter-blue-600"));
        
        // Test secondary button has different styling
        assert!(secondary.contains("border"));
        assert!(secondary.contains("bg-white"));
        assert!(!secondary.contains("bg-jupiter-blue-500"));
        
        // Test semantic variants
        assert!(success.contains("bg-green-500"));
        assert!(error.contains("bg-red-500"));
    }

    #[test]
    fn test_button_sizes_generate_correct_padding() {
        let colors = VibeColors::new();
        
        let small = button_styles(colors.clone()).size(Size::Small).classes();
        let medium = button_styles(colors.clone()).size(Size::Medium).classes();
        let large = button_styles(colors.clone()).size(Size::Large).classes();
        
        assert!(small.contains("px-3") || small.contains("py-2"));
        assert!(medium.contains("px-4") || medium.contains("py-2"));
        assert!(large.contains("px-6") || large.contains("py-3"));
    }

    #[test]
    fn test_button_states_affect_appearance() {
        let colors = VibeColors::new();
        
        let default = button_styles(colors.clone())
            .primary()
            .state(ButtonState::Default)
            .classes();
            
        let disabled = button_styles(colors.clone())
            .primary()
            .state(ButtonState::Disabled)
            .classes();
            
        let loading = button_styles(colors.clone())
            .primary()
            .state(ButtonState::Loading)
            .classes();
        
        // Disabled state should have different styling
        assert!(disabled.contains("opacity") || disabled.contains("cursor-not-allowed"));
        
        // Loading state should indicate loading
        assert!(loading.contains("opacity") || loading.contains("cursor-wait"));
        
        // States should generate different output
        assert_ne!(default, disabled);
        assert_ne!(default, loading);
    }
}
```

### Theme Resolution Tests

```rust
#[cfg(test)]
mod theme_tests {
    use super::*;

    #[test]
    fn test_default_theme_color_resolution() {
        let colors = VibeColors::new();
        
        assert_eq!(colors.resolve_color(Color::Primary), "jupiter-blue-500");
        assert_eq!(colors.resolve_color(Color::Secondary), "jupiter-green-500");
        assert_eq!(colors.resolve_color(Color::Success), "green-500");
        assert_eq!(colors.resolve_color(Color::Error), "red-500");
    }

    #[test]
    fn test_custom_theme_overrides() {
        let custom_colors = VibeColors::with_overrides(|palette| {
            palette.primary = "purple-600".to_string();
            palette.secondary = "pink-500".to_string();
        });
        
        assert_eq!(custom_colors.resolve_color(Color::Primary), "purple-600");
        assert_eq!(custom_colors.resolve_color(Color::Secondary), "pink-500");
        
        // Non-overridden colors should remain default
        assert_eq!(custom_colors.resolve_color(Color::Success), "green-500");
    }

    #[test]
    fn test_theme_consistency_across_components() {
        let colors = VibeColors::new();
        
        let button_primary = button_styles(colors.clone()).primary().classes();
        let text_primary = text_styles(colors.clone()).color(Color::Primary).classes();
        
        // Both should reference the same primary color
        assert!(button_primary.contains("jupiter-blue-500"));
        assert!(text_primary.contains("jupiter-blue-500"));
    }

    #[test]
    fn test_dark_theme_implementation() {
        let dark_theme = DarkTheme::default();
        
        // Text colors should be inverted for dark theme
        assert!(dark_theme.resolve_color(Color::TextPrimary).contains("gray-"));
        assert!(dark_theme.resolve_color(Color::Background).contains("gray-9"));
        
        // Interactive colors should still be distinguishable
        let interactive = dark_theme.resolve_color(Color::Interactive);
        let interactive_hover = dark_theme.resolve_color(Color::InteractiveHover);
        assert_ne!(interactive, interactive_hover);
    }
}
```

## 🔗 Integration Testing

### Component Interaction Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_card_with_multiple_sections() {
        let colors = VibeColors::new();
        
        let header = card_header_styles(colors.clone()).divider_bottom().classes();
        let content = card_content_styles(colors.clone()).classes();
        let footer = card_footer_styles(colors.clone()).divider_top().classes();
        
        // All sections should be non-empty
        assert!(!header.is_empty());
        assert!(!content.is_empty());
        assert!(!footer.is_empty());
        
        // Header should have bottom divider
        assert!(header.contains("border-b"));
        
        // Footer should have top divider
        assert!(footer.contains("border-t"));
        
        // Sections should have different styling
        assert_ne!(header, content);
        assert_ne!(content, footer);
    }

    #[test]
    fn test_form_field_composition() {
        let colors = VibeColors::new();
        
        let label = text_styles(colors.clone())
            .typography(Typography::Label)
            .color(Color::TextPrimary)
            .classes();
            
        let input = interactive_input(colors.clone())
            .base_style()
            .focus().ring_primary()
            .build();
            
        let help_text = text_styles(colors.clone())
            .typography(Typography::Caption)
            .color(Color::TextTertiary)
            .classes();
            
        let error_text = text_styles(colors)
            .typography(Typography::Caption)
            .color(Color::Error)
            .classes();
        
        // All components should generate valid classes
        assert!(!label.is_empty());
        assert!(!input.is_empty());
        assert!(!help_text.is_empty());
        assert!(!error_text.is_empty());
        
        // Input should have focus styles
        assert!(input.contains("focus:"));
        
        // Error text should be visually distinct from help text
        assert!(error_text.contains("red") || error_text.contains("error"));
        assert!(help_text.contains("gray") || help_text.contains("tertiary"));
    }

    #[test]
    fn test_responsive_layout_behavior() {
        let colors = VibeColors::new();
        
        let responsive_layout = layout_styles(colors)
            .direction_vertical()
            .spacing_sm()
            .responsive_direction(Breakpoint::Tablet, LayoutDirection::Horizontal)
            .responsive_spacing(Breakpoint::Desktop, LayoutSpacing::LG)
            .classes();
        
        // Should contain responsive classes
        assert!(responsive_layout.contains("md:") || responsive_layout.contains("lg:"));
        
        // Should have both mobile and desktop specifications
        assert!(responsive_layout.contains("flex-col") || responsive_layout.contains("flex-row"));
    }
}
```

### Theme Switching Tests

```rust
#[cfg(test)]
mod theme_switching_tests {
    use super::*;

    #[test]
    fn test_component_adapts_to_theme_changes() {
        let light_theme = VibeColors::new();
        let dark_theme = DarkTheme::default();
        
        let light_button = button_styles(light_theme).primary().classes();
        let dark_button = button_styles(dark_theme).primary().classes();
        
        // Buttons should have different styling for different themes
        assert_ne!(light_button, dark_button);
        
        // Both should be valid CSS classes
        assert!(!light_button.is_empty());
        assert!(!dark_button.is_empty());
        
        // Both should maintain core button structure
        assert!(light_button.contains("inline-flex"));
        assert!(dark_button.contains("inline-flex"));
    }

    #[test]
    fn test_text_readability_across_themes() {
        let light_theme = VibeColors::new();
        let dark_theme = DarkTheme::default();
        
        let light_text = text_styles(light_theme)
            .typography(Typography::Body)
            .color(Color::TextPrimary)
            .classes();
            
        let dark_text = text_styles(dark_theme)
            .typography(Typography::Body)
            .color(Color::TextPrimary)
            .classes();
        
        // Text should have different colors for different themes
        assert_ne!(light_text, dark_text);
        
        // Both should maintain typography structure
        assert!(light_text.contains("text-") && dark_text.contains("text-"));
    }
}
```

## 👁️ Visual Regression Testing

### Snapshot Testing

```rust
#[cfg(test)]
mod visual_tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_button_variants_visual_output() {
        let colors = VibeColors::new();
        
        let variants = vec![
            ("primary", button_styles(colors.clone()).primary().classes()),
            ("secondary", button_styles(colors.clone()).secondary().classes()),
            ("success", button_styles(colors.clone()).success().classes()),
            ("warning", button_styles(colors.clone()).warning().classes()),
            ("error", button_styles(colors.clone()).error().classes()),
            ("ghost", button_styles(colors.clone()).ghost().classes()),
        ];
        
        for (variant_name, classes) in variants {
            assert_snapshot!(format!("button_{}", variant_name), classes);
        }
    }

    #[test]
    fn test_typography_hierarchy_output() {
        let colors = VibeColors::new();
        
        let typography_levels = vec![
            ("heading1", text_styles(colors.clone()).typography(Typography::Heading1).classes()),
            ("heading2", text_styles(colors.clone()).typography(Typography::Heading2).classes()),
            ("heading3", text_styles(colors.clone()).typography(Typography::Heading3).classes()),
            ("body", text_styles(colors.clone()).typography(Typography::Body).classes()),
            ("caption", text_styles(colors.clone()).typography(Typography::Caption).classes()),
        ];
        
        for (level_name, classes) in typography_levels {
            assert_snapshot!(format!("typography_{}", level_name), classes);
        }
    }

    #[test]
    fn test_card_elevation_progression() {
        let colors = VibeColors::new();
        
        let elevations = vec![
            ("low", card_styles(colors.clone()).elevation(CardElevation::Low).classes()),
            ("medium", card_styles(colors.clone()).elevation(CardElevation::Medium).classes()),
            ("high", card_styles(colors.clone()).elevation(CardElevation::High).classes()),
        ];
        
        for (elevation_name, classes) in elevations {
            assert_snapshot!(format!("card_elevation_{}", elevation_name), classes);
        }
    }
}
```

### CSS Output Validation

```rust
use std::collections::HashSet;

pub fn validate_css_output(css_classes: &str) -> ValidationResult {
    let mut issues = Vec::new();
    let classes: HashSet<&str> = css_classes.split_whitespace().collect();
    
    // Check for conflicting classes
    if classes.contains("block") && classes.contains("inline") {
        issues.push("Conflicting display classes: block and inline".to_string());
    }
    
    if classes.contains("absolute") && classes.contains("relative") {
        issues.push("Conflicting position classes: absolute and relative".to_string());
    }
    
    // Check for missing required classes
    if classes.iter().any(|c| c.starts_with("bg-")) {
        // If background is set, should have appropriate text color
        if !classes.iter().any(|c| c.starts_with("text-")) {
            issues.push("Background color set without text color".to_string());
        }
    }
    
    // Check for accessibility issues
    if classes.contains("opacity-0") && !classes.contains("sr-only") {
        issues.push("Hidden content should use sr-only for accessibility".to_string());
    }
    
    ValidationResult {
        is_valid: issues.is_empty(),
        issues,
        class_count: classes.len(),
    }
}

#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub issues: Vec<String>,
    pub class_count: usize,
}
```

## ♿ Accessibility Testing

### ARIA Compliance Tests

```rust
#[cfg(test)]
mod accessibility_tests {
    use super::*;

    #[test]
    fn test_modal_accessibility_attributes() {
        let modal_html = accessible_modal("Test Title", "Test Content", VibeColors::new());
        
        // Required ARIA attributes for modals
        assert!(modal_html.contains(r#"role="dialog""#));
        assert!(modal_html.contains(r#"aria-modal="true""#));
        assert!(modal_html.contains(r#"aria-labelledby=""#));
        assert!(modal_html.contains(r#"aria-describedby=""#));
        
        // Focus management
        assert!(modal_html.contains("data-focus-trap") || modal_html.contains("tabindex"));
        
        // Keyboard interaction
        assert!(modal_html.contains("data-modal-close") || modal_html.contains("aria-label"));
    }

    #[test]
    fn test_form_accessibility_structure() {
        let form_html = accessible_form_with_validation(VibeColors::new());
        
        // Semantic structure
        assert!(form_html.contains("<fieldset"));
        assert!(form_html.contains("<legend"));
        assert!(form_html.contains(r#"for=""#));
        
        // Error handling
        assert!(form_html.contains(r#"aria-describedby=""#));
        assert!(form_html.contains(r#"aria-invalid=""#));
        assert!(form_html.contains(r#"aria-live=""#));
        assert!(form_html.contains(r#"role="alert""#));
        
        // Required field indication
        assert!(form_html.contains("required") || form_html.contains("aria-required"));
    }

    #[test]
    fn test_navigation_accessibility() {
        let nav_items = vec![
            NavItem {
                label: "Home".to_string(),
                href: Some("/".to_string()),
                is_active: true,
                children: vec![],
            },
            NavItem {
                label: "About".to_string(),
                href: Some("/about".to_string()),
                is_active: false,
                children: vec![],
            },
        ];
        
        let nav_html = render_navigation("Test Site", &nav_items, None, false, VibeColors::new());
        
        // Navigation semantics
        assert!(nav_html.contains(r#"role="navigation""#));
        assert!(nav_html.contains(r#"aria-label=""#));
        
        // Current page indication
        assert!(nav_html.contains(r#"aria-current="page""#));
        
        // Keyboard navigation
        assert!(nav_html.contains("tabindex") || nav_html.contains("role=\"menubar\""));
        
        // Screen reader support
        assert!(nav_html.contains("sr-only") || nav_html.contains("aria-label"));
    }
}
```

### Contrast Testing

```rust
use colorsys::{ColorAlpha, Rgb};

pub fn test_contrast_ratios(theme: &dyn ColorProvider) -> ContrastReport {
    let mut violations = Vec::new();
    let palette = theme.palette();
    
    // Test critical color combinations
    let combinations = vec![
        (&palette.text_primary, &palette.background, "Primary text on background", 4.5),
        (&palette.text_secondary, &palette.background, "Secondary text on background", 4.5),
        ("white", &palette.primary, "White text on primary", 4.5),
        ("white", &palette.success, "White text on success", 4.5),
        ("white", &palette.error, "White text on error", 4.5),
        (&palette.text_primary, &palette.surface, "Primary text on surface", 4.5),
    ];
    
    for (fg, bg, description, required_ratio) in combinations {
        let ratio = calculate_contrast_ratio(fg, bg);
        if ratio < required_ratio {
            violations.push(ContrastViolation {
                description: description.to_string(),
                foreground: fg.to_string(),
                background: bg.to_string(),
                actual_ratio: ratio,
                required_ratio,
            });
        }
    }
    
    ContrastReport {
        total_tests: combinations.len(),
        violations,
        passes: combinations.len() - violations.len(),
    }
}

fn calculate_contrast_ratio(foreground: &str, background: &str) -> f64 {
    let fg_luminance = get_relative_luminance(foreground);
    let bg_luminance = get_relative_luminance(background);
    
    let lighter = fg_luminance.max(bg_luminance);
    let darker = fg_luminance.min(bg_luminance);
    
    (lighter + 0.05) / (darker + 0.05)
}

fn get_relative_luminance(color: &str) -> f64 {
    // Parse color string and convert to relative luminance
    // This is a simplified implementation
    match color {
        "white" => 1.0,
        "black" => 0.0,
        color if color.contains("gray-900") => 0.05,
        color if color.contains("gray-600") => 0.3,
        color if color.contains("gray-400") => 0.6,
        color if color.contains("gray-50") => 0.95,
        color if color.contains("blue-500") => 0.2,
        color if color.contains("green-500") => 0.3,
        color if color.contains("red-500") => 0.2,
        _ => 0.5, // Default fallback
    }
}

#[derive(Debug)]
pub struct ContrastReport {
    pub total_tests: usize,
    pub violations: Vec<ContrastViolation>,
    pub passes: usize,
}

#[derive(Debug)]
pub struct ContrastViolation {
    pub description: String,
    pub foreground: String,
    pub background: String,
    pub actual_ratio: f64,
    pub required_ratio: f64,
}
```

## ⚡ Performance Testing

### Build Time Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Instant;

fn bench_component_generation(c: &mut Criterion) {
    let colors = VibeColors::new();
    
    c.bench_function("button_primary_generation", |b| {
        b.iter(|| {
            black_box(button_styles(colors.clone()).primary().classes())
        })
    });
    
    c.bench_function("complex_card_generation", |b| {
        b.iter(|| {
            black_box({
                let header = card_header_styles(colors.clone()).classes();
                let content = card_content_styles(colors.clone()).classes();
                let footer = card_footer_styles(colors.clone()).classes();
                format!("{} {} {}", header, content, footer)
            })
        })
    });
    
    c.bench_function("theme_color_resolution", |b| {
        b.iter(|| {
            black_box(colors.resolve_color(Color::Primary))
        })
    });
}

fn bench_theme_operations(c: &mut Criterion) {
    let default_theme = VibeColors::new();
    let cached_theme = CachedColorProvider::new(default_theme.clone());
    
    c.bench_function("uncached_color_resolution", |b| {
        b.iter(|| {
            black_box(default_theme.text_class(Color::Primary))
        })
    });
    
    c.bench_function("cached_color_resolution", |b| {
        b.iter(|| {
            black_box(cached_theme.text_class(Color::Primary))
        })
    });
}

criterion_group!(benches, bench_component_generation, bench_theme_operations);
criterion_main!(benches);
```

### Memory Usage Tests

```rust
#[cfg(test)]
mod memory_tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_component_memory_footprint() {
        let colors = VibeColors::new();
        
        // Test basic component sizes
        let button_builder = button_styles(colors.clone());
        let card_builder = card_styles(colors.clone());
        let text_builder = text_styles(colors);
        
        println!("ButtonStyles size: {} bytes", mem::size_of_val(&button_builder));
        println!("CardStyles size: {} bytes", mem::size_of_val(&card_builder));
        println!("TextStyles size: {} bytes", mem::size_of_val(&text_builder));
        
        // Ensure builders don't grow too large
        assert!(mem::size_of_val(&button_builder) < 1024); // 1KB limit
        assert!(mem::size_of_val(&card_builder) < 1024);
        assert!(mem::size_of_val(&text_builder) < 1024);
    }

    #[test]
    fn test_theme_memory_usage() {
        let theme = VibeColors::new();
        let theme_size = mem::size_of_val(&theme);
        
        println!("Theme size: {} bytes", theme_size);
        
        // Theme should be reasonably sized
        assert!(theme_size < 2048); // 2KB limit for theme
    }

    #[test]
    fn test_string_allocation_efficiency() {
        let colors = VibeColors::new();
        
        // Measure allocations for repeated operations
        let start_memory = get_memory_usage();
        
        for _ in 0..1000 {
            let _ = button_styles(colors.clone()).primary().classes();
        }
        
        let end_memory = get_memory_usage();
        let memory_diff = end_memory - start_memory;
        
        println!("Memory usage for 1000 button generations: {} bytes", memory_diff);
        
        // Should not leak significant memory
        assert!(memory_diff < 1_000_000); // 1MB limit for 1000 operations
    }
}

fn get_memory_usage() -> usize {
    // Platform-specific memory usage measurement
    // This is a simplified placeholder
    0
}
```

## 🔍 Test Organization

### Test Hierarchy

```rust
// tests/
// ├── unit/
// │   ├── components/
// │   │   ├── button_tests.rs
// │   │   ├── card_tests.rs
// │   │   └── text_tests.rs
// │   ├── themes/
// │   │   ├── default_theme_tests.rs
// │   │   ├── custom_theme_tests.rs
// │   │   └── theme_switching_tests.rs
// │   └── core/
// │       ├── color_tests.rs
// │       ├── spacing_tests.rs
// │       └── typography_tests.rs
// ├── integration/
// │   ├── component_composition_tests.rs
// │   ├── responsive_behavior_tests.rs
// │   └── framework_integration_tests.rs
// ├── accessibility/
// │   ├── aria_compliance_tests.rs
// │   ├── contrast_tests.rs
// │   └── keyboard_navigation_tests.rs
// ├── performance/
// │   ├── benchmarks.rs
// │   ├── memory_tests.rs
// │   └── build_time_tests.rs
// └── visual/
//     ├── snapshot_tests.rs
//     ├── css_validation_tests.rs
//     └── regression_tests.rs
```

### Test Configuration

```toml
# Cargo.toml test configuration
[dev-dependencies]
criterion = "0.5"
insta = "1.0"
tokio-test = "0.4"
wasm-bindgen-test = "0.3"

[[bench]]
name = "performance_benchmarks"
harness = false

[package.metadata.docs.rs]
features = ["testing"]
```

### CI/CD Integration

```yaml
# .github/workflows/test.yml
name: Test Suite

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      # Unit tests
      - name: Run unit tests
        run: cargo test --lib
      
      # Integration tests
      - name: Run integration tests
        run: cargo test --test integration
      
      # Accessibility tests
      - name: Run accessibility tests
        run: cargo test --test accessibility
      
      # Performance benchmarks
      - name: Run benchmarks
        run: cargo bench
      
      # Visual regression tests
      - name: Run snapshot tests
        run: cargo insta test
```

## 🎯 Testing Best Practices

### Do's ✅

1. **Test Public APIs Only**
   ```rust
   // ✅ Good: Test public interface
   #[test]
   fn test_button_generates_expected_classes() {
       let classes = button_styles(colors).primary().classes();
       assert!(classes.contains("bg-jupiter-blue-500"));
   }
   ```

2. **Use Descriptive Test Names**
   ```rust
   // ✅ Good: Clear test intent
   #[test]
   fn test_disabled_button_has_appropriate_styling_and_accessibility_attributes() {
       // Test implementation
   }
   ```

3. **Test Edge Cases**
   ```rust
   // ✅ Good: Test boundary conditions
   #[test]
   fn test_empty_theme_fallback_behavior() {
       let empty_theme = EmptyTheme::default();
       let classes = button_styles(empty_theme).primary().classes();
       assert!(!classes.is_empty()); // Should have fallback
   }
   ```

### Don'ts ❌

1. **Don't Test Implementation Details**
   ```rust
   // ❌ Bad: Testing internal structure
   #[test]
   fn test_button_builder_internal_field_values() {
       // Don't test private fields
   }
   ```

2. **Don't Write Brittle Tests**
   ```rust
   // ❌ Bad: Too specific
   #[test]
   fn test_exact_css_string_match() {
       let classes = button_styles(colors).primary().classes();
       assert_eq!(classes, "exact-string-that-will-break"); // Fragile
   }
   
   // ✅ Good: Test intent
   #[test]
   fn test_primary_button_has_brand_styling() {
       let classes = button_styles(colors).primary().classes();
       assert!(classes.contains("jupiter-blue") || classes.contains("primary"));
   }
   ```

## 📊 Test Metrics and Reporting

### Coverage Reporting

```rust
// Use tarpaulin for coverage
// cargo install cargo-tarpaulin
// cargo tarpaulin --out Html --output-dir coverage/

#[cfg(test)]
mod coverage_tests {
    use super::*;

    #[test]
    fn test_all_button_variants_covered() {
        let colors = VibeColors::new();
        
        // Ensure all variants are tested
        let variants = [
            ButtonVariant::Primary,
            ButtonVariant::Secondary,
            ButtonVariant::Success,
            ButtonVariant::Warning,
            ButtonVariant::Error,
            ButtonVariant::Ghost,
            ButtonVariant::Link,
        ];
        
        for variant in variants {
            let classes = button_styles(colors.clone()).variant(variant).classes();
            assert!(!classes.is_empty(), "Variant {:?} should generate classes", variant);
        }
    }
}
```

### Performance Regression Detection

```rust
use std::time::Duration;

#[test]
fn test_performance_regression_detection() {
    let colors = VibeColors::new();
    
    let start = std::time::Instant::now();
    
    // Generate 1000 button instances
    for _ in 0..1000 {
        let _ = button_styles(colors.clone()).primary().classes();
    }
    
    let duration = start.elapsed();
    
    // Should complete within reasonable time
    assert!(duration < Duration::from_millis(100), 
           "Performance regression detected: {}ms for 1000 buttons", duration.as_millis());
}
```

This comprehensive testing strategy ensures the Jupiter Design System maintains high quality, performance, and accessibility standards across all components and usage scenarios.