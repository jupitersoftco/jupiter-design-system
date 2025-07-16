# Advanced Usage Patterns

This document covers advanced usage patterns found in the Jupiter Design System codebase.

## Selection Rings

The system uses custom Jupiter blue for selection rings:

```rust
// Selection ring classes
ring-jupiter-blue-300    // Light blue ring for selections
```

### Usage in Components

```rust
// From selection builder
let selected_classes = format!(
    "ring-2 ring-jupiter-blue-300 ring-offset-2 {}",
    base_classes
);

// Conditional selection
let classes = if is_selected {
    "ring-jupiter-blue-300 border-jupiter-blue-500"
} else {
    "border-gray-200"
};
```

## Branded Gradients

The system uses dark gradients for branded surfaces:

```rust
// Branded card gradient
"bg-gradient-to-br from-jupiter-navy-900/80 to-jupiter-blue-900/80 border-white/10 text-white"
```

### Gradient Components

- **from-jupiter-navy-900/80**: Dark navy with 80% opacity
- **to-jupiter-blue-900/80**: Dark blue with 80% opacity  
- **border-white/10**: White border with 10% opacity
- **text-white**: White text for contrast

## ButtonStyles Builder Pattern

Advanced button styling using the builder pattern:

```rust
use jupiter_design_system::builders::button::{ButtonStyles, button_styles};

// Basic usage
let primary_button = ButtonStyles::new(colors.clone())
    .primary()
    .classes();

// Complex configuration
let complex_button = ButtonStyles::new(colors.clone())
    .success()
    .large()
    .hover()
    .full_width()
    .with_icon()
    .classes();

// Custom CSS injection
let custom_button = ButtonStyles::new(colors.clone())
    .primary()
    .large()
    .custom("shadow-xl")
    .custom("transform")
    .custom_classes("hover:scale-110 transition-transform")
    .classes();
```

### Available Button Methods

- **Variants**: `.primary()`, `.secondary()`, `.success()`, `.warning()`, `.error()`, `.ghost()`, `.link()`
- **Sizes**: `.extra_small()`, `.small()`, `.medium()`, `.large()`, `.extra_large()`
- **States**: `.hover()`, `.active()`, `.disabled()`, `.loading()`
- **Modifiers**: `.full_width()`, `.with_icon()`
- **Custom**: `.custom("class")`, `.custom_classes("multiple classes")`

## Real Examples from Codebase

### Basic Usage Example

From `/examples/basic_usage.rs`:

```rust
// Theme creation
let theme = VibeTheme::default();
let colors = VibeColors::new();

// Color class generation
println!("Primary text: {}", colors.text_class(Color::Primary));
println!("Primary background: {}", colors.bg_class(Color::Primary));
println!("Primary border: {}", colors.border_class(Color::Primary));

// Interactive states
println!("Interactive: {}", colors.resolve_color(Color::Interactive));
println!("Hover: {}", colors.resolve_color(Color::InteractiveHover));
println!("Active: {}", colors.resolve_color(Color::InteractiveActive));
println!("Disabled: {}", colors.resolve_color(Color::InteractiveDisabled));

// Custom theme
let custom_colors = VibeColors::with_overrides(|palette| {
    palette.primary = "custom-blue-600".to_string();
    palette.secondary = "custom-green-600".to_string();
});
```

### Button Usage Example

From `/examples/button_usage.rs`:

```rust
// Different button variants
let primary = ButtonStyles::new(colors.clone()).primary().classes();
let secondary = ButtonStyles::new(colors.clone()).secondary().classes();
let success = ButtonStyles::new(colors.clone()).success().classes();
let warning = ButtonStyles::new(colors.clone()).warning().classes();
let error = ButtonStyles::new(colors.clone()).error().classes();
let ghost = ButtonStyles::new(colors.clone()).ghost().classes();
let link = ButtonStyles::new(colors.clone()).link().classes();

// Size variations
let xs = ButtonStyles::new(colors.clone()).primary().extra_small().classes();
let sm = ButtonStyles::new(colors.clone()).primary().small().classes();
let md = ButtonStyles::new(colors.clone()).primary().medium().classes();
let lg = ButtonStyles::new(colors.clone()).primary().large().classes();
let xl = ButtonStyles::new(colors.clone()).primary().extra_large().classes();

// State variations
let hover = ButtonStyles::new(colors.clone()).primary().hover().classes();
let active = ButtonStyles::new(colors.clone()).primary().active().classes();
let disabled = ButtonStyles::new(colors.clone()).primary().disabled().classes();
let loading = ButtonStyles::new(colors.clone()).primary().loading().classes();
```

## Color Usage Statistics

The codebase contains **233 Color:: references** across multiple files:

### Most Used Colors

Based on codebase analysis:
- `Color::Primary` - Used in buttons, text, backgrounds
- `Color::Success` - Success buttons and states
- `Color::Error` - Error buttons and states  
- `Color::TextPrimary` - Primary text content
- `Color::Interactive` - Interactive elements
- `Color::InteractiveHover` - Hover states
- `Color::Surface` - Card and surface backgrounds

### File Distribution

Color usage is distributed across:
- **Builders**: 19 files using Color::
- **Patterns**: 12 files using Color::
- **Examples**: 2 files using Color::
- **Core**: 2 files using Color::

## Advanced Customization

### Theme Comparison

```rust
// Default Jupiter theme
let jupiter_button = ButtonStyles::new(VibeColors::new()).primary().classes();

// Custom theme
let custom_button = ButtonStyles::new(VibeColors::with_overrides(|p| {
    p.primary = "indigo-600".to_string();
})).primary().classes();

// Compare outputs
println!("Jupiter: {}", jupiter_button);
println!("Custom: {}", custom_button);
```

### Convenience Functions

```rust
// Quick styling
let quick_button = button_styles(colors.clone())
    .warning()
    .small()
    .full_width()
    .classes();

// With custom animations
let animated_button = button_styles(colors.clone())
    .custom_classes("animate-pulse")
    .success()
    .medium()
    .classes();
```

## Testing Patterns

Common test patterns found in the codebase:

```rust
// Color resolution testing
assert_eq!(colors.resolve_color(Color::Primary), "jupiter-blue-500");
assert_eq!(colors.text_class(Color::Primary), "text-jupiter-blue-500");
assert_eq!(colors.bg_class(Color::Primary), "bg-jupiter-blue-500");

// Button styling testing
let primary = ButtonStyles::new(colors.clone()).primary().classes();
assert!(primary.contains("bg-jupiter-blue-500"));

// Selection ring testing
assert!(selected.contains("ring-jupiter-blue-300"));
assert!(!unselected.contains("ring-jupiter-blue-300"));

// Branded gradient testing
assert!(branded.contains("from-jupiter-navy-900/80"));
```

## Integration Tips

### With Web Frameworks

```rust
// React/JSX-like usage
let button_classes = ButtonStyles::new(theme)
    .primary()
    .large()
    .classes();

// Use in template
format!("<button class=\"{}\">{}</button>", button_classes, "Click me");
```

### With CSS-in-JS

```rust
// Generate style objects
let styles = ButtonStyles::new(theme)
    .primary()
    .custom("shadow-lg")
    .custom("transform")
    .classes();

// Convert to CSS-in-JS object
let css_object = styles.split_whitespace()
    .map(|class| format!("  {}: true,", class))
    .collect::<Vec<_>>()
    .join("\n");
```

This advanced usage guide covers the real-world patterns found throughout the Jupiter Design System codebase.