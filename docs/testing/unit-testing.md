# Unit Testing Guide

## Overview

Unit tests in the Jupiter Design System focus on testing individual builder APIs in isolation. They validate that each builder produces the correct CSS classes and handles various input scenarios properly.

## Test Structure

### File Organization

Tests are co-located with their implementation files:

```
src/builders/
├── text.rs           # TextStyles implementation
├── text_test.rs      # TextStyles unit tests
├── card.rs           # CardStyles implementation  
├── card_test.rs      # CardStyles unit tests
└── button.rs         # ButtonStyles implementation
    button_test.rs    # ButtonStyles unit tests

src/core/
├── color.rs          # Color system implementation
└── color_test.rs     # Color system unit tests
```

### Test Module Declaration

```rust
// In src/builders/mod.rs
pub mod text;
#[cfg(test)]
mod text_test;
```

### Basic Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::themes::VibeColors;

    // Helper function to create text styles with default color provider
    fn create_text_styles() -> TextStyles<VibeColors> {
        text_styles(VibeColors::default())
    }

    // Tests go here...
}
```

## Core Testing Patterns

### 1. String API Testing

Test that string-based methods produce expected CSS classes:

```rust
#[test]
fn test_text_hierarchy_str() {
    // Test all hierarchy string mappings
    let hierarchies = vec![
        ("title", "font-bold text-4xl text-gray-900 tracking-tight"),
        ("heading", "font-bold text-3xl text-gray-900 tracking-tight"),
        ("subheading", "font-bold text-2xl text-gray-900 tracking-tight"),
        ("h4", "font-bold text-xl text-gray-900 tracking-tight"),
        ("body", "font-normal text-base text-gray-900"),
        ("body-large", "font-normal text-lg text-gray-900"),
        ("body-small", "font-normal text-sm text-gray-900"),
        ("caption", "font-medium text-sm text-gray-600"),
        ("overline", "font-medium text-xs text-gray-400 tracking-wider uppercase"),
        ("code", "bg-gray-100 font-mono px-1 py-0.5 rounded text-sm"),
    ];

    for (hierarchy, expected_content) in hierarchies {
        let classes = create_text_styles().hierarchy_str(hierarchy).classes();
        
        // All text should have base typography
        assert!(
            classes.contains("leading-relaxed"),
            "All text should have leading-relaxed"
        );

        // Check that all expected classes are present
        for expected_class in expected_content.split_whitespace() {
            assert!(
                classes.contains(expected_class),
                "Hierarchy '{}' should contain '{}' but got '{}'",
                hierarchy,
                expected_class,
                classes
            );
        }
    }
}
```

### 2. Convenience Method Testing

Test enum-based convenience methods:

```rust
#[test]
fn test_text_convenience_methods() {
    // Test hierarchy convenience methods
    let title_classes = create_text_styles().title().classes();
    assert!(title_classes.contains("text-4xl"));
    assert!(title_classes.contains("font-bold"));
    assert!(title_classes.contains("tracking-tight"));

    let heading_classes = create_text_styles().heading().classes();
    assert!(heading_classes.contains("text-3xl"));
    assert!(heading_classes.contains("font-bold"));

    let body_classes = create_text_styles().body().classes();
    assert!(body_classes.contains("text-base"));
    assert!(body_classes.contains("font-normal"));

    let caption_classes = create_text_styles().caption().classes();
    assert!(caption_classes.contains("text-sm"));
    assert!(caption_classes.contains("font-medium"));
    assert!(caption_classes.contains("text-gray-600"));
}
```

### 3. Override Behavior Testing

Test that overrides work correctly and remove default values:

```rust
#[test]
fn test_text_size_override() {
    // Test that size override works
    let classes = create_text_styles().title().size_str("sm").classes();
    assert!(classes.contains("text-sm"));
    assert!(!classes.contains("text-4xl")); // Should not contain default title size

    let classes = create_text_styles().body().extra_large().classes();
    assert!(classes.contains("text-xl"));
    assert!(!classes.contains("text-base")); // Should not contain default body size
}

#[test]
fn test_text_weight_override() {
    // Test that weight override works
    let classes = create_text_styles().body().bold().classes();
    assert!(classes.contains("font-bold"));
    assert!(!classes.contains("font-normal")); // Should not contain default body weight

    let classes = create_text_styles().title().light().classes();
    assert!(classes.contains("font-light"));
    assert!(!classes.contains("font-bold")); // Should not contain default title weight
}
```

### 4. Color Override Testing

Test color system integration:

```rust
#[test]
fn test_text_color_override() {
    let classes = create_text_styles().body().primary().classes();
    assert!(classes.contains("text-jupiter-blue-500"));
    assert!(!classes.contains("text-gray-900")); // Should not contain default body color

    let classes = create_text_styles().body().secondary().classes();
    assert!(classes.contains("text-gray-600"));

    let classes = create_text_styles().body().color_str("success").classes();
    assert!(classes.contains("text-green-600"));
}
```

### 5. Method Chaining Testing

Test that fluent interface works correctly:

```rust
#[test]
fn test_text_chaining() {
    let classes = create_text_styles()
        .body()
        .large()
        .bold()
        .primary()
        .center()
        .classes();

    // Should contain all chained modifications
    assert!(classes.contains("text-lg"));              // large
    assert!(classes.contains("font-bold"));            // bold
    assert!(classes.contains("text-jupiter-blue-500")); // primary
    assert!(classes.contains("text-center"));          // center
    
    // Should not contain overridden defaults
    assert!(!classes.contains("text-base"));           // No default body size
    assert!(!classes.contains("font-normal"));         // No default body weight
    assert!(!classes.contains("text-gray-900"));       // No default body color
}
```

### 6. Invalid Input Testing

Test graceful degradation with invalid inputs:

```rust
#[test]
fn test_invalid_string_inputs() {
    let base_classes = create_text_styles().body().classes();

    // Invalid hierarchy strings should not change output
    let invalid_hierarchy = create_text_styles().body().hierarchy_str("invalid").classes();
    assert_eq!(base_classes, invalid_hierarchy);

    // Invalid size strings should not change output
    let invalid_size = create_text_styles().body().size_str("invalid").classes();
    assert_eq!(base_classes, invalid_size);

    // Invalid color strings should not change output
    let invalid_color = create_text_styles().body().color_str("invalid").classes();
    assert_eq!(base_classes, invalid_color);
}
```

## Component Function Testing

Test utility functions that convert props to classes:

```rust
#[test]
fn test_text_classes_from_strings() {
    let classes = text_classes_from_strings(
        VibeColors::default(),
        "title",
        Some("lg"),
        Some("bold"),
        Some("primary"),
        Some("center"),
        true,     // truncate
        None,     // clamp_lines
        Some("custom-class"),
    );

    // Should contain all specified properties
    assert!(classes.contains("text-lg"));              // size override
    assert!(classes.contains("font-bold"));            // weight
    assert!(classes.contains("text-jupiter-blue-500")); // primary color
    assert!(classes.contains("text-center"));          // alignment
    assert!(classes.contains("truncate"));             // truncate
    assert!(classes.contains("custom-class"));         // custom classes
    
    // Should contain title hierarchy features
    assert!(classes.contains("tracking-tight"));       // title tracking
}
```

## Helper Function Testing

Test element selection and utility functions:

```rust
#[test]
fn test_text_element_from_hierarchy() {
    assert_eq!(text_element_from_hierarchy("title"), "h1");
    assert_eq!(text_element_from_hierarchy("heading"), "h2");
    assert_eq!(text_element_from_hierarchy("subheading"), "h3");
    assert_eq!(text_element_from_hierarchy("h4"), "h4");
    assert_eq!(text_element_from_hierarchy("body"), "p");
    assert_eq!(text_element_from_hierarchy("invalid"), "p"); // Default
}

#[test]
fn test_text_clamp_style() {
    assert_eq!(text_clamp_style(Some(2)), "line-clamp-2");
    assert_eq!(text_clamp_style(Some(3)), "line-clamp-3");
    assert_eq!(text_clamp_style(None), "");
}
```

## Card Builder Testing

Similar patterns for card builders:

```rust
#[test]
fn test_card_variants() {
    let flat = create_card_styles().flat().classes();
    assert!(!flat.contains("shadow"));
    assert!(!flat.contains("border"));

    let outlined = create_card_styles().outlined().classes();
    assert!(outlined.contains("border"));
    assert!(!outlined.contains("shadow"));

    let elevated = create_card_styles().elevated().classes();
    assert!(elevated.contains("shadow-lg"));
}

#[test]
fn test_card_sizes() {
    let small = create_card_styles().small().classes();
    assert!(small.contains("p-2"));

    let large = create_card_styles().large().classes();
    assert!(large.contains("p-8"));
}
```

## Running Unit Tests

### Individual Test Execution

```bash
# Run all text builder tests
cargo test text_test

# Run all card builder tests  
cargo test card_test

# Run specific test with output
cargo test test_text_hierarchy_str -- --nocapture

# Run all tests quietly
cargo test --quiet
```

### Test Organization

Tests are organized in simple flat modules:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // All tests in flat structure
    #[test] fn test_hierarchy() { }
    #[test] fn test_colors() { }
    #[test] fn test_overrides() { }
}
```

## Best Practices for Unit Tests

### 1. Clear Test Names
```rust
// ✅ Clear and descriptive
#[test]
fn test_title_hierarchy_applies_large_bold_styling() { }

// ❌ Unclear
#[test] 
fn test_title() { }
```

### 2. Descriptive Error Messages
```rust
// ✅ Helpful error message
assert!(
    classes.contains("text-4xl"),
    "Title hierarchy should use large text (text-4xl), got: '{}'",
    classes
);

// ❌ Minimal error message
assert!(classes.contains("text-4xl"));
```

### 3. Test One Thing at a Time
```rust
// ✅ Focused test
#[test]
fn test_size_override_removes_default() {
    let classes = create_text_styles().title().size_str("sm").classes();
    assert!(classes.contains("text-sm"));
    assert!(!classes.contains("text-4xl"));
}

// ❌ Testing too many things
#[test]
fn test_everything() {
    // Tests size, color, weight, alignment all together
}
```

This unit testing approach ensures comprehensive coverage of builder APIs while maintaining clear, maintainable tests that accurately reflect the actual implementation in the Jupiter Design System.