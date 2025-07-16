# Jupiter Design System - Theme System Architecture

## Overview

The Jupiter Design System uses a trait-based architecture that separates semantic meaning from visual implementation. This allows for flexible theming while maintaining design consistency and accessibility.

## Core Architecture Principles

### 1. Trait-Based Design

The system is built around Rust traits that define **what** something does, not **how** it looks:

```rust
// Define semantic behavior
pub trait ColorProvider {
    fn palette(&self) -> &ColorPalette;
    fn resolve_color(&self, color: Color) -> &str;
    fn text_class(&self, color: Color) -> String;
    fn bg_class(&self, color: Color) -> String;
    fn border_class(&self, color: Color) -> String;
}

// Define theme identity
pub trait Theme {
    fn name(&self) -> &str;
}
```

### 2. Semantic Color System

Colors are referenced by semantic meaning, not implementation:

```rust
#[derive(Debug, Clone, Copy)]
pub enum Color {
    // Brand colors
    Primary, Secondary, Accent,
    
    // Semantic colors  
    Success, Warning, Error, Info,
    
    // Interactive states
    Interactive, InteractiveHover, InteractiveActive, InteractiveDisabled,
    
    // Text hierarchy
    TextPrimary, TextSecondary, TextTertiary, TextInverse,
}
```

### 3. Pattern Composition

Complex components are built by composing abstract patterns:

```rust
#[derive(Debug, Clone)]
pub struct ButtonPattern<C: ColorProvider + Clone> {
    action_semantics: ActionSemantics<C>,     // What does this button do?
    interactive_element: InteractiveElement<C>, // How does it behave?
    focus_management: FocusManagement<C>,     // How does focus work?
}
```

## Architecture Layers

### Layer 1: Core Types (`src/core/`)

Defines fundamental design tokens and their provider traits:

- **Color System** (`color.rs`) - Semantic color definitions and `ColorProvider` trait
- **Typography** (`typography.rs`) - Text hierarchy tokens and `TypographyProvider` trait  
- **Spacing** (`spacing.rs`) - Layout spacing tokens and `SpacingProvider` trait
- **Sizing** (`sizing.rs`) - Component size tokens and `SizeProvider` trait

> **Note**: Currently, only `ColorProvider` is actively used throughout the system. The other provider traits (`SizeProvider`, `SpacingProvider`, `TypographyProvider`) are defined but not yet integrated into all patterns. This allows for future expansion while maintaining the current focus on color theming.

### Layer 2: Abstract Patterns (`src/patterns/`)

Defines reusable interaction and semantic patterns:

- **Action Semantics** (`actions.rs`) - What an action means and its importance
- **Interactive Elements** (`interactions.rs`) - How elements behave when interacted with
- **Focus Management** (`focus.rs`) - Keyboard navigation and accessibility
- **Typography Patterns** (`typography.rs`) - Text hierarchy and styling patterns

### Layer 3: Concrete Themes (`src/themes/`)

Implements the traits with specific visual designs:

- **VibeTheme** - The default Jupiter Design System theme
- **VibeColors** - Color palette implementation for Vibe theme

### Layer 4: Component Builders (`src/builders/`)

Provides chainable APIs for generating component styles:

- **ButtonBuilder** - Generates button CSS classes
- **TextStyles** - Generates typography CSS classes
- **CardStyles** - Generates card layout CSS classes

## Key Design Benefits

### 1. Separation of Concerns

```rust
// Semantic meaning (what it does)
let action = ActionSemantics::new(colors)
    .primary()
    .urgent();

// Visual behavior (how it behaves) 
let interaction = InteractiveElement::new(colors)
    .prominent_interaction()
    .hoverable();

// Focus behavior (accessibility)
let focus = FocusManagement::new(colors)
    .button()
    .prominent_focus();
```

### 2. Theme Swapping

```rust
// Works with any theme that implements ColorProvider
fn create_button<T: ColorProvider + Clone>(theme: T) -> String {
    primary_button(theme)
        .hero_prominence()
        .classes()
}

// Use with default theme
let default_button = create_button(VibeColors::default());

// Use with custom theme
let custom_button = create_button(MyCustomTheme::new());
```

### 3. Progressive Enhancement

```rust
// Start simple
let basic = button_pattern(colors).classes();

// Add semantic meaning
let semantic = button_pattern(colors)
    .primary_action()
    .classes();

// Add interaction behavior
let interactive = button_pattern(colors)
    .primary_action()
    .prominent_interaction()
    .classes();

// Add accessibility
let complete = button_pattern(colors)
    .primary_action() 
    .prominent_interaction()
    .prominent_focus()
    .classes();
```

## Theme Implementation Strategy

### Default Implementation Pattern

1. **Define the trait** with required behavior
2. **Provide a default implementation** that most themes can use
3. **Allow selective overrides** for theme-specific customization

```rust
pub trait ColorProvider {
    fn palette(&self) -> &ColorPalette;
    
    // Default implementations that most themes can use
    fn text_class(&self, color: Color) -> String {
        format!("text-{}", self.resolve_color(color))
    }
    
    // Themes can override if they need custom behavior
    fn bg_class(&self, color: Color) -> String {
        format!("bg-{}", self.resolve_color(color))
    }
}
```

### Composition over Inheritance

Rather than deep inheritance hierarchies, the system composes behavior:

```rust
// Each pattern is independent and composable
pub struct ButtonPattern<C> {
    action_semantics: ActionSemantics<C>,
    interactive_element: InteractiveElement<C>,
    focus_management: FocusManagement<C>,
}

impl<C: ColorProvider + Clone> ButtonPattern<C> {
    pub fn classes(self) -> String {
        // Compose the final result from all patterns
        let action_classes = self.action_semantics.classes();
        let interactive_classes = self.interactive_element.classes();
        let focus_classes = self.focus_management.classes();
        
        format!("{} {} {}", action_classes, interactive_classes, focus_classes)
    }
}
```

## File Organization

```
src/
├── core/           # Fundamental design tokens and types
│   ├── color.rs    # Color system and ColorProvider trait
│   ├── typography.rs
│   ├── spacing.rs
│   └── sizing.rs
├── patterns/       # Abstract, reusable patterns
│   ├── actions.rs  # Action semantics and intent
│   ├── interactions.rs # Interactive behavior patterns
│   ├── focus.rs    # Focus and accessibility patterns
│   ├── typography.rs # Typography hierarchy patterns
│   └── mod.rs
├── themes/         # Concrete theme implementations
│   └── mod.rs      # VibeTheme and VibeColors
├── builders/       # Component-specific builders
│   ├── button.rs
│   ├── text.rs
│   └── mod.rs
└── lib.rs
```

This architecture enables:

- **Consistent behavior** across all components
- **Easy theme creation** by implementing simple traits
- **Flexible customization** without breaking core functionality
- **Type safety** through Rust's trait system
- **Future extensibility** through composition