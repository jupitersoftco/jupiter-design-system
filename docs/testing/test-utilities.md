# Test Utilities Guide

## ⚠️ Current Reality vs Recommendations

This guide covers **both existing utilities and recommended utilities**. Most sophisticated utilities described here are **not yet implemented** and represent future improvements.

## Current Test Utilities

### Existing Helper Functions

The current codebase includes basic helper functions in test modules:

```rust
// Found in text_test.rs and other test files
fn create_text_styles() -> TextStyles<VibeColors> {
    text_styles(VibeColors::default())
}

fn create_card_styles() -> CardStyles<VibeColors> {
    card_styles(VibeColors::default())
}

fn create_button_styles() -> ButtonStyles<VibeColors> {
    button_styles(VibeColors::default())
}
```

### Basic Test Pattern

Current tests follow this pattern:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::themes::VibeColors;

    fn create_text_styles() -> TextStyles<VibeColors> {
        text_styles(VibeColors::default())
    }

    #[test]
    fn test_something() {
        let classes = create_text_styles().title().classes();
        assert!(classes.contains("text-4xl"));
        assert!(classes.contains("font-bold"));
    }
}
```

## Recommended Test Utilities

### Enhanced Helper Functions

To improve test maintainability, consider implementing these utilities:

```rust
/// Assert that classes contain all expected values with descriptive error messages
fn assert_contains_all_classes(classes: &str, expected: &[&str], context: &str) {
    for expected_class in expected {
        assert!(
            classes.contains(expected_class),
            "{}: Expected class '{}' not found in classes: '{}'",
            context, expected_class, classes
        );
    }
}

/// Assert that classes don't contain any forbidden values
fn assert_excludes_all_classes(classes: &str, forbidden: &[&str], context: &str) {
    for forbidden_class in forbidden {
        assert!(
            !classes.contains(forbidden_class),
            "{}: Forbidden class '{}' found in classes: '{}'",
            context, forbidden_class, classes
        );
    }
}

/// Count occurrences of a specific class and assert the count
fn assert_class_count(classes: &str, class: &str, expected_count: usize, context: &str) {
    let actual_count = classes.matches(class).count();
    assert_eq!(
        actual_count, expected_count,
        "{}: Expected {} occurrences of '{}', found {} in: '{}'",
        context, expected_count, class, actual_count, classes
    );
}
```

### CSS Class Constants

To avoid magic strings in tests, consider defining constants:

```rust
// Typography constants
const SIZE_XS: &str = "text-xs";
const SIZE_SM: &str = "text-sm"; 
const SIZE_BASE: &str = "text-base";
const SIZE_LG: &str = "text-lg";
const SIZE_XL: &str = "text-xl";
const SIZE_2XL: &str = "text-2xl";
const SIZE_3XL: &str = "text-3xl";
const SIZE_4XL: &str = "text-4xl";

const WEIGHT_NORMAL: &str = "font-normal";
const WEIGHT_MEDIUM: &str = "font-medium";
const WEIGHT_BOLD: &str = "font-bold";

const JUPITER_BLUE_500: &str = "text-jupiter-blue-500";
const GRAY_600: &str = "text-gray-600";
const GRAY_900: &str = "text-gray-900";

// Layout constants
const PADDING_SM: &str = "p-2";
const PADDING_MD: &str = "p-4";
const PADDING_LG: &str = "p-6";
const PADDING_XL: &str = "p-8";

const SHADOW_LG: &str = "shadow-lg";
const BORDER: &str = "border";
```

### Improved Test Structure

With utilities, tests become more readable:

```rust
#[test]
fn test_title_hierarchy_with_utilities() {
    let classes = create_text_styles().title().classes();
    
    assert_contains_all_classes(
        &classes,
        &[SIZE_4XL, WEIGHT_BOLD, "tracking-tight"],
        "Title hierarchy"
    );
    
    assert_excludes_all_classes(
        &classes,
        &[SIZE_SM, WEIGHT_NORMAL],
        "Title should not have small/normal styling"
    );
}
```

## Data-Driven Testing

### Parameterized Test Helpers

For testing multiple variants systematically:

```rust
/// Test all hierarchy variants with expected classes
fn test_hierarchy_variants() {
    let hierarchies = vec![
        ("title", vec![SIZE_4XL, WEIGHT_BOLD, "tracking-tight"]),
        ("heading", vec![SIZE_3XL, WEIGHT_BOLD, "tracking-tight"]),
        ("subheading", vec![SIZE_2XL, WEIGHT_BOLD, "tracking-tight"]),
        ("h4", vec![SIZE_XL, WEIGHT_BOLD, "tracking-tight"]),
        ("body", vec![SIZE_BASE, WEIGHT_NORMAL]),
        ("body-large", vec![SIZE_LG, WEIGHT_NORMAL]),
        ("body-small", vec![SIZE_SM, WEIGHT_NORMAL]),
        ("caption", vec![SIZE_SM, WEIGHT_MEDIUM, GRAY_600]),
    ];

    for (hierarchy, expected_classes) in hierarchies {
        let classes = create_text_styles().hierarchy_str(hierarchy).classes();
        
        assert_contains_all_classes(
            &classes,
            &expected_classes,
            &format!("Hierarchy: {}", hierarchy)
        );
    }
}
```

### Color Testing Utilities

For systematic color testing:

```rust
fn test_all_color_mappings() {
    let color_mappings = vec![
        ("primary", JUPITER_BLUE_500),
        ("secondary", GRAY_600),
        ("tertiary", "text-gray-400"),
        ("success", "text-green-600"),
        ("warning", "text-yellow-600"),
        ("error", "text-red-600"),
    ];

    for (color_name, expected_class) in color_mappings {
        let classes = create_text_styles()
            .body()
            .color_str(color_name)
            .classes();

        assert!(
            classes.contains(expected_class),
            "Color '{}' should map to '{}' but got: '{}'",
            color_name, expected_class, classes
        );
    }
}
```

## Error Message Utilities

### Context-Rich Assertions

Current tests often have minimal error messages. Enhanced utilities provide context:

```rust
// Current approach
assert!(classes.contains("text-4xl"));

// Enhanced approach
assert_contains_class_with_context(
    &classes,
    "text-4xl",
    "Title hierarchy",
    "Titles need large text for visual prominence and clear hierarchy"
);

fn assert_contains_class_with_context(
    classes: &str,
    expected_class: &str,
    component: &str,
    reason: &str,
) {
    assert!(
        classes.contains(expected_class),
        "{}: Expected class '{}' not found. Reason: {}. Classes: '{}'",
        component, expected_class, reason, classes
    );
}
```

## Builder Testing Utilities

### Override Testing Helpers

```rust
/// Test that an override properly replaces default behavior
fn assert_override_behavior(
    default_builder: impl Fn() -> String,
    override_builder: impl Fn() -> String,
    expected_class: &str,
    removed_class: &str,
    context: &str,
) {
    let default_classes = default_builder();
    let override_classes = override_builder();
    
    // Should contain the override
    assert!(
        override_classes.contains(expected_class),
        "{}: Override should add '{}' in: '{}'",
        context, expected_class, override_classes
    );
    
    // Should remove the default
    assert!(
        !override_classes.contains(removed_class),
        "{}: Override should remove default '{}' from: '{}'",
        context, removed_class, override_classes
    );
}

// Usage example:
#[test]
fn test_size_override_with_utility() {
    assert_override_behavior(
        || create_text_styles().title().classes(),
        || create_text_styles().title().size_str("sm").classes(),
        SIZE_SM,          // expected_class
        SIZE_4XL,         // removed_class
        "Title size override"
    );
}
```

### Method Chaining Validation

```rust
/// Test that method chaining produces expected cumulative results
fn test_chaining_behavior(
    base_method: &str,
    chain_methods: Vec<(&str, &str)>, // (method_name, expected_class)
    context: &str,
) {
    let mut builder = create_text_styles();
    let mut expected_classes = Vec::new();
    
    // Apply base method
    let base_classes = match base_method {
        "title" => builder.title().classes(),
        "body" => builder.body().classes(),
        _ => panic!("Unknown base method: {}", base_method),
    };
    
    // Apply chain methods and collect expectations
    for (method_name, expected_class) in chain_methods {
        expected_classes.push(expected_class);
    }
    
    // Verify all expected classes are present
    assert_contains_all_classes(&base_classes, &expected_classes, context);
}
```

## Performance Testing Utilities

### Benchmark Helpers

```rust
/// Simple performance testing utility
fn benchmark_class_generation<F>(
    name: &str,
    generator: F,
    iterations: usize,
    max_duration_ms: u64,
) where
    F: Fn() -> String,
{
    let start = std::time::Instant::now();
    
    for _ in 0..iterations {
        let _result = generator();
    }
    
    let duration = start.elapsed();
    
    assert!(
        duration.as_millis() < max_duration_ms as u128,
        "Performance test {}: {} iterations took {}ms (expected < {}ms)",
        name, iterations, duration.as_millis(), max_duration_ms
    );
}

// Usage:
#[test]
fn test_text_generation_performance() {
    benchmark_class_generation(
        "text_title_generation",
        || create_text_styles().title().primary().classes(),
        1000,
        50, // max 50ms for 1000 iterations
    );
}
```

## Implementation Recommendations

### Step 1: Extract Common Helpers

Start by extracting the most commonly used patterns:

```rust
// In src/test_utils.rs (new file)
pub fn create_vibe_colors() -> VibeColors {
    VibeColors::default()
}

pub fn assert_contains_classes(classes: &str, expected: &[&str], context: &str) {
    // Implementation as shown above
}

pub fn assert_no_class_duplicates(classes: &str, context: &str) {
    let class_vec: Vec<&str> = classes.split_whitespace().collect();
    let unique_classes: std::collections::HashSet<&str> = 
        class_vec.iter().copied().collect();
    
    assert_eq!(
        class_vec.len(), unique_classes.len(),
        "{}: Found duplicate classes in: '{}'",
        context, classes
    );
}
```

### Step 2: Update Existing Tests

Gradually update existing tests to use the new utilities:

```rust
// Before
#[test]
fn test_title() {
    let classes = create_text_styles().title().classes();
    assert!(classes.contains("text-4xl"));
    assert!(classes.contains("font-bold"));
}

// After  
#[test]
fn test_title_with_utilities() {
    let classes = create_text_styles().title().classes();
    assert_contains_classes(
        &classes,
        &["text-4xl", "font-bold", "tracking-tight"],
        "Title hierarchy"
    );
    assert_no_class_duplicates(&classes, "Title classes");
}
```

### Step 3: Add Constants

Define constants for commonly used CSS classes to eliminate magic strings and improve maintainability.

## Current vs Recommended Approach

### Current Test (Basic)
```rust
#[test]
fn test_text_hierarchy_str() {
    let classes = create_text_styles().hierarchy_str("title").classes();
    assert!(classes.contains("text-4xl"));
    assert!(classes.contains("font-bold"));
}
```

### Recommended Test (With Utilities)
```rust
#[test]
fn test_text_hierarchy_str_with_utilities() {
    let classes = create_text_styles().hierarchy_str("title").classes();
    assert_contains_all_classes(
        &classes,
        &[SIZE_4XL, WEIGHT_BOLD, "tracking-tight"],
        "Title hierarchy string API"
    );
    assert_no_class_duplicates(&classes, "Title hierarchy");
}
```

## Benefits of Enhanced Test Utilities

1. **Consistency** - Standardized testing patterns across the codebase
2. **Maintainability** - Centralized utilities reduce duplication
3. **Readability** - Clear, descriptive test assertions
4. **Error Messages** - Rich context when tests fail
5. **Reliability** - Systematic validation of edge cases

These test utilities would significantly improve the testing experience and maintainability of the Jupiter Design System while preserving the current focus on builder API testing.