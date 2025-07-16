# Complete API Reference

This document provides detailed reference for all public functions, traits, and types in Jupiter Design System.

## Core Module

### Traits

#### `ColorProvider`
```rust
pub trait ColorProvider {
    fn palette(&self) -> &ColorPalette;
    fn resolve_color(&self, color: Color) -> &str { /* default impl */ }
    fn text_class(&self, color: Color) -> String { /* default impl */ }
    fn bg_class(&self, color: Color) -> String { /* default impl */ }
    fn border_class(&self, color: Color) -> String { /* default impl */ }
}
```

#### `SizeProvider`
```rust
pub trait SizeProvider {
    fn resolve_size(&self, size: Size) -> &str;
    fn width_class(&self, size: Size) -> String { /* default impl */ }
    fn height_class(&self, size: Size) -> String { /* default impl */ }
}
```

#### `SpacingProvider`
```rust
pub trait SpacingProvider {
    fn resolve_spacing(&self, spacing: Spacing) -> &str;
    fn padding_class(&self, spacing: Spacing) -> String { /* default impl */ }
    fn margin_class(&self, spacing: Spacing) -> String { /* default impl */ }
}
```

#### `TypographyProvider`
```rust
pub trait TypographyProvider {
    fn resolve_typography(&self, typography: Typography) -> &str;
    fn typography_class(&self, typography: Typography) -> String { /* default impl */ }
    fn font_weight_class(&self, weight: FontWeight) -> String { /* default impl */ }
    fn font_family_class(&self, family: FontFamily) -> String { /* default impl */ }
}
```

### Types

#### `ColorPalette`
```rust
pub struct ColorPalette {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,
    pub surface: String,
    pub background: String,
    pub foreground: String,
    pub border: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub text_tertiary: String,
    pub text_inverse: String,
    pub interactive: String,
    pub interactive_hover: String,
    pub interactive_active: String,
    pub interactive_disabled: String,
}
```

## Pattern Functions

### Action Patterns

#### `action_semantics<C: ColorProvider>(color_provider: C) -> ActionSemantics<C>`
Creates a new action semantics builder for defining action intent and hierarchy.

**Example:**
```rust
let action = action_semantics(colors)
    .primary()
    .urgent()
    .classes();
```

### Button Patterns

#### `button_pattern<C: ColorProvider + Clone>(color_provider: C) -> ButtonPattern<C>`
Creates a new button pattern with full customization options.

**Example:**
```rust
let button = button_pattern(colors)
    .primary_action()
    .hero_prominence()
    .classes();
```

#### `primary_button<C: ColorProvider + Clone>(color_provider: C) -> ButtonPattern<C>`
Creates a pre-configured primary button pattern.

**Example:**
```rust
let button = primary_button(colors).classes();
```

#### `secondary_button<C: ColorProvider + Clone>(color_provider: C) -> ButtonPattern<C>`
Creates a pre-configured secondary button pattern.

#### `destructive_button<C: ColorProvider + Clone>(color_provider: C) -> ButtonPattern<C>`
Creates a pre-configured destructive action button pattern.

#### `hero_button<C: ColorProvider + Clone>(color_provider: C) -> ButtonPattern<C>`
Creates a pre-configured hero CTA button pattern.

#### `navigation_button<C: ColorProvider + Clone>(color_provider: C) -> ButtonPattern<C>`
Creates a pre-configured navigation button pattern.

#### `button_link<C: ColorProvider + Clone>(color_provider: C) -> ButtonPattern<C>`
Creates a button pattern styled as a link.

### Card Patterns

#### `card_pattern<C: ColorProvider + Clone>(color_provider: C) -> CardPattern<C>`
Creates a new card pattern for container styling.

**Example:**
```rust
let card = card_pattern(colors)
    .elevation(CardElevation::Raised)
    .interaction(CardInteraction::Clickable)
    .classes();
```

### Focus Management

#### `focus_management<C: ColorProvider>(color_provider: C) -> FocusManagement<C>`
Creates a new focus management pattern for accessibility.

**Example:**
```rust
let focus = focus_management(colors)
    .button()
    .prominent_focus()
    .classes();
```

### Interactive Elements

#### `interactive_element<C: ColorProvider>(color_provider: C) -> InteractiveElement<C>`
Creates a new interactive element pattern for hover/active states.

**Example:**
```rust
let interactive = interactive_element(colors)
    .hoverable()
    .focusable()
    .pressable()
    .classes();
```

### Typography Patterns

#### `typography_pattern<T: ColorProvider>(color_provider: T) -> TypographyPattern<T>`
Creates a new typography pattern with full customization.

**Example:**
```rust
let text = typography_pattern(colors)
    .hierarchy(TypographyHierarchy::Body)
    .color(TypographyColor::Primary)
    .classes();
```

#### `title_typography<T: ColorProvider>(color_provider: T) -> TypographyPattern<T>`
Creates a pre-configured title typography pattern.

#### `heading_typography<T: ColorProvider>(color_provider: T) -> TypographyPattern<T>`
Creates a pre-configured heading typography pattern.

#### `body_typography<T: ColorProvider>(color_provider: T) -> TypographyPattern<T>`
Creates a pre-configured body text typography pattern.

#### `caption_typography<T: ColorProvider>(color_provider: T) -> TypographyPattern<T>`
Creates a pre-configured caption typography pattern.

#### `code_typography<T: ColorProvider>(color_provider: T) -> TypographyPattern<T>`
Creates a pre-configured code/monospace typography pattern.

### Product Patterns

#### `product_pattern<C: ColorProvider + Clone>(color_provider: C) -> ProductPattern<C>`
Creates a new product pattern for e-commerce components.

**Example:**
```rust
let product = product_pattern(colors)
    .display_pattern(ProductDisplayPattern::Featured)
    .price_pattern(ProductPricePattern::Sale)
    .classes();
```

### Selection Patterns

#### `selection_pattern<C: ColorProvider>(color_provider: C) -> SelectionPattern<C>`
Creates a new selection pattern for checkboxes, radios, and toggles.

**Example:**
```rust
let checkbox = selection_pattern(colors)
    .behavior(SelectionBehavior::Multiple)
    .state(SelectionState::Selected)
    .classes();
```

### State Patterns

#### `state_pattern<C: ColorProvider>(color_provider: C) -> StatePattern<C>`
Creates a new state pattern for component states.

**Example:**
```rust
let state = state_pattern(colors)
    .component_state(ComponentState::Success)
    .with_icon(true)
    .classes();
```

## Builder Functions

### Button Builder

#### `button_styles<C: ColorProvider>(color_provider: C) -> ButtonStyles<C>`
Creates a new button styles builder for CSS class generation.

**Example:**
```rust
let button = button_styles(colors)
    .variant(ButtonVariant::Primary)
    .size(Size::Medium)
    .classes();
```

#### `button_classes_from_strings(variant: &str, size: &str, state: Option<&str>) -> Result<String, String>`
Generates button classes from string parameters.

**Example:**
```rust
let classes = button_classes_from_strings("primary", "large", Some("hover"))?;
```

### Text Builder

#### `text_styles<T: ColorProvider>(color_provider: T) -> TextStyles<T>`
Creates a new text styles builder for typography CSS.

**Example:**
```rust
let text = text_styles(colors)
    .size(TextSize::LG)
    .weight(FontWeight::Bold)
    .classes();
```

#### `text_classes_from_strings(size: &str, weight: &str, color: Option<&str>) -> Result<String, String>`
Generates text classes from string parameters.

### Card Builder

#### `card_styles<C: ColorProvider>(color_provider: C) -> CardStyles<C>`
Creates a new card styles builder for container styling.

**Example:**
```rust
let card = card_styles(colors)
    .padding(Spacing::Large)
    .rounded_lg()
    .shadow_md()
    .classes();
```

### Layout Builder

#### `layout_styles<C: ColorProvider>(color_provider: C) -> LayoutStyles<C>`
Creates a new layout styles builder for flexbox/grid layouts.

**Example:**
```rust
let layout = layout_styles(colors)
    .flex()
    .direction(FlexDirection::Row)
    .gap(Spacing::Medium)
    .classes();
```

### Selection Builder

#### `selection_styles<C: ColorProvider>(color_provider: C) -> SelectionStyles<C>`
Creates a new selection styles builder for form controls.

**Example:**
```rust
let checkbox = selection_styles(colors)
    .variant(SelectionVariant::Checkbox)
    .checked(true)
    .classes();
```

#### `selection_classes_from_strings(variant: &str, size: &str, checked: Option<&str>) -> Result<String, String>`
Generates selection classes from string parameters.

### State Builder

#### `state_styles<C: ColorProvider>(color_provider: C) -> StateStyles<C>`
Creates a new state styles builder for component states.

**Example:**
```rust
let loading = state_styles(colors)
    .loading()
    .size(Size::Medium)
    .classes();
```

### Product Builder

#### `product_builder<C: ColorProvider>(color_provider: C) -> ProductBuilder<C>`
Creates a new product builder for e-commerce components.

**Example:**
```rust
let product = product_builder(colors)
    .variant(ProductVariant::Card)
    .on_sale(true)
    .classes();
```

## Theme Types

### `Theme` Trait
```rust
pub trait Theme {
    fn name(&self) -> &str;
}
```

### `VibeColors`
Default color provider implementation for Jupiter Design System.

```rust
impl VibeColors {
    pub fn new() -> Self;
    pub fn with_overrides(overrides: impl Fn(&mut ColorPalette)) -> Self;
}

impl ColorProvider for VibeColors { /* ... */ }
```

### `VibeTheme`
Default theme implementation.

```rust
impl VibeTheme {
    pub fn new() -> Self;
    pub fn available_themes() -> Vec<&'static str>;
    pub fn theme_description(theme: &str) -> &'static str;
}

impl Theme for VibeTheme { /* ... */ }
```

## Utility Functions

### `DesignSystem`
Main utility struct for design system operations.

```rust
impl DesignSystem {
    pub fn new() -> Self;
    // Additional utility methods
}
```

## Type Aliases and Re-exports

The prelude module (`jupiter_design_system::prelude::*`) re-exports commonly used items:

- All builder functions
- All pattern functions  
- Core types (`Color`, `ColorPalette`, `Size`, `Spacing`, etc.)
- Theme types (`Theme`, `VibeTheme`, `VibeColors`)
- Common enums and structs

## Usage Patterns

### Basic Theme Usage
```rust
use jupiter_design_system::prelude::*;

let colors = VibeColors::new();
let button = primary_button(colors).classes();
```

### Custom Theme Implementation
```rust
use jupiter_design_system::prelude::*;

#[derive(Clone)]
struct MyTheme {
    palette: ColorPalette,
}

impl ColorProvider for MyTheme {
    fn palette(&self) -> &ColorPalette {
        &self.palette
    }
}

impl Theme for MyTheme {
    fn name(&self) -> &str {
        "My Custom Theme"
    }
}
```

### Pattern Composition
```rust
let complex_component = format!(
    "{} {} {}",
    card_pattern(colors.clone()).classes(),
    title_typography(colors.clone()).classes(),
    primary_button(colors).classes()
);
```

### Builder Composition
```rust
let styled_button = format!(
    "{} {} {}",
    button_styles(colors.clone()).primary().classes(),
    HoverBuilder::new(colors.clone()).scale(105).classes(),
    FocusBuilder::new(colors).ring(2).classes()
);
```