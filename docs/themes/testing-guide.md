# Testing Guide for Jupiter Design System Themes

This guide covers comprehensive testing strategies for theme implementations.

## Unit Testing Themes

### Testing ColorProvider Implementation

```rust
#[cfg(test)]
mod theme_tests {
    use super::*;
    use jupiter_design_system::prelude::*;

    #[test]
    fn test_color_provider_implementation() {
        let theme = MyTheme::new();
        
        // Test all colors are defined
        assert!(!theme.resolve_color(Color::Primary).is_empty());
        assert!(!theme.resolve_color(Color::Secondary).is_empty());
        assert!(!theme.resolve_color(Color::Success).is_empty());
        assert!(!theme.resolve_color(Color::Error).is_empty());
        
        // Test CSS class generation
        assert_eq!(theme.text_class(Color::Primary), "text-blue-600");
        assert_eq!(theme.bg_class(Color::Primary), "bg-blue-600");
        assert_eq!(theme.border_class(Color::Primary), "border-blue-600");
    }

    #[test]
    fn test_color_consistency() {
        let theme = MyTheme::new();
        
        // Ensure interactive states form logical progression
        let base = theme.resolve_color(Color::Interactive);
        let hover = theme.resolve_color(Color::InteractiveHover);
        let active = theme.resolve_color(Color::InteractiveActive);
        
        // Should be different values
        assert_ne!(base, hover);
        assert_ne!(hover, active);
        assert_ne!(base, active);
    }

    #[test]
    fn test_theme_identity() {
        let theme = MyTheme::new();
        assert_eq!(theme.name(), "My Custom Theme");
    }
}
```

### Testing Pattern Integration

```rust
#[test]
fn test_pattern_compatibility() {
    let theme = MyTheme::new();
    
    // Test button patterns
    let primary_btn = primary_button(theme.clone()).classes();
    assert!(primary_btn.contains(theme.resolve_color(Color::Primary)));
    
    // Test typography patterns
    let title = title_typography(theme.clone()).classes();
    assert!(title.contains("text-4xl"));
    assert!(title.contains("font-bold"));
    
    // Test card patterns
    let card = card_pattern(theme.clone())
        .elevation(CardElevation::Raised)
        .classes();
    assert!(card.contains("shadow"));
}

#[test]
fn test_custom_theme_overrides() {
    let theme = MyTheme::with_primary("purple-600");
    
    let button = primary_button(theme.clone()).classes();
    assert!(button.contains("purple-600"));
    assert!(!button.contains("blue-600"));
}
```

### Testing Accessibility

```rust
#[test]
fn test_color_contrast() {
    let theme = MyTheme::new();
    
    // Test text on background contrast
    let text_color = theme.resolve_color(Color::TextPrimary);
    let bg_color = theme.resolve_color(Color::Background);
    
    // In real tests, use a contrast checking library
    assert_ne!(text_color, bg_color);
}

#[test]
fn test_focus_indicators() {
    let theme = MyTheme::new();
    
    let button = primary_button(theme.clone())
        .focused()
        .classes();
    
    // Should have focus indicator classes
    assert!(button.contains("focus:outline-none"));
    assert!(button.contains("focus:ring"));
}

#[test]
fn test_disabled_states() {
    let theme = MyTheme::new();
    
    let disabled_btn = primary_button(theme.clone())
        .disabled(true)
        .classes();
    
    assert!(disabled_btn.contains("opacity-50"));
    assert!(disabled_btn.contains("cursor-not-allowed"));
    
    // Test accessibility attributes
    let attrs = primary_button(theme)
        .disabled(true)
        .accessibility_attributes();
    
    assert!(attrs.iter().any(|(k, v)| k == &"aria-disabled" && v == "true"));
}
```

## Integration Testing

### Testing with Mock Components

```rust
#[test]
fn test_theme_with_mock_components() {
    let theme = MyTheme::new();
    
    // Mock a complete component
    struct MockButton<'a> {
        theme: &'a MyTheme,
        text: &'a str,
        variant: ButtonVariant,
    }
    
    impl<'a> MockButton<'a> {
        fn render(&self) -> String {
            let classes = button_styles(self.theme.clone())
                .variant(self.variant)
                .classes();
                
            format!(r#"<button class="{}">{}</button>"#, classes, self.text)
        }
    }
    
    let button = MockButton {
        theme: &theme,
        text: "Click me",
        variant: ButtonVariant::Primary,
    };
    
    let html = button.render();
    assert!(html.contains(theme.resolve_color(Color::Primary)));
    assert!(html.contains("Click me"));
}
```

### Testing Theme Switching

```rust
#[test]
fn test_theme_switching() {
    let light_theme = MyTheme::light();
    let dark_theme = MyTheme::dark();
    
    let light_button = primary_button(light_theme).classes();
    let dark_button = primary_button(dark_theme).classes();
    
    // Ensure different themes produce different output
    assert_ne!(light_button, dark_button);
    
    // Test specific differences
    assert!(light_button.contains("text-white"));
    assert!(dark_button.contains("text-gray-900"));
}
```

## Visual Regression Testing

### Snapshot Testing

```rust
#[test]
fn test_theme_snapshots() {
    use insta::assert_snapshot;
    
    let theme = MyTheme::new();
    
    // Snapshot button classes
    assert_snapshot!(primary_button(theme.clone()).classes());
    assert_snapshot!(secondary_button(theme.clone()).classes());
    
    // Snapshot typography
    assert_snapshot!(title_typography(theme.clone()).classes());
    assert_snapshot!(body_typography(theme.clone()).classes());
    
    // Snapshot complete component
    let card_with_content = format!(
        r#"<div class="{}">
            <h2 class="{}">Card Title</h2>
            <p class="{}">Card content</p>
            <button class="{}">Action</button>
        </div>"#,
        card_pattern(theme.clone()).classes(),
        heading_typography(theme.clone()).classes(),
        body_typography(theme.clone()).classes(),
        primary_button(theme).classes()
    );
    
    assert_snapshot!(card_with_content);
}
```

### Visual Diff Testing

```rust
use image_compare::{Algorithm, Metric, Similarity};

#[test]
fn test_visual_consistency() {
    // Render component with theme
    let theme = MyTheme::new();
    let component_html = render_component_with_theme(&theme);
    
    // Capture screenshot (pseudo-code)
    let screenshot = capture_screenshot(&component_html);
    
    // Compare with baseline
    let baseline = load_baseline_image("button_primary.png");
    let result = image_compare::compare(&baseline, &screenshot, Algorithm::RootMeanSquared);
    
    match result {
        Ok(Similarity::Similar(score)) if score > 0.95 => {
            // Images are similar enough
        }
        _ => panic!("Visual regression detected"),
    }
}
```

## Property-Based Testing

### Testing Theme Invariants

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_color_class_format(color in prop::sample::select(vec![
        Color::Primary,
        Color::Secondary,
        Color::Success,
        Color::Error,
    ])) {
        let theme = MyTheme::new();
        
        // Text classes should follow pattern
        let text_class = theme.text_class(color);
        prop_assert!(text_class.starts_with("text-"));
        
        // Background classes should follow pattern
        let bg_class = theme.bg_class(color);
        prop_assert!(bg_class.starts_with("bg-"));
        
        // Border classes should follow pattern
        let border_class = theme.border_class(color);
        prop_assert!(border_class.starts_with("border-"));
    }
    
    #[test]
    fn test_button_states_are_valid(
        disabled in any::<bool>(),
        loading in any::<bool>(),
        selected in any::<bool>()
    ) {
        let theme = MyTheme::new();
        
        let button = button_pattern(theme)
            .disabled(disabled)
            .loading(loading)
            .selected(selected)
            .classes();
            
        // Should always produce valid classes
        prop_assert!(!button.is_empty());
        prop_assert!(!button.contains("undefined"));
        prop_assert!(!button.contains("null"));
    }
}
```

## Performance Testing

### Benchmarking Theme Operations

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_theme_operations(c: &mut Criterion) {
    let theme = MyTheme::new();
    
    c.bench_function("color resolution", |b| {
        b.iter(|| {
            black_box(theme.resolve_color(Color::Primary))
        });
    });
    
    c.bench_function("button pattern generation", |b| {
        b.iter(|| {
            black_box(primary_button(theme.clone()).classes())
        });
    });
    
    c.bench_function("complex component", |b| {
        b.iter(|| {
            let card = card_pattern(theme.clone())
                .elevation(CardElevation::Raised)
                .interaction(CardInteraction::Clickable)
                .classes();
                
            let button = primary_button(theme.clone())
                .hero_prominence()
                .classes();
                
            let text = title_typography(theme.clone())
                .color(TypographyColor::Primary)
                .classes();
                
            black_box(format!("{} {} {}", card, button, text))
        });
    });
}

criterion_group!(benches, benchmark_theme_operations);
criterion_main!(benches);
```

### Memory Usage Testing

```rust
#[test]
fn test_theme_memory_usage() {
    use std::mem;
    
    let theme_size = mem::size_of::<MyTheme>();
    println!("Theme size: {} bytes", theme_size);
    
    // Ensure theme is reasonably sized
    assert!(theme_size < 1024); // Less than 1KB
    
    // Test that themes can be cloned efficiently
    let theme1 = MyTheme::new();
    let theme2 = theme1.clone();
    
    // Both should point to same data if using Arc internally
    assert_eq!(
        theme1.resolve_color(Color::Primary),
        theme2.resolve_color(Color::Primary)
    );
}
```

## Testing in CI/CD

### GitHub Actions Example

```yaml
name: Theme Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v2
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Run unit tests
      run: cargo test --lib
    
    - name: Run integration tests
      run: cargo test --test '*'
    
    - name: Run snapshot tests
      run: cargo insta test
    
    - name: Check snapshot changes
      run: cargo insta review
    
    - name: Run benchmarks
      run: cargo bench --no-run
    
    - name: Test all theme variants
      run: |
        cargo test --features "theme-light"
        cargo test --features "theme-dark"
        cargo test --features "theme-high-contrast"
```

### Pre-commit Hooks

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Run theme tests
cargo test --lib theme_tests

# Check theme documentation
cargo doc --no-deps --document-private-items

# Validate theme exports
cargo check --lib

# Run quick benchmarks
cargo bench --bench theme_benchmarks -- --quick
```

## Testing Best Practices

### 1. Test Organization

```rust
// Organize tests by concern
mod color_tests {
    // Color-specific tests
}

mod pattern_tests {
    // Pattern integration tests
}

mod accessibility_tests {
    // WCAG compliance tests
}

mod performance_tests {
    // Performance benchmarks
}
```

### 2. Test Data Builders

```rust
// Create test helpers
struct ThemeTestBuilder {
    palette: ColorPalette,
}

impl ThemeTestBuilder {
    fn new() -> Self {
        Self {
            palette: ColorPalette::default(),
        }
    }
    
    fn with_primary(mut self, color: &str) -> Self {
        self.palette.primary = color.to_string();
        self
    }
    
    fn with_dark_mode(mut self) -> Self {
        // Set dark mode colors
        self.palette.background = "gray-900".to_string();
        self.palette.text_primary = "white".to_string();
        self
    }
    
    fn build(self) -> TestTheme {
        TestTheme { palette: self.palette }
    }
}

#[test]
fn test_with_builder() {
    let theme = ThemeTestBuilder::new()
        .with_primary("purple-600")
        .with_dark_mode()
        .build();
        
    assert_eq!(theme.resolve_color(Color::Primary), "purple-600");
    assert_eq!(theme.resolve_color(Color::Background), "gray-900");
}
```

### 3. Parameterized Tests

```rust
use rstest::rstest;

#[rstest]
#[case(Color::Primary, "text-blue-600")]
#[case(Color::Secondary, "text-gray-600")]
#[case(Color::Success, "text-green-600")]
#[case(Color::Error, "text-red-600")]
fn test_text_classes(
    #[case] color: Color,
    #[case] expected: &str,
) {
    let theme = MyTheme::new();
    assert_eq!(theme.text_class(color), expected);
}

#[rstest]
#[case(ButtonVariant::Primary, true, "opacity-50")]
#[case(ButtonVariant::Secondary, true, "opacity-50")]
#[case(ButtonVariant::Primary, false, "hover:")]
fn test_button_states(
    #[case] variant: ButtonVariant,
    #[case] disabled: bool,
    #[case] expected_class: &str,
) {
    let theme = MyTheme::new();
    let classes = button_styles(theme)
        .variant(variant)
        .disabled(disabled)
        .classes();
        
    if disabled {
        assert!(classes.contains(expected_class));
    } else {
        assert!(classes.contains(expected_class));
    }
}
```

### 4. Error Case Testing

```rust
#[test]
#[should_panic(expected = "Invalid color")]
fn test_invalid_color_handling() {
    let mut theme = MyTheme::new();
    theme.palette.primary = "".to_string(); // Invalid
    
    // Should panic or handle gracefully
    let _ = theme.text_class(Color::Primary);
}

#[test]
fn test_fallback_behavior() {
    let mut theme = MyTheme::new();
    theme.palette.interactive_hover = "".to_string();
    
    // Should fall back to base interactive color
    let hover = theme.resolve_color(Color::InteractiveHover);
    assert_eq!(hover, theme.resolve_color(Color::Interactive));
}
```

## Testing Checklist

- [ ] All color tokens resolve to valid values
- [ ] CSS classes follow expected patterns
- [ ] Theme switching works correctly
- [ ] Accessibility requirements are met
- [ ] Performance benchmarks pass
- [ ] Visual regression tests pass
- [ ] All patterns work with the theme
- [ ] Error cases are handled
- [ ] Documentation examples work
- [ ] Memory usage is reasonable
- [ ] Theme can be serialized/deserialized
- [ ] Works with all supported frameworks