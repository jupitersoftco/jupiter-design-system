# Interactive Builders Guide

The Interactive Builders system provides a clean, fluent API for building interactive components with pseudo-class states like hover, focus, active, and disabled.

## Overview

Instead of messy string formatting for pseudo-classes:

```rust
// ❌ Old way - messy and error-prone
format!("{} {} focus:{} hover:{}",
    theme.colors.bg_class(Color::Background),
    theme.colors.border_class(Color::Border),
    theme.colors.border_class(Color::Primary),
    theme.colors.border_class(Color::Foreground)
)
```

Use the clean fluent API:

```rust
// ✅ New way - clean and type-safe
interactive_input(colors)
    .standard_style()
    .hover().border_primary().shadow_md()
    .focus().border_primary().ring_primary().outline_none()
    .disabled().opacity_50().cursor_not_allowed()
    .build()
```

## Core Concepts

### Interactive Base

The `InteractiveBase<C>` is the foundation that provides:
- **Base classes**: Always applied styles
- **State builders**: Fluent APIs for hover, focus, active, disabled states
- **Type safety**: Compile-time validation of method chains
- **Order independence**: Chain states in any order

### State Builders

Each pseudo-class state has its own builder:

#### **HoverBuilder**
```rust
.hover()
    .border_primary()      // Set border to primary color
    .bg_primary()          // Set background to primary color  
    .darken()              // Darken background using InteractiveHover color
    .scale_105()           // Scale transform on hover
    .shadow_md()           // Add medium shadow
    .shadow_lg()           // Add large shadow
    .classes("custom-class") // Add arbitrary classes
```

#### **FocusBuilder**
```rust
.focus()
    .border_primary()      // Set border to primary color
    .outline_none()        // Remove outline
    .ring_primary()        // Add ring with primary color
    .classes("custom-class") // Add arbitrary classes
```

#### **ActiveBuilder**
```rust
.active()
    .scale_95()            // Scale down on press
    .classes("custom-class") // Add arbitrary classes
```

#### **DisabledBuilder**
```rust
.disabled()
    .opacity_50()          // Set opacity to 50%
    .cursor_not_allowed()  // Set cursor to not-allowed
    .classes("custom-class") // Add arbitrary classes
```

## Specialized Builders

### InputBuilder

Specialized for form inputs with convenient presets:

```rust
use jupiter_design_system::builders::interactive::interactive_input;

// Standard input with full interactive states
let input_classes = interactive_input(colors)
    .standard_style()  // Applies w-full, padding, border, etc.
    .hover().border_primary().shadow_md()
    .focus().border_primary().ring_primary().outline_none()
    .disabled().opacity_50().cursor_not_allowed()
    .build();

// Custom input with base style
let custom_input = interactive_input(colors)
    .base_style()  // Basic input styling
    .hover().border_primary()
    .focus().ring_primary()
    .build();
```

### ButtonBuilder

Specialized for buttons with variant presets:

```rust
use jupiter_design_system::builders::interactive::interactive_button;

// Primary button with interactions
let primary_button = interactive_button(colors)
    .primary()  // Applies primary variant styles
    .hover().darken().scale_105()
    .focus().ring_primary()
    .active().scale_95()
    .build();

// Secondary button
let secondary_button = interactive_button(colors)
    .secondary()  // Applies secondary variant styles
    .hover().border_primary()
    .focus().ring_primary()
    .build();

// Ghost button
let ghost_button = interactive_button(colors)
    .ghost()  // Applies ghost variant styles
    .hover().bg_primary()
    .focus().ring_primary()
    .build();
```

## Usage Examples

### Basic Input Field

```rust
use jupiter_design_system::builders::interactive::interactive_input;

impl MyTheme {
    pub fn input_field(&self) -> String {
        interactive_input(self.colors.clone())
            .standard_style()
            .hover().border_primary().shadow_md()
            .focus().border_primary().ring_primary().outline_none()
            .disabled().opacity_50().cursor_not_allowed()
            .build()
    }
}
```

### Social Media Button

```rust
use jupiter_design_system::builders::interactive::interactive_button;

impl MyTheme {
    pub fn social_icon_button(&self) -> String {
        interactive_button(self.colors.clone())
            .base_classes("w-8 h-8 flex items-center justify-center rounded-full text-sm")
            .primary()
            .hover().darken()
            .focus().ring_primary()
            .build()
    }
}
```

### Textarea with Custom Styling

```rust
use jupiter_design_system::builders::interactive::interactive_input;

impl MyTheme {
    pub fn textarea_field(&self) -> String {
        interactive_input(self.colors.clone())
            .base_classes("w-full px-4 py-3 resize-vertical")
            .standard_style()
            .hover().border_primary().shadow_md()
            .focus().border_primary().ring_primary().outline_none()
            .disabled().opacity_50().cursor_not_allowed()
            .build()
    }
}
```

## Advanced Features

### Chaining Order Independence

The builders are designed to be order-independent:

```rust
// These produce equivalent results
let classes1 = interactive_input(colors)
    .standard_style()
    .hover().border_primary()
    .focus().ring_primary()
    .build();

let classes2 = interactive_input(colors)
    .standard_style()
    .focus().ring_primary()
    .hover().border_primary()
    .build();
```

### Custom Classes

Add arbitrary classes to any state:

```rust
let custom_input = interactive_input(colors)
    .base_style()
    .hover().classes("transform transition-transform duration-200")
    .focus().classes("ring-offset-4 ring-opacity-50")
    .build();
```

### Generic Interactive Elements

For elements that don't fit input or button patterns:

```rust
use jupiter_design_system::builders::interactive::interactive_element;

let card_classes = interactive_element(colors)
    .base("p-4 rounded-lg bg-white shadow-sm")
    .hover().classes("shadow-md transform translate-y-[-2px]")
    .focus().classes("ring-2 ring-blue-500 ring-offset-2")
    .build();
```

## Generated CSS Structure

The builders generate clean, organized CSS classes:

```css
/* Base classes always applied */
w-full px-4 py-3 border rounded-md transition-colors

/* Hover pseudo-classes */
hover:(border-blue-500 shadow-md)

/* Focus pseudo-classes */
focus:(border-blue-500 ring-2 ring-offset-2 ring-blue-300 outline-none)

/* Disabled pseudo-classes */
disabled:(opacity-50 cursor-not-allowed)
```

## Best Practices

### 1. Use Specialized Builders

Prefer `interactive_input()` and `interactive_button()` over generic `interactive_element()` when possible.

### 2. Consistent State Patterns

Establish consistent patterns for your design system:

```rust
// Standard input pattern
.hover().border_primary().shadow_md()
.focus().border_primary().ring_primary().outline_none()
.disabled().opacity_50().cursor_not_allowed()

// Standard button pattern
.hover().darken().scale_105()
.focus().ring_primary()
.active().scale_95()
```

### 3. Theme Integration

Use the builders within your theme methods:

```rust
impl MyTheme {
    pub fn primary_button(&self) -> String {
        interactive_button(self.colors.clone())
            .primary()
            .hover().darken().scale_105()
            .focus().ring_primary()
            .active().scale_95()
            .build()
    }
}
```

### 4. Testing

The builders are fully testable:

```rust
#[test]
fn test_input_has_proper_states() {
    let colors = MyColors::default();
    let classes = interactive_input(colors)
        .standard_style()
        .hover().border_primary()
        .focus().ring_primary()
        .build();
    
    assert!(classes.contains("hover:"));
    assert!(classes.contains("focus:"));
    assert!(classes.contains("w-full"));
}
```

## Migration Guide

### From Format Strings

Replace manual string formatting:

```rust
// Before
format!("input {} focus:{} hover:{}",
    base_classes,
    focus_classes,
    hover_classes
)

// After
interactive_input(colors)
    .base_classes(&base_classes)
    .hover().classes(&hover_classes)
    .focus().classes(&focus_classes)
    .build()
```

### From Conditional Logic

Replace complex conditional logic:

```rust
// Before
let mut classes = vec!["base-class"];
if is_hoverable {
    classes.push("hover:border-primary");
}
if is_focusable {
    classes.push("focus:ring-2");
}
classes.join(" ")

// After
let mut builder = interactive_element(colors)
    .base("base-class");

if is_hoverable {
    builder = builder.hover().border_primary();
}
if is_focusable {
    builder = builder.focus().classes("ring-2");
}

builder.build()
```

## API Reference

### Core Functions

- `interactive_input(colors)` → `InputBuilder<C>`
- `interactive_button(colors)` → `ButtonBuilder<C>`
- `interactive_element(colors)` → `InteractiveBase<C>`

### State Methods

All builders support these state transitions:
- `.hover()` → `HoverBuilder<C>`
- `.focus()` → `FocusBuilder<C>`
- `.active()` → `ActiveBuilder<C>`
- `.disabled()` → `DisabledBuilder<C>`

### Build Methods

All builders support:
- `.build()` → `String` - Generate final CSS classes
- `.classes(custom)` → Add arbitrary classes to current state

The Interactive Builders system provides a clean, type-safe way to build interactive components while maintaining the flexibility and power of the Jupiter Design System.