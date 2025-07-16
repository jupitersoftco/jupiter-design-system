# Theming & Color Modes

This document covers theme configuration, customization, and color mode support in the Jupiter Design System.

## Overview

The Jupiter Design System uses a trait-based theming architecture that allows for flexible color customization while maintaining semantic consistency. Themes are implemented through the `ColorProvider` trait and can be easily swapped or customized.

## Theme Architecture

### ColorProvider Trait

The foundation of the theming system is the `ColorProvider` trait:

```rust
pub trait ColorProvider {
    fn palette(&self) -> &ColorPalette;
    fn resolve_color(&self, color: Color) -> &str;
    fn text_class(&self, color: Color) -> String;
    fn bg_class(&self, color: Color) -> String;
    fn border_class(&self, color: Color) -> String;
}
```

This trait provides methods for:
- **`palette()`**: Access to the underlying color palette
- **`resolve_color()`**: Get the raw color value (e.g., "jupiter-blue-500")
- **`text_class()`**: Generate text color CSS classes (e.g., "text-jupiter-blue-500")
- **`bg_class()`**: Generate background color CSS classes (e.g., "bg-jupiter-blue-500")
- **`border_class()`**: Generate border color CSS classes (e.g., "border-jupiter-blue-500")

### Default Theme: VibeColors

The default theme implementation is `VibeColors`, which provides the Jupiter brand color scheme:

```rust
use jupiter_design_system::themes::VibeColors;

// Create default theme
let theme = VibeColors::default();

// Use in components
let button = Button::new("Click me")
    .variant(ButtonVariant::Primary)
    .build(&theme);
```

## Theme Customization

### Method 1: Override System

The easiest way to customize colors is using the override system:

```rust
use jupiter_design_system::themes::VibeColors;

let custom_theme = VibeColors::with_overrides(|palette| {
    // Customize brand colors
    palette.primary = "purple-600".to_string();
    palette.secondary = "pink-500".to_string();
    palette.accent = "orange-400".to_string();
    
    // Adjust semantic colors
    palette.success = "emerald-500".to_string();
    palette.warning = "amber-400".to_string();
    
    // Customize neutral colors
    palette.surface = "slate-50".to_string();
    palette.background = "slate-100".to_string();
});

// Use the custom theme
let themed_button = Button::new("Custom Theme")
    .variant(ButtonVariant::Primary)
    .build(&custom_theme);
```

### Method 2: Custom ColorProvider Implementation

For more advanced theming, implement the `ColorProvider` trait directly:

```rust
use jupiter_design_system::core::*;

pub struct CustomTheme {
    palette: ColorPalette,
}

impl CustomTheme {
    pub fn new() -> Self {
        Self {
            palette: ColorPalette {
                primary: "indigo-600".to_string(),
                secondary: "purple-500".to_string(),
                accent: "pink-400".to_string(),
                success: "green-600".to_string(),
                warning: "yellow-500".to_string(),
                error: "red-600".to_string(),
                info: "blue-600".to_string(),
                surface: "white".to_string(),
                background: "gray-50".to_string(),
                foreground: "gray-900".to_string(),
                border: "gray-300".to_string(),
                text_primary: "gray-900".to_string(),
                text_secondary: "gray-700".to_string(),
                text_tertiary: "gray-500".to_string(),
                text_inverse: "white".to_string(),
                interactive: "indigo-600".to_string(),
                interactive_hover: "indigo-700".to_string(),
                interactive_active: "indigo-800".to_string(),
                interactive_disabled: "gray-400".to_string(),
            },
        }
    }
}

impl ColorProvider for CustomTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
    
    fn resolve_color(&self, color: Color) -> &str {
        // Implementation matches VibeColors pattern
        match color {
            Color::Primary => &self.palette.primary,
            Color::Secondary => &self.palette.secondary,
            // ... other colors
        }
    }
    
    fn text_class(&self, color: Color) -> String {
        format!("text-{}", self.resolve_color(color))
    }
    
    fn bg_class(&self, color: Color) -> String {
        format!("bg-{}", self.resolve_color(color))
    }
    
    fn border_class(&self, color: Color) -> String {
        format!("border-{}", self.resolve_color(color))
    }
}

// Usage
let custom_theme = CustomTheme::new();
let button = Button::new("Custom Implementation")
    .build(&custom_theme);
```

## Common Theme Patterns

### Brand Theme

Create a theme that matches your brand colors:

```rust
fn create_brand_theme() -> impl ColorProvider {
    VibeColors::with_overrides(|palette| {
        // Brand-specific colors
        palette.primary = "brand-blue-600".to_string();
        palette.secondary = "brand-green-500".to_string();
        palette.accent = "brand-orange-400".to_string();
        
        // Ensure interactive elements use brand primary
        palette.interactive = "brand-blue-600".to_string();
        palette.interactive_hover = "brand-blue-700".to_string();
        palette.interactive_active = "brand-blue-800".to_string();
    })
}

// Usage with custom CSS variables
// In your CSS:
// :root {
//   --brand-blue-600: #1e40af;
//   --brand-green-500: #10b981;
//   --brand-orange-400: #fb923c;
// }
```

### Dark Mode Theme

Create a dark mode variant:

```rust
fn create_dark_theme() -> impl ColorProvider {
    VibeColors::with_overrides(|palette| {
        // Dark backgrounds
        palette.surface = "gray-800".to_string();
        palette.background = "gray-900".to_string();
        palette.foreground = "white".to_string();
        palette.border = "gray-600".to_string();
        
        // Light text on dark backgrounds
        palette.text_primary = "white".to_string();
        palette.text_secondary = "gray-300".to_string();
        palette.text_tertiary = "gray-400".to_string();
        palette.text_inverse = "gray-900".to_string();
        
        // Adjust disabled state for dark mode
        palette.interactive_disabled = "gray-600".to_string();
        
        // Keep semantic colors bright for visibility
        palette.success = "green-400".to_string();
        palette.warning = "amber-300".to_string();
        palette.error = "red-400".to_string();
        palette.info = "blue-400".to_string();
    })
}

// Dynamic theme switching
fn get_theme(is_dark_mode: bool) -> Box<dyn ColorProvider> {
    if is_dark_mode {
        Box::new(create_dark_theme())
    } else {
        Box::new(VibeColors::default())
    }
}
```

### High Contrast Theme

For accessibility compliance:

```rust
fn create_high_contrast_theme() -> impl ColorProvider {
    VibeColors::with_overrides(|palette| {
        // Maximum contrast pairs
        palette.primary = "black".to_string();
        palette.surface = "white".to_string();
        palette.background = "white".to_string();
        palette.foreground = "black".to_string();
        palette.border = "black".to_string();
        
        // High contrast text
        palette.text_primary = "black".to_string();
        palette.text_secondary = "black".to_string();
        palette.text_tertiary = "gray-800".to_string();
        palette.text_inverse = "white".to_string();
        
        // High contrast semantic colors
        palette.success = "green-800".to_string();
        palette.error = "red-800".to_string();
        palette.warning = "orange-700".to_string();
        palette.info = "blue-800".to_string();
        
        // High contrast interactive states
        palette.interactive = "blue-800".to_string();
        palette.interactive_hover = "blue-900".to_string();
        palette.interactive_active = "black".to_string();
        palette.interactive_disabled = "gray-500".to_string();
    })
}
```

### Monochrome Theme

For a minimal, grayscale design:

```rust
fn create_monochrome_theme() -> impl ColorProvider {
    VibeColors::with_overrides(|palette| {
        // All colors use grayscale
        palette.primary = "gray-900".to_string();
        palette.secondary = "gray-700".to_string();
        palette.accent = "gray-600".to_string();
        
        // Semantic colors in grayscale
        palette.success = "gray-800".to_string();
        palette.warning = "gray-600".to_string();
        palette.error = "gray-900".to_string();
        palette.info = "gray-700".to_string();
        
        // Interactive states
        palette.interactive = "gray-800".to_string();
        palette.interactive_hover = "gray-900".to_string();
        palette.interactive_active = "black".to_string();
    })
}
```

## Advanced Theming Techniques

### Theme Provider Context

For applications with multiple themes, create a theme provider system:

```rust
use std::collections::HashMap;

pub struct ThemeManager {
    themes: HashMap<String, Box<dyn ColorProvider>>,
    current_theme: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut themes: HashMap<String, Box<dyn ColorProvider>> = HashMap::new();
        themes.insert("default".to_string(), Box::new(VibeColors::default()));
        themes.insert("dark".to_string(), Box::new(create_dark_theme()));
        themes.insert("high-contrast".to_string(), Box::new(create_high_contrast_theme()));
        
        Self {
            themes,
            current_theme: "default".to_string(),
        }
    }
    
    pub fn set_theme(&mut self, theme_name: &str) {
        if self.themes.contains_key(theme_name) {
            self.current_theme = theme_name.to_string();
        }
    }
    
    pub fn current_theme(&self) -> &dyn ColorProvider {
        self.themes.get(&self.current_theme).unwrap().as_ref()
    }
    
    pub fn register_theme(&mut self, name: String, theme: Box<dyn ColorProvider>) {
        self.themes.insert(name, theme);
    }
}

// Usage
let mut theme_manager = ThemeManager::new();
theme_manager.set_theme("dark");

let button = Button::new("Themed Button")
    .build(theme_manager.current_theme());
```

### CSS Custom Properties Integration

For dynamic theme switching with CSS custom properties:

```rust
fn generate_css_variables(theme: &impl ColorProvider) -> String {
    let palette = theme.palette();
    format!(
        r#":root {{
            --color-primary: {};
            --color-secondary: {};
            --color-success: {};
            --color-error: {};
            --color-surface: {};
            --color-text-primary: {};
            /* ... other variables */
        }}"#,
        palette.primary,
        palette.secondary,
        palette.success,
        palette.error,
        palette.surface,
        palette.text_primary,
    )
}

// Generate CSS for current theme
let theme = VibeColors::default();
let css_vars = generate_css_variables(&theme);

// In your HTML head:
// <style>{css_vars}</style>
```

### Theme Validation

Ensure theme consistency and accessibility:

```rust
pub fn validate_theme(theme: &impl ColorProvider) -> Vec<String> {
    let mut warnings = Vec::new();
    let palette = theme.palette();
    
    // Check for missing colors
    if palette.primary.is_empty() {
        warnings.push("Primary color is not set".to_string());
    }
    
    // Check for accessibility issues
    if palette.text_primary == palette.surface {
        warnings.push("Text and surface colors are identical - poor contrast".to_string());
    }
    
    // Check for semantic consistency
    if palette.success.contains("red") {
        warnings.push("Success color appears to be red - semantic inconsistency".to_string());
    }
    
    warnings
}

// Usage
let theme = VibeColors::default();
let warnings = validate_theme(&theme);
for warning in warnings {
    println!("Theme warning: {}", warning);
}
```

## Runtime Theme Switching

### Web Application Example

```rust
// JavaScript integration for web apps
fn generate_theme_switcher_js() -> String {
    r#"
    function switchTheme(themeName) {
        const themes = {
            'light': generateLightTheme(),
            'dark': generateDarkTheme(),
            'high-contrast': generateHighContrastTheme()
        };
        
        const theme = themes[themeName];
        if (!theme) return;
        
        // Update CSS custom properties
        Object.entries(theme).forEach(([key, value]) => {
            document.documentElement.style.setProperty(`--${key}`, value);
        });
        
        // Store preference
        localStorage.setItem('preferred-theme', themeName);
    }
    
    // Auto-detect system preference
    function detectSystemTheme() {
        return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }
    
    // Initialize theme on page load
    const savedTheme = localStorage.getItem('preferred-theme') || detectSystemTheme();
    switchTheme(savedTheme);
    "#.to_string()
}
```

### Theme Serialization

For storing and loading themes:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SerializableTheme {
    name: String,
    palette: ColorPalette,
}

impl SerializableTheme {
    pub fn from_provider(name: String, provider: &impl ColorProvider) -> Self {
        Self {
            name,
            palette: provider.palette().clone(),
        }
    }
    
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

// Usage
let theme = VibeColors::default();
let serializable = SerializableTheme::from_provider("custom".to_string(), &theme);
let json = serializable.to_json().unwrap();

// Save to file or database
std::fs::write("my-theme.json", json).unwrap();

// Load from file
let loaded_json = std::fs::read_to_string("my-theme.json").unwrap();
let loaded_theme = SerializableTheme::from_json(&loaded_json).unwrap();
```

## Best Practices

### Theme Design Guidelines

**Maintain semantic consistency**
```rust
// Good: Semantic colors have consistent meaning
palette.success = "green-500".to_string();  // Always green for success
palette.error = "red-500".to_string();      // Always red for errors
```

**Ensure sufficient contrast**
```rust
// Good: High contrast combinations
palette.text_primary = "gray-900".to_string();  // Dark text
palette.surface = "white".to_string();          // Light surface
// Results in 21:1 contrast ratio
```

**Test across all themes**
```rust
// Good: Test component with multiple themes
let themes = vec![
    Box::new(VibeColors::default()) as Box<dyn ColorProvider>,
    Box::new(create_dark_theme()) as Box<dyn ColorProvider>,
    Box::new(create_high_contrast_theme()) as Box<dyn ColorProvider>,
];

for theme in themes {
    let button = Button::new("Test").build(theme.as_ref());
    // Verify button looks good with this theme
}
```

### Performance Considerations

**Cache theme instances**
```rust
// Good: Reuse theme instances
lazy_static! {
    static ref DEFAULT_THEME: VibeColors = VibeColors::default();
    static ref DARK_THEME: impl ColorProvider = create_dark_theme();
}

// Use cached instances
let button = Button::new("Cached Theme").build(&*DEFAULT_THEME);
```

**Minimize theme switches**
```rust
// Good: Batch operations with same theme
let theme = get_current_theme();
let components = vec![
    Button::new("One").build(&theme),
    Button::new("Two").build(&theme),
    Button::new("Three").build(&theme),
];
```

## Migration Guide

### From Hardcoded Colors

```rust
// Old: Hardcoded colors
let button = r#"<button class="bg-blue-500 text-white">Click</button>"#;

// New: Themed colors
let theme = VibeColors::default();
let button = Button::new("Click")
    .variant(ButtonVariant::Primary)
    .build(&theme);
```

### Adding Theme Support to Existing Components

```rust
// Old: Component without theme support
pub struct OldCard {
    content: String,
}

impl OldCard {
    pub fn render(&self) -> String {
        format!(r#"<div class="bg-white text-gray-900">{}</div>"#, self.content)
    }
}

// New: Theme-aware component
pub struct NewCard {
    content: String,
}

impl NewCard {
    pub fn render(&self, theme: &impl ColorProvider) -> String {
        format!(
            r#"<div class="{} {}">{}</div>"#,
            theme.bg_class(Color::Surface),
            theme.text_class(Color::TextPrimary),
            self.content
        )
    }
}
```

The theming system provides powerful customization capabilities while maintaining the semantic consistency and type safety that makes the Jupiter Design System robust and reliable.