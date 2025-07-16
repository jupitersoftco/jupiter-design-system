# Testing Best Practices

## Core Principles

### 1. Tests as Documentation
Tests should serve as living documentation that explains how the design system works. Anyone reading a test should understand the intended behavior and usage patterns.

```rust
// ❌ Poor test - unclear intent
#[test]
fn test_text() {
    let t = text_styles(c).title().classes();
    assert!(t.contains("text-4xl"));
}

// ✅ Excellent test - clear documentation
#[test]
fn test_title_hierarchy_provides_visual_prominence() {
    let title_classes = text_styles(colors)
        .title()
        .classes();

    // Title hierarchy should be visually prominent with large text
    assert!(title_classes.contains("text-4xl"),
        "Title hierarchy should use large text (text-4xl) for visual prominence");
    
    // Title should be bold to establish clear hierarchy
    assert!(title_classes.contains("font-bold"),
        "Title hierarchy should use bold weight to establish clear hierarchy");
    
    // Title should have tight tracking for better readability at large sizes
    assert!(title_classes.contains("tracking-tight"),
        "Title hierarchy should use tight letter spacing for large text readability");
}
```

### 2. Test Behavior, Not Implementation
Focus on testing what the system does (behavior) rather than how it does it (implementation).

```rust
// ❌ Testing implementation details
#[test]
fn test_title_uses_specific_css_classes() {
    let pattern = TypographyPattern::new(colors);
    // Testing internal structure
    assert_eq!(pattern.hierarchy, TypographyHierarchy::Body); // Implementation detail
}

// ✅ Testing behavior
#[test]
fn test_title_creates_visual_hierarchy() {
    let title_classes = text_styles(colors).title().classes();
    let body_classes = text_styles(colors).body().classes();
    
    // Title should be visually larger than body text
    assert!(title_classes.contains("text-4xl"));
    assert!(body_classes.contains("text-base"));
    
    // Title should be bolder than body text
    assert!(title_classes.contains("font-bold"));
    assert!(body_classes.contains("font-normal"));
}
```

### 3. Comprehensive Coverage
Test all public APIs, enum variants, and edge cases. No public functionality should be untested.

```rust
#[test]
fn test_all_typography_colors_comprehensive() {
    let color_mappings = vec![
        ("primary", "text-jupiter-blue-500"),
        ("secondary", "text-gray-600"),
        ("tertiary", "text-gray-400"),
        ("success", "text-green-600"),
        ("warning", "text-yellow-600"),
        ("error", "text-red-600"),
        ("info", "text-blue-600"),
    ];

    for (color_name, expected_class) in color_mappings {
        let classes = text_styles(colors.clone())
            .body()
            .color_str(color_name)
            .classes();

        assert!(
            classes.contains(expected_class),
            "Color '{}' should map to class '{}' but got: '{}'",
            color_name, expected_class, classes
        );
    }
}
```

## Test Organization

### File Structure Best Practices

```
src/
├── builders/
│   ├── text.rs           # Implementation (~400 lines)
│   ├── text_test.rs      # Tests (~400+ lines)
│   ├── card.rs           # Implementation (~500 lines)
│   └── card_test.rs      # Tests (~500+ lines)
├── patterns/
│   ├── typography.rs     # Implementation (~300 lines)
│   └── typography_test.rs # Tests (~300+ lines)
└── core/
    ├── color.rs          # Implementation (~200 lines)
    └── color_test.rs     # Tests (~200+ lines)
```

**Key Points:**
- Test files should be roughly the same size as implementation files
- Co-locate tests with implementation for easy maintenance
- Use descriptive test file names that match implementation files

### Test Module Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::themes::VibeColors;

    // Helper functions at the top
    fn create_text_styles() -> TextStyles<VibeColors> {
        text_styles(VibeColors::default())
    }

    // Group tests by functionality
    mod hierarchy_tests {
        use super::*;

        #[test] fn test_title_hierarchy() { }
        #[test] fn test_heading_hierarchy() { }
        #[test] fn test_body_hierarchy() { }
    }

    mod color_tests {
        use super::*;

        #[test] fn test_primary_color() { }
        #[test] fn test_secondary_color() { }
        #[test] fn test_auto_color_selection() { }
    }

    mod api_tests {
        use super::*;

        #[test] fn test_method_chaining() { }
        #[test] fn test_string_apis() { }
        #[test] fn test_builder_reuse() { }
    }
}
```

## Naming Conventions

### Test Function Names

Use descriptive names that explain the behavior being tested:

```rust
// ❌ Poor naming
#[test] fn test_text() { }
#[test] fn test_card_big() { }
#[test] fn test_button_disabled() { }

// ✅ Excellent naming
#[test] fn test_text_builder_applies_hierarchy_defaults() { }
#[test] fn test_card_large_size_increases_padding() { }
#[test] fn test_button_disabled_state_removes_interactions() { }

// ✅ Even better - explains the "why"
#[test] fn test_title_hierarchy_ensures_visual_prominence() { }
#[test] fn test_card_elevation_provides_visual_depth() { }
#[test] fn test_disabled_button_prevents_user_confusion() { }
```

### Variable Names

Use descriptive variable names in tests:

```rust
// ❌ Unclear variables
#[test]
fn test_something() {
    let t = text_styles(c).title().classes();
    let b = text_styles(c).body().classes();
    assert!(t.contains("text-4xl"));
}

// ✅ Clear variables
#[test]
fn test_title_hierarchy_larger_than_body() {
    let title_classes = text_styles(colors).title().classes();
    let body_classes = text_styles(colors).body().classes();
    
    assert!(title_classes.contains("text-4xl"));
    assert!(body_classes.contains("text-base"));
}
```

## Error Messages

### Descriptive Error Messages

Always provide context in error messages:

```rust
// ❌ Poor error messages
assert!(classes.contains("text-lg"));
assert_eq!(a, b);

// ✅ Excellent error messages
assert!(
    classes.contains("text-lg"),
    "Expected size override 'text-lg' not found in classes: '{}'",
    classes
);

assert_eq!(
    enum_classes, string_classes,
    "Enum API and string API should produce identical output.\nEnum: '{}'\nString: '{}'",
    enum_classes, string_classes
);

// ✅ Even better - explain the impact
assert!(
    title_classes.contains("font-bold"),
    "Title hierarchy must use bold font weight to establish clear visual hierarchy. \
     Without bold weight, users cannot distinguish titles from body text. \
     Current classes: '{}'",
    title_classes
);
```

### Context-Rich Assertions

Provide context about what went wrong and why it matters:

```rust
#[test]
fn test_button_states_provide_clear_feedback() {
    let default_button = button_styles(colors.clone()).primary().classes();
    let disabled_button = button_styles(colors).primary().disabled().classes();

    assert!(
        default_button.contains("hover:bg-jupiter-blue-600"),
        "Default buttons must provide hover feedback to indicate interactivity. \
         Without hover states, users cannot tell if buttons are clickable. \
         Current classes: '{}'",
        default_button
    );

    assert!(
        disabled_button.contains("opacity-50"),
        "Disabled buttons must be visually distinct to prevent user confusion. \
         Users need clear visual indication that the button cannot be clicked. \
         Current classes: '{}'",
        disabled_button
    );

    assert!(
        !disabled_button.contains("hover:bg-jupiter-blue-600"),
        "Disabled buttons must not have hover states to reinforce their disabled state. \
         Hover effects on disabled buttons create confusing user experience. \
         Current classes: '{}'",
        disabled_button
    );
}
```

## Test Data Management

### Consistent Test Data

Use helper functions to create consistent test data:

```rust
// Helper functions for consistent test data
fn create_vibe_colors() -> VibeColors {
    VibeColors::default()
}

fn create_psychedelic_colors() -> PsychedelicColors {
    PsychedelicColors::default()
}

fn create_sample_text_props() -> TextProps {
    TextProps {
        variant: "body".to_string(),
        size: None,
        weight: None,
        color: None,
        align: None,
        truncate: false,
        clamp_lines: None,
        element: None,
        class: None,
    }
}
```

### Parameterized Testing

Use data-driven tests to reduce duplication:

```rust
#[test]
fn test_all_hierarchy_mappings() {
    let hierarchy_expectations = vec![
        ("title", vec!["text-4xl", "font-bold", "tracking-tight"]),
        ("heading", vec!["text-3xl", "font-bold", "tracking-tight"]),
        ("subheading", vec!["text-2xl", "font-bold", "tracking-tight"]),
        ("h4", vec!["text-xl", "font-bold", "tracking-tight"]),
        ("body", vec!["text-base", "font-normal"]),
        ("body-large", vec!["text-lg", "font-normal"]),
        ("body-small", vec!["text-sm", "font-normal"]),
        ("caption", vec!["text-sm", "font-medium"]),
        ("overline", vec!["text-xs", "font-medium", "uppercase", "tracking-wider"]),
        ("code", vec!["font-mono", "bg-gray-100", "px-1", "py-0.5", "rounded"]),
    ];

    for (hierarchy, expected_classes) in hierarchy_expectations {
        let classes = create_text_styles()
            .hierarchy_str(hierarchy)
            .classes();

        for expected_class in expected_classes {
            assert!(
                classes.contains(expected_class),
                "Hierarchy '{}' should contain class '{}' for proper visual presentation.\nActual classes: '{}'",
                hierarchy, expected_class, classes
            );
        }
    }
}
```

## Performance Best Practices

### Efficient Test Execution

Keep tests fast and efficient:

```rust
// ✅ Fast test - minimal setup
#[test]
fn test_title_hierarchy() {
    let classes = text_styles(VibeColors::default()).title().classes();
    assert!(classes.contains("text-4xl"));
}

// ❌ Slow test - unnecessary complexity
#[test]
fn test_title_hierarchy_slow() {
    for _ in 0..1000 {  // Unnecessary loop
        let colors = VibeColors::default();
        let pattern = TypographyPattern::new(colors);
        let builder = TextStyles::new(pattern);
        let classes = builder.title().classes();
        assert!(classes.contains("text-4xl"));
    }
}
```

### Batch Related Tests

Group related assertions in single tests when appropriate:

```rust
#[test]
fn test_card_elevation_complete_behavior() {
    let flat = card_styles(colors.clone()).flat().classes();
    let outlined = card_styles(colors.clone()).outlined().classes();
    let elevated = card_styles(colors).elevated().classes();

    // Test all elevation variants together for efficiency
    assert!(!flat.contains("shadow"));
    assert!(!flat.contains("border"));

    assert!(outlined.contains("border"));
    assert!(!outlined.contains("shadow"));

    assert!(elevated.contains("shadow-lg"));
    assert!(elevated.contains("hover:shadow-xl"));
}
```

## Edge Case Testing

### Invalid Input Handling

Test graceful degradation with invalid inputs:

```rust
#[test]
fn test_invalid_inputs_graceful_degradation() {
    let base_classes = text_styles(colors).body().classes();

    // Test various invalid inputs
    let invalid_inputs = vec![
        ("", "empty string"),
        ("invalid", "invalid option"),
        ("title-wrong", "similar but wrong"),
        ("TITLE", "wrong case"),
        ("title ", "trailing space"),
        (" title", "leading space"),
    ];

    for (invalid_input, description) in invalid_inputs {
        let result_classes = text_styles(colors.clone())
            .body()
            .hierarchy_str(invalid_input)
            .classes();

        assert_eq!(
            base_classes, result_classes,
            "Invalid input '{}' ({}) should not change output. \
             Expected: '{}'\nActual: '{}'",
            invalid_input, description, base_classes, result_classes
        );
    }
}
```

### Boundary Testing

Test edge cases and boundary conditions:

```rust
#[test]
fn test_class_deduplication_edge_cases() {
    // Test various deduplication scenarios
    let test_cases = vec![
        ("text-gray-900 text-gray-900", 1, "simple duplicate"),
        ("text-gray-900 p-4 text-gray-900 p-4", 1, "multiple duplicates"),
        ("text-gray-900", 1, "no duplicates"),
        ("", 0, "empty string"),
        ("text-gray-900 text-gray-800", 2, "similar but different"),
    ];

    for (custom_classes, expected_gray_900_count, description) in test_cases {
        let classes = text_styles(colors.clone())
            .body()
            .custom_classes(custom_classes)
            .classes();

        let actual_count = classes.matches("text-gray-900").count();
        assert_eq!(
            actual_count, expected_gray_900_count,
            "Test case '{}': Expected {} occurrences of 'text-gray-900', found {}.\nClasses: '{}'",
            description, expected_gray_900_count, actual_count, classes
        );
    }
}
```

## Maintainability Patterns

### Test Helper Functions

Create reusable test utilities:

```rust
// Helper functions for common test patterns
fn assert_contains_all_classes(classes: &str, expected: &[&str], context: &str) {
    for expected_class in expected {
        assert!(
            classes.contains(expected_class),
            "{}: Expected class '{}' not found in '{}'",
            context, expected_class, classes
        );
    }
}

fn assert_visual_hierarchy(larger: &str, smaller: &str, context: &str) {
    let size_order = vec!["text-xs", "text-sm", "text-base", "text-lg", "text-xl", "text-2xl", "text-3xl", "text-4xl"];
    
    let larger_index = size_order.iter().position(|&s| larger.contains(s));
    let smaller_index = size_order.iter().position(|&s| smaller.contains(s));
    
    match (larger_index, smaller_index) {
        (Some(l), Some(s)) => assert!(
            l > s,
            "{}: '{}' should be visually larger than '{}'",
            context, larger, smaller
        ),
        _ => panic!("{}: Could not determine size hierarchy", context),
    }
}

fn assert_no_conflicting_classes(classes: &str, context: &str) {
    let conflicts = vec![
        (vec!["text-left", "text-center", "text-right"], "text alignment"),
        (vec!["p-2", "p-4", "p-6", "p-8"], "padding"),
        (vec!["m-2", "m-4", "m-6", "m-8"], "margin"),
    ];

    for (conflicting_classes, conflict_type) in conflicts {
        let found_classes: Vec<&str> = conflicting_classes
            .iter()
            .filter(|&class| classes.contains(class))
            .copied()
            .collect();

        assert!(
            found_classes.len() <= 1,
            "{}: Found conflicting {} classes: {:?} in '{}'",
            context, conflict_type, found_classes, classes
        );
    }
}
```

### Test Constants

Use constants for repeated values:

```rust
const JUPITER_BLUE_500: &str = "text-jupiter-blue-500";
const JUPITER_BLUE_600: &str = "hover:bg-jupiter-blue-600";
const TITLE_SIZE: &str = "text-4xl";
const TITLE_WEIGHT: &str = "font-bold";
const TITLE_TRACKING: &str = "tracking-tight";

#[test]
fn test_title_visual_characteristics() {
    let classes = text_styles(colors).title().primary().classes();
    
    assert_contains_all_classes(
        &classes,
        &[TITLE_SIZE, TITLE_WEIGHT, TITLE_TRACKING, JUPITER_BLUE_500],
        "Title with primary color"
    );
}
```

## Documentation Integration

### Living Documentation

Tests should serve as examples for developers:

```rust
/// Example: Creating a prominent page title
#[test]
fn example_page_title() {
    let page_title = text_styles(colors)
        .title()           // Large, bold text for prominence
        .primary()         // Brand color for consistency
        .center()          // Centered for formal layouts
        .classes();

    // This produces a prominent, centered title suitable for page headers
    assert_contains_all_classes(
        &page_title,
        &["text-4xl", "font-bold", "text-jupiter-blue-500", "text-center"],
        "Page title example"
    );
}

/// Example: Creating accessible button states
#[test] 
fn example_accessible_buttons() {
    let interactive_button = button_styles(colors.clone())
        .primary()
        .classes();

    let disabled_button = button_styles(colors)
        .primary()
        .disabled()
        .classes();

    // Interactive buttons provide clear hover feedback
    assert!(interactive_button.contains("hover:bg-jupiter-blue-600"));
    
    // Disabled buttons are visually distinct and non-interactive
    assert!(disabled_button.contains("opacity-50"));
    assert!(disabled_button.contains("cursor-not-allowed"));
    assert!(!disabled_button.contains("hover:"));
}
```

## Continuous Improvement

### Test Metrics

Track test quality metrics:

```rust
#[test]
fn test_coverage_metrics() {
    // This test ensures we maintain comprehensive coverage
    let hierarchy_variants = vec![
        "title", "heading", "subheading", "h4", "body", 
        "body-large", "body-small", "caption", "overline", "code"
    ];

    let color_variants = vec![
        "primary", "secondary", "tertiary", "success", 
        "warning", "error", "info"
    ];

    let size_variants = vec![
        "xs", "sm", "md", "lg", "xl", "2xl", "3xl", "4xl"
    ];

    // Ensure all variants are testable
    for hierarchy in &hierarchy_variants {
        let classes = text_styles(colors.clone())
            .hierarchy_str(hierarchy)
            .classes();
        assert!(!classes.is_empty(), "Hierarchy '{}' should produce classes", hierarchy);
    }

    println!("✅ Coverage: {} hierarchies, {} colors, {} sizes", 
        hierarchy_variants.len(), color_variants.len(), size_variants.len());
}
```

### Performance Monitoring

Include performance benchmarks in tests:

```rust
#[test]
fn test_performance_benchmarks() {
    let start = std::time::Instant::now();
    
    // Simulate typical usage patterns
    for _ in 0..1000 {
        let _classes = text_styles(colors.clone())
            .title()
            .primary()
            .center()
            .classes();
    }
    
    let duration = start.elapsed();
    
    assert!(
        duration.as_millis() < 100,
        "Performance regression: class generation took {}ms (expected < 100ms)",
        duration.as_millis()
    );
}
```

These best practices ensure that tests in the Jupiter Design System are reliable, maintainable, and serve as comprehensive documentation for the design system's behavior and intended usage patterns.