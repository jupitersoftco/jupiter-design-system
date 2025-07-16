# Color System Best Practices

## 🎨 Color Philosophy

The Jupiter Design System's color system is built around semantic meaning rather than specific values. This approach ensures consistency, accessibility, and flexibility across themes and contexts.

### Core Principles

1. **Semantic Over Specific**: Use `Color::Primary` instead of `"blue-500"`
2. **Context-Aware**: Colors adapt automatically to theme changes
3. **Accessible by Default**: Built-in contrast and accessibility considerations
4. **Consistent Relationships**: Predictable color relationships across themes

## 🌈 Color Categories

### Brand Colors
Define your organization's visual identity:

```rust
Color::Primary      // Main brand color (jupiter-blue-500)
Color::Secondary    // Secondary brand color (jupiter-green-500)  
Color::Accent       // Accent/highlight color (jupiter-orange-500)
```

**Best Practices:**
- Use `Primary` for main CTAs and important interactive elements
- Use `Secondary` for supporting actions and secondary CTAs
- Use `Accent` sparingly for highlights and special emphasis

### Semantic Colors
Convey meaning and state information:

```rust
Color::Success      // Positive actions, confirmations (green-500)
Color::Warning      // Caution, important notices (amber-500)
Color::Error        // Errors, destructive actions (red-500)
Color::Info         // Informational content (blue-500)
```

**Best Practices:**
- Always use semantic colors for status communication
- Combine with appropriate icons for accessibility
- Test color combinations for colorblind users

### Neutral Colors
Provide structure and hierarchy:

```rust
Color::Surface      // Card backgrounds, elevated surfaces (white)
Color::Background   // Page backgrounds (gray-50)
Color::Foreground   // Primary content areas (gray-900)
Color::Border       // Dividers, outlines (gray-200)
```

**Best Practices:**
- Use `Surface` for elevated content areas (cards, modals)
- Use `Background` for main page backgrounds
- Use `Border` for subtle divisions and outlines

### Text Colors
Establish clear text hierarchy:

```rust
Color::TextPrimary    // Main headings, important text (gray-900)
Color::TextSecondary  // Body text, descriptions (gray-600)
Color::TextTertiary   // Captions, metadata (gray-400)
Color::TextInverse    // Text on dark backgrounds (white)
```

**Best Practices:**
- Use `TextPrimary` for headings and important content
- Use `TextSecondary` for body text and descriptions
- Use `TextTertiary` for supporting information only
- Always test text contrast ratios

### Interactive Colors
Handle user interaction states:

```rust
Color::Interactive         // Default interactive state
Color::InteractiveHover    // Hover state
Color::InteractiveActive   // Active/pressed state
Color::InteractiveDisabled // Disabled state
```

## 🛠️ Usage Patterns

### Basic Color Application

```rust
use jupiter_design_system::prelude::*;

let colors = VibeColors::new();

// Text styling
let primary_text = colors.text_class(Color::Primary);
let secondary_text = colors.text_class(Color::TextSecondary);

// Background styling
let primary_bg = colors.bg_class(Color::Primary);
let surface_bg = colors.bg_class(Color::Surface);

// Border styling
let primary_border = colors.border_class(Color::Primary);
let neutral_border = colors.border_class(Color::Border);
```

### Component Color Integration

```rust
// Button with semantic colors
let success_button = button_styles(colors)
    .variant(ButtonVariant::Success)
    .classes();

let warning_button = button_styles(colors)
    .variant(ButtonVariant::Warning)
    .classes();

// Text with proper hierarchy
let heading = text_styles(colors)
    .color(Color::TextPrimary)
    .typography(Typography::Heading1)
    .classes();

let body_text = text_styles(colors)
    .color(Color::TextSecondary)
    .typography(Typography::Body)
    .classes();
```

### Interactive State Handling

```rust
use jupiter_design_system::builders::interactive::*;

// Interactive element with proper state colors
let interactive_card = interactive_element(colors)
    .base_style("p-4 rounded-lg")
    .bg_color(Color::Surface)
    .hover()
        .bg_color(Color::Background)
        .border_color(Color::Interactive)
    .focus()
        .ring_color(Color::InteractiveFocus)
        .outline_none()
    .build();
```

## 🎭 Theming Best Practices

### Default Theme Usage

```rust
// Recommended: Use default theme as baseline
let colors = VibeColors::new();

// Colors automatically map to Jupiter brand values:
// Primary -> jupiter-blue-500
// Secondary -> jupiter-green-500
// Success -> green-500
```

### Custom Theme Creation

```rust
// Override specific colors while maintaining relationships
let custom_colors = VibeColors::with_overrides(|palette| {
    // Update brand colors
    palette.primary = "purple-600".to_string();
    palette.secondary = "pink-500".to_string();
    
    // Update interactive colors to match
    palette.interactive = "purple-600".to_string();
    palette.interactive_hover = "purple-700".to_string();
    palette.interactive_active = "purple-800".to_string();
});
```

### Dark Theme Implementation

```rust
#[derive(Debug, Clone)]
pub struct DarkTheme {
    palette: ColorPalette,
}

impl Default for DarkTheme {
    fn default() -> Self {
        Self {
            palette: ColorPalette {
                // Invert neutrals for dark theme
                surface: "gray-800".to_string(),
                background: "gray-900".to_string(),
                foreground: "gray-100".to_string(),
                border: "gray-700".to_string(),
                
                // Adjust text colors
                text_primary: "gray-100".to_string(),
                text_secondary: "gray-300".to_string(),
                text_tertiary: "gray-500".to_string(),
                text_inverse: "gray-900".to_string(),
                
                // Keep brand colors or adjust for better contrast
                primary: "jupiter-blue-400".to_string(),
                secondary: "jupiter-green-400".to_string(),
                
                // Maintain semantic color meanings
                success: "green-400".to_string(),
                warning: "amber-400".to_string(),
                error: "red-400".to_string(),
                info: "blue-400".to_string(),
                
                // Update interactive states
                interactive: "jupiter-blue-400".to_string(),
                interactive_hover: "jupiter-blue-300".to_string(),
                interactive_active: "jupiter-blue-500".to_string(),
                interactive_disabled: "gray-600".to_string(),
            },
        }
    }
}

impl ColorProvider for DarkTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}
```

## 🚦 Color Usage Guidelines

### Do's ✅

1. **Use Semantic Colors for Meaning**
   ```rust
   // ✅ Good: Conveys meaning
   let submit_button = button_styles(colors)
       .variant(ButtonVariant::Success)
       .classes();
   
   let cancel_button = button_styles(colors)
       .variant(ButtonVariant::Secondary)
       .classes();
   ```

2. **Maintain Color Relationships**
   ```rust
   // ✅ Good: Related interactive states
   let interactive_input = interactive_input(colors)
       .standard_style()
       .hover().border_color(Color::Interactive)
       .focus().border_color(Color::Interactive).ring_color(Color::Interactive)
       .build();
   ```

3. **Test Color Combinations**
   ```rust
   // ✅ Good: Test with multiple themes
   #[test]
   fn test_color_combinations() {
       let light_theme = VibeColors::new();
       let dark_theme = DarkTheme::default();
       
       let light_button = button_styles(light_theme).primary().classes();
       let dark_button = button_styles(dark_theme).primary().classes();
       
       // Verify both generate valid classes
       assert!(!light_button.is_empty());
       assert!(!dark_button.is_empty());
   }
   ```

### Don'ts ❌

1. **Don't Use Hardcoded Colors**
   ```rust
   // ❌ Bad: Hardcoded, not theme-aware
   let hardcoded_style = "bg-blue-500 text-white border-blue-600";
   
   // ✅ Good: Theme-aware
   let themed_button = button_styles(colors).primary().classes();
   ```

2. **Don't Mix Design System with Custom Colors**
   ```rust
   // ❌ Bad: Mixing systems
   let mixed_classes = format!("{} bg-custom-blue", 
       button_styles(colors).secondary().classes());
   
   // ✅ Good: Consistent system usage
   let consistent_button = button_styles(custom_colors).primary().classes();
   ```

3. **Don't Skip Semantic Meaning**
   ```rust
   // ❌ Bad: Generic usage
   let delete_button = button_styles(colors).primary().classes();
   
   // ✅ Good: Semantic usage
   let delete_button = button_styles(colors)
       .variant(ButtonVariant::Error)
       .classes();
   ```

## 🎯 Accessibility Considerations

### Contrast Requirements

The design system automatically handles contrast requirements, but always verify:

```rust
// Built-in accessible combinations
let accessible_text = text_styles(colors)
    .color(Color::TextPrimary)  // High contrast on backgrounds
    .classes();

let accessible_button = button_styles(colors)
    .primary()  // Meets WCAG AA contrast requirements
    .classes();
```

### Color-Only Communication

Never rely solely on color to convey information:

```rust
// ✅ Good: Color + icon/text for status
let success_message = format!(
    "<div class=\"{}\">✓ Success: Operation completed</div>",
    text_styles(colors).color(Color::Success).classes()
);

let error_message = format!(
    "<div class=\"{}\">⚠ Error: Operation failed</div>",
    text_styles(colors).color(Color::Error).classes()
);
```

### Focus Indicators

Ensure interactive elements have clear focus indicators:

```rust
let accessible_button = interactive_button(colors)
    .primary()
    .focus()
        .ring_color(Color::Interactive)
        .ring_offset_2()
        .outline_none()
    .build();
```

## 🧪 Testing Color Implementation

### Theme Switching Tests

```rust
#[cfg(test)]
mod color_tests {
    use super::*;

    #[test]
    fn test_theme_consistency() {
        let default_theme = VibeColors::new();
        let custom_theme = VibeColors::with_overrides(|palette| {
            palette.primary = "red-500".to_string();
        });

        // Test that components respect theme changes
        let default_button = button_styles(default_theme).primary().classes();
        let custom_button = button_styles(custom_theme).primary().classes();

        assert!(default_button.contains("jupiter-blue"));
        assert!(custom_button.contains("red-500"));
    }

    #[test]
    fn test_semantic_color_relationships() {
        let colors = VibeColors::new();
        
        // Test that related colors are properly connected
        let interactive_base = colors.resolve_color(Color::Interactive);
        let interactive_hover = colors.resolve_color(Color::InteractiveHover);
        
        assert_ne!(interactive_base, interactive_hover);
        // Additional relationship tests...
    }
}
```

### Visual Regression Testing

```rust
// Generate color swatches for visual testing
pub fn generate_color_swatches(colors: impl ColorProvider) -> Vec<String> {
    vec![
        colors.bg_class(Color::Primary),
        colors.bg_class(Color::Secondary),
        colors.bg_class(Color::Success),
        colors.bg_class(Color::Warning),
        colors.bg_class(Color::Error),
        colors.bg_class(Color::Info),
    ]
}
```

## 📊 Performance Considerations

### Color Resolution Caching

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CachedColorProvider {
    base_provider: VibeColors,
    cache: HashMap<Color, String>,
}

impl CachedColorProvider {
    pub fn new(base: VibeColors) -> Self {
        Self {
            base_provider: base,
            cache: HashMap::new(),
        }
    }
}

impl ColorProvider for CachedColorProvider {
    fn palette(&self) -> &ColorPalette {
        self.base_provider.palette()
    }
    
    fn resolve_color(&self, color: Color) -> &str {
        // Implement caching logic for expensive color operations
        self.base_provider.resolve_color(color)
    }
}
```

## 🔗 Related Documentation

- [Theming & Customization](./theming.md) - Advanced theme creation
- [Accessibility Guide](./accessibility.md) - Comprehensive accessibility practices
- [Component Guidelines](./components.md) - Using colors in components
- [Examples](./examples/) - Real-world color usage patterns