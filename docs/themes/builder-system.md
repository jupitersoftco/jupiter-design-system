# Builder System Documentation

The builder system provides chainable APIs for generating CSS classes with design system constraints. Builders offer a more granular, CSS-focused approach compared to the abstract pattern system.

## Overview

While patterns represent abstract concepts (e.g., "primary action"), builders generate specific CSS classes for styling. They provide:

- Type-safe CSS class generation
- Chainable API for intuitive usage
- Framework-agnostic output (Tailwind by default)
- Compile-time validation of design system rules

## 1. Button Builder (`builders/button.rs`)

The button builder provides fine-grained control over button styling.

### ButtonVariant
```rust
pub enum ButtonVariant {
    Primary,    // Main action buttons
    Secondary,  // Alternative actions
    Ghost,      // Minimal styling
    Link,       // Text-only buttons
}
```

### ButtonState
```rust
pub enum ButtonState {
    Default,
    Hover,
    Active,
    Disabled,
}
```

### Basic Usage
```rust
use jupiter_design_system::builders::*;

let colors = VibeColors::default();

// Simple primary button
let primary = button_styles(colors.clone())
    .variant(ButtonVariant::Primary)
    .size(Size::Medium)
    .classes();

// Ghost button with custom size
let ghost = button_styles(colors.clone())
    .variant(ButtonVariant::Ghost)
    .size(Size::Small)
    .full_width()
    .classes();

// Link style button
let link = button_styles(colors)
    .variant(ButtonVariant::Link)
    .state(ButtonState::Hover)
    .classes();
```

### Advanced Features
```rust
// From string parsing (useful for dynamic variants)
let classes = button_classes_from_strings(
    "primary",    // variant
    "large",      // size  
    Some("hover") // state
);

// Detailed builder options
let detailed = button_styles(colors)
    .primary()           // Shorthand for variant
    .large()             // Shorthand for size
    .rounded()           // Border radius
    .shadow()            // Box shadow
    .full_width()        // Width 100%
    .icon_left()         // Space for icon
    .loading()           // Loading state
    .classes();
```

## 2. Text Styles Builder (`builders/text.rs`)

Fine-grained typography control.

### Basic Usage
```rust
// Simple text styling
let heading = text_styles(colors.clone())
    .size(TextSize::XL)
    .weight(FontWeight::Bold)
    .color(Color::Primary)
    .classes();

// Body text with line height
let body = text_styles(colors.clone())
    .size(TextSize::Base)
    .leading(LineHeight::Relaxed)
    .color(Color::TextPrimary)
    .classes();

// Aligned and truncated text
let caption = text_styles(colors)
    .size(TextSize::SM)
    .align(TextAlign::Center)
    .truncate()
    .classes();
```

### Advanced Features
```rust
// Complete text styling
let styled_text = text_styles(colors)
    .size(TextSize::LG)
    .weight(FontWeight::Medium)
    .color(Color::Primary)
    .leading(LineHeight::Tight)
    .tracking(LetterSpacing::Wide)
    .align(TextAlign::Right)
    .transform(TextTransform::Uppercase)
    .decoration(TextDecoration::Underline)
    .truncate()
    .classes();

// From strings (for dynamic styling)
let dynamic_text = text_classes_from_strings(
    "large",     // size
    "bold",      // weight
    Some("primary") // color
);
```

## 3. Card Builder (`builders/card.rs`)

Container and layout styling.

### Basic Usage
```rust
// Simple card
let card = card_styles(colors.clone())
    .padding(Spacing::Medium)
    .rounded()
    .shadow()
    .classes();

// Interactive card
let interactive_card = card_styles(colors.clone())
    .padding(Spacing::Large)
    .rounded_lg()
    .shadow_md()
    .hover_shadow_lg()
    .border()
    .classes();

// Colored card
let colored_card = card_styles(colors)
    .background(Color::Surface)
    .border_color(Color::Primary)
    .padding_x(Spacing::Large)
    .padding_y(Spacing::Medium)
    .classes();
```

### Layout Features
```rust
// Card with sections
let sectioned_card = card_styles(colors)
    .header_padding(Spacing::Medium)
    .body_padding(Spacing::Large)
    .footer_padding(Spacing::Medium)
    .divide_y() // Dividers between sections
    .classes();

// Responsive card
let responsive_card = card_styles(colors)
    .padding(Spacing::Small)
    .md_padding(Spacing::Medium) // Medium screens
    .lg_padding(Spacing::Large)   // Large screens
    .classes();
```

## 4. Layout Builder (`builders/layout.rs`)

Flexbox and grid layout utilities.

### Basic Usage
```rust
// Flex row layout
let row = layout_styles(colors.clone())
    .flex()
    .direction(FlexDirection::Row)
    .gap(Spacing::Medium)
    .align(AlignItems::Center)
    .justify(JustifyContent::SpaceBetween)
    .classes();

// Grid layout
let grid = layout_styles(colors.clone())
    .grid()
    .cols(3)
    .gap(Spacing::Large)
    .classes();

// Responsive grid
let responsive_grid = layout_styles(colors)
    .grid()
    .cols(1)
    .md_cols(2)
    .lg_cols(3)
    .gap(Spacing::Medium)
    .classes();
```

### Advanced Layouts
```rust
// Complex flex layout
let complex_flex = layout_styles(colors)
    .flex()
    .direction(FlexDirection::Column)
    .md_direction(FlexDirection::Row) // Row on medium+
    .gap(Spacing::Small)
    .lg_gap(Spacing::Large)
    .align(AlignItems::Start)
    .md_align(AlignItems::Center)
    .justify(JustifyContent::Start)
    .wrap()
    .classes();

// Container with max width
let container = layout_styles(colors)
    .container()
    .max_width(MaxWidth::XL)
    .center() // mx-auto
    .padding_x(Spacing::Medium)
    .classes();
```

## 5. Interactive Builder (`builders/interactive.rs`)

Interaction state builders for any element.

### Basic Interactive Elements
```rust
// Hoverable element
let hoverable = HoverBuilder::new(colors.clone())
    .scale(105) // scale-105 on hover
    .brightness(110) // brightness-110 on hover
    .transition_all()
    .duration(200)
    .classes();

// Focusable element
let focusable = FocusBuilder::new(colors.clone())
    .ring(2) // ring-2 on focus
    .ring_color(Color::Primary)
    .ring_offset(2)
    .outline_none()
    .classes();

// Active state
let active = ActiveBuilder::new(colors)
    .scale(95) // scale-95 when pressed
    .brightness(90)
    .classes();
```

### Input Builder
```rust
// Text input styling
let input = InputBuilder::new(colors.clone())
    .size(Size::Medium)
    .variant(InputVariant::Outlined)
    .state(InputState::Default)
    .classes();

// Search input with icon
let search = InputBuilder::new(colors.clone())
    .variant(InputVariant::Filled)
    .icon_left()
    .rounded_full()
    .classes();

// Error state input
let error_input = InputBuilder::new(colors)
    .variant(InputVariant::Outlined)
    .state(InputState::Error)
    .message("Invalid email")
    .classes();
```

## 6. Selection Builder (`builders/selection.rs`)

Checkbox, radio, and toggle styling.

### Basic Usage
```rust
// Checkbox styling
let checkbox = selection_styles(colors.clone())
    .variant(SelectionVariant::Checkbox)
    .size(Size::Medium)
    .checked(true)
    .classes();

// Radio button
let radio = selection_styles(colors.clone())
    .variant(SelectionVariant::Radio)
    .size(Size::Small)
    .checked(false)
    .classes();

// Toggle switch
let toggle = selection_styles(colors)
    .variant(SelectionVariant::Toggle)
    .size(Size::Large)
    .checked(true)
    .disabled(false)
    .classes();
```

### Advanced Features
```rust
// Custom styled checkbox
let custom_checkbox = selection_styles(colors)
    .variant(SelectionVariant::Checkbox)
    .color(Color::Primary)
    .hover_color(Color::InteractiveHover)
    .checked_bg(Color::Primary)
    .focus_ring(true)
    .classes();

// From strings
let dynamic_selection = selection_classes_from_strings(
    "checkbox",  // variant
    "medium",    // size
    Some("true") // checked
);
```

## 7. State Builder (`builders/state.rs`)

Component state visualization.

### Basic Usage
```rust
// Loading state
let loading = state_styles(colors.clone())
    .loading()
    .size(Size::Medium)
    .classes();

// Success state
let success = state_styles(colors.clone())
    .success()
    .with_icon()
    .classes();

// Error state with message
let error = state_styles(colors)
    .error()
    .with_message("Something went wrong")
    .classes();
```

### Advanced States
```rust
// Skeleton loader
let skeleton = state_styles(colors.clone())
    .skeleton()
    .height(SkeletonHeight::Text)
    .animate()
    .classes();

// Progress indicator
let progress = state_styles(colors)
    .progress(75) // 75% complete
    .color(Color::Primary)
    .height(Size::Small)
    .classes();
```

## 8. Product Builder (`builders/product.rs`)

E-commerce specific builders.

### Basic Usage
```rust
// Product card
let product_card = product_builder(colors.clone())
    .variant(ProductVariant::Card)
    .image_aspect(AspectRatio::Square)
    .show_price(true)
    .show_rating(true)
    .classes();

// Product list item
let list_item = product_builder(colors.clone())
    .variant(ProductVariant::List)
    .image_size(ImageSize::Thumbnail)
    .show_description(true)
    .classes();

// Product grid
let grid_item = product_builder(colors)
    .variant(ProductVariant::Grid)
    .on_sale(true)
    .out_of_stock(false)
    .classes();
```

### Advanced Features
```rust
// Complete product display
let featured_product = product_builder(colors)
    .variant(ProductVariant::Featured)
    .image_aspect(AspectRatio::Wide)
    .badge("New")
    .sale_percentage(25)
    .show_price(true)
    .show_original_price(true)
    .show_rating(true)
    .rating_value(4.5)
    .show_reviews_count(true)
    .reviews_count(128)
    .wishlist_button(true)
    .quick_add_button(true)
    .classes();
```

## Builder Composition

Builders can be composed for complex components:

```rust
// Complete button with all builders
let complete_button = {
    let button = button_styles(colors.clone())
        .primary()
        .medium()
        .classes();
    
    let hover = HoverBuilder::new(colors.clone())
        .brightness(110)
        .classes();
    
    let focus = FocusBuilder::new(colors.clone())
        .ring(2)
        .ring_color(Color::Primary)
        .classes();
    
    let active = ActiveBuilder::new(colors)
        .scale(95)
        .classes();
    
    format!("{} {} {} {}", button, hover, focus, active)
};

// Card with layout
let card_with_layout = {
    let card = card_styles(colors.clone())
        .padding(Spacing::Large)
        .rounded_lg()
        .shadow_md()
        .classes();
    
    let layout = layout_styles(colors)
        .flex()
        .direction(FlexDirection::Column)
        .gap(Spacing::Medium)
        .classes();
    
    format!("{} {}", card, layout)
};
```

## Builder vs Pattern Comparison

### When to Use Builders
- Need fine-grained control over CSS classes
- Building custom components
- Working directly with CSS frameworks
- Need responsive utilities
- Want specific visual effects

### When to Use Patterns
- Want semantic, meaningful abstractions
- Building consistent UI systems
- Need accessibility built-in
- Want to express intent, not implementation
- Building reusable component libraries

### Example Comparison
```rust
// Pattern approach (semantic)
let button = primary_button(colors.clone())
    .hero_prominence()
    .classes();

// Builder approach (visual)
let button = button_styles(colors)
    .variant(ButtonVariant::Primary)
    .size(Size::Large)
    .shadow_lg()
    .padding_x(Spacing::XLarge)
    .classes();
```

## Best Practices

1. **Use builders for implementation details** - When you need specific CSS classes
2. **Use patterns for semantic meaning** - When expressing what something does
3. **Compose builders** - Combine multiple builders for complex components
4. **Keep it type-safe** - Use enums instead of strings where possible
5. **Consider responsiveness** - Use responsive variants (md_, lg_) for adaptive designs
6. **Cache common combinations** - Store frequently used builder combinations