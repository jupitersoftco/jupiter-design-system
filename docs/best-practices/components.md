# Component Usage Best Practices

## 🧩 Component Philosophy

The Jupiter Design System provides flexible component builders that generate CSS classes while maintaining consistency, accessibility, and framework agnosticism.

### Core Principles

1. **Builder Pattern**: Fluent, chainable APIs for component configuration
2. **Framework Agnostic**: Generate CSS classes usable with any frontend framework
3. **Type Safety**: Compile-time validation of component configurations
4. **State Management**: Built-in support for interactive states and variants
5. **Theme Integration**: Automatic theme-aware styling

## 🔧 Component Builder System

### Available Builders

The system provides builders for various component types:

```rust
// Core component builders
ButtonStyles      // Button components with variants and states
CardStyles        // Card layouts and containers
TextStyles        // Text components with typography
LayoutStyles      // Layout containers and arrangements

// Specialized builders
InteractiveBase   // Interactive elements with pseudo-states
ProductStyles     // E-commerce product displays
SelectionStyles   // Selection controls (tabs, filters, chips)
StateStyles       // Application state displays (loading, error, empty)
```

### Builder Pattern Usage

All builders follow the same pattern:

```rust
// 1. Create builder instance
let builder = ComponentStyles::new(color_provider);

// 2. Configure with fluent API
let configured = builder
    .variant(Variant::Primary)
    .size(Size::Large)
    .state(State::Default);

// 3. Build final CSS classes
let classes = configured.classes();
```

## 🎨 Button Components

### Basic Button Usage

```rust
use jupiter_design_system::prelude::*;

let colors = VibeColors::new();

// Primary button
let primary_button = button_styles(colors.clone())
    .primary()
    .classes();

// Secondary button with custom size
let secondary_button = button_styles(colors.clone())
    .secondary()
    .large()
    .classes();

// Full-width button
let full_width_button = button_styles(colors.clone())
    .primary()
    .full_width()
    .classes();
```

### Button Variants

```rust
// Semantic button variants
let submit_button = button_styles(colors.clone())
    .variant(ButtonVariant::Success)
    .classes();

let cancel_button = button_styles(colors.clone())
    .variant(ButtonVariant::Secondary)
    .classes();

let delete_button = button_styles(colors.clone())
    .variant(ButtonVariant::Error)
    .classes();

let info_button = button_styles(colors.clone())
    .variant(ButtonVariant::Ghost)
    .classes();
```

### Button States

```rust
// Interactive button states
let default_state = button_styles(colors.clone())
    .primary()
    .state(ButtonState::Default)
    .classes();

let hover_state = button_styles(colors.clone())
    .primary()
    .state(ButtonState::Hover)
    .classes();

let disabled_state = button_styles(colors.clone())
    .primary()
    .state(ButtonState::Disabled)
    .classes();

let loading_state = button_styles(colors.clone())
    .primary()
    .state(ButtonState::Loading)
    .classes();
```

### Interactive Button Builder

For complex interactive behaviors:

```rust
use jupiter_design_system::builders::interactive::*;

let interactive_button = interactive_button(colors)
    .primary()
    .hover()
        .darken()
        .scale_105()
        .shadow_lg()
    .focus()
        .ring_primary()
        .ring_offset_2()
        .outline_none()
    .active()
        .scale_95()
        .shadow_sm()
    .disabled()
        .opacity_50()
        .cursor_not_allowed()
    .build();
```

## 📋 Card Components

### Basic Card Usage

```rust
// Standard card
let basic_card = card_styles(colors.clone())
    .elevation(CardElevation::Medium)
    .spacing(CardSpacing::Large)
    .classes();

// Interactive card
let interactive_card = card_styles(colors.clone())
    .elevation(CardElevation::Low)
    .interaction(CardInteraction::Clickable)
    .spacing(CardSpacing::Medium)
    .classes();
```

### Card Layout Sections

```rust
// Card header with divider
let card_header = card_header_styles(colors.clone())
    .divider_bottom()
    .spacing_md()
    .classes();

// Card content area
let card_content = card_content_styles(colors.clone())
    .spacing_lg()
    .classes();

// Card footer with actions
let card_footer = card_footer_styles(colors.clone())
    .divider_top()
    .spacing_md()
    .alignment_end()
    .classes();
```

### Complete Card Example

```rust
fn create_product_card(colors: impl ColorProvider) -> (String, String, String) {
    let header = card_header_styles(colors.clone())
        .divider_bottom()
        .spacing_md()
        .classes();
    
    let content = card_content_styles(colors.clone())
        .spacing_lg()
        .classes();
    
    let footer = card_footer_styles(colors)
        .divider_top()
        .spacing_md()
        .alignment_between()
        .classes();
    
    (header, content, footer)
}
```

## 📝 Text Components

### Typography Integration

```rust
// Semantic text styles
let page_title = text_styles(colors.clone())
    .typography(Typography::Heading1)
    .color(Color::TextPrimary)
    .classes();

let section_heading = text_styles(colors.clone())
    .typography(Typography::Heading2)
    .color(Color::TextPrimary)
    .classes();

let body_text = text_styles(colors.clone())
    .typography(Typography::Body)
    .color(Color::TextSecondary)
    .classes();

let caption_text = text_styles(colors.clone())
    .typography(Typography::Caption)
    .color(Color::TextTertiary)
    .classes();
```

### Text State Variations

```rust
// Success message
let success_text = text_styles(colors.clone())
    .typography(Typography::Body)
    .color(Color::Success)
    .weight(FontWeight::Medium)
    .classes();

// Error message
let error_text = text_styles(colors.clone())
    .typography(Typography::Body)
    .color(Color::Error)
    .weight(FontWeight::Medium)
    .classes();

// Muted text
let muted_text = text_styles(colors.clone())
    .typography(Typography::BodySmall)
    .color(Color::TextTertiary)
    .classes();
```

### Text Truncation and Clamping

```rust
// Single-line truncation
let truncated_text = text_styles(colors.clone())
    .typography(Typography::Body)
    .truncate(true)
    .classes();

// Multi-line clamping
let clamped_text = text_clamp_style(3)  // 3 lines
    + " " + &text_styles(colors)
        .typography(Typography::Body)
        .classes();
```

## 🏗️ Layout Components

### Container Layouts

```rust
// Page container
let page_container = layout_styles(colors.clone())
    .spacing_lg()
    .direction_vertical()
    .alignment_center()
    .classes();

// Content grid
let content_grid = layout_styles(colors.clone())
    .spacing_md()
    .direction_horizontal()
    .alignment_start()
    .classes();

// Sidebar layout
let sidebar_layout = layout_styles(colors.clone())
    .spacing_sm()
    .direction_vertical()
    .divider_right()
    .classes();
```

### Responsive Layouts

```rust
// Mobile-first responsive layout
let responsive_layout = layout_styles(colors.clone())
    .spacing_sm()                    // Mobile: tight spacing
    .direction_vertical()            // Mobile: stack vertically
    .responsive_spacing(Breakpoint::Tablet, LayoutSpacing::MD)
    .responsive_direction(Breakpoint::Tablet, LayoutDirection::Horizontal)
    .responsive_spacing(Breakpoint::Desktop, LayoutSpacing::LG)
    .classes();
```

## 🎯 Interactive Elements

### Form Inputs

```rust
// Standard input with interactive states
let form_input = interactive_input(colors.clone())
    .base_style()
    .hover()
        .border_color(Color::Interactive)
        .shadow_sm()
    .focus()
        .border_color(Color::Interactive)
        .ring_color(Color::Interactive)
        .outline_none()
    .disabled()
        .opacity_50()
        .cursor_not_allowed()
    .build();
```

### Interactive Cards

```rust
// Clickable card with hover effects
let clickable_card = interactive_element(colors.clone())
    .base("p-6 rounded-lg bg-white border border-gray-200 cursor-pointer")
    .hover()
        .shadow_lg()
        .scale_102()
        .border_color(Color::Interactive)
    .focus()
        .ring_color(Color::Interactive)
        .ring_offset_2()
    .active()
        .scale_100()
    .build();
```

### Navigation Elements

```rust
// Navigation link
let nav_link = interactive_element(colors.clone())
    .base("px-4 py-2 rounded-md text-sm font-medium")
    .hover()
        .bg_color(Color::Interactive)
        .text_color(Color::TextInverse)
    .focus()
        .ring_color(Color::Interactive)
        .outline_none()
    .build();
```

## 🎪 Specialized Components

### Product Display Components

```rust
// Product showcase
let product_showcase = product_showcase_styles(colors.clone())
    .featured(true)
    .size(Size::Large)
    .classes();

// Product grid tile
let product_tile = product_tile_styles(colors.clone())
    .compact(true)
    .interactive(true)
    .classes();

// Product preview
let product_preview = product_preview_styles(colors.clone())
    .size(Size::Medium)
    .classes();
```

### Selection Components

```rust
// Tab selection
let tab_selection = tab_selection_styles(colors.clone())
    .active(true)
    .size(Size::Medium)
    .classes();

// Filter chips
let filter_chip = chip_selection_styles(colors.clone())
    .selected(true)
    .removable(true)
    .classes();

// Filter dropdown
let filter_selection = filter_selection_styles(colors.clone())
    .multiple(true)
    .searchable(true)
    .classes();
```

### State Display Components

```rust
// Loading state
let loading_state = loading_state_styles(colors.clone())
    .with_spinner(true)
    .message("Loading content...")
    .classes();

// Error state
let error_state = error_state_styles(colors.clone())
    .with_icon(true)
    .retry_action(true)
    .classes();

// Empty state
let empty_state = empty_state_styles(colors.clone())
    .with_illustration(true)
    .primary_action(true)
    .classes();

// Success state
let success_state = success_state_styles(colors.clone())
    .with_icon(true)
    .auto_dismiss(true)
    .classes();
```

## 🔄 Component Composition

### Building Complex Components

```rust
// Complete form field
fn create_form_field(
    label: &str,
    placeholder: &str,
    colors: impl ColorProvider,
) -> (String, String, String) {
    let label_classes = text_styles(colors.clone())
        .typography(Typography::Label)
        .color(Color::TextSecondary)
        .weight(FontWeight::Medium)
        .classes();
    
    let input_classes = interactive_input(colors.clone())
        .base_style()
        .hover().border_primary()
        .focus().border_primary().ring_primary()
        .build();
    
    let layout_classes = layout_styles(colors)
        .spacing_sm()
        .direction_vertical()
        .alignment_start()
        .classes();
    
    (label_classes, input_classes, layout_classes)
}
```

### Modal Component

```rust
fn create_modal_components(colors: impl ColorProvider) -> (String, String, String, String) {
    let overlay_classes = "fixed inset-0 bg-black bg-opacity-50 z-40";
    
    let modal_container = layout_styles(colors.clone())
        .spacing_none()
        .direction_vertical()
        .alignment_center()
        .classes() + " fixed inset-0 z-50 flex items-center justify-center p-4";
    
    let modal_content = card_styles(colors.clone())
        .elevation(CardElevation::High)
        .spacing(CardSpacing::Large)
        .classes() + " max-w-md w-full max-h-screen overflow-auto";
    
    let modal_actions = layout_styles(colors)
        .spacing_sm()
        .direction_horizontal()
        .alignment_end()
        .divider_top()
        .classes();
    
    (overlay_classes.to_string(), modal_container, modal_content, modal_actions)
}
```

## 🚦 Component Usage Guidelines

### Do's ✅

1. **Use Semantic Variants**
   ```rust
   // ✅ Good: Semantic meaning
   let submit_button = button_styles(colors)
       .variant(ButtonVariant::Success)
       .classes();
   
   let cancel_button = button_styles(colors)
       .variant(ButtonVariant::Secondary)
       .classes();
   ```

2. **Configure Before Building**
   ```rust
   // ✅ Good: Complete configuration
   let button = button_styles(colors)
       .variant(ButtonVariant::Primary)
       .size(Size::Large)
       .state(ButtonState::Default)
       .full_width(false)
       .classes();
   ```

3. **Use Interactive Builders for Complex States**
   ```rust
   // ✅ Good: Rich interactive behavior
   let interactive_card = interactive_element(colors)
       .base("p-4 rounded-lg")
       .hover().shadow_lg().scale_105()
       .focus().ring_primary()
       .build();
   ```

### Don'ts ❌

1. **Don't Mix Component Systems**
   ```rust
   // ❌ Bad: Mixing with custom classes
   let mixed_button = button_styles(colors).primary().classes() + " my-custom-class";
   
   // ✅ Good: Use builder extensions
   let extended_button = button_styles(colors)
       .primary()
       .custom_classes(vec!["my-design-system-class".to_string()])
       .classes();
   ```

2. **Don't Skip State Management**
   ```rust
   // ❌ Bad: No disabled state
   let button = button_styles(colors).primary().classes();
   
   // ✅ Good: Proper state handling
   let button = button_styles(colors)
       .primary()
       .state(if is_disabled { ButtonState::Disabled } else { ButtonState::Default })
       .classes();
   ```

3. **Don't Ignore Responsive Design**
   ```rust
   // ❌ Bad: Fixed layout
   let layout = layout_styles(colors)
       .direction_horizontal()
       .classes();
   
   // ✅ Good: Responsive layout
   let layout = layout_styles(colors)
       .direction_vertical()  // Mobile first
       .responsive_direction(Breakpoint::Tablet, LayoutDirection::Horizontal)
       .classes();
   ```

## 🧪 Testing Components

### Component Generation Tests

```rust
#[cfg(test)]
mod component_tests {
    use super::*;

    #[test]
    fn test_button_variant_generation() {
        let colors = VibeColors::new();
        
        let primary = button_styles(colors.clone()).primary().classes();
        let secondary = button_styles(colors.clone()).secondary().classes();
        let success = button_styles(colors).success().classes();
        
        assert!(primary.contains("bg-jupiter-blue-500"));
        assert!(secondary.contains("border"));
        assert!(success.contains("bg-green-500"));
    }

    #[test]
    fn test_interactive_state_building() {
        let colors = VibeColors::new();
        
        let interactive = interactive_button(colors)
            .primary()
            .hover().darken()
            .focus().ring_primary()
            .build();
        
        assert!(interactive.contains("hover:"));
        assert!(interactive.contains("focus:"));
    }
}
```

### Integration Tests

```rust
#[test]
fn test_component_composition() {
    let colors = VibeColors::new();
    
    // Test that components work together
    let (header, content, footer) = create_product_card(colors);
    
    assert!(!header.is_empty());
    assert!(!content.is_empty());
    assert!(!footer.is_empty());
    
    // Ensure no class conflicts
    assert!(!header.contains(&content));
    assert!(!content.contains(&footer));
}
```

## ⚡ Performance Optimization

### Component Caching

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ComponentCache {
    button_cache: HashMap<(ButtonVariant, Size, ButtonState), String>,
    card_cache: HashMap<(CardElevation, CardSpacing), String>,
}

impl ComponentCache {
    pub fn get_button(&mut self, 
        variant: ButtonVariant,
        size: Size,
        state: ButtonState,
        colors: impl ColorProvider,
    ) -> &String {
        self.button_cache.entry((variant, size, state)).or_insert_with(|| {
            button_styles(colors)
                .variant(variant)
                .size(size)
                .state(state)
                .classes()
        })
    }
}
```

### Lazy Component Generation

```rust
lazy_static! {
    static ref COMMON_BUTTONS: HashMap<&'static str, String> = {
        let colors = VibeColors::new();
        let mut map = HashMap::new();
        
        map.insert("primary", button_styles(colors.clone()).primary().classes());
        map.insert("secondary", button_styles(colors.clone()).secondary().classes());
        map.insert("success", button_styles(colors).success().classes());
        
        map
    };
}

pub fn get_common_button(variant: &str) -> Option<&'static String> {
    COMMON_BUTTONS.get(variant)
}
```

## 🔗 Framework Integration Examples

### Dioxus Integration

```rust
// Button component
#[component]
fn Button(
    variant: ButtonVariant,
    size: Size,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let colors = use_context::<VibeColors>();
    let classes = button_styles(colors)
        .variant(variant)
        .size(size)
        .classes();
    
    rsx! {
        button {
            class: "{classes}",
            onclick: move |e| onclick.call(e),
            {children}
        }
    }
}
```

### Yew Integration

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub size: Size,
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let colors = use_context::<VibeColors>().unwrap();
    let classes = button_styles(colors)
        .variant(props.variant)
        .size(props.size)
        .classes();
    
    html! {
        <button class={classes} onclick={&props.onclick}>
            {for props.children.iter()}
        </button>
    }
}
```

## 🔗 Related Documentation

- [Color System](./colors.md) - Color integration in components
- [Typography](./typography.md) - Text styling in components
- [Spacing & Layout](./spacing-layout.md) - Layout patterns and spacing
- [Theming Guide](./theming.md) - Custom theme integration
- [Accessibility Guide](./accessibility.md) - Accessible component practices
- [Examples](./examples/) - Complete component implementations