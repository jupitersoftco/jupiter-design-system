# Theming & Customization Best Practices

## 🎨 Theming Philosophy

The Jupiter Design System's theming system enables complete visual customization while maintaining consistency, accessibility, and component interoperability.

### Core Principles

1. **Semantic Tokens**: Theme colors by meaning, not appearance
2. **Systematic Relationships**: Maintain color relationships across themes
3. **Accessibility First**: Ensure adequate contrast in all themes
4. **Framework Agnostic**: Themes work with any frontend framework
5. **Runtime Flexibility**: Switch themes without rebuilding

## 🏗️ Theme Architecture

### ColorProvider Trait

All themes implement the `ColorProvider` trait:

```rust
pub trait ColorProvider {
    fn palette(&self) -> &ColorPalette;
    fn resolve_color(&self, color: Color) -> &str;
    fn text_class(&self, color: Color) -> String;
    fn bg_class(&self, color: Color) -> String;
    fn border_class(&self, color: Color) -> String;
}
```

### Color Palette Structure

Every theme contains a complete color palette:

```rust
pub struct ColorPalette {
    // Brand colors
    pub primary: String,
    pub secondary: String,
    pub accent: String,

    // Semantic colors
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,

    // Neutral colors
    pub surface: String,
    pub background: String,
    pub foreground: String,
    pub border: String,

    // Text colors
    pub text_primary: String,
    pub text_secondary: String,
    pub text_tertiary: String,
    pub text_inverse: String,

    // Interactive states
    pub interactive: String,
    pub interactive_hover: String,
    pub interactive_active: String,
    pub interactive_disabled: String,
}
```

## 🎭 Default Vibe Theme

### Using the Default Theme

```rust
use jupiter_design_system::themes::VibeColors;

// Create default theme
let colors = VibeColors::new();

// Use with components
let button_classes = button_styles(colors)
    .primary()
    .classes();
// Results in: "bg-jupiter-blue-500 text-white ..."
```

### Default Theme Colors

The Vibe theme provides Jupiter's brand colors:

```rust
// Brand colors
colors.resolve_color(Color::Primary);    // "jupiter-blue-500"
colors.resolve_color(Color::Secondary);  // "jupiter-green-500"
colors.resolve_color(Color::Accent);     // "jupiter-orange-500"

// Semantic colors
colors.resolve_color(Color::Success);    // "green-500"
colors.resolve_color(Color::Warning);    // "amber-500"
colors.resolve_color(Color::Error);      // "red-500"
colors.resolve_color(Color::Info);       // "blue-500"

// Neutral colors
colors.resolve_color(Color::Surface);    // "white"
colors.resolve_color(Color::Background); // "gray-50"
colors.resolve_color(Color::Border);     // "gray-200"
```

## 🛠️ Custom Theme Creation

### Method 1: Theme Overrides

Quick customization of the default theme:

```rust
// Override specific colors
let custom_colors = VibeColors::with_overrides(|palette| {
    // Brand customization
    palette.primary = "purple-600".to_string();
    palette.secondary = "pink-500".to_string();
    palette.accent = "orange-400".to_string();
    
    // Update related interactive colors
    palette.interactive = "purple-600".to_string();
    palette.interactive_hover = "purple-700".to_string();
    palette.interactive_active = "purple-800".to_string();
});

// Use customized theme
let button = button_styles(custom_colors)
    .primary()
    .classes();
// Results in: "bg-purple-600 text-white ..."
```

### Method 2: Complete Custom Theme

Create a fully custom theme implementation:

```rust
#[derive(Debug, Clone)]
pub struct MyCustomTheme {
    palette: ColorPalette,
}

impl Default for MyCustomTheme {
    fn default() -> Self {
        Self {
            palette: ColorPalette {
                // Brand colors
                primary: "indigo-600".to_string(),
                secondary: "teal-500".to_string(),
                accent: "amber-400".to_string(),

                // Semantic colors
                success: "emerald-500".to_string(),
                warning: "orange-500".to_string(),
                error: "rose-500".to_string(),
                info: "sky-500".to_string(),

                // Neutral colors
                surface: "white".to_string(),
                background: "slate-50".to_string(),
                foreground: "slate-900".to_string(),
                border: "slate-200".to_string(),

                // Text colors
                text_primary: "slate-900".to_string(),
                text_secondary: "slate-600".to_string(),
                text_tertiary: "slate-400".to_string(),
                text_inverse: "white".to_string(),

                // Interactive states
                interactive: "indigo-600".to_string(),
                interactive_hover: "indigo-700".to_string(),
                interactive_active: "indigo-800".to_string(),
                interactive_disabled: "slate-300".to_string(),
            },
        }
    }
}

impl ColorProvider for MyCustomTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

// Usage
let my_theme = MyCustomTheme::default();
let button = button_styles(my_theme).primary().classes();
```

## 🌙 Dark Theme Implementation

### Dark Theme Pattern

```rust
#[derive(Debug, Clone)]
pub struct DarkTheme {
    palette: ColorPalette,
}

impl Default for DarkTheme {
    fn default() -> Self {
        Self {
            palette: ColorPalette {
                // Inverted neutrals for dark theme
                surface: "gray-800".to_string(),
                background: "gray-900".to_string(),
                foreground: "gray-100".to_string(),
                border: "gray-700".to_string(),

                // Inverted text colors
                text_primary: "gray-100".to_string(),
                text_secondary: "gray-300".to_string(),
                text_tertiary: "gray-500".to_string(),
                text_inverse: "gray-900".to_string(),

                // Adjusted brand colors for better contrast
                primary: "jupiter-blue-400".to_string(),
                secondary: "jupiter-green-400".to_string(),
                accent: "jupiter-orange-400".to_string(),

                // Adjusted semantic colors
                success: "green-400".to_string(),
                warning: "amber-400".to_string(),
                error: "red-400".to_string(),
                info: "blue-400".to_string(),

                // Dark theme interactive states
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

### Light/Dark Theme Switching

```rust
#[derive(Debug, Clone)]
pub enum ThemeMode {
    Light,
    Dark,
}

pub fn get_theme_provider(mode: ThemeMode) -> Box<dyn ColorProvider> {
    match mode {
        ThemeMode::Light => Box::new(VibeColors::new()),
        ThemeMode::Dark => Box::new(DarkTheme::default()),
    }
}

// Usage with dynamic switching
fn render_with_theme(theme_mode: ThemeMode) -> String {
    let colors = get_theme_provider(theme_mode);
    
    button_styles(colors.as_ref())
        .primary()
        .classes()
}
```

## 🎨 Brand-Specific Themes

### Enterprise Theme

```rust
#[derive(Debug, Clone)]
pub struct EnterpriseTheme {
    palette: ColorPalette,
}

impl Default for EnterpriseTheme {
    fn default() -> Self {
        Self {
            palette: ColorPalette {
                // Conservative brand colors
                primary: "blue-700".to_string(),
                secondary: "slate-600".to_string(),
                accent: "blue-500".to_string(),

                // Professional semantic colors
                success: "green-600".to_string(),
                warning: "amber-600".to_string(),
                error: "red-600".to_string(),
                info: "blue-600".to_string(),

                // Clean neutrals
                surface: "white".to_string(),
                background: "gray-25".to_string(),
                foreground: "gray-900".to_string(),
                border: "gray-300".to_string(),

                // Clear text hierarchy
                text_primary: "gray-900".to_string(),
                text_secondary: "gray-700".to_string(),
                text_tertiary: "gray-500".to_string(),
                text_inverse: "white".to_string(),

                // Conservative interactive states
                interactive: "blue-700".to_string(),
                interactive_hover: "blue-800".to_string(),
                interactive_active: "blue-900".to_string(),
                interactive_disabled: "gray-400".to_string(),
            },
        }
    }
}
```

### Creative Theme

```rust
#[derive(Debug, Clone)]
pub struct CreativeTheme {
    palette: ColorPalette,
}

impl Default for CreativeTheme {
    fn default() -> Self {
        Self {
            palette: ColorPalette {
                // Vibrant brand colors
                primary: "fuchsia-500".to_string(),
                secondary: "cyan-500".to_string(),
                accent: "yellow-400".to_string(),

                // Energetic semantic colors
                success: "lime-500".to_string(),
                warning: "orange-400".to_string(),
                error: "pink-500".to_string(),
                info: "purple-500".to_string(),

                // Creative neutrals
                surface: "white".to_string(),
                background: "slate-50".to_string(),
                foreground: "slate-900".to_string(),
                border: "slate-200".to_string(),

                // Expressive text colors
                text_primary: "slate-900".to_string(),
                text_secondary: "slate-600".to_string(),
                text_tertiary: "slate-400".to_string(),
                text_inverse: "white".to_string(),

                // Dynamic interactive states
                interactive: "fuchsia-500".to_string(),
                interactive_hover: "fuchsia-600".to_string(),
                interactive_active: "fuchsia-700".to_string(),
                interactive_disabled: "slate-300".to_string(),
            },
        }
    }
}
```

## 🎯 Theme Management Patterns

### Theme Provider Context

```rust
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ThemeContext {
    provider: Rc<dyn ColorProvider>,
    name: String,
}

impl ThemeContext {
    pub fn new(provider: impl ColorProvider + 'static, name: String) -> Self {
        Self {
            provider: Rc::new(provider),
            name,
        }
    }
    
    pub fn provider(&self) -> &dyn ColorProvider {
        self.provider.as_ref()
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
}

// Usage in component frameworks
// let theme_context = ThemeContext::new(VibeColors::new(), "default".to_string());
```

### Theme Registry

```rust
use std::collections::HashMap;

pub struct ThemeRegistry {
    themes: HashMap<String, Box<dyn ColorProvider>>,
    active_theme: String,
}

impl ThemeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            themes: HashMap::new(),
            active_theme: "default".to_string(),
        };
        
        // Register default themes
        registry.register("default", Box::new(VibeColors::new()));
        registry.register("dark", Box::new(DarkTheme::default()));
        registry.register("enterprise", Box::new(EnterpriseTheme::default()));
        
        registry
    }
    
    pub fn register(&mut self, name: &str, provider: Box<dyn ColorProvider>) {
        self.themes.insert(name.to_string(), provider);
    }
    
    pub fn set_active(&mut self, name: &str) -> Result<(), String> {
        if self.themes.contains_key(name) {
            self.active_theme = name.to_string();
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", name))
        }
    }
    
    pub fn active(&self) -> &dyn ColorProvider {
        self.themes.get(&self.active_theme)
            .unwrap()
            .as_ref()
    }
}
```

### Dynamic Theme Loading

```rust
// Theme configuration from external sources
#[derive(Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub colors: ColorPalette,
}

impl ThemeConfig {
    pub fn load_from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
    
    pub fn into_provider(self) -> impl ColorProvider {
        ConfigurableTheme {
            palette: self.colors,
            name: self.name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfigurableTheme {
    palette: ColorPalette,
    name: String,
}

impl ColorProvider for ConfigurableTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}
```

## 🧪 Theme Testing

### Theme Validation

```rust
#[cfg(test)]
mod theme_tests {
    use super::*;

    #[test]
    fn test_theme_completeness() {
        let theme = MyCustomTheme::default();
        let palette = theme.palette();
        
        // Ensure all required colors are defined
        assert!(!palette.primary.is_empty());
        assert!(!palette.secondary.is_empty());
        assert!(!palette.success.is_empty());
        assert!(!palette.text_primary.is_empty());
        assert!(!palette.interactive.is_empty());
    }

    #[test]
    fn test_theme_consistency() {
        let light_theme = VibeColors::new();
        let dark_theme = DarkTheme::default();
        
        // Test component generation with different themes
        let light_button = button_styles(light_theme).primary().classes();
        let dark_button = button_styles(dark_theme).primary().classes();
        
        // Both should generate valid classes
        assert!(!light_button.is_empty());
        assert!(!dark_button.is_empty());
        
        // Classes should be different (theme-specific)
        assert_ne!(light_button, dark_button);
    }
}
```

### Contrast Testing

```rust
// Color contrast validation
pub fn validate_theme_contrast(theme: &dyn ColorProvider) -> Vec<String> {
    let mut issues = Vec::new();
    let palette = theme.palette();
    
    // Test text on background combinations
    if !has_sufficient_contrast(&palette.text_primary, &palette.background) {
        issues.push("Insufficient contrast: text_primary on background".to_string());
    }
    
    if !has_sufficient_contrast(&palette.text_secondary, &palette.background) {
        issues.push("Insufficient contrast: text_secondary on background".to_string());
    }
    
    // Test interactive elements
    if !has_sufficient_contrast("white", &palette.primary) {
        issues.push("Insufficient contrast: white text on primary".to_string());
    }
    
    issues
}

fn has_sufficient_contrast(foreground: &str, background: &str) -> bool {
    // Implement WCAG contrast ratio calculation
    // This is a simplified placeholder
    true // Replace with actual contrast calculation
}
```

### Visual Regression Testing

```rust
// Generate theme swatches for visual testing
pub fn generate_theme_swatches(theme: &dyn ColorProvider) -> Vec<(String, String)> {
    let palette = theme.palette();
    
    vec![
        ("Primary".to_string(), palette.primary.clone()),
        ("Secondary".to_string(), palette.secondary.clone()),
        ("Success".to_string(), palette.success.clone()),
        ("Warning".to_string(), palette.warning.clone()),
        ("Error".to_string(), palette.error.clone()),
        ("Surface".to_string(), palette.surface.clone()),
        ("Background".to_string(), palette.background.clone()),
    ]
}
```

## 🚦 Theming Guidelines

### Do's ✅

1. **Maintain Color Relationships**
   ```rust
   // ✅ Good: Related interactive states
   let custom_theme = VibeColors::with_overrides(|palette| {
       palette.primary = "purple-600".to_string();
       palette.interactive = "purple-600".to_string();
       palette.interactive_hover = "purple-700".to_string();
       palette.interactive_active = "purple-800".to_string();
   });
   ```

2. **Test All Theme Combinations**
   ```rust
   // ✅ Good: Comprehensive testing
   #[test]
   fn test_all_themes() {
       let themes: Vec<Box<dyn ColorProvider>> = vec![
           Box::new(VibeColors::new()),
           Box::new(DarkTheme::default()),
           Box::new(EnterpriseTheme::default()),
       ];
       
       for theme in themes {
           let button = button_styles(theme.as_ref()).primary().classes();
           assert!(!button.is_empty());
       }
   }
   ```

3. **Use Semantic Color Names**
   ```rust
   // ✅ Good: Semantic naming
   palette.primary = "brand-blue".to_string();
   palette.success = "positive-green".to_string();
   palette.error = "negative-red".to_string();
   ```

### Don'ts ❌

1. **Don't Break Color Relationships**
   ```rust
   // ❌ Bad: Inconsistent interactive states
   let broken_theme = VibeColors::with_overrides(|palette| {
       palette.primary = "purple-600".to_string();
       // Forgot to update interactive colors - they'll still be blue!
   });
   ```

2. **Don't Ignore Accessibility**
   ```rust
   // ❌ Bad: Poor contrast
   let poor_contrast = VibeColors::with_overrides(|palette| {
       palette.text_primary = "gray-400".to_string();  // Too light
       palette.background = "gray-300".to_string();    // Poor contrast
   });
   ```

3. **Don't Hardcode Theme Values**
   ```rust
   // ❌ Bad: Hardcoded colors
   let hardcoded_style = "bg-blue-500 text-white";
   
   // ✅ Good: Theme-aware
   let themed_button = button_styles(colors).primary().classes();
   ```

## ⚡ Performance Considerations

### Theme Caching

```rust
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CachedThemeProvider {
    base_provider: Arc<dyn ColorProvider>,
    class_cache: HashMap<(Color, String), String>,
}

impl CachedThemeProvider {
    pub fn new(provider: impl ColorProvider + 'static) -> Self {
        Self {
            base_provider: Arc::new(provider),
            class_cache: HashMap::new(),
        }
    }
}

impl ColorProvider for CachedThemeProvider {
    fn palette(&self) -> &ColorPalette {
        self.base_provider.palette()
    }
    
    fn text_class(&self, color: Color) -> String {
        // Implement caching for expensive operations
        self.base_provider.text_class(color)
    }
}
```

### Lazy Theme Loading

```rust
lazy_static! {
    static ref DEFAULT_THEME: VibeColors = VibeColors::new();
    static ref DARK_THEME: DarkTheme = DarkTheme::default();
}

pub fn get_cached_theme(name: &str) -> Option<&'static dyn ColorProvider> {
    match name {
        "default" => Some(&*DEFAULT_THEME),
        "dark" => Some(&*DARK_THEME),
        _ => None,
    }
}
```

## 🔗 Related Documentation

- [Color System](./colors.md) - Understanding the color system
- [Component Guidelines](./components.md) - Using themes with components
- [Accessibility Guide](./accessibility.md) - Accessible theming practices
- [Examples](./examples/) - Complete theme implementations