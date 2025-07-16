# Basic Usage Examples

This guide demonstrates fundamental usage patterns for the Jupiter Design System, perfect for getting started or as a reference for common scenarios.

## 🎯 Overview

Learn the essential patterns for:
- Creating basic components
- Working with themes
- Building simple layouts
- Handling interactive states

## 🚀 Getting Started

### Setup

```rust
use jupiter_design_system::prelude::*;
use jupiter_design_system::themes::VibeColors;

// Create your theme
let colors = VibeColors::new();
```

## 🔘 Button Examples

### Basic Button Variants

```rust
// Primary button (main actions)
let primary_button = button_styles(colors.clone())
    .primary()
    .classes();
// Result: "inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-md bg-jupiter-blue-500 text-white hover:bg-jupiter-blue-600 focus:outline-none focus:ring-2 focus:ring-jupiter-blue-500 focus:ring-offset-2"

// Secondary button (supporting actions)
let secondary_button = button_styles(colors.clone())
    .secondary()
    .classes();

// Success button (positive actions)
let success_button = button_styles(colors.clone())
    .success()
    .classes();

// Warning button (caution actions)
let warning_button = button_styles(colors.clone())
    .warning()
    .classes();

// Error button (destructive actions)
let error_button = button_styles(colors.clone())
    .error()
    .classes();

// Ghost button (subtle actions)
let ghost_button = button_styles(colors.clone())
    .ghost()
    .classes();
```

### Button Sizes

```rust
// Extra small button
let xs_button = button_styles(colors.clone())
    .primary()
    .size(Size::XSmall)
    .classes();

// Small button
let small_button = button_styles(colors.clone())
    .primary()
    .size(Size::Small)
    .classes();

// Medium button (default)
let medium_button = button_styles(colors.clone())
    .primary()
    .size(Size::Medium)
    .classes();

// Large button
let large_button = button_styles(colors.clone())
    .primary()
    .size(Size::Large)
    .classes();

// Extra large button
let xl_button = button_styles(colors.clone())
    .primary()
    .size(Size::XLarge)
    .classes();
```

### Button States

```rust
// Default state
let default_state = button_styles(colors.clone())
    .primary()
    .state(ButtonState::Default)
    .classes();

// Hover state (for previewing hover effect)
let hover_state = button_styles(colors.clone())
    .primary()
    .state(ButtonState::Hover)
    .classes();

// Active state (pressed)
let active_state = button_styles(colors.clone())
    .primary()
    .state(ButtonState::Active)
    .classes();

// Disabled state
let disabled_state = button_styles(colors.clone())
    .primary()
    .state(ButtonState::Disabled)
    .classes();

// Loading state
let loading_state = button_styles(colors.clone())
    .primary()
    .state(ButtonState::Loading)
    .classes();
```

### Full-Width Buttons

```rust
// Full-width button for mobile or form submissions
let full_width_button = button_styles(colors.clone())
    .primary()
    .full_width()
    .classes();

// Large full-width button for CTAs
let cta_button = button_styles(colors.clone())
    .primary()
    .size(Size::Large)
    .full_width()
    .classes();
```

## 📝 Text Examples

### Typography Hierarchy

```rust
// Main page heading
let page_title = text_styles(colors.clone())
    .typography(Typography::Heading1)
    .color(Color::TextPrimary)
    .classes();

// Section heading
let section_heading = text_styles(colors.clone())
    .typography(Typography::Heading2)
    .color(Color::TextPrimary)
    .classes();

// Subsection heading
let subsection_heading = text_styles(colors.clone())
    .typography(Typography::Heading3)
    .color(Color::TextPrimary)
    .classes();

// Body text
let body_text = text_styles(colors.clone())
    .typography(Typography::Body)
    .color(Color::TextSecondary)
    .classes();

// Small body text
let small_text = text_styles(colors.clone())
    .typography(Typography::BodySmall)
    .color(Color::TextSecondary)
    .classes();

// Caption text
let caption_text = text_styles(colors.clone())
    .typography(Typography::Caption)
    .color(Color::TextTertiary)
    .classes();

// Label text (for forms)
let label_text = text_styles(colors.clone())
    .typography(Typography::Label)
    .color(Color::TextPrimary)
    .classes();
```

### Text Colors

```rust
// Primary text (main content)
let primary_text = text_styles(colors.clone())
    .typography(Typography::Body)
    .color(Color::TextPrimary)
    .classes();

// Secondary text (supporting content)
let secondary_text = text_styles(colors.clone())
    .typography(Typography::Body)
    .color(Color::TextSecondary)
    .classes();

// Tertiary text (metadata, captions)
let tertiary_text = text_styles(colors.clone())
    .typography(Typography::Body)
    .color(Color::TextTertiary)
    .classes();

// Brand colored text
let brand_text = text_styles(colors.clone())
    .typography(Typography::Body)
    .color(Color::Primary)
    .classes();

// Success text
let success_text = text_styles(colors.clone())
    .typography(Typography::Body)
    .color(Color::Success)
    .classes();

// Error text
let error_text = text_styles(colors.clone())
    .typography(Typography::Body)
    .color(Color::Error)
    .classes();
```

## 📋 Card Examples

### Basic Cards

```rust
// Simple card
let basic_card = card_styles(colors.clone())
    .elevation(CardElevation::Low)
    .spacing(CardSpacing::Medium)
    .classes();

// Elevated card
let elevated_card = card_styles(colors.clone())
    .elevation(CardElevation::Medium)
    .spacing(CardSpacing::Large)
    .classes();

// High elevation card (modals, popovers)
let high_card = card_styles(colors.clone())
    .elevation(CardElevation::High)
    .spacing(CardSpacing::Medium)
    .classes();
```

### Interactive Cards

```rust
// Clickable card
let clickable_card = card_styles(colors.clone())
    .elevation(CardElevation::Low)
    .interaction(CardInteraction::Clickable)
    .spacing(CardSpacing::Medium)
    .classes();

// Hoverable card
let hoverable_card = card_styles(colors.clone())
    .elevation(CardElevation::Low)
    .interaction(CardInteraction::Hover)
    .spacing(CardSpacing::Medium)
    .classes();
```

### Card Sections

```rust
// Card header
let card_header = card_header_styles(colors.clone())
    .divider_bottom()
    .spacing_md()
    .classes();

// Card content
let card_content = card_content_styles(colors.clone())
    .spacing_lg()
    .classes();

// Card footer
let card_footer = card_footer_styles(colors.clone())
    .divider_top()
    .spacing_md()
    .alignment_end()
    .classes();
```

## 🏗️ Layout Examples

### Basic Layouts

```rust
// Vertical stack layout
let vertical_layout = layout_styles(colors.clone())
    .direction_vertical()
    .spacing_md()
    .alignment_start()
    .classes();

// Horizontal layout
let horizontal_layout = layout_styles(colors.clone())
    .direction_horizontal()
    .spacing_md()
    .alignment_center()
    .classes();

// Centered layout
let centered_layout = layout_styles(colors.clone())
    .direction_vertical()
    .spacing_lg()
    .alignment_center()
    .classes();

// Space between layout
let space_between_layout = layout_styles(colors.clone())
    .direction_horizontal()
    .spacing_md()
    .alignment_between()
    .classes();
```

### Layout with Dividers

```rust
// Top divider
let top_divider_layout = layout_styles(colors.clone())
    .divider_top()
    .spacing_md()
    .classes();

// Bottom divider
let bottom_divider_layout = layout_styles(colors.clone())
    .divider_bottom()
    .spacing_md()
    .classes();

// Left sidebar divider
let sidebar_layout = layout_styles(colors.clone())
    .divider_right()
    .spacing_sm()
    .direction_vertical()
    .classes();
```

## 🎨 Color Usage Examples

### Background Colors

```rust
// Primary background
let primary_bg = colors.bg_class(Color::Primary);

// Surface background (cards, panels)
let surface_bg = colors.bg_class(Color::Surface);

// Page background
let page_bg = colors.bg_class(Color::Background);

// Success background
let success_bg = colors.bg_class(Color::Success);

// Error background
let error_bg = colors.bg_class(Color::Error);
```

### Text Colors

```rust
// Primary text color
let primary_text_color = colors.text_class(Color::TextPrimary);

// Secondary text color
let secondary_text_color = colors.text_class(Color::TextSecondary);

// Brand color text
let brand_text_color = colors.text_class(Color::Primary);

// Success text color
let success_text_color = colors.text_class(Color::Success);

// Error text color
let error_text_color = colors.text_class(Color::Error);
```

### Border Colors

```rust
// Default border
let default_border = colors.border_class(Color::Border);

// Primary border
let primary_border = colors.border_class(Color::Primary);

// Interactive border
let interactive_border = colors.border_class(Color::Interactive);

// Error border
let error_border = colors.border_class(Color::Error);
```

## 🔧 Complete Example: Login Form

Here's a complete example combining multiple elements:

```rust
use jupiter_design_system::prelude::*;

fn create_login_form() -> LoginFormClasses {
    let colors = VibeColors::new();
    
    LoginFormClasses {
        // Form container
        form_container: layout_styles(colors.clone())
            .direction_vertical()
            .spacing_lg()
            .classes(),
        
        // Form card
        form_card: card_styles(colors.clone())
            .elevation(CardElevation::Medium)
            .spacing(CardSpacing::Large)
            .classes(),
        
        // Form title
        title: text_styles(colors.clone())
            .typography(Typography::Heading1)
            .color(Color::TextPrimary)
            .classes(),
        
        // Form subtitle
        subtitle: text_styles(colors.clone())
            .typography(Typography::Body)
            .color(Color::TextSecondary)
            .classes(),
        
        // Field container
        field_container: layout_styles(colors.clone())
            .direction_vertical()
            .spacing_sm()
            .classes(),
        
        // Form labels
        label: text_styles(colors.clone())
            .typography(Typography::Label)
            .color(Color::TextPrimary)
            .weight(FontWeight::Medium)
            .classes(),
        
        // Form inputs
        input: interactive_input(colors.clone())
            .base_style()
            .hover().border_primary()
            .focus().border_primary().ring_primary().outline_none()
            .build(),
        
        // Error input
        error_input: interactive_input(colors.clone())
            .base_style()
            .border_color(Color::Error)
            .hover().border_color(Color::Error)
            .focus().border_color(Color::Error).ring_color(Color::Error)
            .build(),
        
        // Error text
        error_text: text_styles(colors.clone())
            .typography(Typography::Caption)
            .color(Color::Error)
            .classes(),
        
        // Help text
        help_text: text_styles(colors.clone())
            .typography(Typography::Caption)
            .color(Color::TextTertiary)
            .classes(),
        
        // Submit button
        submit_button: button_styles(colors.clone())
            .primary()
            .size(Size::Large)
            .full_width()
            .classes(),
        
        // Secondary action button
        secondary_button: button_styles(colors.clone())
            .ghost()
            .size(Size::Medium)
            .classes(),
        
        // Button container
        button_container: layout_styles(colors)
            .direction_vertical()
            .spacing_md()
            .classes(),
    }
}

pub struct LoginFormClasses {
    pub form_container: String,
    pub form_card: String,
    pub title: String,
    pub subtitle: String,
    pub field_container: String,
    pub label: String,
    pub input: String,
    pub error_input: String,
    pub error_text: String,
    pub help_text: String,
    pub submit_button: String,
    pub secondary_button: String,
    pub button_container: String,
}

// Usage in your component framework:
// let classes = create_login_form();
// 
// <div class={classes.form_container}>
//   <div class={classes.form_card}>
//     <h1 class={classes.title}>Sign In</h1>
//     <p class={classes.subtitle}>Welcome back! Please sign in to continue.</p>
//     
//     <div class={classes.field_container}>
//       <label class={classes.label}>Email</label>
//       <input class={classes.input} type="email" />
//       <div class={classes.help_text}>We'll never share your email.</div>
//     </div>
//     
//     <div class={classes.button_container}>
//       <button class={classes.submit_button}>Sign In</button>
//       <button class={classes.secondary_button}>Forgot Password?</button>
//     </div>
//   </div>
// </div>
```

## 🎯 Key Takeaways

### Best Practices Demonstrated

1. **Consistent Theme Usage**: Always pass the same color provider instance
2. **Semantic Variants**: Use meaningful button variants (primary, success, error)
3. **Proper Typography Hierarchy**: Use appropriate typography levels
4. **Logical Layout Structure**: Organize layouts with proper spacing and alignment
5. **Accessibility**: Built-in focus states and semantic color usage

### Common Patterns

1. **Component Composition**: Combine multiple style builders for complex components
2. **State Management**: Use appropriate states for different interaction modes
3. **Responsive Design**: Leverage the size system for different screen sizes
4. **Color Semantics**: Use semantic colors to convey meaning

### Next Steps

- Explore [Complete Components](./complete-components.md) for more complex examples
- Learn [Interactive Patterns](./interactive-patterns.md) for advanced interactions
- Check [Framework Integration](./dioxus-integration.md) for your specific framework
- Study [Theme Customization](./theme-customization.md) to create custom themes

## 🔗 Related Examples

- [Complete Components](./complete-components.md) - Full-featured component implementations
- [Interactive Patterns](./interactive-patterns.md) - Advanced interactive examples
- [Form Applications](./form-applications.md) - Complex form patterns
- [Theme Customization](./theme-customization.md) - Custom theme examples