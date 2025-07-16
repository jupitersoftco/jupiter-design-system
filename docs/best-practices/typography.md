# Typography Best Practices

## 📝 Typography Philosophy

Typography in the Jupiter Design System follows semantic principles that create clear visual hierarchy, improve readability, and ensure consistent communication across all interfaces.

### Core Principles

1. **Semantic Hierarchy**: Text styles convey meaning and importance, not just appearance
2. **Systematic Scaling**: Proportional type scale for harmonious visual relationships
3. **Accessibility First**: Readable font sizes, adequate contrast, and scalable text
4. **Consistent Voice**: Unified typographic personality across all touchpoints

## 📏 Typography System

### Hierarchy Levels

The system provides semantic typography levels that automatically handle sizing, weight, and spacing:

```rust
TypographyHierarchy::Title        // Main page title (h1)
TypographyHierarchy::Heading      // Section heading (h2)  
TypographyHierarchy::Subheading   // Sub-section heading (h3)
TypographyHierarchy::H4           // Minor heading (h4)
TypographyHierarchy::Body         // Regular body text
TypographyHierarchy::BodyLarge    // Emphasis body text
TypographyHierarchy::BodySmall    // Detail body text
TypographyHierarchy::Caption      // Small supplementary text
TypographyHierarchy::Overline     // Category/label text
TypographyHierarchy::Code         // Monospace text
```

### Size Scale

Systematic type scale following design principles:

```rust
TypographySize::XS     // 0.75rem (12px) - Captions, metadata
TypographySize::SM     // 0.875rem (14px) - Small body text
TypographySize::MD     // 1rem (16px) - Base body text
TypographySize::LG     // 1.125rem (18px) - Large body text
TypographySize::XL     // 1.25rem (20px) - Small headings
TypographySize::XL2    // 1.5rem (24px) - Medium headings
TypographySize::XL3    // 1.875rem (30px) - Large headings
TypographySize::XL4    // 2.25rem (36px) - Display headings
```

### Weight System

Font weight progression for emphasis and hierarchy:

```rust
TypographyWeight::Light      // 300 - Subtle, decorative
TypographyWeight::Normal     // 400 - Body text default
TypographyWeight::Medium     // 500 - Emphasis
TypographyWeight::Semibold   // 600 - Subheadings
TypographyWeight::Bold       // 700 - Headings
TypographyWeight::ExtraBold  // 800 - Display text
```

### Color Semantics

Typography colors that convey meaning:

```rust
TypographyColor::Primary     // Brand emphasis
TypographyColor::Secondary   // Supporting emphasis
TypographyColor::Muted       // De-emphasized text
TypographyColor::Disabled    // Inactive text
TypographyColor::Success     // Positive messaging
TypographyColor::Warning     // Cautionary messaging
TypographyColor::Error       // Error messaging
TypographyColor::Auto        // Context-appropriate color
```

## 🛠️ Implementation Patterns

### Basic Typography Usage

```rust
use jupiter_design_system::prelude::*;
use jupiter_design_system::patterns::typography::*;

let colors = VibeColors::new();

// Page title
let title_classes = typography_pattern()
    .hierarchy(TypographyHierarchy::Title)
    .color(TypographyColor::Primary)
    .build();

// Section heading
let heading_classes = typography_pattern()
    .hierarchy(TypographyHierarchy::Heading)
    .color(TypographyColor::Auto)
    .build();

// Body text
let body_classes = typography_pattern()
    .hierarchy(TypographyHierarchy::Body)
    .color(TypographyColor::Auto)
    .build();
```

### Semantic Text Patterns

The system provides semantic functions for common text patterns:

```rust
// Semantic heading patterns
let page_title = title_typography(colors);
let section_heading = heading_typography(colors);
let body_text = body_typography(colors);
let caption_text = caption_typography(colors);
let code_text = code_typography(colors);

// Usage in components
// rsx! {
//     h1 { class: "{page_title}", "Page Title" }
//     h2 { class: "{section_heading}", "Section Heading" }
//     p { class: "{body_text}", "Body text content" }
//     small { class: "{caption_text}", "Caption text" }
//     code { class: "{code_text}", "code snippet" }
// }
```

### Advanced Typography Configuration

```rust
// Custom typography with full control
let custom_text = text_styles(colors)
    .typography(Typography::Heading2)
    .color(Color::Primary)
    .weight(FontWeight::SemiBold)
    .family(FontFamily::Sans)
    .classes();

// Typography pattern with alignment and overflow
let styled_text = typography_pattern()
    .hierarchy(TypographyHierarchy::Body)
    .size(TypographySize::LG)
    .weight(TypographyWeight::Medium)
    .color(TypographyColor::Primary)
    .alignment(TypographyAlignment::Center)
    .overflow(TypographyOverflow::Ellipsis)
    .build();
```

## 📋 Typography Hierarchy Guidelines

### Page Structure Hierarchy

```rust
// ✅ Good: Clear hierarchy
let page_structure = vec![
    ("Page Title", title_typography(colors)),
    ("Section Heading", heading_typography(colors)),
    ("Subsection", typography_pattern()
        .hierarchy(TypographyHierarchy::Subheading)
        .build()),
    ("Body Text", body_typography(colors)),
    ("Caption", caption_typography(colors)),
];
```

### Semantic HTML Integration

```rust
// Map typography patterns to semantic HTML
fn get_heading_styles(level: u8, colors: &impl ColorProvider) -> String {
    match level {
        1 => title_typography(colors.clone()),
        2 => heading_typography(colors.clone()),
        3 => typography_pattern()
            .hierarchy(TypographyHierarchy::Subheading)
            .build(),
        4 => typography_pattern()
            .hierarchy(TypographyHierarchy::H4)
            .build(),
        _ => body_typography(colors.clone()),
    }
}

// Usage
let h1_classes = get_heading_styles(1, &colors);
let h2_classes = get_heading_styles(2, &colors);
```

### Content Type Patterns

```rust
// Article typography
let article_title = title_typography(colors);
let article_subtitle = typography_pattern()
    .hierarchy(TypographyHierarchy::Heading)
    .color(TypographyColor::Muted)
    .build();
let article_body = body_typography(colors);

// Interface typography
let button_text = typography_pattern()
    .hierarchy(TypographyHierarchy::Body)
    .weight(TypographyWeight::Medium)
    .color(TypographyColor::Auto)
    .build();

let label_text = typography_pattern()
    .hierarchy(TypographyHierarchy::Caption)
    .weight(TypographyWeight::Medium)
    .color(TypographyColor::Auto)
    .build();
```

## 🎨 Color and Typography Integration

### Text Color Best Practices

```rust
// Hierarchy-appropriate colors
let primary_heading = typography_pattern()
    .hierarchy(TypographyHierarchy::Title)
    .color(TypographyColor::Primary)  // Brand emphasis
    .build();

let secondary_heading = typography_pattern()
    .hierarchy(TypographyHierarchy::Heading)
    .color(TypographyColor::Auto)     // Context-appropriate
    .build();

let body_text = typography_pattern()
    .hierarchy(TypographyHierarchy::Body)
    .color(TypographyColor::Auto)     // Standard readability
    .build();

let muted_text = typography_pattern()
    .hierarchy(TypographyHierarchy::Caption)
    .color(TypographyColor::Muted)    // De-emphasized
    .build();
```

### State-Based Typography

```rust
// Success message
let success_text = typography_pattern()
    .hierarchy(TypographyHierarchy::Body)
    .color(TypographyColor::Success)
    .weight(TypographyWeight::Medium)
    .build();

// Error message
let error_text = typography_pattern()
    .hierarchy(TypographyHierarchy::Body)
    .color(TypographyColor::Error)
    .weight(TypographyWeight::Medium)
    .build();

// Disabled text
let disabled_text = typography_pattern()
    .hierarchy(TypographyHierarchy::Body)
    .color(TypographyColor::Disabled)
    .build();
```

### Theme-Aware Typography

```rust
// Typography that adapts to theme
fn create_adaptive_typography(colors: impl ColorProvider) -> Vec<String> {
    vec![
        // Light text automatically becomes dark in dark themes
        typography_pattern()
            .hierarchy(TypographyHierarchy::Title)
            .color(TypographyColor::Auto)
            .build(),
        
        // Brand colors maintain identity across themes
        typography_pattern()
            .hierarchy(TypographyHierarchy::Heading)
            .color(TypographyColor::Primary)
            .build(),
    ]
}
```

## 📱 Responsive Typography

### Mobile-First Approach

```rust
// Typography that scales appropriately
let responsive_title = typography_pattern()
    .hierarchy(TypographyHierarchy::Title)
    .size(TypographySize::XL2)  // Smaller on mobile
    .responsive_size(Breakpoint::Tablet, TypographySize::XL3)
    .responsive_size(Breakpoint::Desktop, TypographySize::XL4)
    .build();

let responsive_body = typography_pattern()
    .hierarchy(TypographyHierarchy::Body)
    .size(TypographySize::SM)   // Compact on mobile
    .responsive_size(Breakpoint::Tablet, TypographySize::MD)
    .build();
```

### Line Height and Spacing

```rust
// Optimal reading experience
let reading_text = typography_pattern()
    .hierarchy(TypographyHierarchy::Body)
    .line_height(1.6)           // Comfortable reading
    .letter_spacing("normal")
    .build();

let display_text = typography_pattern()
    .hierarchy(TypographyHierarchy::Title)
    .line_height(1.2)           // Tighter for headlines
    .letter_spacing("-0.025em") // Slightly condensed
    .build();
```

## 🚦 Typography Usage Guidelines

### Do's ✅

1. **Use Semantic Hierarchy**
   ```rust
   // ✅ Good: Semantic meaning
   let page_title = title_typography(colors);
   let section_heading = heading_typography(colors);
   let body_content = body_typography(colors);
   ```

2. **Maintain Consistent Scale**
   ```rust
   // ✅ Good: System-based sizing
   let heading = typography_pattern()
       .hierarchy(TypographyHierarchy::Heading)
       .build();
   
   let subheading = typography_pattern()
       .hierarchy(TypographyHierarchy::Subheading)
       .build();
   ```

3. **Use Appropriate Color Semantics**
   ```rust
   // ✅ Good: Meaningful color usage
   let success_message = typography_pattern()
       .color(TypographyColor::Success)
       .build();
   
   let error_message = typography_pattern()
       .color(TypographyColor::Error)
       .build();
   ```

### Don'ts ❌

1. **Don't Skip Hierarchy Levels**
   ```rust
   // ❌ Bad: Skipping hierarchy
   let title = title_typography(colors);
   let small_text = caption_typography(colors);
   // Missing intermediate levels
   
   // ✅ Good: Progressive hierarchy
   let title = title_typography(colors);
   let heading = heading_typography(colors);
   let body = body_typography(colors);
   ```

2. **Don't Use Arbitrary Sizes**
   ```rust
   // ❌ Bad: Custom sizing
   let custom_text = "text-17px font-custom";
   
   // ✅ Good: System sizing
   let system_text = typography_pattern()
       .size(TypographySize::LG)
       .build();
   ```

3. **Don't Ignore Color Context**
   ```rust
   // ❌ Bad: Hardcoded colors
   let hardcoded_text = "text-blue-500";
   
   // ✅ Good: Semantic colors
   let semantic_text = typography_pattern()
       .color(TypographyColor::Primary)
       .build();
   ```

## ♿ Accessibility Considerations

### Text Contrast

```rust
// High contrast combinations
let accessible_text = typography_pattern()
    .hierarchy(TypographyHierarchy::Body)
    .color(TypographyColor::Auto)  // Ensures adequate contrast
    .build();

// Test contrast with backgrounds
fn test_text_contrast(text_color: TypographyColor, bg_color: Color) {
    // Implementation to verify WCAG contrast requirements
}
```

### Font Size Requirements

```rust
// Minimum readable sizes
let minimum_body = typography_pattern()
    .size(TypographySize::SM)  // 14px minimum for body text
    .build();

let minimum_interactive = typography_pattern()
    .size(TypographySize::MD)  // 16px minimum for interactive text
    .build();
```

### Screen Reader Considerations

```rust
// Semantic HTML integration
fn render_accessible_heading(level: u8, text: &str, colors: &impl ColorProvider) -> String {
    let classes = get_heading_styles(level, colors);
    format!("<h{} class=\"{}\">{}</h{}>", level, classes, text, level)
}

// Screen reader-friendly patterns
let screen_reader_text = typography_pattern()
    .hierarchy(TypographyHierarchy::Body)
    .screen_reader_only(true)  // Visually hidden but accessible
    .build();
```

## 🧪 Testing Typography

### Visual Consistency Tests

```rust
#[cfg(test)]
mod typography_tests {
    use super::*;

    #[test]
    fn test_hierarchy_consistency() {
        let colors = VibeColors::new();
        
        let title = title_typography(colors.clone());
        let heading = heading_typography(colors.clone());
        let body = body_typography(colors);
        
        // Test that hierarchy is maintained
        assert!(title.contains("text-xl4") || title.contains("text-3xl"));
        assert!(heading.contains("text-xl2") || heading.contains("text-2xl"));
        assert!(body.contains("text-base") || body.contains("text-md"));
    }

    #[test]
    fn test_semantic_color_mapping() {
        let colors = VibeColors::new();
        
        let success_text = typography_pattern()
            .color(TypographyColor::Success)
            .build();
        
        let error_text = typography_pattern()
            .color(TypographyColor::Error)
            .build();
        
        assert!(success_text.contains("text-green"));
        assert!(error_text.contains("text-red"));
    }
}
```

### Responsive Behavior Tests

```rust
#[test]
fn test_responsive_typography() {
    let responsive_title = typography_pattern()
        .hierarchy(TypographyHierarchy::Title)
        .responsive(true)
        .build();
    
    // Should contain responsive classes
    assert!(responsive_title.contains("sm:") || responsive_title.contains("md:"));
}
```

## 📊 Performance Considerations

### Typography Caching

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CachedTypographyProvider {
    cache: HashMap<(TypographyHierarchy, TypographyColor), String>,
}

impl CachedTypographyProvider {
    pub fn get_or_create(&mut self, hierarchy: TypographyHierarchy, color: TypographyColor) -> &String {
        self.cache.entry((hierarchy, color)).or_insert_with(|| {
            typography_pattern()
                .hierarchy(hierarchy)
                .color(color)
                .build()
        })
    }
}
```

### Bundle Size Optimization

```rust
// Use constants for frequently used patterns
const TITLE_CLASSES: &str = "text-4xl font-bold text-gray-900";
const HEADING_CLASSES: &str = "text-2xl font-semibold text-gray-800";
const BODY_CLASSES: &str = "text-base font-normal text-gray-700";

// Generate only when customization is needed
fn get_typography_classes(hierarchy: TypographyHierarchy, custom: bool) -> String {
    if !custom {
        match hierarchy {
            TypographyHierarchy::Title => TITLE_CLASSES.to_string(),
            TypographyHierarchy::Heading => HEADING_CLASSES.to_string(),
            TypographyHierarchy::Body => BODY_CLASSES.to_string(),
            _ => typography_pattern().hierarchy(hierarchy).build(),
        }
    } else {
        typography_pattern().hierarchy(hierarchy).build()
    }
}
```

## 🔗 Related Documentation

- [Color System](./colors.md) - Typography color integration
- [Accessibility Guide](./accessibility.md) - Comprehensive accessibility practices
- [Component Guidelines](./components.md) - Using typography in components
- [Examples](./examples/) - Real-world typography usage patterns