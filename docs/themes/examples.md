# Theme Implementation Examples

This document provides complete, working examples of theme implementations for different use cases.

## Current Implementation Status

> **Note**: The examples below focus on `ColorProvider` and `Theme` traits, which are the core traits currently integrated throughout the system. Additional provider traits (`SizeProvider`, `SpacingProvider`, `TypographyProvider`) are defined but not yet required for theme implementation.

## Example 1: Minimal Custom Theme

A simple theme that customizes just the primary color:

```rust
use jupiter_design_system::prelude::*;

#[derive(Debug, Clone)]
pub struct MinimalTheme {
    palette: ColorPalette,
}

impl MinimalTheme {
    pub fn new() -> Self {
        let mut palette = ColorPalette::default();
        palette.primary = "purple-600".to_string();
        palette.interactive = "purple-600".to_string();
        palette.interactive_hover = "purple-700".to_string();
        palette.interactive_active = "purple-800".to_string();
        
        Self { palette }
    }
}

impl ColorProvider for MinimalTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

impl Theme for MinimalTheme {
    fn name(&self) -> &str {
        "Minimal Purple"
    }
}

// Usage
fn example_usage() {
    let theme = MinimalTheme::new();
    
    let button = primary_button(theme.clone()).classes();
    // Result: "bg-purple-600 text-white px-6 py-3 rounded-md font-semibold hover:bg-purple-700"
    
    let title = title_typography(theme)
        .color(TypographyColor::Primary)
        .classes();
    // Result: "text-purple-600 text-4xl font-bold tracking-tight leading-relaxed"
}
```

## Example 2: Corporate Brand Theme

A complete corporate theme with custom color palette and branding:

```rust
use jupiter_design_system::prelude::*;

#[derive(Debug, Clone)]
pub struct CorporateTheme {
    palette: ColorPalette,
    brand_name: String,
}

impl CorporateTheme {
    pub fn new(brand_name: impl Into<String>) -> Self {
        Self {
            palette: Self::create_corporate_palette(),
            brand_name: brand_name.into(),
        }
    }
    
    fn create_corporate_palette() -> ColorPalette {
        ColorPalette {
            // Corporate brand colors
            primary: "corporate-blue-600".to_string(),
            secondary: "corporate-gray-500".to_string(),
            accent: "corporate-orange-500".to_string(),

            // Professional semantic colors
            success: "emerald-600".to_string(),
            warning: "amber-600".to_string(),
            error: "rose-600".to_string(),
            info: "sky-600".to_string(),

            // Clean corporate surfaces
            surface: "white".to_string(),
            background: "slate-50".to_string(),
            foreground: "slate-900".to_string(),
            border: "slate-200".to_string(),

            // Professional text hierarchy
            text_primary: "slate-900".to_string(),
            text_secondary: "slate-600".to_string(),
            text_tertiary: "slate-400".to_string(),
            text_inverse: "white".to_string(),

            // Corporate interactive states
            interactive: "corporate-blue-600".to_string(),
            interactive_hover: "corporate-blue-700".to_string(),
            interactive_active: "corporate-blue-800".to_string(),
            interactive_disabled: "slate-300".to_string(),
        }
    }
    
    /// Create dark mode variant for executive dashboards
    pub fn executive_dark() -> Self {
        let mut palette = Self::create_corporate_palette();
        
        // Dark executive theme
        palette.surface = "slate-800".to_string();
        palette.background = "slate-900".to_string();
        palette.foreground = "white".to_string();
        palette.border = "slate-700".to_string();
        
        palette.text_primary = "slate-100".to_string();
        palette.text_secondary = "slate-300".to_string();
        palette.text_tertiary = "slate-500".to_string();
        palette.text_inverse = "slate-900".to_string();
        
        // Brighter colors for dark background
        palette.primary = "corporate-blue-400".to_string();
        palette.interactive = "corporate-blue-400".to_string();
        palette.interactive_hover = "corporate-blue-300".to_string();
        palette.interactive_active = "corporate-blue-500".to_string();
        
        Self { 
            palette,
            brand_name: "Corporate Executive".to_string(),
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
        &self.brand_name
    }
}

// Usage examples
fn corporate_examples() {
    let theme = CorporateTheme::new("Acme Corp");
    
    // Executive dashboard button
    let cta_button = hero_button(theme.clone())
        .urgent()
        .classes();
    
    // Form submission button
    let submit = primary_button(theme.clone())
        .form_context()
        .classes();
    
    // Secondary action button
    let cancel = secondary_button(theme.clone())
        .form_context()
        .classes();
    
    // Corporate page title
    let page_title = title_typography(theme.clone())
        .color(TypographyColor::Primary)
        .alignment(TypographyAlignment::Center)
        .classes();
    
    // Executive dark theme
    let dark_theme = CorporateTheme::executive_dark();
    let dark_button = primary_button(dark_theme).classes();
}
```

## Example 3: Gaming Theme

A vibrant gaming theme with custom visual effects:

```rust
use jupiter_design_system::prelude::*;

#[derive(Debug, Clone)]
pub struct GamingTheme {
    palette: ColorPalette,
    intensity: GamingIntensity,
}

#[derive(Debug, Clone)]
pub enum GamingIntensity {
    Subtle,
    Standard, 
    Extreme,
}

impl GamingTheme {
    pub fn new(intensity: GamingIntensity) -> Self {
        Self {
            palette: Self::create_gaming_palette(&intensity),
            intensity,
        }
    }
    
    fn create_gaming_palette(intensity: &GamingIntensity) -> ColorPalette {
        ColorPalette {
            // Vibrant gaming colors
            primary: "neon-cyan-500".to_string(),
            secondary: "neon-purple-500".to_string(),
            accent: "neon-pink-500".to_string(),

            // Gaming semantic colors
            success: "neon-green-500".to_string(),
            warning: "neon-yellow-500".to_string(),
            error: "neon-red-500".to_string(),
            info: "neon-blue-500".to_string(),

            // Dark gaming surfaces
            surface: match intensity {
                GamingIntensity::Subtle => "gray-800".to_string(),
                GamingIntensity::Standard => "slate-900".to_string(),
                GamingIntensity::Extreme => "black".to_string(),
            },
            background: "black".to_string(),
            foreground: "neon-cyan-400".to_string(),
            border: "neon-cyan-600".to_string(),

            // High contrast gaming text
            text_primary: "neon-cyan-100".to_string(),
            text_secondary: "neon-cyan-300".to_string(),
            text_tertiary: "neon-cyan-500".to_string(),
            text_inverse: "black".to_string(),

            // Glowing interactive states
            interactive: "neon-cyan-500".to_string(),
            interactive_hover: "neon-cyan-400".to_string(),
            interactive_active: "neon-cyan-600".to_string(),
            interactive_disabled: "gray-600".to_string(),
        }
    }
}

impl ColorProvider for GamingTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
    
    // Override for gaming-specific glow effects
    fn bg_class(&self, color: Color) -> String {
        let base_class = format!("bg-{}", self.resolve_color(color));
        
        match self.intensity {
            GamingIntensity::Extreme => {
                format!("{} shadow-lg shadow-{}/50 glow-effect", 
                    base_class, self.resolve_color(color))
            },
            GamingIntensity::Standard => {
                format!("{} shadow-md shadow-{}/30", 
                    base_class, self.resolve_color(color))
            },
            GamingIntensity::Subtle => base_class,
        }
    }
}

impl Theme for GamingTheme {
    fn name(&self) -> &str {
        match self.intensity {
            GamingIntensity::Subtle => "Gaming Subtle",
            GamingIntensity::Standard => "Gaming Standard",
            GamingIntensity::Extreme => "Gaming Extreme",
        }
    }
}

// Gaming-specific component patterns
impl GamingTheme {
    pub fn power_button(&self) -> String {
        hero_button(self.clone())
            .prominent_interaction()
            .custom("animate-pulse-glow font-bold text-lg")
            .classes()
    }
    
    pub fn level_up_button(&self) -> String {
        primary_button(self.clone())
            .custom("animate-bounce-subtle transform hover:scale-105")
            .classes()
    }
    
    pub fn achievement_text(&self) -> String {
        title_typography(self.clone())
            .color(TypographyColor::Accent)
            .custom("animate-fade-in text-shadow-glow")
            .classes()
    }
}

// Usage
fn gaming_examples() {
    let gaming = GamingTheme::new(GamingIntensity::Standard);
    
    // Main game action button
    let play_button = gaming.power_button();
    
    // Achievement notification
    let achievement = gaming.achievement_text();
    
    // Menu navigation
    let menu_item = navigation_button(gaming.clone())
        .gentle_interaction()
        .custom("hover:text-neon-cyan-300")
        .classes();
}
```

## Example 4: E-commerce Theme

A conversion-focused e-commerce theme:

```rust
use jupiter_design_system::prelude::*;

#[derive(Debug, Clone)]
pub struct EcommerceTheme {
    palette: ColorPalette,
    conversion_mode: ConversionMode,
}

#[derive(Debug, Clone)]
pub enum ConversionMode {
    Standard,
    HighConversion, // Orange CTAs for better conversion
    TrustFocused,   // Blue/green for trust
}

impl EcommerceTheme {
    pub fn new(mode: ConversionMode) -> Self {
        Self {
            palette: Self::create_ecommerce_palette(&mode),
            conversion_mode: mode,
        }
    }
    
    fn create_ecommerce_palette(mode: &ConversionMode) -> ColorPalette {
        let (primary, accent) = match mode {
            ConversionMode::Standard => ("blue-600".to_string(), "blue-500".to_string()),
            ConversionMode::HighConversion => ("orange-600".to_string(), "orange-500".to_string()),
            ConversionMode::TrustFocused => ("emerald-600".to_string(), "emerald-500".to_string()),
        };
        
        ColorPalette {
            primary: primary.clone(),
            secondary: "gray-600".to_string(),
            accent,

            // E-commerce semantic colors
            success: "emerald-600".to_string(), // Purchase success
            warning: "amber-600".to_string(),   // Low stock warnings
            error: "red-600".to_string(),       // Out of stock
            info: "blue-600".to_string(),       // Product info

            // Clean shopping surfaces
            surface: "white".to_string(),
            background: "gray-50".to_string(),
            foreground: "gray-900".to_string(),
            border: "gray-200".to_string(),

            // E-commerce text hierarchy
            text_primary: "gray-900".to_string(),
            text_secondary: "gray-600".to_string(),
            text_tertiary: "gray-500".to_string(),
            text_inverse: "white".to_string(),

            // Conversion-optimized interactive states
            interactive: primary.clone(),
            interactive_hover: match mode {
                ConversionMode::Standard => "blue-700".to_string(),
                ConversionMode::HighConversion => "orange-700".to_string(),
                ConversionMode::TrustFocused => "emerald-700".to_string(),
            },
            interactive_active: match mode {
                ConversionMode::Standard => "blue-800".to_string(),
                ConversionMode::HighConversion => "orange-800".to_string(),
                ConversionMode::TrustFocused => "emerald-800".to_string(),
            },
            interactive_disabled: "gray-300".to_string(),
        }
    }
}

impl ColorProvider for EcommerceTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

impl Theme for EcommerceTheme {
    fn name(&self) -> &str {
        match self.conversion_mode {
            ConversionMode::Standard => "E-commerce Standard",
            ConversionMode::HighConversion => "E-commerce High Conversion",
            ConversionMode::TrustFocused => "E-commerce Trust Focused",
        }
    }
}

// E-commerce specific patterns
impl EcommerceTheme {
    pub fn add_to_cart_button(&self) -> String {
        primary_button(self.clone())
            .prominent_interaction()
            .custom("font-semibold tracking-wide")
            .classes()
    }
    
    pub fn buy_now_button(&self) -> String {
        hero_button(self.clone())
            .urgent()
            .custom("text-lg font-bold animate-subtle-pulse")
            .classes()
    }
    
    pub fn price_text(&self) -> String {
        body_typography(self.clone())
            .size(TypographySize::LG)
            .weight(TypographyWeight::Bold)
            .color(TypographyColor::Primary)
            .classes()
    }
    
    pub fn sale_badge(&self) -> String {
        format!(
            "{} {} px-3 py-1 rounded-full text-sm font-bold uppercase tracking-wide",
            self.bg_class(Color::Error),
            self.text_class(Color::TextInverse)
        )
    }
    
    pub fn trust_indicator(&self) -> String {
        format!(
            "{} {} px-2 py-1 rounded text-sm font-medium",
            self.bg_class(Color::Success),
            self.text_class(Color::TextInverse)
        )
    }
}

// Usage
fn ecommerce_examples() {
    let store = EcommerceTheme::new(ConversionMode::HighConversion);
    
    // Product page buttons
    let add_to_cart = store.add_to_cart_button();
    let buy_now = store.buy_now_button();
    
    // Product information
    let price = store.price_text();
    let sale_badge = store.sale_badge();
    let trust_badge = store.trust_indicator();
    
    // Navigation
    let category_link = navigation_button(store.clone())
        .tertiary_prominence()
        .classes();
    
    // Trust-focused variant for checkout
    let checkout_theme = EcommerceTheme::new(ConversionMode::TrustFocused);
    let secure_checkout = primary_button(checkout_theme)
        .form_context()
        .custom("w-full font-semibold")
        .classes();
}
```

## Example 5: Accessibility-First Theme

A theme designed specifically for maximum accessibility:

```rust
use jupiter_design_system::prelude::*;

#[derive(Debug, Clone)]
pub struct AccessibilityTheme {
    palette: ColorPalette,
    contrast_level: ContrastLevel,
}

#[derive(Debug, Clone)]
pub enum ContrastLevel {
    AA,   // WCAG AA compliance (4.5:1 for normal text)
    AAA,  // WCAG AAA compliance (7:1 for normal text)
}

impl AccessibilityTheme {
    pub fn new(contrast_level: ContrastLevel) -> Self {
        Self {
            palette: Self::create_accessible_palette(&contrast_level),
            contrast_level,
        }
    }
    
    fn create_accessible_palette(level: &ContrastLevel) -> ColorPalette {
        match level {
            ContrastLevel::AA => ColorPalette {
                // WCAG AA compliant colors
                primary: "blue-700".to_string(),        // 4.5:1 on white
                secondary: "gray-700".to_string(),      // 4.5:1 on white
                accent: "purple-700".to_string(),       // 4.5:1 on white

                success: "green-700".to_string(),       // 4.5:1 on white
                warning: "yellow-800".to_string(),      // 4.5:1 on white
                error: "red-700".to_string(),           // 4.5:1 on white
                info: "blue-700".to_string(),           // 4.5:1 on white

                surface: "white".to_string(),
                background: "white".to_string(),
                foreground: "black".to_string(),
                border: "gray-400".to_string(),         // 3:1 on white

                text_primary: "black".to_string(),      // 21:1 on white
                text_secondary: "gray-700".to_string(), // 4.5:1 on white
                text_tertiary: "gray-600".to_string(),  // 4.5:1 on white (large text)
                text_inverse: "white".to_string(),

                interactive: "blue-700".to_string(),
                interactive_hover: "blue-800".to_string(),
                interactive_active: "blue-900".to_string(),
                interactive_disabled: "gray-400".to_string(),
            },
            ContrastLevel::AAA => ColorPalette {
                // WCAG AAA compliant colors - much higher contrast
                primary: "blue-800".to_string(),        // 7:1 on white
                secondary: "gray-800".to_string(),      // 7:1 on white
                accent: "purple-800".to_string(),       // 7:1 on white

                success: "green-800".to_string(),       // 7:1 on white
                warning: "yellow-900".to_string(),      // 7:1 on white
                error: "red-800".to_string(),           // 7:1 on white
                info: "blue-800".to_string(),           // 7:1 on white

                surface: "white".to_string(),
                background: "white".to_string(),
                foreground: "black".to_string(),
                border: "gray-600".to_string(),         // Higher contrast borders

                text_primary: "black".to_string(),      // 21:1 on white
                text_secondary: "gray-800".to_string(), // 7:1 on white
                text_tertiary: "gray-700".to_string(),  // 7:1 on white
                text_inverse: "white".to_string(),

                interactive: "blue-800".to_string(),
                interactive_hover: "blue-900".to_string(),
                interactive_active: "black".to_string(),
                interactive_disabled: "gray-500".to_string(),
            },
        }
    }
}

impl ColorProvider for AccessibilityTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
    
    // Enhanced focus indicators for accessibility
    fn text_class(&self, color: Color) -> String {
        let base = format!("text-{}", self.resolve_color(color));
        // Add underline for better text link accessibility
        if matches!(color, Color::Interactive | Color::Primary) {
            format!("{} underline underline-offset-2", base)
        } else {
            base
        }
    }
}

impl Theme for AccessibilityTheme {
    fn name(&self) -> &str {
        match self.contrast_level {
            ContrastLevel::AA => "Accessibility AA",
            ContrastLevel::AAA => "Accessibility AAA",
        }
    }
}

// Accessibility-enhanced patterns
impl AccessibilityTheme {
    pub fn accessible_button(&self) -> String {
        primary_button(self.clone())
            .prominent_focus()
            .custom("min-h-44px min-w-44px") // WCAG minimum touch target
            .classes()
    }
    
    pub fn high_contrast_text(&self) -> String {
        body_typography(self.clone())
            .color(TypographyColor::Primary)
            .weight(TypographyWeight::Medium) // Slightly bolder for better readability
            .custom("leading-loose") // More line spacing
            .classes()
    }
    
    pub fn keyboard_navigation_item(&self) -> String {
        navigation_button(self.clone())
            .prominent_focus()
            .custom("focus:outline-4 focus:outline-offset-4")
            .classes()
    }
}

// Usage
fn accessibility_examples() {
    let theme_aa = AccessibilityTheme::new(ContrastLevel::AA);
    let theme_aaa = AccessibilityTheme::new(ContrastLevel::AAA);
    
    // Accessible form button
    let submit = theme_aa.accessible_button();
    
    // High contrast text for important information
    let important_text = theme_aaa.high_contrast_text();
    
    // Keyboard navigation
    let nav_item = theme_aa.keyboard_navigation_item();
    
    // Accessible link
    let link = button_link(theme_aa.clone())
        .custom("font-medium")
        .classes();
}
```

## Using Themes in Applications

### React Example

```jsx
import { useState } from 'react';

// Theme context
const ThemeContext = React.createContext();

function App() {
    const [currentTheme, setCurrentTheme] = useState('corporate');
    
    const themes = {
        minimal: new MinimalTheme(),
        corporate: new CorporateTheme('My Company'),
        gaming: new GamingTheme(GamingIntensity.Standard),
        ecommerce: new EcommerceTheme(ConversionMode.HighConversion),
        accessible: new AccessibilityTheme(ContrastLevel.AA),
    };
    
    return (
        <ThemeContext.Provider value={themes[currentTheme]}>
            <div className={themes[currentTheme].bg_class('Background')}>
                <Header />
                <MainContent />
                <Footer />
            </div>
        </ThemeContext.Provider>
    );
}

function Button({ children, variant = 'primary' }) {
    const theme = useContext(ThemeContext);
    
    const classes = variant === 'primary' 
        ? primary_button(theme).classes()
        : secondary_button(theme).classes();
    
    return (
        <button className={classes}>
            {children}
        </button>
    );
}
```

### Framework Integration

```rust
// Integration with web frameworks
pub trait WebFramework {
    fn button_component<T: ColorProvider + Clone>(
        theme: T,
        variant: ButtonVariant,
        children: &str
    ) -> String;
}

pub struct DioxusFramework;

impl WebFramework for DioxusFramework {
    fn button_component<T: ColorProvider + Clone>(
        theme: T,
        variant: ButtonVariant,
        children: &str
    ) -> String {
        let classes = match variant {
            ButtonVariant::Primary => primary_button(theme).classes(),
            ButtonVariant::Secondary => secondary_button(theme).classes(),
            ButtonVariant::Hero => hero_button(theme).classes(),
        };
        
        format!(r#"button {{ class: "{}", "{}" }}"#, classes, children)
    }
}
```

These examples demonstrate the flexibility and power of the Jupiter Design System's trait-based theme architecture, showing how to create themes for different use cases while maintaining consistency and accessibility.