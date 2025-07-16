# Color Palettes & Scales

This document describes the color palette structure and scales used in the Jupiter Design System.

## Overview

The Jupiter Design System uses a structured approach to color palettes, mapping semantic tokens to concrete color values. The system is built on Tailwind CSS color scales, with custom Jupiter brand colors.

## Palette Structure

### ColorPalette Struct

The `ColorPalette` struct is the core data structure that maps semantic color tokens to actual color values:

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

## Default Palette (Jupiter Theme)

The default Jupiter theme implements a vibrant, psychedelic color scheme:

### Brand Colors

| Token | Value | Description |
|-------|-------|-------------|
| `primary` | `jupiter-blue-500` | Jupiter custom blue - primary brand color |
| `secondary` | `jupiter-green-500` | Jupiter custom green - secondary brand color |
| `accent` | `jupiter-orange-500` | Jupiter custom orange - accent color |

### Semantic Colors

Built on standard Tailwind CSS colors for consistency:

| Token | Value | Hex Code | Usage |
|-------|-------|----------|-------|
| `success` | `green-500` | `#10b981` | Success states, confirmations |
| `warning` | `amber-500` | `#f59e0b` | Warning states, cautions |
| `error` | `red-500` | `#ef4444` | Error states, danger |
| `info` | `blue-500` | `#3b82f6` | Informational content |

### Neutral Scale

Neutral colors provide the foundation for layouts:

| Token | Value | Hex Code | Usage |
|-------|-------|----------|-------|
| `surface` | `white` | `#ffffff` | Card surfaces, elevated elements |
| `background` | `gray-50` | `#f9fafb` | Page backgrounds |
| `foreground` | `gray-900` | `#111827` | Primary content |
| `border` | `gray-200` | `#e5e7eb` | Default borders |

### Text Hierarchy

Text colors provide content hierarchy:

| Token | Value | Hex Code | Usage |
|-------|-------|----------|-------|
| `text_primary` | `gray-900` | `#111827` | Headlines, primary text |
| `text_secondary` | `gray-600` | `#4b5563` | Body text, descriptions |
| `text_tertiary` | `gray-400` | `#9ca3af` | Captions, metadata |
| `text_inverse` | `white` | `#ffffff` | Text on dark backgrounds |

### Interactive States

Interactive colors for UI elements:

| Token | Value | Hex Code | Usage |
|-------|-------|----------|-------|
| `interactive` | `jupiter-blue-500` | `#3b82f6` | Default interactive elements |
| `interactive_hover` | `jupiter-blue-600` | `#2563eb` | Hover states |
| `interactive_active` | `jupiter-blue-700` | `#1d4ed8` | Active/pressed states |
| `interactive_disabled` | `gray-300` | `#d1d5db` | Disabled elements |

## Custom Jupiter Colors

The Jupiter brand uses **4 custom color families** that extend Tailwind's palette:

### Jupiter Navy Scale

Used for branded surfaces and dark gradients:

| Scale | Value | Usage |
|-------|-------|-------|
| `jupiter-navy-50` | Very light navy | Backgrounds |
| `jupiter-navy-100` | Light navy | Subtle backgrounds |
| `jupiter-navy-200` | Lighter navy | Borders |
| `jupiter-navy-300` | Light navy | Disabled states |
| `jupiter-navy-400` | Medium-light navy | Secondary elements |
| `jupiter-navy-500` | Primary navy | Navy brand color |
| `jupiter-navy-600` | Medium-dark navy | Hover states |
| `jupiter-navy-700` | Dark navy | Active states |
| `jupiter-navy-800` | Darker navy | High contrast |
| `jupiter-navy-900` | **Darkest navy** | Branded gradients |

### Jupiter Blue Scale

| Scale | Value | Hex Code | Usage |
|-------|-------|----------|-------|
| `jupiter-blue-50` | `#eff6ff` | Very light blue | Backgrounds |
| `jupiter-blue-100` | `#dbeafe` | Light blue | Subtle backgrounds |
| `jupiter-blue-200` | `#bfdbfe` | Lighter blue | Borders, dividers |
| `jupiter-blue-300` | `#93c5fd` | Light blue | Disabled states |
| `jupiter-blue-400` | `#60a5fa` | Medium-light blue | Secondary elements |
| `jupiter-blue-500` | `#3b82f6` | **Primary blue** | Primary actions |
| `jupiter-blue-600` | `#2563eb` | Medium-dark blue | Hover states |
| `jupiter-blue-700` | `#1d4ed8` | Dark blue | Active states |
| `jupiter-blue-800` | `#1e40af` | Darker blue | High contrast |
| `jupiter-blue-900` | `#1e3a8a` | Darkest blue | **Branded gradients** |

### Jupiter Green Scale

| Scale | Value | Hex Code | Usage |
|-------|-------|----------|-------|
| `jupiter-green-50` | `#f0fdf4` | Very light green | Backgrounds |
| `jupiter-green-100` | `#dcfce7` | Light green | Subtle backgrounds |
| `jupiter-green-200` | `#bbf7d0` | Lighter green | Borders |
| `jupiter-green-300` | `#86efac` | Light green | Disabled states |
| `jupiter-green-400` | `#4ade80` | Medium-light green | Secondary elements |
| `jupiter-green-500` | `#22c55e` | **Primary green** | Secondary brand |
| `jupiter-green-600` | `#16a34a` | Medium-dark green | Hover states |
| `jupiter-green-700` | `#15803d` | Dark green | Active states |
| `jupiter-green-800` | `#166534` | Darker green | High contrast |
| `jupiter-green-900` | `#14532d` | Darkest green | Maximum contrast |

### Jupiter Orange Scale

| Scale | Value | Hex Code | Usage |
|-------|-------|----------|-------|
| `jupiter-orange-50` | `#fff7ed` | Very light orange | Backgrounds |
| `jupiter-orange-100` | `#ffedd5` | Light orange | Subtle backgrounds |
| `jupiter-orange-200` | `#fed7aa` | Lighter orange | Borders |
| `jupiter-orange-300` | `#fdba74` | Light orange | Disabled states |
| `jupiter-orange-400` | `#fb923c` | Medium-light orange | Secondary elements |
| `jupiter-orange-500` | `#f97316` | **Primary orange** | Accent color |
| `jupiter-orange-600` | `#ea580c` | Medium-dark orange | Hover states |
| `jupiter-orange-700` | `#c2410c` | Dark orange | Active states |
| `jupiter-orange-800` | `#9a3412` | Darker orange | High contrast |
| `jupiter-orange-900` | `#7c2d12` | Darkest orange | Maximum contrast |

## Palette Customization

### Creating Custom Palettes

You can create custom color palettes by implementing the `ColorProvider` trait or using the override system:

```rust
use jupiter_design_system::themes::*;

// Method 1: Using overrides
let custom_theme = VibeColors::with_overrides(|palette| {
    palette.primary = "purple-600".to_string();
    palette.secondary = "pink-500".to_string();
    palette.accent = "indigo-500".to_string();
});

// Method 2: Manual palette construction
let custom_palette = ColorPalette {
    primary: "purple-600".to_string(),
    secondary: "pink-500".to_string(),
    accent: "indigo-500".to_string(),
    // ... other colors
    ..Default::default()
};
```

### Override Patterns

Common customization patterns:

```rust
// Brand-specific colors
let brand_theme = VibeColors::with_overrides(|p| {
    p.primary = "brand-primary-500".to_string();
    p.secondary = "brand-secondary-500".to_string();
});

// Dark mode adjustments
let dark_theme = VibeColors::with_overrides(|p| {
    p.surface = "gray-800".to_string();
    p.background = "gray-900".to_string();
    p.foreground = "white".to_string();
    p.text_primary = "white".to_string();
    p.text_secondary = "gray-300".to_string();
});

// High contrast theme
let high_contrast = VibeColors::with_overrides(|p| {
    p.primary = "black".to_string();
    p.background = "white".to_string();
    p.border = "black".to_string();
});
```

## Color Scale Guidelines

### Choosing Scale Values

When selecting colors from scales, follow these guidelines:

- **50-100**: Subtle backgrounds, very light states
- **200-300**: Borders, dividers, disabled states  
- **400-500**: Default states, primary actions
- **600-700**: Hover states, secondary actions
- **800-900**: Active states, high contrast

### Semantic Consistency

Maintain semantic consistency across scales:

- **Success**: Always use green scales (`green-*`)
- **Warning**: Always use amber/yellow scales (`amber-*`, `yellow-*`)
- **Error**: Always use red scales (`red-*`)
- **Info**: Always use blue scales (`blue-*`)

### Accessibility Considerations

Ensure sufficient contrast ratios:

- **Text on backgrounds**: 4.5:1 minimum for normal text, 3:1 for large text
- **Interactive elements**: 3:1 minimum against adjacent colors
- **Focus indicators**: 3:1 minimum against background

Test color combinations:

```rust
// Good contrast examples
text_primary + surface    // gray-900 on white (21:1 ratio)
text_secondary + surface  // gray-600 on white (7.2:1 ratio)

// Check custom combinations
primary + surface         // jupiter-blue-500 on white
error + surface          // red-500 on white
```

## Technical Implementation

### Tailwind CSS Integration

Colors are implemented as Tailwind CSS classes:

```css
/* Generated classes for jupiter-blue-500 */
.text-jupiter-blue-500 { color: #3b82f6; }
.bg-jupiter-blue-500 { background-color: #3b82f6; }
.border-jupiter-blue-500 { border-color: #3b82f6; }
```

### Serialization Support

All palette structures support serialization:

```rust
use serde::{Deserialize, Serialize};

// Palette can be serialized to JSON
let json = serde_json::to_string(&palette)?;

// And deserialized from JSON
let palette: ColorPalette = serde_json::from_str(&json)?;
```

This enables storing and loading custom palettes from configuration files.