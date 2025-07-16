# Visual Testing Guide

## ⚠️ Theoretical Approach - Not Implemented

This guide outlines a **theoretical approach** to visual testing for the Jupiter Design System. **Most patterns described here are not currently implemented** and represent future possibilities.

## Visual Testing Philosophy

### 1. Style Validation Over Pixel Perfection
Focus on validating that the correct CSS classes are applied rather than pixel-perfect visual matching. This approach is more maintainable and less brittle than traditional screenshot testing.

### 2. Component-Level Visual Testing
Test individual components in isolation before testing complex layouts. This makes it easier to identify the source of visual regressions.

### 3. Cross-Browser Consistency
Ensure that the design system renders consistently across different browsers and devices, accounting for browser-specific CSS differences.

## Testing Strategy

### Current Approach: CSS Class Validation

Since Jupiter Design System generates CSS classes, visual testing primarily focuses on validating that the correct classes are applied:

```rust
#[test]
fn test_visual_hierarchy_classes() {
    let title = text_styles(colors)
        .title()
        .primary()
        .center()
        .classes();

    // Validate visual hierarchy classes
    assert!(title.contains("text-4xl"));        // Large size for visual prominence
    assert!(title.contains("font-bold"));       // Bold weight for hierarchy
    assert!(title.contains("tracking-tight"));  // Tight tracking for titles
    assert!(title.contains("text-jupiter-blue-500")); // Primary color
    assert!(title.contains("text-center"));     // Center alignment
    assert!(title.contains("leading-relaxed")); // Comfortable line height

    // Ensure visual consistency markers are present
    assert!(!title.contains("text-xs"));        // Should not have small text
    assert!(!title.contains("font-light"));     // Should not have light weight
}
```

### Future Approach: Browser Testing

When integrated with web frameworks, visual testing can be enhanced with browser-based testing:

```javascript
// Example using Playwright or similar
describe('Jupiter Design System Visual Tests', () => {
  test('title typography renders correctly', async ({ page }) => {
    await page.goto('/design-system/typography');
    
    const title = page.locator('[data-testid="title-example"]');
    
    // Validate computed styles
    await expect(title).toHaveCSS('font-size', '36px');      // text-4xl
    await expect(title).toHaveCSS('font-weight', '700');     // font-bold
    await expect(title).toHaveCSS('letter-spacing', '-0.025em'); // tracking-tight
    await expect(title).toHaveCSS('color', 'rgb(59, 130, 246)'); // jupiter-blue-500
    await expect(title).toHaveCSS('text-align', 'center');   // text-center
    
    // Visual regression testing
    await expect(title).toHaveScreenshot('title-typography.png');
  });
});
```

## Component Visual Testing

### Typography Components

Test visual characteristics of typography patterns:

```rust
#[test]
fn test_typography_visual_characteristics() {
    let hierarchies = vec![
        ("title", vec!["text-4xl", "font-bold", "tracking-tight"]),
        ("heading", vec!["text-3xl", "font-bold", "tracking-tight"]),
        ("subheading", vec!["text-2xl", "font-bold", "tracking-tight"]),
        ("body", vec!["text-base", "font-normal"]),
        ("caption", vec!["text-sm", "font-medium"]),
        ("overline", vec!["text-xs", "font-medium", "uppercase", "tracking-wider"]),
    ];

    for (hierarchy, expected_visual_classes) in hierarchies {
        let classes = text_styles(colors.clone())
            .hierarchy_str(hierarchy)
            .classes();

        for visual_class in expected_visual_classes {
            assert!(
                classes.contains(visual_class),
                "Hierarchy '{}' missing visual class '{}' in: '{}'",
                hierarchy, visual_class, classes
            );
        }

        // Ensure base typography is always applied
        assert!(classes.contains("leading-relaxed"),
            "All typography should have comfortable line height");
    }
}
```

### Card Visual Testing

Test card visual appearance and elevation:

```rust
#[test]
fn test_card_visual_elevation() {
    let flat_card = card_styles(colors.clone())
        .flat()
        .classes();

    let outlined_card = card_styles(colors.clone())
        .outlined()
        .classes();

    let elevated_card = card_styles(colors.clone())
        .elevated()
        .classes();

    // Flat card should have minimal visual weight
    assert!(!flat_card.contains("shadow"));
    assert!(!flat_card.contains("border"));

    // Outlined card should have border but no shadow
    assert!(outlined_card.contains("border"));
    assert!(!outlined_card.contains("shadow"));

    // Elevated card should have shadow for depth
    assert!(elevated_card.contains("shadow-lg"));
    assert!(elevated_card.contains("hover:shadow-xl")); // Enhanced on hover
}
```

### Button Visual States

Test button visual feedback states:

```rust
#[test]
fn test_button_visual_states() {
    let primary_button = button_styles(colors.clone())
        .primary()
        .classes();

    let secondary_button = button_styles(colors.clone())
        .secondary()
        .classes();

    let disabled_button = button_styles(colors.clone())
        .primary()
        .disabled()
        .classes();

    // Primary should have solid background
    assert!(primary_button.contains("bg-jupiter-blue-500"));
    assert!(primary_button.contains("text-white"));
    assert!(primary_button.contains("hover:bg-jupiter-blue-600"));

    // Secondary should have outlined appearance
    assert!(secondary_button.contains("border"));
    assert!(secondary_button.contains("border-jupiter-blue-500"));
    assert!(secondary_button.contains("text-jupiter-blue-500"));

    // Disabled should have reduced visual prominence
    assert!(disabled_button.contains("opacity-50"));
    assert!(disabled_button.contains("cursor-not-allowed"));
    assert!(!disabled_button.contains("hover:bg-jupiter-blue-600")); // No hover
}
```

## Color and Theme Visual Testing

### Color Contrast Validation

Test that color combinations meet accessibility standards:

```rust
#[test]
fn test_color_contrast_classes() {
    let dark_text_combinations = vec![
        ("text-gray-900", "bg-white"),      // High contrast
        ("text-gray-800", "bg-gray-50"),    // Good contrast
        ("text-jupiter-blue-900", "bg-jupiter-blue-50"), // Branded contrast
    ];

    for (text_color, bg_color) in dark_text_combinations {
        let classes = text_styles(colors.clone())
            .body()
            .custom_classes(&format!("{} {}", text_color, bg_color))
            .classes();

        assert!(classes.contains(text_color));
        assert!(classes.contains(bg_color));
        
        // Could be extended with actual contrast ratio calculation
        // when integrated with color parsing libraries
    }
}
```

### Theme Visual Consistency

Test that themes maintain visual consistency:

```rust
#[test]
fn test_theme_visual_consistency() {
    let vibe_primary = text_styles(VibeColors::default())
        .title()
        .primary()
        .classes();

    let psychedelic_primary = text_styles(PsychedelicColors::default())
        .title()
        .primary()
        .classes();

    // Both themes should maintain same visual hierarchy
    assert!(vibe_primary.contains("text-4xl"));
    assert!(vibe_primary.contains("font-bold"));
    assert!(psychedelic_primary.contains("text-4xl"));
    assert!(psychedelic_primary.contains("font-bold"));

    // Colors should be different but structurally similar
    assert!(vibe_primary.contains("text-jupiter-blue-500"));
    assert!(psychedelic_primary.contains("text-psychedelic-primary"));

    // Visual weight should be consistent
    let vibe_class_count = vibe_primary.split_whitespace().count();
    let psychedelic_class_count = psychedelic_primary.split_whitespace().count();
    
    // Should have similar number of classes (within reasonable range)
    let class_diff = (vibe_class_count as i32 - psychedelic_class_count as i32).abs();
    assert!(class_diff <= 2, 
        "Theme class counts too different: vibe={}, psychedelic={}", 
        vibe_class_count, psychedelic_class_count);
}
```

## Responsive Visual Testing

### Responsive Typography

Test that typography scales appropriately:

```rust
#[test]
fn test_responsive_typography_classes() {
    let responsive_title = text_styles(colors)
        .title()
        .responsive()
        .classes();

    // Should include responsive modifiers
    assert!(responsive_title.contains("text-4xl")); // Base size
    
    // Future: Could test for responsive classes like:
    // assert!(responsive_title.contains("sm:text-5xl"));  // Larger on small screens
    // assert!(responsive_title.contains("lg:text-6xl"));  // Even larger on large screens
}
```

### Responsive Layout

Test responsive layout classes:

```rust
#[test]
fn test_responsive_card_layout() {
    let responsive_card = card_styles(colors)
        .responsive()
        .classes();

    // Should adapt to different screen sizes
    assert!(responsive_card.contains("p-4")); // Base padding
    
    // Future: Could test for responsive padding:
    // assert!(responsive_card.contains("md:p-6"));  // Medium screens
    // assert!(responsive_card.contains("lg:p-8"));  // Large screens
}
```

## Animation and Interaction Visual Testing

### Transition Classes

Test that interactive elements have appropriate transitions:

```rust
#[test]
fn test_interaction_transitions() {
    let interactive_button = button_styles(colors.clone())
        .primary()
        .classes();

    let interactive_card = card_styles(colors)
        .hoverable()
        .classes();

    // Buttons should have smooth transitions
    assert!(interactive_button.contains("transition"));
    assert!(interactive_button.contains("duration-150")); // Quick transitions

    // Cards should have elevation transitions
    assert!(interactive_card.contains("transition-shadow"));
    assert!(interactive_card.contains("duration-300")); // Smooth elevation change
}
```

### Loading States

Test loading state visual indicators:

```rust
#[test]
fn test_loading_state_visuals() {
    let loading_button = button_styles(colors)
        .primary()
        .loading()
        .classes();

    // Should have loading visual indicators
    assert!(loading_button.contains("cursor-wait"));
    assert!(loading_button.contains("opacity-75")); // Slightly faded
    
    // Should disable interactions
    assert!(!loading_button.contains("hover:bg-jupiter-blue-600"));
}
```

## Visual Testing Tools Integration

### Storybook Integration

When using Storybook for component development:

```javascript
// .storybook/test-runner.js
export default {
  async postRender(page, context) {
    // Test that Jupiter Design System classes are applied
    const element = page.locator('[data-testid="component"]');
    
    // Validate expected classes based on story args
    if (context.args.variant === 'title') {
      await expect(element).toHaveClass(/text-4xl/);
      await expect(element).toHaveClass(/font-bold/);
    }
    
    // Visual regression testing
    await expect(element).toHaveScreenshot(
      `${context.story.replace(/\s+/g, '-')}.png`
    );
  }
};
```

### Chromatic Integration

For automated visual regression testing:

```yaml
# .github/workflows/visual-tests.yml
name: Visual Tests
on: [push, pull_request]

jobs:
  visual-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install dependencies
        run: npm ci
      - name: Run Chromatic
        uses: chromaui/action@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
          projectToken: ${{ secrets.CHROMATIC_PROJECT_TOKEN }}
          buildScriptName: build-storybook
```

## Accessibility Visual Testing

### Focus States

Test that focus states are visually apparent:

```rust
#[test]
fn test_focus_visual_indicators() {
    let focusable_button = button_styles(colors.clone())
        .primary()
        .focusable()
        .classes();

    let focusable_card = card_styles(colors)
        .interactive()
        .classes();

    // Should have clear focus indicators
    assert!(focusable_button.contains("focus:ring-2"));
    assert!(focusable_button.contains("focus:ring-jupiter-blue-500"));
    assert!(focusable_button.contains("focus:ring-offset-2"));

    // Interactive cards should also be focusable
    assert!(focusable_card.contains("focus:ring-2"));
}
```

### High Contrast Mode

Test compatibility with high contrast modes:

```rust
#[test]
fn test_high_contrast_compatibility() {
    let high_contrast_text = text_styles(colors.clone())
        .body()
        .high_contrast()
        .classes();

    let high_contrast_button = button_styles(colors)
        .primary()
        .high_contrast()
        .classes();

    // Should have stronger contrast ratios
    assert!(high_contrast_text.contains("text-black") || 
            high_contrast_text.contains("text-gray-900"));
    
    // Borders should be more prominent
    assert!(high_contrast_button.contains("border-2"));
}
```

## Performance Visual Testing

### CSS Bundle Size

Test that visual styles don't bloat the CSS bundle:

```rust
#[test]
fn test_css_class_efficiency() {
    // Generate classes for common component combinations
    let common_patterns = vec![
        text_styles(colors.clone()).title().primary().classes(),
        text_styles(colors.clone()).body().secondary().classes(),
        card_styles(colors.clone()).elevated().large().classes(),
        button_styles(colors.clone()).primary().classes(),
    ];

    for classes in common_patterns {
        // Classes should be reasonably sized
        assert!(classes.len() < 500, 
            "CSS classes too long: {} chars in '{}'", 
            classes.len(), classes);

        // Should not have excessive duplication
        let unique_classes: std::collections::HashSet<&str> = 
            classes.split_whitespace().collect();
        let total_classes = classes.split_whitespace().count();
        
        let duplication_ratio = total_classes as f32 / unique_classes.len() as f32;
        assert!(duplication_ratio < 1.1, 
            "Too much class duplication: ratio {:.2}", duplication_ratio);
    }
}
```

## Best Practices

### Visual Test Organization

1. **Component-Level Tests** - Test individual components first
2. **Layout Tests** - Test component combinations and layouts
3. **Theme Tests** - Test cross-theme visual consistency
4. **Responsive Tests** - Test responsive behavior
5. **Accessibility Tests** - Test visual accessibility features

### Maintainable Visual Testing

1. **Focus on Intent** - Test the semantic intent of styles, not pixel perfection
2. **Use Semantic Classes** - Test for meaningful CSS classes rather than computed values
3. **Test Edge Cases** - Include tests for error states, loading states, and edge cases
4. **Document Visual Requirements** - Use tests as documentation for visual requirements

### Error Messages

Provide clear visual testing error messages:

```rust
assert!(
    classes.contains("shadow-lg"),
    "Elevated card missing shadow for visual depth. Classes: '{}'",
    classes
);
```

Visual testing ensures that the Jupiter Design System produces consistent, accessible, and performant visual output across all contexts and usage scenarios.