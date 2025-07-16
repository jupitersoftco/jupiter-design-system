# Core Traits Reference

This document details the essential traits in Jupiter Design System and their purposes.

## Primary System Traits

The Jupiter Design System is built around five core traits that define different aspects of design tokens:

- **ColorProvider** - Color palette and semantic color resolution ✅ *Fully integrated*
- **SizeProvider** - Component sizing and responsive breakpoints 🚧 *Defined but not yet integrated*
- **SpacingProvider** - Padding, margin, and layout spacing 🚧 *Defined but not yet integrated*
- **TypographyProvider** - Text styling, weights, and font families 🚧 *Defined but not yet integrated*
- **Theme** - Theme identity and metadata ✅ *Fully integrated*

> **Current Status**: The system currently focuses on `ColorProvider` for theming, with comprehensive color-based patterns and components. The other provider traits are defined for future expansion but are not yet required for theme implementation.

## ColorProvider Trait

The `ColorProvider` trait is the foundation of the theme system, defining how colors are resolved and applied.

### Definition

```rust
pub trait ColorProvider {
    /// Get the color palette for this provider
    fn palette(&self) -> &ColorPalette;

    /// Resolve a semantic color to its CSS class or value
    fn resolve_color(&self, color: Color) -> &str;

    /// Get a Tailwind CSS class for text color
    fn text_class(&self, color: Color) -> String;

    /// Get a Tailwind CSS class for background color
    fn bg_class(&self, color: Color) -> String;

    /// Get a Tailwind CSS class for border color
    fn border_class(&self, color: Color) -> String;
}
```

### Purpose

- **Semantic Color Resolution**: Maps abstract color tokens to concrete CSS classes
- **Framework Agnostic**: Works with any CSS framework (Tailwind, custom CSS, etc.)
- **Theme Consistency**: Ensures all components use the same color values

### Default Implementation

The trait provides default implementations for the CSS class methods:

```rust
fn text_class(&self, color: Color) -> String {
    format!("text-{}", self.resolve_color(color))
}

fn bg_class(&self, color: Color) -> String {
    format!("bg-{}", self.resolve_color(color))
}

fn border_class(&self, color: Color) -> String {
    format!("border-{}", self.resolve_color(color))
}
```

### Example Implementation

```rust
use jupiter_design_system::core::color::{ColorProvider, ColorPalette};

pub struct MyTheme {
    palette: ColorPalette,
}

impl ColorProvider for MyTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

// text_class, bg_class, and border_class are automatically implemented
```

## Theme Trait

The `Theme` trait provides identity and metadata for theme implementations.

### Definition

```rust
pub trait Theme {
    /// Get theme name
    fn name(&self) -> &str;
}
```

### Purpose

- **Theme Identity**: Provides a human-readable name for the theme
- **Theme Selection**: Enables runtime theme switching
- **Debugging**: Helps identify which theme is active

### Example Implementation

```rust
use jupiter_design_system::themes::Theme;

pub struct MyCustomTheme {
    // theme data
}

impl Theme for MyCustomTheme {
    fn name(&self) -> &str {
        "My Custom Theme"
    }
}
```

## Color Enum

The `Color` enum defines all semantic color tokens in the system.

### Brand Colors

```rust
Color::Primary     // Main brand color
Color::Secondary   // Secondary brand color  
Color::Accent      // Accent/highlight color
```

### Semantic Colors

```rust
Color::Success     // Success states (green)
Color::Warning     // Warning states (yellow/orange)
Color::Error       // Error states (red)
Color::Info        // Informational states (blue)
```

### Surface Colors

```rust
Color::Surface     // Component backgrounds
Color::Background  // Page/app background
Color::Foreground  // High contrast elements
Color::Border      // Component borders
```

### Text Colors

```rust
Color::TextPrimary    // High emphasis text
Color::TextSecondary  // Medium emphasis text
Color::TextTertiary   // Low emphasis text
Color::TextInverse    // Text on dark backgrounds
```

### Interactive States

```rust
Color::Interactive         // Default interactive color
Color::InteractiveHover    // Hover state color
Color::InteractiveActive   // Active/pressed state color
Color::InteractiveDisabled // Disabled state color
```

## ColorPalette Struct

The `ColorPalette` struct contains the concrete color values for a theme.

### Structure

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
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

    // Surface colors
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

### Usage

```rust
let palette = ColorPalette {
    primary: "blue-600".to_string(),
    secondary: "green-500".to_string(),
    // ... other colors
};
```

## Pattern Traits (Advanced)

For advanced theme customization, you can implement pattern-specific traits:

### ActionSemantics Pattern

Defines how actions are styled based on their semantic meaning:

```rust
// Used internally by ActionSemantics<T: ColorProvider>
pub enum ActionIntent {
    Primary,      // Main action
    Secondary,    // Alternative action
    Destructive,  // Delete/remove actions
    Navigation,   // Menu items, links
}
```

### Interactive Element Pattern

Defines how elements respond to user interaction:

```rust
// Used internally by InteractiveElement<T: ColorProvider>
pub enum InteractiveState {
    Default,  // Normal state
    Hover,    // Mouse hover
    Active,   // Pressed/clicked
    Focused,  // Keyboard focus
    Disabled, // Non-interactive
}
```

### Typography Pattern

Defines text hierarchy and styling:

```rust
// Used internally by TypographyPattern<T: ColorProvider>
pub enum TypographyHierarchy {
    Title,        // Page title (h1)
    Heading,      // Section heading (h2)
    Subheading,   // Subsection (h3)
    Body,         // Regular text
    Caption,      // Small descriptive text
}
```

## Trait Composition

Traits are designed to work together through composition:

```rust
use jupiter_design_system::prelude::*;

// A complete theme implements multiple traits
#[derive(Debug, Clone)]
pub struct MyTheme {
    colors: ColorPalette,
}

impl ColorProvider for MyTheme {
    fn palette(&self) -> &ColorPalette {
        &self.colors
    }
}

impl Theme for MyTheme {
    fn name(&self) -> &str {
        "My Custom Theme"
    }
}

// Use with any pattern that accepts ColorProvider
let button = primary_button(MyTheme::new()).classes();
let text = title_typography(MyTheme::new()).classes();
```

## SizeProvider Trait

The `SizeProvider` trait manages component sizing and responsive breakpoints.

### Definition

```rust
pub trait SizeProvider {
    /// Resolve size to CSS class value
    fn resolve_size(&self, size: Size) -> &str;

    /// Get width class
    fn width_class(&self, size: Size) -> String;

    /// Get height class  
    fn height_class(&self, size: Size) -> String;
}
```

### Size Enum

```rust
#[derive(Debug, Clone, Copy)]
pub enum Size {
    XSmall,   // Extra small components
    Small,    // Small components
    Medium,   // Default/medium size
    Large,    // Large components
    XLarge,   // Extra large components
}
```

### Breakpoint Enum

```rust
#[derive(Debug, Clone, Copy)]
pub enum Breakpoint {
    Mobile,   // Mobile devices
    Tablet,   // Tablet devices
    Desktop,  // Desktop screens
    Large,    // Large desktop screens
}
```

### Example Implementation

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

## SpacingProvider Trait

The `SpacingProvider` trait manages consistent spacing throughout the design system.

### Definition

```rust
pub trait SpacingProvider {
    /// Resolve spacing to CSS class
    fn resolve_spacing(&self, spacing: Spacing) -> &str;

    /// Get padding class
    fn padding_class(&self, spacing: Spacing) -> String;

    /// Get margin class
    fn margin_class(&self, spacing: Spacing) -> String;
}
```

### Spacing Enum

```rust
#[derive(Debug, Clone, Copy)]
pub enum Spacing {
    None,     // No spacing (0)
    XSmall,   // Extra small spacing
    Small,    // Small spacing
    Medium,   // Medium spacing (default)
    Large,    // Large spacing
    XLarge,   // Extra large spacing
    XXLarge,  // Extra extra large spacing
}
```

### Example Implementation

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

## TypographyProvider Trait

The `TypographyProvider` trait manages text styling, font weights, and font families.

### Definition

```rust
pub trait TypographyProvider {
    /// Resolve typography to CSS class
    fn resolve_typography(&self, typography: Typography) -> &str;

    /// Get typography class
    fn typography_class(&self, typography: Typography) -> String;

    /// Get font weight class
    fn font_weight_class(&self, weight: FontWeight) -> String;

    /// Get font family class
    fn font_family_class(&self, family: FontFamily) -> String;
}
```

### Typography Hierarchy

```rust
#[derive(Debug, Clone, Copy)]
pub enum Typography {
    Heading1,   // Main page title
    Heading2,   // Section heading
    Heading3,   // Subsection heading
    Heading4,   // Minor heading
    Heading5,   // Small heading
    Heading6,   // Smallest heading
    Body,       // Regular body text
    BodySmall,  // Small body text
    Caption,    // Caption text
    Label,      // Form labels
}
```

### Font Weight System

```rust
#[derive(Debug, Clone, Copy)]
pub enum FontWeight {
    Light,     // font-light (300)
    Normal,    // font-normal (400)
    Medium,    // font-medium (500)
    SemiBold,  // font-semibold (600)
    Bold,      // font-bold (700)
}
```

### Font Family System

```rust
#[derive(Debug, Clone, Copy)]
pub enum FontFamily {
    Sans,   // Sans-serif fonts
    Serif,  // Serif fonts
    Mono,   // Monospace fonts
}
```

### Example Implementation

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

This composition approach allows you to:

- **Implement only what you need** - Start with `ColorProvider` and add more as needed
- **Mix and match** - Use different providers for different patterns if needed
- **Extend gradually** - Add new traits without breaking existing functionality

## Complete Theme Implementation

A complete theme typically implements multiple traits:

```rust
#[derive(Debug, Clone)]
pub struct CompleteTheme {
    colors: ColorPalette,
    name: String,
}

impl ColorProvider for CompleteTheme {
    fn palette(&self) -> &ColorPalette { &self.colors }
}

impl SizeProvider for CompleteTheme {
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

impl SpacingProvider for CompleteTheme {
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

impl TypographyProvider for CompleteTheme {
    fn resolve_typography(&self, typography: Typography) -> &str {
        match typography {
            Typography::Heading1 => "4xl",
            Typography::Body => "base",
            Typography::Caption => "xs",
            // ... other mappings
        }
    }
}

impl Theme for CompleteTheme {
    fn name(&self) -> &str { &self.name }
}
```