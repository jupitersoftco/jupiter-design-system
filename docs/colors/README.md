# Jupiter Design System Color Documentation

Welcome to the Jupiter Design System color documentation. This section provides comprehensive guidance on using and customizing the color system.

## Overview

The Jupiter Design System implements a semantic color system built on Rust traits and Tailwind CSS. Colors are organized into semantic tokens that map to concrete values, enabling consistent theming and easy customization.

## Quick Navigation

- **[Color Tokens](./tokens.md)** - Complete reference of all color tokens
- **[Palettes & Scales](./palettes.md)** - Color palette structure and scales
- **[Theming](./theming.md)** - Theme configuration and customization
- **[Usage Examples](./examples.md)** - Practical usage patterns and best practices
- **[Implementation Notes](./implementation-notes.md)** - ⚠️ **Required setup and configuration**
- **[Advanced Usage](./advanced-usage.md)** - Complex patterns and real-world examples

## Key Features

- **Semantic Naming**: All colors use semantic names (Primary, Success, etc.) rather than specific values
- **Type Safety**: Rust enum-based tokens prevent invalid color usage
- **Framework Agnostic**: Generates CSS classes compatible with any UI framework
- **Customizable**: Full override support for custom themes
- **Tailwind Integration**: Built on Tailwind CSS color system

## Architecture

The color system is built around three core concepts:

1. **Color Tokens** (`Color` enum) - Semantic color identifiers
2. **Color Palette** (`ColorPalette` struct) - Maps tokens to concrete values
3. **Color Provider** (`ColorProvider` trait) - Utilities for color resolution

## Getting Started

```rust
use jupiter_design_system::prelude::*;

// Create a theme provider
let theme = VibeColors::default();

// Use semantic colors in components
let button = Button::new("Click me")
    .variant(ButtonVariant::Primary)  // Uses Color::Primary
    .build(&theme);

// Generate CSS classes
let text_class = theme.text_class(Color::TextPrimary);   // "text-gray-900"
let bg_class = theme.bg_class(Color::Surface);           // "bg-white"
```

## Color Philosophy

Jupiter's color system follows these principles:

- **Semantic over Specific**: Use `Color::Primary` instead of "blue-500"
- **Consistent Context**: Colors have consistent meaning across components
- **Accessible by Default**: Default palette meets WCAG accessibility standards
- **Customizable**: Easy to override for brand-specific needs