# Theme Migration Guide

This guide helps you migrate themes and understand version changes in the Jupiter Design System.

## Migration Overview

The Jupiter Design System follows semantic versioning. This guide covers:

- **Minor version updates** (e.g., 1.1.0 → 1.2.0) - New features, backward compatible
- **Major version updates** (e.g., 1.x.x → 2.0.0) - Breaking changes
- **Legacy theme migration** - Updating old theme implementations

## Current Version: 1.x.x

### Recent Changes (1.1.0 → 1.2.0)

#### New Features
- Added `TypographyOverflow::Clamp(u32)` for line clamping
- New convenience methods for theme creation
- Enhanced focus management patterns

#### Migration Steps
```rust
// Before (1.1.0)
let typography = typography_pattern(colors)
    .hierarchy(TypographyHierarchy::Body);

// After (1.2.0) - same syntax, but now supports line clamping
let typography = typography_pattern(colors)
    .hierarchy(TypographyHierarchy::Body)
    .overflow(TypographyOverflow::Clamp(3)); // New feature
```

No breaking changes - existing code continues to work.

## Migrating from Legacy Implementations

### Pre-1.0 Color System Migration

If you have a theme implemented before the trait system:

#### Old Implementation (Pre-1.0)
```rust
// Old direct class generation
pub struct OldTheme {
    primary_color: String,
    secondary_color: String,
}

impl OldTheme {
    pub fn button_classes(&self) -> String {
        format!("bg-{} text-white px-4 py-2", self.primary_color)
    }
}
```

#### New Implementation (1.x.x)
```rust
use jupiter_design_system::prelude::*;

#[derive(Debug, Clone)]
pub struct ModernTheme {
    palette: ColorPalette,
}

impl ModernTheme {
    pub fn migrate_from_old(old: OldTheme) -> Self {
        let palette = ColorPalette {
            primary: old.primary_color,
            secondary: old.secondary_color,
            // Set reasonable defaults for new fields
            ..Default::default()
        };
        Self { palette }
    }
}

impl ColorProvider for ModernTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

impl Theme for ModernTheme {
    fn name(&self) -> &str {
        "Migrated Theme"
    }
}

// Usage - now works with all patterns
let button = primary_button(theme).classes();
```

### Step-by-Step Migration Process

#### Step 1: Identify Current Implementation
```rust
// Audit your current theme structure
#[derive(Debug)]
pub struct CurrentThemeAudit {
    pub has_color_provider: bool,
    pub has_theme_trait: bool,
    pub color_count: usize,
    pub missing_colors: Vec<String>,
}

impl CurrentThemeAudit {
    pub fn audit_theme<T>(_theme: T) -> Self {
        // Implement audit logic based on your current theme
        Self {
            has_color_provider: false, // Check if implemented
            has_theme_trait: false,    // Check if implemented
            color_count: 0,           // Count available colors
            missing_colors: vec![     // List missing semantic colors
                "interactive".to_string(),
                "interactive_hover".to_string(),
                // ... etc
            ],
        }
    }
}
```

#### Step 2: Create Migration Helper
```rust
pub struct ThemeMigrator;

impl ThemeMigrator {
    /// Migrate from old color structure to new ColorPalette
    pub fn migrate_colors(old_colors: YourOldColorStruct) -> ColorPalette {
        ColorPalette {
            // Map old color names to new semantic names
            primary: old_colors.brand_primary,
            secondary: old_colors.brand_secondary,
            
            // Map state colors
            success: old_colors.positive_color,
            error: old_colors.negative_color,
            warning: old_colors.caution_color,
            info: old_colors.info_color,
            
            // Map surface colors
            surface: old_colors.card_background,
            background: old_colors.page_background,
            foreground: old_colors.text_color,
            border: old_colors.border_color,
            
            // Set interactive states (new requirement)
            interactive: old_colors.brand_primary.clone(),
            interactive_hover: Self::darken_color(&old_colors.brand_primary),
            interactive_active: Self::darken_color_more(&old_colors.brand_primary),
            interactive_disabled: old_colors.disabled_color,
            
            // Map text hierarchy
            text_primary: old_colors.text_color,
            text_secondary: old_colors.muted_text,
            text_tertiary: old_colors.light_text,
            text_inverse: old_colors.inverse_text,
            
            // Use fallbacks for missing colors
            accent: old_colors.accent.unwrap_or_else(|| old_colors.brand_secondary.clone()),
        }
    }
    
    fn darken_color(color: &str) -> String {
        // Implement color darkening logic for your color system
        // For Tailwind: "blue-500" -> "blue-600"
        // For custom: implement your darkening logic
        if color.ends_with("-500") {
            color.replace("-500", "-600")
        } else {
            format!("{}-dark", color)
        }
    }
    
    fn darken_color_more(color: &str) -> String {
        // Even darker for active state
        if color.ends_with("-500") {
            color.replace("-500", "-700")
        } else {
            format!("{}-darker", color)
        }
    }
}
```

#### Step 3: Implement New Traits
```rust
#[derive(Debug, Clone)]
pub struct MigratedTheme {
    palette: ColorPalette,
    original_name: String,
}

impl MigratedTheme {
    pub fn from_legacy(legacy: YourOldTheme) -> Self {
        Self {
            palette: ThemeMigrator::migrate_colors(legacy.colors),
            original_name: legacy.name.unwrap_or_else(|| "Migrated Theme".to_string()),
        }
    }
}

impl ColorProvider for MigratedTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

impl Theme for MigratedTheme {
    fn name(&self) -> &str {
        &self.original_name
    }
}
```

#### Step 4: Test Migration
```rust
#[cfg(test)]
mod migration_tests {
    use super::*;
    
    #[test]
    fn test_legacy_migration() {
        // Create old theme
        let old_theme = YourOldTheme {
            brand_primary: "blue-500".to_string(),
            brand_secondary: "gray-500".to_string(),
            // ... other old fields
        };
        
        // Migrate to new system
        let new_theme = MigratedTheme::from_legacy(old_theme);
        
        // Test that colors are properly mapped
        assert_eq!(new_theme.resolve_color(Color::Primary), "blue-500");
        assert_eq!(new_theme.resolve_color(Color::Secondary), "gray-500");
        
        // Test that new features work
        let button_classes = primary_button(new_theme.clone()).classes();
        assert!(button_classes.contains("blue-500"));
        
        // Test patterns work
        let typography = title_typography(new_theme).classes();
        assert!(typography.contains("text-4xl"));
    }
    
    #[test]
    fn test_missing_colors_have_defaults() {
        let theme = MigratedTheme::from_legacy(minimal_old_theme());
        
        // Ensure all required colors are present
        assert!(!theme.resolve_color(Color::Interactive).is_empty());
        assert!(!theme.resolve_color(Color::InteractiveHover).is_empty());
        assert!(!theme.resolve_color(Color::TextPrimary).is_empty());
    }
}
```

## Real-World Migration Scenarios

### Scenario 1: Migrating from Inline Styles

If your current codebase uses inline style generation:

#### Before (Inline Styles)
```rust
pub struct OldButtonTheme {
    primary_color: String,    // hex color
    text_color: String,       // hex color
    hover_color: String,      // hex color
}

impl OldButtonTheme {
    pub fn button_style(&self) -> String {
        format!(
            "background-color: {}; color: {}; padding: 8px 16px; border-radius: 4px;",
            self.primary_color, self.text_color
        )
    }
    
    pub fn button_hover_style(&self) -> String {
        format!("background-color: {};", self.hover_color)
    }
}

// Usage
let theme = OldButtonTheme {
    primary_color: "#3B82F6".to_string(),
    text_color: "#FFFFFF".to_string(),
    hover_color: "#2563EB".to_string(),
};
let style = theme.button_style();
```

#### After (Jupiter Design System)
```rust
use jupiter_design_system::prelude::*;

#[derive(Debug, Clone)]
pub struct MigratedTheme {
    palette: ColorPalette,
}

impl MigratedTheme {
    pub fn from_old_theme(old: OldButtonTheme) -> Self {
        // Map hex colors to Tailwind classes
        let primary = Self::hex_to_tailwind(&old.primary_color);
        let hover = Self::hex_to_tailwind(&old.hover_color);
        
        let mut palette = ColorPalette::default();
        palette.primary = primary.clone();
        palette.interactive = primary;
        palette.interactive_hover = hover;
        palette.text_inverse = "white".to_string();
        
        Self { palette }
    }
    
    fn hex_to_tailwind(hex: &str) -> String {
        match hex.to_lowercase().as_str() {
            "#3b82f6" => "blue-500".to_string(),
            "#2563eb" => "blue-600".to_string(),
            "#1d4ed8" => "blue-700".to_string(),
            _ => "blue-500".to_string(), // fallback
        }
    }
}

impl ColorProvider for MigratedTheme {
    fn palette(&self) -> &ColorPalette { &self.palette }
}

// Usage
let old_theme = load_old_theme();
let new_theme = MigratedTheme::from_old_theme(old_theme);
let button_classes = primary_button(new_theme).classes();
```

### Scenario 2: Migrating from CSS-in-JS

If migrating from a CSS-in-JS solution:

#### Before (emotion/styled-components style)
```javascript
const theme = {
  colors: {
    primary: '#007bff',
    secondary: '#6c757d',
    success: '#28a745',
    danger: '#dc3545',
  },
  spacing: {
    small: '8px',
    medium: '16px',
    large: '24px',
  },
  borderRadius: '4px',
};

const Button = styled.button`
  background: ${props => props.primary ? theme.colors.primary : theme.colors.secondary};
  color: white;
  padding: ${theme.spacing.medium};
  border-radius: ${theme.borderRadius};
  
  &:hover {
    opacity: 0.8;
  }
`;
```

#### Migration Strategy
```rust
use jupiter_design_system::prelude::*;
use serde::{Deserialize, Serialize};

// 1. Create a compatible structure
#[derive(Debug, Deserialize, Serialize)]
pub struct CSSInJSTheme {
    colors: CSSInJSColors,
    spacing: CSSInJSSpacing,
    border_radius: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CSSInJSColors {
    primary: String,
    secondary: String,
    success: String,
    danger: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CSSInJSSpacing {
    small: String,
    medium: String,
    large: String,
}

// 2. Create migration logic
impl From<CSSInJSTheme> for ColorPalette {
    fn from(css_theme: CSSInJSTheme) -> Self {
        ColorPalette {
            primary: Self::css_color_to_tailwind(&css_theme.colors.primary),
            secondary: Self::css_color_to_tailwind(&css_theme.colors.secondary),
            success: Self::css_color_to_tailwind(&css_theme.colors.success),
            error: Self::css_color_to_tailwind(&css_theme.colors.danger),
            // ... map other colors
            ..Default::default()
        }
    }
}

// 3. Create bridge for gradual migration
pub struct BridgeTheme {
    jupiter_theme: Box<dyn ColorProvider>,
    legacy_theme: CSSInJSTheme,
}

impl BridgeTheme {
    pub fn button_classes(&self, primary: bool) -> String {
        if primary {
            primary_button(self.jupiter_theme.clone()).classes()
        } else {
            secondary_button(self.jupiter_theme.clone()).classes()
        }
    }
    
    // Keep legacy method during transition
    pub fn get_legacy_color(&self, name: &str) -> &str {
        match name {
            "primary" => &self.legacy_theme.colors.primary,
            "secondary" => &self.legacy_theme.colors.secondary,
            _ => "#000000",
        }
    }
}
```

### Scenario 3: Migrating from Component Library Themes

If migrating from Material-UI, Ant Design, or similar:

#### Before (Material-UI style)
```typescript
const muiTheme = createTheme({
  palette: {
    primary: {
      main: '#1976d2',
      light: '#42a5f5',
      dark: '#1565c0',
    },
    secondary: {
      main: '#dc004e',
      light: '#e33371',
      dark: '#9a0036',
    },
  },
  typography: {
    h1: {
      fontSize: '2.5rem',
      fontWeight: 500,
    },
    body1: {
      fontSize: '1rem',
      lineHeight: 1.5,
    },
  },
});
```

#### Migration Approach
```rust
use jupiter_design_system::prelude::*;

pub struct MaterialUITheme {
    primary: MaterialUIColor,
    secondary: MaterialUIColor,
    typography: MaterialUITypography,
}

pub struct MaterialUIColor {
    main: String,
    light: String,
    dark: String,
}

impl From<MaterialUITheme> for ColorPalette {
    fn from(mui: MaterialUITheme) -> Self {
        ColorPalette {
            primary: Self::mui_to_tailwind(&mui.primary.main),
            interactive: Self::mui_to_tailwind(&mui.primary.main),
            interactive_hover: Self::mui_to_tailwind(&mui.primary.dark),
            interactive_active: Self::mui_to_tailwind(&mui.primary.dark),
            
            secondary: Self::mui_to_tailwind(&mui.secondary.main),
            
            // MUI doesn't have these, use sensible defaults
            success: "green-600".to_string(),
            warning: "yellow-600".to_string(),
            error: "red-600".to_string(),
            info: "blue-600".to_string(),
            
            // Surface colors
            surface: "white".to_string(),
            background: "gray-50".to_string(),
            foreground: "gray-900".to_string(),
            border: "gray-300".to_string(),
            
            // Text hierarchy  
            text_primary: "gray-900".to_string(),
            text_secondary: "gray-600".to_string(),
            text_tertiary: "gray-400".to_string(),
            text_inverse: "white".to_string(),
            
            interactive_disabled: "gray-300".to_string(),
        }
    }
}

// Component migration example
pub fn migrate_mui_button(variant: &str, color: &str, theme: &impl ColorProvider) -> String {
    match (variant, color) {
        ("contained", "primary") => primary_button(theme.clone()).classes(),
        ("contained", "secondary") => secondary_button(theme.clone()).classes(),
        ("outlined", "primary") => button_styles(theme.clone())
            .variant(ButtonVariant::Ghost)
            .classes(),
        ("text", _) => button_styles(theme.clone())
            .variant(ButtonVariant::Link)
            .classes(),
        _ => button_styles(theme.clone()).classes(),
    }
}
```

### Scenario 4: Multi-Brand Theme Migration

For applications supporting multiple brands:

#### Before (Multi-brand system)
```rust
pub enum Brand {
    Acme,
    Globex,
    Initech,
}

pub struct BrandTheme {
    brand: Brand,
    colors: HashMap<String, String>,
    fonts: HashMap<String, String>,
}

impl BrandTheme {
    pub fn get_color(&self, key: &str) -> Option<&String> {
        self.colors.get(key)
    }
    
    pub fn button_style(&self) -> String {
        let bg = self.get_color("primary").unwrap_or(&"blue".to_string());
        format!("background: {}; color: white; padding: 10px 20px;", bg)
    }
}
```

#### After (Jupiter Multi-brand)
```rust
use jupiter_design_system::prelude::*;

pub trait BrandedTheme: ColorProvider + Theme {
    fn brand_id(&self) -> &str;
    fn logo_url(&self) -> &str;
}

pub struct AcmeTheme {
    palette: ColorPalette,
}

impl AcmeTheme {
    pub fn new() -> Self {
        let mut palette = ColorPalette::default();
        palette.primary = "acme-blue".to_string();
        palette.secondary = "acme-gray".to_string();
        // ... other brand colors
        Self { palette }
    }
}

impl ColorProvider for AcmeTheme {
    fn palette(&self) -> &ColorPalette { &self.palette }
}

impl Theme for AcmeTheme {
    fn name(&self) -> &str { "Acme Corporation" }
}

impl BrandedTheme for AcmeTheme {
    fn brand_id(&self) -> &str { "acme" }
    fn logo_url(&self) -> &str { "/assets/acme-logo.svg" }
}

// Factory pattern for multi-brand
pub struct ThemeFactory;

impl ThemeFactory {
    pub fn create_theme(brand: Brand) -> Box<dyn BrandedTheme> {
        match brand {
            Brand::Acme => Box::new(AcmeTheme::new()),
            Brand::Globex => Box::new(GlobexTheme::new()),
            Brand::Initech => Box::new(InitechTheme::new()),
        }
    }
}

// Usage
let brand = detect_brand_from_domain();
let theme = ThemeFactory::create_theme(brand);
let button = primary_button(theme.as_ref()).classes();
```

### Scenario 5: Server-Side Rendering Migration

For SSR applications:

#### Before (Runtime theme loading)
```javascript
// Old approach - theme loaded at runtime
const loadTheme = async (userId) => {
  const response = await fetch(`/api/user/${userId}/theme`);
  const theme = await response.json();
  applyTheme(theme);
};
```

#### After (Compile-time + runtime hybrid)
```rust
use jupiter_design_system::prelude::*;

// Static themes compiled into binary
pub struct CompiledThemes {
    default: VibeTheme,
    corporate: CorporateTheme,
    accessible: AccessibilityTheme,
}

// Dynamic theme loader for custom themes
pub struct DynamicTheme {
    palette: ColorPalette,
    metadata: ThemeMetadata,
}

impl DynamicTheme {
    pub async fn load_from_api(user_id: &str) -> Result<Self, Error> {
        let url = format!("/api/user/{}/theme", user_id);
        let response = fetch(&url).await?;
        let theme_data: ThemeData = response.json().await?;
        
        Ok(Self {
            palette: theme_data.into(),
            metadata: theme_data.metadata,
        })
    }
}

// Hybrid approach for SSR
pub struct SSRThemeProvider {
    compiled: CompiledThemes,
    dynamic_cache: Cache<String, DynamicTheme>,
}

impl SSRThemeProvider {
    pub async fn get_theme(&self, user_id: Option<&str>) -> Box<dyn ColorProvider> {
        match user_id {
            Some(id) => {
                // Try cache first
                if let Some(cached) = self.dynamic_cache.get(id) {
                    return Box::new(cached.clone());
                }
                
                // Load dynamic theme
                match DynamicTheme::load_from_api(id).await {
                    Ok(theme) => {
                        self.dynamic_cache.insert(id.to_string(), theme.clone());
                        Box::new(theme)
                    }
                    Err(_) => Box::new(self.compiled.default.clone()),
                }
            }
            None => Box::new(self.compiled.default.clone()),
        }
    }
}
```

## Breaking Changes Guide

### Future Breaking Changes (2.0.0)

When version 2.0.0 is released, expect these potential breaking changes:

#### Possible Changes
1. **New required trait methods** - May require implementing additional methods
2. **Color enum changes** - New semantic colors might be added
3. **Pattern API changes** - Method signatures might change for better ergonomics

#### Preparation for 2.0.0
```rust
// Follow these patterns to future-proof your themes:

impl ColorProvider for MyTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
    
    // Override default implementations explicitly to avoid surprises
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

// Use builder patterns defensively
let button = primary_button(theme)
    .primary_prominence()   // Use explicit methods
    .standard_interaction() // Don't rely on defaults
    .classes();
```

## Common Migration Issues

### Issue 1: Missing Interactive States
**Problem**: Old themes don't have interactive states
```rust
// Error: interactive_hover not defined
let hover_class = theme.resolve_color(Color::InteractiveHover);
```

**Solution**: Implement fallback logic
```rust
impl ColorProvider for MigratedTheme {
    fn resolve_color(&self, color: Color) -> &str {
        match color {
            Color::InteractiveHover => {
                // Fallback to primary if hover not defined
                if self.palette.interactive_hover.is_empty() {
                    &self.palette.primary
                } else {
                    &self.palette.interactive_hover
                }
            }
            _ => {
                // Use default implementation
                let palette = self.palette();
                match color {
                    Color::Primary => &palette.primary,
                    // ... other matches
                    _ => &palette.primary, // Safe fallback
                }
            }
        }
    }
}
```

### Issue 2: Different Color Naming
**Problem**: Your old theme uses different color names
```rust
// Old theme has "accent_color" but new system expects "accent"
pub struct OldColors {
    accent_color: String, // Old name
}
```

**Solution**: Create mapping layer
```rust
impl From<OldColors> for ColorPalette {
    fn from(old: OldColors) -> Self {
        ColorPalette {
            accent: old.accent_color,           // Map old name to new
            primary: old.primary_brand_color,   // Map old name to new
            secondary: old.secondary_brand_color, // Map old name to new
            // ... other mappings
            ..Default::default()
        }
    }
}
```

### Issue 3: Custom CSS Framework
**Problem**: Your theme generates custom CSS classes, not Tailwind
```rust
// Your old theme
impl OldTheme {
    fn button_class(&self) -> String {
        "my-custom-button".to_string()
    }
}
```

**Solution**: Override the CSS generation methods
```rust
impl ColorProvider for CustomFrameworkTheme {
    fn palette(&self) -> &ColorPalette { &self.palette }
    
    // Override to generate your custom classes
    fn text_class(&self, color: Color) -> String {
        format!("my-text-{}", self.resolve_color(color))
    }
    
    fn bg_class(&self, color: Color) -> String {
        format!("my-bg-{}", self.resolve_color(color))
    }
    
    fn border_class(&self, color: Color) -> String {
        format!("my-border-{}", self.resolve_color(color))
    }
}
```

## Migration Checklist

### Pre-Migration
- [ ] Audit current theme implementation
- [ ] Identify all colors used in current theme
- [ ] List any custom CSS generation logic
- [ ] Document current component usage patterns

### During Migration
- [ ] Implement `ColorProvider` trait
- [ ] Implement `Theme` trait
- [ ] Map all old colors to new `ColorPalette`
- [ ] Add missing semantic colors with reasonable defaults
- [ ] Test with existing components

### Post-Migration
- [ ] Update component usage to use new patterns
- [ ] Test accessibility with new color system
- [ ] Update documentation
- [ ] Train team on new pattern APIs

### Validation
- [ ] All components render correctly
- [ ] Color contrast ratios maintained
- [ ] Focus indicators work properly
- [ ] Theme switching works (if applicable)
- [ ] Performance is acceptable

## Getting Help

If you encounter issues during migration:

1. **Check the examples** in `docs/themes/examples.md`
2. **Review best practices** in `docs/themes/best-practices.md`
3. **Look at the default implementation** in `src/themes/mod.rs`
4. **Create a minimal reproduction** of your issue
5. **Open an issue** with your migration problem

The trait-based system is designed to be flexible and accommodate different use cases, so most migration challenges can be solved with the right approach.