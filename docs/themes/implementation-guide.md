# Theme Implementation Guide

This guide walks you through implementing your own theme for Jupiter Design System.

## Step 1: Define Your Color Palette

Start by creating a color palette that matches your brand and design requirements.

### Basic Color Palette

```rust
use jupiter_design_system::core::color::ColorPalette;

fn create_my_palette() -> ColorPalette {
    ColorPalette {
        // Brand colors - use your brand's color scheme
        primary: "indigo-600".to_string(),
        secondary: "emerald-500".to_string(),
        accent: "orange-400".to_string(),

        // Semantic colors - conventional meanings
        success: "green-600".to_string(),
        warning: "yellow-500".to_string(),
        error: "red-600".to_string(),
        info: "blue-600".to_string(),

        // Surface colors - backgrounds and containers
        surface: "white".to_string(),
        background: "gray-50".to_string(),
        foreground: "gray-900".to_string(),
        border: "gray-200".to_string(),

        // Text hierarchy - ensure good contrast
        text_primary: "gray-900".to_string(),
        text_secondary: "gray-600".to_string(),
        text_tertiary: "gray-400".to_string(),
        text_inverse: "white".to_string(),

        // Interactive states - provide clear feedback
        interactive: "indigo-600".to_string(),
        interactive_hover: "indigo-700".to_string(),
        interactive_active: "indigo-800".to_string(),
        interactive_disabled: "gray-300".to_string(),
    }
}
```

## Step 2: Create Your Color Provider

Implement the `ColorProvider` trait for your theme:

```rust
use jupiter_design_system::core::color::{ColorProvider, ColorPalette};

#[derive(Debug, Clone)]
pub struct MyTheme {
    palette: ColorPalette,
}

impl MyTheme {
    pub fn new() -> Self {
        Self {
            palette: create_my_palette(),
        }
    }
}

impl ColorProvider for MyTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

impl Default for MyTheme {
    fn default() -> Self {
        Self::new()
    }
}
```

## Step 3: Implement Additional Providers (Future Expansion)

The system defines additional provider traits for future expansion. While not currently required, implementing them prepares your theme for upcoming features:

### Size Provider
```rust
impl SizeProvider for MyTheme {
    fn resolve_size(&self, size: Size) -> &str {
        match size {
            Size::XSmall => "xs",
            Size::Small => "sm",
            Size::Medium => "md",
            Size::Large => "lg", 
            Size::XLarge => "xl",
        }
    }
}
```

### Spacing Provider
```rust
impl SpacingProvider for MyTheme {
    fn resolve_spacing(&self, spacing: Spacing) -> &str {
        match spacing {
            Spacing::None => "0",
            Spacing::XSmall => "1",
            Spacing::Small => "2", 
            Spacing::Medium => "4",
            Spacing::Large => "6",
            Spacing::XLarge => "8",
            Spacing::XXLarge => "12",
        }
    }
}
```

### Typography Provider
```rust
impl TypographyProvider for MyTheme {
    fn resolve_typography(&self, typography: Typography) -> &str {
        match typography {
            Typography::Heading1 => "4xl",
            Typography::Heading2 => "3xl",
            Typography::Heading3 => "2xl",
            Typography::Heading4 => "xl",
            Typography::Heading5 => "lg",
            Typography::Heading6 => "base",
            Typography::Body => "base",
            Typography::BodySmall => "sm",
            Typography::Caption => "xs",
            Typography::Label => "sm",
        }
    }
}
```

## Step 4: Add Theme Identity

Implement the `Theme` trait to provide identity:

```rust
use jupiter_design_system::themes::Theme;

impl Theme for MyTheme {
    fn name(&self) -> &str {
        "My Custom Theme"
    }
}
```

## Step 5: Test Your Theme

Create a simple test to verify your theme works:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use jupiter_design_system::prelude::*;

    #[test]
    fn test_my_theme() {
        let theme = MyTheme::new();
        
        // Test color resolution
        assert_eq!(theme.resolve_color(Color::Primary), "indigo-600");
        assert_eq!(theme.text_class(Color::Primary), "text-indigo-600");
        
        // Test with patterns
        let button_classes = primary_button(theme.clone()).classes();
        assert!(button_classes.contains("indigo-600"));
        
        let text_classes = title_typography(theme).classes();
        assert!(text_classes.contains("text-4xl"));
    }
}
```

## Step 6: Add Customization Options

Allow users to customize your theme:

```rust
impl MyTheme {
    /// Create theme with custom color overrides
    pub fn with_colors(mut palette: ColorPalette) -> Self {
        Self { palette }
    }
    
    /// Create theme with primary color override
    pub fn with_primary(primary: impl Into<String>) -> Self {
        let mut palette = create_my_palette();
        palette.primary = primary.into();
        palette.interactive = palette.primary.clone();
        Self { palette }
    }
    
    /// Create dark mode variant
    pub fn dark() -> Self {
        let mut palette = create_my_palette();
        
        // Invert surface colors for dark mode
        palette.surface = "gray-800".to_string();
        palette.background = "gray-900".to_string();
        palette.foreground = "white".to_string();
        palette.border = "gray-700".to_string();
        
        // Adjust text colors
        palette.text_primary = "white".to_string();
        palette.text_secondary = "gray-300".to_string();
        palette.text_tertiary = "gray-500".to_string();
        palette.text_inverse = "gray-900".to_string();
        
        Self { palette }
    }
}
```

## Step 7: Framework Integration

### Tailwind CSS Integration

If using Tailwind CSS, ensure your color values are valid Tailwind classes:

```rust
// Good - uses Tailwind color names
primary: "blue-600".to_string(),

// Good - uses custom CSS custom properties
primary: "primary".to_string(), // assumes you have --color-primary CSS variable

// Avoid - hex values won't work with utility classes
// primary: "#3B82F6".to_string(),
```

### Custom CSS Integration

For custom CSS frameworks, return appropriate class names:

```rust
impl ColorProvider for MyTheme {
    fn palette(&self) -> &ColorPalette { &self.palette }
    
    // Override to generate custom CSS classes
    fn text_class(&self, color: Color) -> String {
        format!("my-text-{}", self.resolve_color(color))
    }
    
    fn bg_class(&self, color: Color) -> String {
        format!("my-bg-{}", self.resolve_color(color))
    }
}
```

## Step 8: Advanced Patterns (Optional)

### Custom Action Styling

For advanced theming, you can customize how actions are styled:

```rust
use jupiter_design_system::patterns::{ActionSemantics, ActionIntent};

impl MyTheme {
    pub fn action_classes(&self, intent: ActionIntent) -> String {
        match intent {
            ActionIntent::Primary => format!(
                "{} {} px-6 py-3 rounded-lg font-semibold",
                self.bg_class(Color::Primary),
                self.text_class(Color::TextInverse)
            ),
            ActionIntent::Destructive => format!(
                "{} {} px-6 py-3 rounded-lg font-semibold",
                self.bg_class(Color::Error),
                self.text_class(Color::TextInverse)
            ),
            _ => self.default_action_classes(intent),
        }
    }
}
```

### Typography Customization

Customize typography for your brand:

```rust
use jupiter_design_system::patterns::{TypographyPattern, TypographyHierarchy};

impl MyTheme {
    pub fn typography_classes(&self, hierarchy: TypographyHierarchy) -> String {
        let base_classes = "font-sans leading-relaxed";
        let hierarchy_classes = match hierarchy {
            TypographyHierarchy::Title => "text-5xl font-bold tracking-tight",
            TypographyHierarchy::Heading => "text-3xl font-bold tracking-tight", 
            TypographyHierarchy::Body => "text-base font-normal",
            _ => return base_classes.to_string(),
        };
        format!("{} {}", base_classes, hierarchy_classes)
    }
}
```

## Complete Example

Here's a complete theme implementation:

```rust
use jupiter_design_system::prelude::*;

#[derive(Debug, Clone)]
pub struct CorporateTheme {
    palette: ColorPalette,
    variant: ThemeVariant,
}

#[derive(Debug, Clone)]
pub enum ThemeVariant {
    Light,
    Dark,
}

impl CorporateTheme {
    pub fn new() -> Self {
        Self {
            palette: Self::create_palette(ThemeVariant::Light),
            variant: ThemeVariant::Light,
        }
    }
    
    pub fn dark() -> Self {
        Self {
            palette: Self::create_palette(ThemeVariant::Dark),
            variant: ThemeVariant::Dark,
        }
    }
    
    fn create_palette(variant: ThemeVariant) -> ColorPalette {
        match variant {
            ThemeVariant::Light => ColorPalette {
                primary: "corporate-blue".to_string(),
                secondary: "corporate-gray".to_string(),
                accent: "corporate-orange".to_string(),
                surface: "white".to_string(),
                background: "gray-50".to_string(),
                text_primary: "gray-900".to_string(),
                // ... other colors
                ..Default::default()
            },
            ThemeVariant::Dark => ColorPalette {
                primary: "corporate-blue-light".to_string(),
                secondary: "corporate-gray-light".to_string(),
                accent: "corporate-orange-light".to_string(),
                surface: "gray-800".to_string(),
                background: "gray-900".to_string(),
                text_primary: "white".to_string(),
                // ... other colors
                ..Default::default()
            }
        }
    }
}

impl ColorProvider for CorporateTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

impl Theme for CorporateTheme {
    fn name(&self) -> &str {
        match self.variant {
            ThemeVariant::Light => "Corporate Light",
            ThemeVariant::Dark => "Corporate Dark",
        }
    }
}

// Usage
fn example_usage() {
    let theme = CorporateTheme::new();
    
    // Use with any pattern
    let hero_button = hero_button(theme.clone())
        .urgent()
        .classes();
        
    let title = title_typography(theme)
        .color(TypographyColor::Primary)
        .classes();
}
```

## Next Steps

1. **Test thoroughly** - Test your theme with all component patterns
2. **Document usage** - Create examples showing how to use your theme
3. **Consider accessibility** - Ensure color contrast meets WCAG guidelines
4. **Version compatibility** - Test with different versions of Jupiter Design System
5. **Share** - Consider contributing your theme back to the community

See [Best Practices](./best-practices.md) for additional guidance on theme design and implementation.