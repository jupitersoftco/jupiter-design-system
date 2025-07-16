# Usage Examples & Best Practices

This document provides practical examples and best practices for using the Jupiter Design System color system.

## Basic Usage Examples

### Getting Started

```rust
use jupiter_design_system::prelude::*;

// Create a theme provider
let theme = VibeColors::default();

// Basic color resolution
let primary_color = theme.resolve_color(Color::Primary);    // "jupiter-blue-500"
let text_class = theme.text_class(Color::TextPrimary);      // "text-gray-900"
let bg_class = theme.bg_class(Color::Surface);              // "bg-white"
let border_class = theme.border_class(Color::Border);       // "border-gray-200"
```

### Component Styling

```rust
use jupiter_design_system::patterns::*;

let theme = VibeColors::default();

// Button with semantic colors
let primary_button = Button::new("Save Changes")
    .variant(ButtonVariant::Primary)     // Uses Color::Primary
    .build(&theme);

let secondary_button = Button::new("Cancel")
    .variant(ButtonVariant::Secondary)   // Uses Color::Secondary
    .build(&theme);

let danger_button = Button::new("Delete")
    .variant(ButtonVariant::Error)       // Uses Color::Error
    .build(&theme);
```

### Card Components

```rust
// Basic card with semantic styling
let card = Card::new()
    .add_class(&theme.bg_class(Color::Surface))         // White background
    .add_class(&theme.text_class(Color::TextPrimary))   // Dark text
    .add_class(&theme.border_class(Color::Border))      // Gray border
    .add_class("p-6 rounded-lg shadow-sm")              // Layout styles
    .content("Card content here")
    .build(&theme);

// Success state card
let success_card = Card::new()
    .add_class(&theme.bg_class(Color::Success))         // Green background
    .add_class(&theme.text_class(Color::TextInverse))   // White text
    .add_class("p-4 rounded-md")
    .content("✓ Operation completed successfully")
    .build(&theme);
```

## Advanced Usage Patterns

### Form Components

```rust
// Input field with validation states
fn create_input_field(
    theme: &impl ColorProvider,
    label: &str,
    is_error: bool,
    is_focused: bool
) -> String {
    let border_color = if is_error {
        theme.border_class(Color::Error)
    } else if is_focused {
        theme.border_class(Color::Primary)
    } else {
        theme.border_class(Color::Border)
    };
    
    let text_color = theme.text_class(Color::TextPrimary);
    let bg_color = theme.bg_class(Color::Surface);
    
    format!(
        r#"<input class="px-3 py-2 rounded-md {} {} {} focus:outline-none focus:ring-2" 
           placeholder="{}" />"#,
        border_color, text_color, bg_color, label
    )
}

// Usage
let theme = VibeColors::default();
let email_input = create_input_field(&theme, "Enter email", false, false);
let error_input = create_input_field(&theme, "Invalid email", true, false);
```

### Status Indicators

```rust
// Status badge component
fn status_badge(theme: &impl ColorProvider, status: &str, message: &str) -> String {
    let (bg_color, text_color) = match status {
        "success" => (
            theme.bg_class(Color::Success),
            theme.text_class(Color::TextInverse)
        ),
        "warning" => (
            theme.bg_class(Color::Warning),
            theme.text_class(Color::TextInverse)
        ),
        "error" => (
            theme.bg_class(Color::Error),
            theme.text_class(Color::TextInverse)
        ),
        "info" => (
            theme.bg_class(Color::Info),
            theme.text_class(Color::TextInverse)
        ),
        _ => (
            theme.bg_class(Color::Border),
            theme.text_class(Color::TextSecondary)
        ),
    };
    
    format!(
        r#"<span class="px-2 py-1 rounded-full text-xs font-medium {} {}">{}</span>"#,
        bg_color, text_color, message
    )
}

// Usage examples
let theme = VibeColors::default();
let success_badge = status_badge(&theme, "success", "Completed");
let warning_badge = status_badge(&theme, "warning", "Pending");
let error_badge = status_badge(&theme, "error", "Failed");
```

### Interactive Elements

```rust
// Link component with hover states
fn create_link(theme: &impl ColorProvider, href: &str, text: &str) -> String {
    let text_color = theme.text_class(Color::Interactive);
    let hover_color = theme.text_class(Color::InteractiveHover);
    
    format!(
        r#"<a href="{}" class="{} hover:{} underline transition-colors duration-200">{}</a>"#,
        href, text_color, hover_color, text
    )
}

// Navigation item with active state
fn nav_item(theme: &impl ColorProvider, text: &str, is_active: bool) -> String {
    let (text_color, bg_color) = if is_active {
        (
            theme.text_class(Color::Primary),
            theme.bg_class(Color::Background)
        )
    } else {
        (
            theme.text_class(Color::TextSecondary),
            "bg-transparent".to_string()
        )
    };
    
    format!(
        r#"<a class="px-3 py-2 rounded-md {} {} hover:{}">{}</a>"#,
        text_color,
        bg_color,
        theme.bg_class(Color::Background),
        text
    )
}
```

## Custom Theme Examples

### Brand-Specific Theme

```rust
// Create a custom theme for your brand
fn create_brand_theme() -> impl ColorProvider {
    VibeColors::with_overrides(|palette| {
        // Brand colors
        palette.primary = "purple-600".to_string();
        palette.secondary = "pink-500".to_string();
        palette.accent = "orange-400".to_string();
        
        // Adjust interactive colors to match brand
        palette.interactive = "purple-600".to_string();
        palette.interactive_hover = "purple-700".to_string();
        palette.interactive_active = "purple-800".to_string();
    })
}

// Use the custom theme
let brand_theme = create_brand_theme();
let branded_button = Button::new("Brand Action")
    .variant(ButtonVariant::Primary)
    .build(&brand_theme);
```

### Dark Mode Theme

```rust
// Dark mode color adjustments
fn create_dark_theme() -> impl ColorProvider {
    VibeColors::with_overrides(|palette| {
        // Dark backgrounds
        palette.surface = "gray-800".to_string();
        palette.background = "gray-900".to_string();
        palette.border = "gray-700".to_string();
        
        // Light text
        palette.text_primary = "white".to_string();
        palette.text_secondary = "gray-300".to_string();
        palette.text_tertiary = "gray-500".to_string();
        palette.foreground = "white".to_string();
        
        // Adjust interactive states for dark mode
        palette.interactive_disabled = "gray-600".to_string();
    })
}

// Toggle between light and dark themes
fn themed_component(is_dark_mode: bool) -> String {
    let theme = if is_dark_mode {
        create_dark_theme()
    } else {
        VibeColors::default()
    };
    
    Card::new()
        .add_class(&theme.bg_class(Color::Surface))
        .add_class(&theme.text_class(Color::TextPrimary))
        .content("This card adapts to the theme")
        .build(&theme)
}
```

### High Contrast Theme

```rust
// High contrast theme for accessibility
fn create_high_contrast_theme() -> impl ColorProvider {
    VibeColors::with_overrides(|palette| {
        // Maximum contrast
        palette.primary = "black".to_string();
        palette.surface = "white".to_string();
        palette.background = "white".to_string();
        palette.foreground = "black".to_string();
        palette.border = "black".to_string();
        
        // High contrast text
        palette.text_primary = "black".to_string();
        palette.text_secondary = "black".to_string();
        palette.text_inverse = "white".to_string();
        
        // High contrast states
        palette.success = "green-800".to_string();
        palette.error = "red-800".to_string();
        palette.warning = "yellow-600".to_string();
    })
}
```

## Real-World Application Examples

### Dashboard Layout

```rust
fn create_dashboard(theme: &impl ColorProvider) -> String {
    let sidebar = format!(
        r#"<aside class="{} {} {} p-6">
            <nav><!-- Navigation items --></nav>
        </aside>"#,
        theme.bg_class(Color::Surface),
        theme.text_class(Color::TextPrimary),
        theme.border_class(Color::Border)
    );
    
    let main_content = format!(
        r#"<main class="{} {} p-8">
            <h1 class="{} text-2xl font-bold mb-6">Dashboard</h1>
            <!-- Dashboard content -->
        </main>"#,
        theme.bg_class(Color::Background),
        theme.text_class(Color::TextPrimary),
        theme.text_class(Color::TextPrimary)
    );
    
    format!("<div class=\"flex h-screen\">{}{}</div>", sidebar, main_content)
}
```

### Form Layout

```rust
fn create_contact_form(theme: &impl ColorProvider) -> String {
    let form_bg = theme.bg_class(Color::Surface);
    let text_color = theme.text_class(Color::TextPrimary);
    let border_color = theme.border_class(Color::Border);
    let primary_button = Button::new("Send Message")
        .variant(ButtonVariant::Primary)
        .build(theme);
    
    format!(
        r#"<form class="{} {} {} p-8 rounded-lg shadow-md max-w-md mx-auto">
            <h2 class="text-xl font-semibold mb-6">Contact Us</h2>
            
            <div class="mb-4">
                <label class="block {} text-sm font-medium mb-2">Name</label>
                <input class="w-full px-3 py-2 {} {} rounded-md focus:outline-none focus:ring-2 focus:ring-opacity-50" 
                       type="text" />
            </div>
            
            <div class="mb-4">
                <label class="block {} text-sm font-medium mb-2">Email</label>
                <input class="w-full px-3 py-2 {} {} rounded-md focus:outline-none focus:ring-2 focus:ring-opacity-50" 
                       type="email" />
            </div>
            
            <div class="mb-6">
                <label class="block {} text-sm font-medium mb-2">Message</label>
                <textarea class="w-full px-3 py-2 {} {} rounded-md focus:outline-none focus:ring-2 focus:ring-opacity-50 h-32" 
                          rows="4"></textarea>
            </div>
            
            {}
        </form>"#,
        form_bg, text_color, border_color,
        text_color, theme.bg_class(Color::Surface), border_color,
        text_color, theme.bg_class(Color::Surface), border_color,
        text_color, theme.bg_class(Color::Surface), border_color,
        primary_button
    )
}
```

## Best Practices

### Do's ✅

**Use semantic tokens consistently**
```rust
// Good: Semantic and consistent
let success_message = format!(
    r#"<div class="{} {} p-4">Success!</div>"#,
    theme.bg_class(Color::Success),
    theme.text_class(Color::TextInverse)
);
```

**Leverage the ColorProvider trait**
```rust
// Good: Using provided methods
let button_classes = format!(
    "{} {} {}",
    theme.text_class(Color::TextInverse),
    theme.bg_class(Color::Primary),
    theme.border_class(Color::Primary)
);
```

**Test color combinations for accessibility**
```rust
// Good: Checking contrast
fn ensure_readable_text(theme: &impl ColorProvider) {
    // Use high contrast combinations
    let header = format!(
        "{}",  // text-gray-900 on bg-white = 21:1 ratio
        theme.text_class(Color::TextPrimary)
    );
}
```

**Use appropriate colors for context**
```rust
// Good: Context-appropriate colors
let error_alert = format!(
    r#"<div class="{} {} border-l-4 {}">Error occurred</div>"#,
    theme.bg_class(Color::Error),
    theme.text_class(Color::TextInverse),
    theme.border_class(Color::Error)
);
```

### Don'ts ❌

**Don't hardcode color values**
```rust
// Bad: Hardcoded values
let button = r#"<button class="bg-blue-500 text-white">Click me</button>"#;

// Good: Semantic tokens
let button = format!(
    r#"<button class="{} {}">Click me</button>"#,
    theme.bg_class(Color::Primary),
    theme.text_class(Color::TextInverse)
);
```

**Don't mix color contexts**
```rust
// Bad: Using error color for success
let success_message = format!(
    "{}",
    theme.text_class(Color::Error)  // Wrong semantic meaning
);

// Good: Correct semantic usage
let success_message = format!(
    "{}",
    theme.text_class(Color::Success)
);
```

**Don't bypass the ColorProvider**
```rust
// Bad: Manual class construction
let classes = "text-jupiter-blue-500 bg-white border-gray-200";

// Good: Using ColorProvider methods
let classes = format!(
    "{} {} {}",
    theme.text_class(Color::Primary),
    theme.bg_class(Color::Surface),
    theme.border_class(Color::Border)
);
```

**Don't assume specific color values**
```rust
// Bad: Assuming specific values
if theme.resolve_color(Color::Primary) == "jupiter-blue-500" {
    // This breaks with custom themes
}

// Good: Use tokens semantically
let primary_button = Button::new("Action")
    .variant(ButtonVariant::Primary)  // Always uses Color::Primary
    .build(theme);
```

## Performance Tips

**Cache ColorProvider instances**
```rust
// Good: Reuse theme instances
let theme = VibeColors::default();
let button1 = Button::new("One").build(&theme);
let button2 = Button::new("Two").build(&theme);
```

**Batch color resolution**
```rust
// Good: Resolve colors once
let colors = (
    theme.text_class(Color::TextPrimary),
    theme.bg_class(Color::Surface),
    theme.border_class(Color::Border),
);

// Use resolved colors multiple times
let card1 = format!("class=\"{} {} {}\"", colors.0, colors.1, colors.2);
let card2 = format!("class=\"{} {} {}\"", colors.0, colors.1, colors.2);
```

**Use appropriate data structures**
```rust
// Good: Pre-compute common combinations
struct CommonClasses {
    card_base: String,
    button_primary: String,
    text_heading: String,
}

impl CommonClasses {
    fn new(theme: &impl ColorProvider) -> Self {
        Self {
            card_base: format!(
                "{} {} {}",
                theme.bg_class(Color::Surface),
                theme.text_class(Color::TextPrimary),
                theme.border_class(Color::Border)
            ),
            button_primary: format!(
                "{} {}",
                theme.bg_class(Color::Primary),
                theme.text_class(Color::TextInverse)
            ),
            text_heading: theme.text_class(Color::TextPrimary),
        }
    }
}
```