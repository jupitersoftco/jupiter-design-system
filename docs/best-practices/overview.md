# Overview & Getting Started

## 🎯 Design System Philosophy

The Jupiter Design System follows a trait-based architecture that embodies the principle:

> **"Make the right thing easy, and the wrong thing hard"**

This philosophy manifests in several key ways:

### Type Safety First
- **Compile-time validation**: Invalid design combinations are caught during compilation
- **IDE assistance**: Autocomplete reveals all available options and prevents typos
- **Refactoring confidence**: Changes propagate safely across the entire codebase

### Semantic Over Specific
- **Meaningful names**: `Color::Primary` instead of `blue-500`
- **Context-aware**: Colors adapt automatically when themes change
- **Intent-driven**: Code expresses design intent, not implementation details

### Consistency by Default
- **Automatic theming**: All components respect the active theme without additional configuration
- **Unified patterns**: Same API patterns across all components
- **Predictable behavior**: Once you learn one component, you understand them all

## 🚀 Quick Start Guide

### 1. Basic Setup

Add the design system to your `Cargo.toml`:

```toml
[dependencies]
jupiter-design-system = { path = "../jupiter-design-system" }
```

### 2. Essential Imports

```rust
use jupiter_design_system::prelude::*;
use jupiter_design_system::themes::VibeColors;
```

### 3. Create a Theme Provider

```rust
// Default Jupiter theme
let colors = VibeColors::new();

// Or with custom overrides
let custom_colors = VibeColors::with_overrides(|palette| {
    palette.primary = "purple-600".to_string();
    palette.secondary = "pink-500".to_string();
});
```

### 4. Build Components

```rust
// Simple button
let button_classes = button_styles(colors)
    .primary()
    .large()
    .classes();

// Complex button with full configuration
let complex_button = button_styles(colors)
    .variant(ButtonVariant::Success)
    .size(Size::Large)
    .state(ButtonState::Hover)
    .full_width(true)
    .classes();
```

### 5. Integration with Frameworks

The design system generates CSS class strings that work with any frontend framework:

#### Dioxus Example
```rust
rsx! {
    button {
        class: "{button_classes}",
        onclick: handle_click,
        "Submit Form"
    }
}
```

#### Yew Example
```rust
html! {
    <button class={button_classes} onclick={handle_click}>
        {"Submit Form"}
    </button>
}
```

#### Leptos Example
```rust
view! {
    <button class={button_classes} on:click=handle_click>
        "Submit Form"
    </button>
}
```

## 🏗️ Core Architecture Concepts

### Trait-Based Design

The system uses traits to provide extensibility and type safety:

```rust
// Color provider trait
pub trait ColorProvider {
    fn palette(&self) -> &ColorPalette;
    fn resolve_color(&self, color: Color) -> &str;
    fn text_class(&self, color: Color) -> String;
    fn bg_class(&self, color: Color) -> String;
    fn border_class(&self, color: Color) -> String;
}

// Builder pattern trait
pub trait ButtonBuilder {
    fn variant(self, variant: ButtonVariant) -> Self;
    fn size(self, size: Size) -> Self;
    fn state(self, state: ButtonState) -> Self;
    fn full_width(self, full_width: bool) -> Self;
    fn build(self) -> String;
}
```

### Design Tokens

The system provides semantic design tokens organized by category:

#### Colors
```rust
Color::Primary       // Brand primary color
Color::Secondary     // Brand secondary color
Color::Success       // Success states
Color::Warning       // Warning states
Color::Error         // Error states
Color::TextPrimary   // Primary text color
Color::Surface       // Surface backgrounds
Color::Interactive   // Interactive elements
```

#### Sizing
```rust
Size::XSmall    // Extra small components
Size::Small     // Small components
Size::Medium    // Medium components (default)
Size::Large     // Large components
Size::XLarge    // Extra large components
```

#### Typography
```rust
Typography::Heading1    // Main page headings
Typography::Heading2    // Section headings
Typography::Body        // Body text
Typography::Caption     // Small supplementary text
Typography::Label       // Form labels
```

#### Spacing
```rust
Spacing::None      // No spacing
Spacing::XSmall    // 4px equivalent
Spacing::Small     // 8px equivalent
Spacing::Medium    // 16px equivalent
Spacing::Large     // 24px equivalent
Spacing::XLarge    // 32px equivalent
Spacing::XXLarge   // 48px equivalent
```

## 🎨 Component Building Patterns

### Builder Pattern Usage

All components follow the same builder pattern:

```rust
// 1. Start with component function
let styles = button_styles(color_provider)

// 2. Configure with fluent API
    .variant(ButtonVariant::Primary)
    .size(Size::Large)
    .state(ButtonState::Default)

// 3. Build final classes
    .classes();
```

### State Management

Components support multiple states:

```rust
// Default state
let default_button = button_styles(colors).primary().classes();

// Hover state
let hover_button = button_styles(colors)
    .primary()
    .state(ButtonState::Hover)
    .classes();

// Disabled state
let disabled_button = button_styles(colors)
    .primary()
    .state(ButtonState::Disabled)
    .classes();
```

### Interactive States with Clean API

For components requiring pseudo-class states:

```rust
use jupiter_design_system::builders::interactive::*;

// Clean button with hover/focus/active states
let interactive_button = interactive_button(colors)
    .primary()
    .hover().darken().scale_105()
    .focus().ring_primary()
    .active().scale_95()
    .build();

// Clean input with interactive states
let interactive_input = interactive_input(colors)
    .standard_style()
    .hover().border_primary().shadow_md()
    .focus().border_primary().ring_primary().outline_none()
    .disabled().opacity_50().cursor_not_allowed()
    .build();
```

## 🔄 Theme Integration

### Using Default Theme

```rust
let colors = VibeColors::new();
let button = button_styles(colors).primary().classes();
// Uses jupiter-blue-500 for primary color
```

### Custom Theme Overrides

```rust
let custom_colors = VibeColors::with_overrides(|palette| {
    palette.primary = "purple-600".to_string();
    palette.secondary = "pink-500".to_string();
    palette.success = "emerald-500".to_string();
});

let button = button_styles(custom_colors).primary().classes();
// Uses purple-600 for primary color
```

### Implementing Custom Theme Provider

```rust
#[derive(Debug, Clone)]
pub struct MyCustomTheme {
    palette: ColorPalette,
}

impl ColorProvider for MyCustomTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

impl Default for MyCustomTheme {
    fn default() -> Self {
        Self {
            palette: ColorPalette {
                primary: "brand-blue".to_string(),
                secondary: "brand-green".to_string(),
                // ... other colors
            },
        }
    }
}
```

## 🧪 Testing Your Implementation

### Basic Smoke Test

```rust
#[test]
fn test_button_generation() {
    let colors = VibeColors::new();
    let button_classes = button_styles(colors).primary().classes();
    
    assert!(button_classes.contains("bg-jupiter-blue-500"));
    assert!(button_classes.contains("text-white"));
}
```

### Theme Switching Test

```rust
#[test]
fn test_theme_switching() {
    let default_colors = VibeColors::new();
    let custom_colors = VibeColors::with_overrides(|palette| {
        palette.primary = "red-500".to_string();
    });
    
    let default_button = button_styles(default_colors).primary().classes();
    let custom_button = button_styles(custom_colors).primary().classes();
    
    assert!(default_button.contains("jupiter-blue-500"));
    assert!(custom_button.contains("red-500"));
}
```

## 🚧 Common Patterns

### Conditional Styling

```rust
let button_variant = if is_destructive {
    ButtonVariant::Error
} else {
    ButtonVariant::Primary
};

let button_classes = button_styles(colors)
    .variant(button_variant)
    .size(Size::Medium)
    .classes();
```

### Responsive Design

```rust
// Component handles responsive classes internally
let card_classes = card_styles(colors)
    .elevation(CardElevation::Medium)
    .spacing(CardSpacing::Large)
    .classes();
// Generates: "bg-white shadow-md p-6 sm:p-8 lg:p-12"
```

### Component Composition

```rust
// Header with consistent styling
let header_classes = text_styles(colors)
    .typography(Typography::Heading1)
    .color(Color::TextPrimary)
    .classes();

// Subheader with related styling
let subheader_classes = text_styles(colors)
    .typography(Typography::Heading2)
    .color(Color::TextSecondary)
    .classes();
```

## 📈 Performance Considerations

### Compile-Time Generation
- All CSS classes are resolved at compile time
- Zero runtime overhead for class generation
- Optimal bundle sizes with dead code elimination

### Caching Color Providers
```rust
// Cache expensive theme creation
lazy_static! {
    static ref DEFAULT_COLORS: VibeColors = VibeColors::new();
}

fn get_default_colors() -> &'static VibeColors {
    &DEFAULT_COLORS
}
```

## 🔗 Next Steps

- **[Color System Best Practices](./colors.md)** - Deep dive into color usage
- **[Component Guidelines](./components.md)** - Advanced component building
- **[Theming Guide](./theming.md)** - Custom theme implementation
- **[Examples](./examples/)** - Real-world usage patterns