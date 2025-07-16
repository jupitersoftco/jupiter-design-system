# Theme Implementation Best Practices

This guide covers best practices for implementing robust, maintainable, and accessible themes in Jupiter Design System.

## Design Principles

### 1. Semantic Over Visual

Always prioritize semantic meaning over specific visual appearance:

```rust
// ✅ Good - semantic naming
palette.primary = "brand-blue-600".to_string();
palette.success = "green-600".to_string();
palette.text_primary = "gray-900".to_string();

// ❌ Avoid - visual-only naming  
palette.blue = "blue-600".to_string();
palette.green = "green-600".to_string();
palette.dark_text = "gray-900".to_string();
```

### 2. Consistency Across States

Ensure interactive states form a coherent progression:

```rust
// ✅ Good - clear progression
palette.interactive = "blue-600".to_string();
palette.interactive_hover = "blue-700".to_string();
palette.interactive_active = "blue-800".to_string();
palette.interactive_disabled = "blue-300".to_string();

// ❌ Avoid - inconsistent progression
palette.interactive = "blue-600".to_string();
palette.interactive_hover = "green-500".to_string(); // Different hue
palette.interactive_active = "blue-400".to_string(); // Lighter than base
```

### 3. Framework Agnostic Design

Design themes to work across different CSS frameworks:

```rust
impl ColorProvider for MyTheme {
    fn resolve_color(&self, color: Color) -> &str {
        // Return framework-agnostic values
        match color {
            Color::Primary => &self.palette.primary,
            // ...
        }
    }
    
    // Override for specific frameworks if needed
    fn text_class(&self, color: Color) -> String {
        match self.framework {
            Framework::Tailwind => format!("text-{}", self.resolve_color(color)),
            Framework::Bootstrap => format!("text-{}", self.resolve_color(color)),
            Framework::Custom => format!("color-{}", self.resolve_color(color)),
        }
    }
}
```

## Accessibility Best Practices

### 1. Color Contrast

Ensure adequate color contrast for text readability:

```rust
// Use tools like WebAIM or Colour Contrast Analyser
// Minimum ratios: 4.5:1 for normal text, 3:1 for large text

fn create_accessible_palette() -> ColorPalette {
    ColorPalette {
        // High contrast for primary text
        text_primary: "gray-900".to_string(),    // Very dark
        background: "white".to_string(),          // Light background
        
        // Medium contrast for secondary text  
        text_secondary: "gray-700".to_string(),   // Still readable
        
        // Low contrast for tertiary (use sparingly)
        text_tertiary: "gray-500".to_string(),    // Light, for labels only
        
        // Interactive elements need good contrast
        interactive: "blue-700".to_string(),      // Dark enough to read
        // ...
    }
}
```

### 2. Focus Indicators

Provide clear focus indicators for keyboard navigation:

```rust
impl MyTheme {
    pub fn focus_ring_classes(&self) -> String {
        // Ensure focus is always visible
        format!(
            "focus:outline-none focus:ring-2 focus:ring-{} focus:ring-offset-2",
            self.resolve_color(Color::Interactive)
        )
    }
}
```

### 3. Reduced Motion Support

Respect user preferences for reduced motion:

```rust
impl MyTheme {
    pub fn transition_classes(&self) -> String {
        // Use prefers-reduced-motion media query
        "transition-colors duration-200 motion-reduce:transition-none".to_string()
    }
}
```

## Color System Design

### 1. Systematic Color Scales

Use systematic color scales for consistency:

```rust
pub struct ColorScale {
    pub c50: String,   // Lightest
    pub c100: String,
    pub c200: String,
    pub c300: String,
    pub c400: String,
    pub c500: String,  // Base color
    pub c600: String,
    pub c700: String,
    pub c800: String,
    pub c900: String,  // Darkest
}

impl MyTheme {
    fn create_primary_scale() -> ColorScale {
        ColorScale {
            c50: "blue-50".to_string(),
            c100: "blue-100".to_string(),
            c200: "blue-200".to_string(),
            c300: "blue-300".to_string(),
            c400: "blue-400".to_string(),
            c500: "blue-500".to_string(),  // Base
            c600: "blue-600".to_string(),
            c700: "blue-700".to_string(),
            c800: "blue-800".to_string(),
            c900: "blue-900".to_string(),
        }
    }
}
```

### 2. Dark Mode Considerations

Design with dark mode in mind from the start:

```rust
#[derive(Debug, Clone)]
pub enum ThemeMode {
    Light,
    Dark,
    Auto, // Follows system preference
}

impl MyTheme {
    pub fn with_mode(mode: ThemeMode) -> Self {
        let palette = match mode {
            ThemeMode::Light => Self::light_palette(),
            ThemeMode::Dark => Self::dark_palette(),
            ThemeMode::Auto => Self::auto_palette(),
        };
        Self { palette, mode }
    }
    
    fn dark_palette() -> ColorPalette {
        ColorPalette {
            // Invert surface hierarchy
            surface: "gray-800".to_string(),
            background: "gray-900".to_string(),
            
            // Adjust text for dark backgrounds
            text_primary: "gray-100".to_string(),
            text_secondary: "gray-300".to_string(),
            text_tertiary: "gray-500".to_string(),
            
            // Keep brand colors recognizable but adjust for dark
            primary: "blue-400".to_string(), // Lighter than light mode
            // ...
        }
    }
}
```

## Component Integration

### 1. Pattern Composition

Design themes to work well with pattern composition:

```rust
impl MyTheme {
    /// Get classes that work well together
    pub fn component_classes(&self, component: ComponentType) -> String {
        let base = self.base_classes();
        let color = self.color_classes(component);
        let interaction = self.interaction_classes(component);
        
        format!("{} {} {}", base, color, interaction)
    }
}
```

### 2. State Management

Handle all interactive states consistently:

```rust
impl MyTheme {
    pub fn button_classes(&self, state: InteractiveState) -> String {
        let base = "px-4 py-2 rounded-md font-medium transition-colors";
        
        let state_classes = match state {
            InteractiveState::Default => format!(
                "{} {}",
                self.bg_class(Color::Primary),
                self.text_class(Color::TextInverse)
            ),
            InteractiveState::Hover => format!(
                "{} {}",
                self.bg_class(Color::InteractiveHover),
                self.text_class(Color::TextInverse)
            ),
            InteractiveState::Active => format!(
                "{} {}",
                self.bg_class(Color::InteractiveActive),
                self.text_class(Color::TextInverse)
            ),
            InteractiveState::Disabled => format!(
                "{} {} opacity-50 cursor-not-allowed",
                self.bg_class(Color::InteractiveDisabled),
                self.text_class(Color::TextPrimary)
            ),
        };
        
        format!("{} {}", base, state_classes)
    }
}
```

## Performance Considerations

### 1. Class String Optimization

Optimize class string generation for performance:

```rust
use std::sync::OnceLock;

impl MyTheme {
    // Cache commonly used class combinations
    fn primary_button_classes() -> &'static str {
        static CLASSES: OnceLock<String> = OnceLock::new();
        CLASSES.get_or_init(|| {
            "bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700".to_string()
        })
    }
}
```

### 2. Minimize Runtime Computation

Pre-compute class combinations where possible:

```rust
impl MyTheme {
    pub fn new() -> Self {
        let palette = Self::create_palette();
        
        // Pre-compute common combinations
        let button_primary = format!(
            "{} {} px-4 py-2 rounded-md",
            palette.primary, palette.text_inverse
        );
        
        Self { 
            palette,
            cached_classes: ClassCache {
                button_primary,
                // ... other cached combinations
            }
        }
    }
}
```

## Testing Best Practices

### 1. Visual Regression Testing

Test visual consistency across theme changes:

```rust
#[cfg(test)]
mod theme_tests {
    use super::*;
    
    #[test]
    fn test_color_consistency() {
        let theme = MyTheme::new();
        
        // Test that all required colors are defined
        assert!(!theme.resolve_color(Color::Primary).is_empty());
        assert!(!theme.resolve_color(Color::TextPrimary).is_empty());
        
        // Test color progression makes sense
        let base = theme.resolve_color(Color::Interactive);
        let hover = theme.resolve_color(Color::InteractiveHover);
        assert_ne!(base, hover, "Hover state should differ from base");
    }
    
    #[test]
    fn test_accessibility() {
        let theme = MyTheme::new();
        
        // Test that focus indicators are present
        let focus_classes = theme.focus_ring_classes();
        assert!(focus_classes.contains("focus:ring"));
    }
}
```

### 2. Cross-Framework Testing

Test themes across different CSS frameworks:

```rust
#[test]
fn test_framework_compatibility() {
    let theme = MyTheme::new();
    
    // Test Tailwind classes
    assert!(theme.text_class(Color::Primary).starts_with("text-"));
    
    // Test custom framework
    let custom_theme = MyTheme::with_framework(Framework::Custom);
    assert!(custom_theme.text_class(Color::Primary).starts_with("color-"));
}
```

## Documentation Practices

### 1. Usage Examples

Provide clear usage examples:

```rust
/// # Examples
/// 
/// ```rust
/// use my_design_system::MyTheme;
/// use jupiter_design_system::prelude::*;
/// 
/// let theme = MyTheme::new();
/// 
/// // Primary button
/// let button = primary_button(theme.clone()).classes();
/// 
/// // Dark mode
/// let dark_theme = MyTheme::dark();
/// let dark_button = primary_button(dark_theme).classes();
/// ```
impl MyTheme {
    pub fn new() -> Self { /* ... */ }
}
```

### 2. Color Palette Documentation

Document your color choices:

```rust
impl MyTheme {
    /// Creates the brand color palette
    /// 
    /// Colors chosen for:
    /// - Primary: Corporate blue (#1E40AF) - main brand color
    /// - Success: Green (#059669) - positive actions, WCAG AA compliant
    /// - Error: Red (#DC2626) - destructive actions, WCAG AA compliant
    fn create_palette() -> ColorPalette {
        // ...
    }
}
```

## Migration and Versioning

### 1. Backward Compatibility

Maintain backward compatibility when possible:

```rust
impl MyTheme {
    /// Legacy constructor for v1 compatibility
    #[deprecated(note = "Use MyTheme::new() instead")]
    pub fn create() -> Self {
        Self::new()
    }
    
    /// Migration helper from old color names
    pub fn migrate_from_v1(old_colors: V1ColorPalette) -> Self {
        let palette = ColorPalette {
            primary: old_colors.brand_primary,
            secondary: old_colors.brand_secondary,
            // Map old names to new semantic names
            success: old_colors.positive,
            error: old_colors.negative,
            // ...
        };
        Self { palette }
    }
}
```

### 2. Version Documentation

Document breaking changes clearly:

```rust
/// # Breaking Changes in v2.0
/// 
/// - `brand_primary` renamed to `primary`
/// - `positive` renamed to `success` 
/// - `negative` renamed to `error`
/// - Added new `interactive_*` color tokens
/// 
/// Use `migrate_from_v1()` for automatic migration.
impl MyTheme {
    // ...
}
```

Following these best practices will help you create themes that are:
- **Accessible** to all users
- **Maintainable** over time  
- **Performant** in production
- **Consistent** across components
- **Compatible** with different frameworks