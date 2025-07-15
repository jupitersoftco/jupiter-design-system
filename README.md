# Jupiter Design System

A trait-based design system for Jupiter Startups applications that provides type-safe design tokens, component builders, and theme management.

## 🎯 Philosophy

> **"Make the right thing easy, and the wrong thing hard"**

This design system uses a trait-based approach to ensure:

- **Type Safety**: Invalid design combinations are caught at compile time
- **Consistency**: All components automatically respect active theme
- **Discoverability**: IDE autocomplete shows all available options
- **Flexibility**: Easy to extend and customize for specific needs

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
jupiter-design-system = { path = "../jupiter-design-system" }
```

Basic usage:

```rust
use jupiter_design_system::prelude::*;
use jupiter_design_system::core::color::{VibeColors, ColorProvider};
use jupiter_design_system::components::button::Button;

// Create a theme
let colors = VibeColors::new();

// Generate CSS classes
let button_classes = Button::primary(colors)
    .size(Size::Large)
    .full_width(true)
    .build();

// Result: "inline-flex items-center justify-center font-medium ... bg-jupiter-blue-500 text-white w-full"
```

## 🎨 Design System Features

### Core Design Tokens

#### Colors

- **Semantic Colors**: Primary, Secondary, Success, Warning, Error, Info
- **Neutral Colors**: Surface, Background, Foreground, Border
- **Text Colors**: TextPrimary, TextSecondary, TextTertiary, TextInverse
- **Interactive States**: Interactive, InteractiveHover, InteractiveActive, InteractiveDisabled

#### Sizing

- **Component Sizes**: XSmall, Small, Medium, Large, XLarge
- **Breakpoints**: Mobile, Tablet, Desktop, Large

#### Typography

- **Text Styles**: Heading1-6, Body, BodySmall, Caption, Label
- **Font Weights**: Light, Normal, Medium, SemiBold, Bold
- **Font Families**: Sans, Serif, Mono

#### Spacing

- **Spacing Scale**: None, XSmall, Small, Medium, Large, XLarge, XXLarge

### Component Builders

#### Interactive Builders System

**NEW**: Clean, fluent API for interactive components with pseudo-class states:

```rust
use jupiter_design_system::builders::interactive::*;

// Clean input with interactive states
let input_classes = interactive_input(colors)
    .standard_style()
    .hover().border_primary().shadow_md()
    .focus().border_primary().ring_primary().outline_none()
    .disabled().opacity_50().cursor_not_allowed()
    .build();

// Clean button with interactive states
let button_classes = interactive_button(colors)
    .primary()
    .hover().darken().scale_105()
    .focus().ring_primary()
    .active().scale_95()
    .build();
```

**Replaces messy string formatting**:

```rust
// ❌ Old way - messy and error-prone
format!("{} {} focus:{} hover:{}",
    base_classes, border_classes, focus_classes, hover_classes)

// ✅ New way - clean and type-safe
interactive_input(colors)
    .standard_style()
    .hover().border_primary()
    .focus().ring_primary()
    .build()
```

#### Button Component

```rust
use jupiter_design_system::components::button::{Button, ButtonVariant, ButtonState};

// Basic variants
let primary = Button::primary(colors.clone()).build();
let secondary = Button::secondary(colors.clone()).build();
let success = Button::success(colors.clone()).build();
let warning = Button::warning(colors.clone()).build();
let error = Button::error(colors.clone()).build();
let ghost = Button::ghost(colors.clone()).build();
let link = Button::link(colors.clone()).build();

// Fluent API
let complex_button = Button::new(colors)
    .variant(ButtonVariant::Success)
    .size(Size::Large)
    .state(ButtonState::Hover)
    .full_width(true)
    .with_icon(true)
    .build();
```

## 🏗️ Trait-Based Architecture

The design system uses traits to provide extensibility and type safety:

### Color Provider Trait

```rust
pub trait ColorProvider {
    fn palette(&self) -> &ColorPalette;
    fn resolve_color(&self, color: Color) -> &str;
    fn text_class(&self, color: Color) -> String;
    fn bg_class(&self, color: Color) -> String;
    fn border_class(&self, color: Color) -> String;
}
```

### Button Builder Trait

```rust
pub trait ButtonBuilder {
    fn variant(self, variant: ButtonVariant) -> Self;
    fn size(self, size: Size) -> Self;
    fn state(self, state: ButtonState) -> Self;
    fn full_width(self, full_width: bool) -> Self;
    fn with_icon(self, has_icon: bool) -> Self;
    fn build(self) -> String;
}
```

## 🎭 Theme System

### Default Vibe Theme

```rust
let colors = VibeColors::new();

// Brand colors
colors.resolve_color(Color::Primary);    // "jupiter-blue-500"
colors.resolve_color(Color::Secondary);  // "jupiter-green-500"
```

### Custom Theme

```rust
let custom_colors = VibeColors::with_overrides(|palette| {
    palette.primary = "purple-600".to_string();
    palette.secondary = "pink-500".to_string();
});

let button = Button::primary(custom_colors).build();
// Uses custom purple-600 color
```

### Implementing Your Own Theme

```rust
#[derive(Debug, Clone)]
pub struct MyCustomColors {
    palette: ColorPalette,
}

impl ColorProvider for MyCustomColors {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

// Now use with any component
let button = Button::primary(MyCustomColors::new()).build();
```

## 📦 Component Examples

### Button Variants

```rust
// All button variants with consistent API
let buttons = vec![
    Button::primary(colors.clone()).build(),
    Button::secondary(colors.clone()).build(),
    Button::success(colors.clone()).build(),
    Button::warning(colors.clone()).build(),
    Button::error(colors.clone()).build(),
    Button::ghost(colors.clone()).build(),
    Button::link(colors.clone()).build(),
];
```

### Button Sizes

```rust
// All sizes available
let sizes = vec![
    Button::primary(colors.clone()).size(Size::XSmall).build(),
    Button::primary(colors.clone()).size(Size::Small).build(),
    Button::primary(colors.clone()).size(Size::Medium).build(),
    Button::primary(colors.clone()).size(Size::Large).build(),
    Button::primary(colors.clone()).size(Size::XLarge).build(),
];
```

### Button States

```rust
// Interactive states
let states = vec![
    Button::primary(colors.clone()).state(ButtonState::Default).build(),
    Button::primary(colors.clone()).state(ButtonState::Hover).build(),
    Button::primary(colors.clone()).state(ButtonState::Active).build(),
    Button::primary(colors.clone()).state(ButtonState::Disabled).build(),
    Button::primary(colors.clone()).state(ButtonState::Loading).build(),
];
```

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run specific component tests:

```bash
cargo test button
cargo test color
```

## 📖 Examples

Run the examples to see the design system in action:

```bash
# Basic color usage
cargo run --example basic_usage

# Comprehensive button examples
cargo run --example button_usage
```

## 🎯 Benefits

### Type Safety

- **Compile-time validation**: Invalid combinations caught early
- **IDE support**: Autocomplete shows available options
- **Refactoring safety**: Changes are caught across the codebase

### Consistency

- **Automatic theming**: All components respect active theme
- **Semantic naming**: Color names convey meaning, not implementation
- **Unified API**: Same patterns across all components

### Flexibility

- **Custom themes**: Easy to implement custom color schemes
- **Extensible**: Add new components following established patterns
- **Composable**: Mix and match different providers and components

### Developer Experience

- **Fluent API**: Natural, readable component building
- **Self-documenting**: Code clearly expresses design intent
- **Zero runtime cost**: All resolved to strings at compile time

## 🔧 Advanced Usage

### Custom Color Provider

```rust
#[derive(Debug, Clone)]
pub struct DarkTheme {
    palette: ColorPalette,
}

impl Default for DarkTheme {
    fn default() -> Self {
        Self {
            palette: ColorPalette {
                primary: "blue-400".to_string(),
                background: "gray-900".to_string(),
                surface: "gray-800".to_string(),
                text_primary: "white".to_string(),
                // ... other colors
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

### Component Integration

```rust
// Easy integration with web frameworks
fn render_button(text: &str, variant: ButtonVariant) -> String {
    let colors = VibeColors::new();
    let classes = Button::new(colors)
        .variant(variant)
        .size(Size::Medium)
        .build();

    format!("<button class=\"{}\">{}</button>", classes, text)
}
```

## 🗺️ Roadmap

- [x] **Core Design Tokens**: Colors, Spacing, Typography, Sizing
- [x] **Button Component**: Full-featured button builder
- [x] **Interactive Builders**: Clean fluent API for pseudo-class states (hover, focus, active, disabled)
- [ ] **Card Component**: Flexible card layouts
- [ ] **Text Component**: Typography with semantic scaling
- [ ] **Form Components**: Input, Select, Checkbox, Radio
- [ ] **Layout Components**: Container, Grid, Flexbox helpers
- [ ] **Navigation Components**: Navbar, Breadcrumbs, Pagination
- [ ] **Feedback Components**: Toast, Modal, Alert
- [ ] **Data Components**: Table, List, Badge

## 📄 License

This design system is built for Jupiter Startups applications.

## 🤝 Contributing

1. Follow the established trait-based patterns
2. Add comprehensive tests for new components
3. Update documentation and examples
4. Ensure all components work with custom themes

---

**Built with ❤️ for Jupiter Startups**
