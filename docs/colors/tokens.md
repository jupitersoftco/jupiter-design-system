# Color Tokens

This document provides a complete reference of all color tokens in the Jupiter Design System.

## Overview

Color tokens are semantic identifiers that represent specific colors in your design system. They are implemented as Rust enum variants in the `Color` enum, providing type safety and preventing invalid color usage.

## Token Categories

The Jupiter Design System provides **19 semantic color tokens** organized into logical categories:

> ⚠️ **Note**: The test suite in `color_test.rs` has a bug - it only tests 18 variants instead of all 19 because `Color::Accent` is missing from the test array.

### Brand Colors

Brand colors represent your product's visual identity and should be used for primary actions and brand elements.

| Token | Purpose | Default Value | CSS Classes |
|-------|---------|---------------|-------------|
| `Primary` | Primary actions, main CTAs | `jupiter-blue-500` | `text-jupiter-blue-500`, `bg-jupiter-blue-500`, `border-jupiter-blue-500` |
| `Secondary` | Secondary actions, supporting elements | `jupiter-green-500` | `text-jupiter-green-500`, `bg-jupiter-green-500`, `border-jupiter-green-500` |
| `Accent` | Highlights, accent elements | `jupiter-orange-500` | `text-jupiter-orange-500`, `bg-jupiter-orange-500`, `border-jupiter-orange-500` |

### Semantic Colors

Semantic colors convey meaning and state information to users.

| Token | Purpose | Default Value | CSS Classes |
|-------|---------|---------------|-------------|
| `Success` | Success states, confirmation | `green-500` | `text-green-500`, `bg-green-500`, `border-green-500` |
| `Warning` | Warning states, caution | `amber-500` | `text-amber-500`, `bg-amber-500`, `border-amber-500` |
| `Error` | Error states, danger | `red-500` | `text-red-500`, `bg-red-500`, `border-red-500` |
| `Info` | Informational content | `blue-500` | `text-blue-500`, `bg-blue-500`, `border-blue-500` |

### Neutral Colors

Neutral colors provide the foundation for layouts and content hierarchy.

| Token | Purpose | Default Value | CSS Classes |
|-------|---------|---------------|-------------|
| `Surface` | Card surfaces, elevated elements | `white` | `text-white`, `bg-white`, `border-white` |
| `Background` | Page backgrounds, base layer | `gray-50` | `text-gray-50`, `bg-gray-50`, `border-gray-50` |
| `Foreground` | Primary content color | `gray-900` | `text-gray-900`, `bg-gray-900`, `border-gray-900` |
| `Border` | Default border color | `gray-200` | `text-gray-200`, `bg-gray-200`, `border-gray-200` |

### Text Colors

Text colors provide hierarchy and readability for textual content.

| Token | Purpose | Default Value | CSS Classes |
|-------|---------|---------------|-------------|
| `TextPrimary` | Primary text, headlines | `gray-900` | `text-gray-900`, `bg-gray-900`, `border-gray-900` |
| `TextSecondary` | Secondary text, descriptions | `gray-600` | `text-gray-600`, `bg-gray-600`, `border-gray-600` |
| `TextTertiary` | Tertiary text, captions | `gray-400` | `text-gray-400`, `bg-gray-400`, `border-gray-400` |
| `TextInverse` | Text on dark backgrounds | `white` | `text-white`, `bg-white`, `border-white` |

### Interactive States

Interactive colors represent different states of interactive elements.

| Token | Purpose | Default Value | CSS Classes |
|-------|---------|---------------|-------------|
| `Interactive` | Default interactive elements | `jupiter-blue-500` | `text-jupiter-blue-500`, `bg-jupiter-blue-500`, `border-jupiter-blue-500` |
| `InteractiveHover` | Hover state | `jupiter-blue-600` | `text-jupiter-blue-600`, `bg-jupiter-blue-600`, `border-jupiter-blue-600` |
| `InteractiveActive` | Active/pressed state | `jupiter-blue-700` | `text-jupiter-blue-700`, `bg-jupiter-blue-700`, `border-jupiter-blue-700` |
| `InteractiveDisabled` | Disabled state | `gray-300` | `text-gray-300`, `bg-gray-300`, `border-gray-300` |

## Usage Examples

### Basic Usage

```rust
use jupiter_design_system::prelude::*;

let theme = VibeColors::default();

// Get CSS classes for different properties
let text_class = theme.text_class(Color::Primary);      // "text-jupiter-blue-500"
let bg_class = theme.bg_class(Color::Surface);          // "bg-white"
let border_class = theme.border_class(Color::Border);   // "border-gray-200"

// Resolve raw color value
let color_value = theme.resolve_color(Color::Success);  // "green-500"
```

### In Component Building

```rust
use jupiter_design_system::patterns::*;

let theme = VibeColors::default();

// Button using semantic colors
let primary_button = Button::new("Primary Action")
    .variant(ButtonVariant::Primary)  // Uses Color::Primary
    .build(&theme);

// Card with semantic styling
let card = Card::new()
    .add_class(&theme.bg_class(Color::Surface))      // Surface background
    .add_class(&theme.text_class(Color::TextPrimary)) // Primary text
    .add_class(&theme.border_class(Color::Border))   // Border color
    .build(&theme);
```

### Custom Color Resolution

```rust
use jupiter_design_system::core::*;

// Get all available color variants
let all_colors = [
    Color::Primary, Color::Secondary, Color::Accent,
    Color::Success, Color::Warning, Color::Error, Color::Info,
    Color::Surface, Color::Background, Color::Foreground, Color::Border,
    Color::TextPrimary, Color::TextSecondary, Color::TextTertiary, Color::TextInverse,
    Color::Interactive, Color::InteractiveHover, Color::InteractiveActive, Color::InteractiveDisabled,
];

// Generate CSS classes for all colors
for color in all_colors {
    let text = theme.text_class(color);
    let bg = theme.bg_class(color);
    let border = theme.border_class(color);
    println!("{:?}: text={}, bg={}, border={}", color, text, bg, border);
}
```

## Best Practices

### Do's

✅ **Use semantic tokens**: `Color::Primary` instead of specific values  
✅ **Consistent context**: Use `Color::Success` for all success states  
✅ **Type safety**: Let Rust catch invalid color usage at compile time  
✅ **Provider methods**: Use `text_class()`, `bg_class()`, `border_class()`  

### Don'ts

❌ **Don't hardcode values**: Avoid `"text-blue-500"` directly  
❌ **Don't mix contexts**: Don't use `Color::Error` for non-error states  
❌ **Don't bypass provider**: Avoid manual CSS class construction  
❌ **Don't assume values**: Token values may change between themes  

## Complete Token Reference

All 19 color tokens in order:

```rust
pub enum Color {
    // Brand colors (3)
    Primary, Secondary, Accent,
    
    // Semantic colors (4) 
    Success, Warning, Error, Info,
    
    // Neutral colors (4)
    Surface, Background, Foreground, Border,
    
    // Text colors (4)
    TextPrimary, TextSecondary, TextTertiary, TextInverse,
    
    // Interactive states (4)
    Interactive, InteractiveHover, InteractiveActive, InteractiveDisabled,
}
```

## Token Stability

All color tokens are considered **stable** and safe for production use. New tokens may be added in minor versions, but existing tokens will not be removed or significantly changed without a major version bump.