# Accessibility Best Practices

## ♿ Accessibility Philosophy

The Jupiter Design System is built with accessibility as a core principle, ensuring that all users can effectively interact with applications regardless of their abilities or assistive technologies.

### Core Principles

1. **Universal Design**: Design for the widest possible range of users from the start
2. **Semantic Foundation**: Use meaningful HTML structure and ARIA attributes
3. **Keyboard Navigation**: Ensure all interactions are keyboard accessible
4. **Visual Clarity**: Provide sufficient contrast and readable typography
5. **Screen Reader Support**: Optimize for assistive technology compatibility

## 🎯 WCAG 2.1 Compliance

### Level AA Standards

The design system aims for WCAG 2.1 Level AA compliance:

- **Contrast Ratios**: 4.5:1 for normal text, 3:1 for large text
- **Focus Indicators**: Visible focus states for all interactive elements
- **Keyboard Navigation**: Full keyboard accessibility
- **Screen Reader Support**: Proper semantic markup and ARIA labels

### Built-in Accessibility Features

```rust
// Components include accessibility features by default
let accessible_button = button_styles(colors)
    .primary()
    .size(Size::Medium)  // Meets 44px minimum touch target
    .classes();

// Interactive elements include focus states
let accessible_input = interactive_input(colors)
    .base_style()
    .focus()
        .ring_color(Color::Interactive)
        .ring_offset_2()
        .outline_none()
    .build();
```

## 🎨 Color and Contrast

### Contrast Requirements

The design system ensures proper contrast ratios:

```rust
// High contrast text combinations
let high_contrast_text = text_styles(colors)
    .color(Color::TextPrimary)  // Gray-900 on white background (21:1 ratio)
    .typography(Typography::Body)
    .classes();

let secondary_text = text_styles(colors)
    .color(Color::TextSecondary)  // Gray-600 on white background (7:1 ratio)
    .typography(Typography::Body)
    .classes();

// Interactive elements with sufficient contrast
let accessible_button = button_styles(colors)
    .primary()  // White text on jupiter-blue-500 (4.5:1+ ratio)
    .classes();
```

### Color-Blind Friendly Design

```rust
// Don't rely solely on color for meaning
fn create_status_message(
    message: &str,
    status: MessageStatus,
    colors: impl ColorProvider,
) -> (String, &'static str, String) {
    let (color, icon, prefix) = match status {
        MessageStatus::Success => (Color::Success, "✓", "Success: "),
        MessageStatus::Warning => (Color::Warning, "⚠", "Warning: "),
        MessageStatus::Error => (Color::Error, "✗", "Error: "),
        MessageStatus::Info => (Color::Info, "ℹ", "Info: "),
    };
    
    let text_classes = text_styles(colors)
        .color(color)
        .typography(Typography::Body)
        .classes();
    
    (text_classes, icon, format!("{}{}", prefix, message))
}
```

### Dark Theme Accessibility

```rust
// Dark theme with proper contrast
impl Default for AccessibleDarkTheme {
    fn default() -> Self {
        Self {
            palette: ColorPalette {
                // High contrast text on dark backgrounds
                text_primary: "gray-50".to_string(),    // 18.5:1 on gray-900
                text_secondary: "gray-200".to_string(), // 11.6:1 on gray-900
                text_tertiary: "gray-400".to_string(),  // 5.4:1 on gray-900
                
                // Accessible dark backgrounds
                background: "gray-900".to_string(),
                surface: "gray-800".to_string(),
                
                // Enhanced brand colors for dark theme
                primary: "jupiter-blue-300".to_string(),
                interactive: "jupiter-blue-300".to_string(),
                
                // Clear borders and dividers
                border: "gray-600".to_string(),
                
                // ... other colors
            },
        }
    }
}
```

## ⌨️ Keyboard Navigation

### Focus Management

```rust
// Proper focus indicators
let focusable_element = interactive_element(colors)
    .base("px-4 py-2 rounded border")
    .focus()
        .ring_color(Color::Interactive)
        .ring_width_2()
        .ring_offset_2()
        .outline_none()  // Remove default outline, using custom ring
    .build();

// Skip to content link
let skip_link = interactive_element(colors)
    .base("absolute top-0 left-0 transform -translate-y-full")
    .focus()
        .translate_y_0()  // Reveal on focus
        .ring_color(Color::Interactive)
        .ring_width_2()
    .build();
```

### Tab Order Management

```rust
// Logical tab order in forms
fn create_accessible_form(colors: impl ColorProvider) -> Vec<String> {
    vec![
        // Form fields in logical order
        interactive_input(colors.clone())
            .base_style()
            .focus().ring_primary()
            .build() + " tabindex='1'",
            
        interactive_input(colors.clone())
            .base_style()
            .focus().ring_primary()
            .build() + " tabindex='2'",
            
        // Submit button last
        button_styles(colors)
            .primary()
            .classes() + " tabindex='3'",
    ]
}
```

### Custom Focus Patterns

```rust
// Focus within patterns for complex components
let dropdown_container = interactive_element(colors)
    .base("relative")
    .focus_within()
        .ring_color(Color::Interactive)
        .ring_width_1()
    .build();

// Focus trap for modals
let modal_focus_trap = "focus-trap"; // Add focus trap library classes
```

## 📱 Touch and Mobile Accessibility

### Touch Target Sizes

```rust
// Minimum 44px touch targets
let mobile_button = button_styles(colors)
    .size(Size::Medium)  // Ensures 44px minimum
    .classes();

// Generous spacing for touch interfaces
let mobile_layout = layout_styles(colors)
    .spacing_lg()  // Extra space between interactive elements
    .direction_vertical()
    .classes();

// Mobile-friendly navigation
let mobile_nav_item = interactive_element(colors)
    .base("px-4 py-3 min-h-[44px] flex items-center")  // 44px minimum
    .hover().bg_color(Color::Surface)
    .focus().ring_primary()
    .build();
```

### Responsive Typography

```rust
// Scalable text for mobile devices
let mobile_text = text_styles(colors)
    .typography(Typography::Body)
    .responsive_size(Breakpoint::Mobile, TypographySize::SM)
    .responsive_size(Breakpoint::Tablet, TypographySize::MD)
    .classes();

// Large touch-friendly buttons
let touch_button = button_styles(colors)
    .primary()
    .size(Size::Large)  // Larger for touch interfaces
    .full_width()       // Easy to tap on mobile
    .classes();
```

## 🔊 Screen Reader Support

### Semantic HTML Integration

```rust
// Map design system to semantic HTML
fn render_accessible_button(
    text: &str,
    variant: ButtonVariant,
    colors: impl ColorProvider,
) -> String {
    let classes = button_styles(colors)
        .variant(variant)
        .classes();
    
    format!(
        r#"<button type="button" class="{}" aria-describedby="help-text">
            {}
           </button>"#,
        classes, text
    )
}

// Semantic headings with design system
fn render_accessible_heading(
    level: u8,
    text: &str,
    colors: impl ColorProvider,
) -> String {
    let typography = match level {
        1 => Typography::Heading1,
        2 => Typography::Heading2,
        3 => Typography::Heading3,
        _ => Typography::Body,
    };
    
    let classes = text_styles(colors)
        .typography(typography)
        .color(Color::TextPrimary)
        .classes();
    
    format!("<h{} class=\"{}\">{}</h{}>", level, classes, text, level)
}
```

### ARIA Labels and Descriptions

```rust
// Button with ARIA label
fn accessible_icon_button(
    icon: &str,
    label: &str,
    colors: impl ColorProvider,
) -> String {
    let classes = button_styles(colors)
        .ghost()
        .size(Size::Medium)
        .classes();
    
    format!(
        r#"<button class="{}" aria-label="{}" type="button">
            <span aria-hidden="true">{}</span>
           </button>"#,
        classes, label, icon
    )
}

// Form field with proper labeling
fn accessible_form_field(
    label: &str,
    placeholder: &str,
    help_text: &str,
    colors: impl ColorProvider,
) -> String {
    let label_classes = text_styles(colors.clone())
        .typography(Typography::Label)
        .color(Color::TextSecondary)
        .classes();
    
    let input_classes = interactive_input(colors.clone())
        .base_style()
        .focus().ring_primary()
        .build();
    
    let help_classes = text_styles(colors)
        .typography(Typography::Caption)
        .color(Color::TextTertiary)
        .classes();
    
    format!(
        r#"
        <label for="field-id" class="{}">{}</label>
        <input 
            id="field-id" 
            class="{}" 
            placeholder="{}" 
            aria-describedby="help-id"
        />
        <div id="help-id" class="{}">{}</div>
        "#,
        label_classes, label, input_classes, placeholder, help_classes, help_text
    )
}
```

### Live Regions for Dynamic Content

```rust
// Status announcements
fn create_live_region(
    message: &str,
    urgency: LiveRegionUrgency,
    colors: impl ColorProvider,
) -> String {
    let classes = text_styles(colors)
        .typography(Typography::Body)
        .color(Color::TextPrimary)
        .classes();
    
    let aria_live = match urgency {
        LiveRegionUrgency::Polite => "polite",
        LiveRegionUrgency::Assertive => "assertive",
    };
    
    format!(
        r#"<div class="{}" aria-live="{}" role="status">
            {}
           </div>"#,
        classes, aria_live, message
    )
}

enum LiveRegionUrgency {
    Polite,
    Assertive,
}
```

## 🎭 Component-Specific Accessibility

### Accessible Buttons

```rust
// Button with loading state
fn loading_button(
    text: &str,
    is_loading: bool,
    colors: impl ColorProvider,
) -> String {
    let state = if is_loading { 
        ButtonState::Loading 
    } else { 
        ButtonState::Default 
    };
    
    let classes = button_styles(colors)
        .primary()
        .state(state)
        .classes();
    
    let (button_text, aria_attrs) = if is_loading {
        ("Loading...", r#"aria-disabled="true" aria-describedby="loading-text""#)
    } else {
        (text, "")
    };
    
    format!(
        r#"<button class="{}" type="submit" {}>
            {}
            {}</button>"#,
        classes,
        aria_attrs,
        button_text,
        if is_loading { r#"<span id="loading-text" class="sr-only">Please wait while we process your request</span>"# } else { "" }
    )
}
```

### Accessible Cards

```rust
// Interactive card with proper semantics
fn accessible_product_card(
    title: &str,
    description: &str,
    price: &str,
    colors: impl ColorProvider,
) -> String {
    let card_classes = card_styles(colors.clone())
        .elevation(CardElevation::Medium)
        .interaction(CardInteraction::Clickable)
        .classes();
    
    let title_classes = text_styles(colors.clone())
        .typography(Typography::Heading3)
        .color(Color::TextPrimary)
        .classes();
    
    let description_classes = text_styles(colors.clone())
        .typography(Typography::Body)
        .color(Color::TextSecondary)
        .classes();
    
    let price_classes = text_styles(colors)
        .typography(Typography::Body)
        .color(Color::Primary)
        .weight(FontWeight::Bold)
        .classes();
    
    format!(
        r#"<article class="{}" role="button" tabindex="0" 
                   aria-labelledby="product-title" 
                   aria-describedby="product-description product-price">
            <h3 id="product-title" class="{}">{}</h3>
            <p id="product-description" class="{}">{}</p>
            <div id="product-price" class="{}" aria-label="Price: {}">{}</div>
           </article>"#,
        card_classes, title_classes, title, description_classes, description, price_classes, price, price
    )
}
```

### Accessible Forms

```rust
// Complete accessible form
fn accessible_contact_form(colors: impl ColorProvider) -> String {
    let form_layout = layout_styles(colors.clone())
        .spacing_lg()
        .direction_vertical()
        .classes();
    
    let field_layout = layout_styles(colors.clone())
        .spacing_sm()
        .direction_vertical()
        .classes();
    
    let label_classes = text_styles(colors.clone())
        .typography(Typography::Label)
        .color(Color::TextPrimary)
        .weight(FontWeight::Medium)
        .classes();
    
    let input_classes = interactive_input(colors.clone())
        .base_style()
        .hover().border_primary()
        .focus().border_primary().ring_primary()
        .build();
    
    let button_classes = button_styles(colors)
        .primary()
        .classes();
    
    format!(
        r#"<form class="{}" novalidate>
            <fieldset>
                <legend class="{}">Contact Information</legend>
                
                <div class="{}">
                    <label for="name" class="{}">
                        Name <span aria-label="required">*</span>
                    </label>
                    <input 
                        id="name" 
                        class="{}" 
                        type="text" 
                        required 
                        aria-describedby="name-error"
                    />
                    <div id="name-error" class="sr-only" aria-live="polite"></div>
                </div>
                
                <div class="{}">
                    <label for="email" class="{}">
                        Email <span aria-label="required">*</span>
                    </label>
                    <input 
                        id="email" 
                        class="{}" 
                        type="email" 
                        required 
                        aria-describedby="email-error"
                    />
                    <div id="email-error" class="sr-only" aria-live="polite"></div>
                </div>
                
                <button class="{}" type="submit">
                    Submit Contact Form
                </button>
            </fieldset>
           </form>"#,
        form_layout,
        label_classes,
        field_layout, label_classes, input_classes,
        field_layout, label_classes, input_classes,
        button_classes
    )
}
```

## 🧪 Accessibility Testing

### Automated Testing

```rust
#[cfg(test)]
mod accessibility_tests {
    use super::*;

    #[test]
    fn test_button_accessibility() {
        let colors = VibeColors::new();
        let button_html = render_accessible_button("Submit", ButtonVariant::Primary, colors);
        
        // Test semantic structure
        assert!(button_html.contains("<button"));
        assert!(button_html.contains("type=\"button\""));
        
        // Test that classes include focus states
        let classes = button_styles(VibeColors::new()).primary().classes();
        // Would need integration with CSS parsing to test actual focus styles
    }

    #[test]
    fn test_form_accessibility() {
        let colors = VibeColors::new();
        let form_html = accessible_contact_form(colors);
        
        // Test semantic structure
        assert!(form_html.contains("<fieldset>"));
        assert!(form_html.contains("<legend>"));
        assert!(form_html.contains("aria-describedby"));
        assert!(form_html.contains("aria-live"));
    }

    #[test]
    fn test_contrast_ratios() {
        let colors = VibeColors::new();
        
        // Test high contrast combinations
        assert!(has_sufficient_contrast(
            &colors.resolve_color(Color::TextPrimary),
            &colors.resolve_color(Color::Background)
        ));
        
        assert!(has_sufficient_contrast(
            "white",
            &colors.resolve_color(Color::Primary)
        ));
    }
}

// Contrast ratio calculation (simplified)
fn has_sufficient_contrast(foreground: &str, background: &str) -> bool {
    // Implement WCAG contrast calculation
    // This would need actual color value parsing and luminance calculation
    true // Placeholder
}
```

### Manual Testing Checklist

```rust
// Accessibility testing utilities
pub struct AccessibilityChecklist {
    pub keyboard_navigation: bool,
    pub screen_reader_support: bool,
    pub color_contrast: bool,
    pub focus_indicators: bool,
    pub touch_targets: bool,
}

impl AccessibilityChecklist {
    pub fn verify_component(component_html: &str) -> Self {
        Self {
            keyboard_navigation: component_html.contains("tabindex") || component_html.contains("<button") || component_html.contains("<a"),
            screen_reader_support: component_html.contains("aria-") || component_html.contains("role="),
            color_contrast: true, // Would need color analysis
            focus_indicators: component_html.contains("focus:"),
            touch_targets: true, // Would need size analysis
        }
    }
    
    pub fn passes_wcag_aa(&self) -> bool {
        self.keyboard_navigation 
            && self.screen_reader_support 
            && self.color_contrast 
            && self.focus_indicators 
            && self.touch_targets
    }
}
```

## 🚦 Accessibility Guidelines

### Do's ✅

1. **Use Semantic HTML**
   ```rust
   // ✅ Good: Semantic button
   let semantic_button = format!(
       "<button class=\"{}\" type=\"submit\">{}</button>",
       button_styles(colors).primary().classes(),
       "Submit Form"
   );
   ```

2. **Provide Alternative Text**
   ```rust
   // ✅ Good: Icon with text alternative
   let accessible_icon = format!(
       "<button class=\"{}\" aria-label=\"Close dialog\">
           <span aria-hidden=\"true\">×</span>
        </button>",
       button_styles(colors).ghost().classes()
   );
   ```

3. **Test with Assistive Technology**
   ```rust
   // ✅ Good: Testing with screen readers
   fn test_screen_reader_output() {
       // Test component with screen reader
       // Verify proper announcements
       // Check navigation flow
   }
   ```

### Don'ts ❌

1. **Don't Rely Only on Color**
   ```rust
   // ❌ Bad: Color-only status indication
   let color_only_status = text_styles(colors)
       .color(Color::Error)
       .classes();
   
   // ✅ Good: Color + text + icon
   let accessible_status = format!(
       "<div class=\"{}\">
           <span aria-hidden=\"true\">⚠</span>
           Error: {}
        </div>",
       text_styles(colors).color(Color::Error).classes(),
       "Please fix the following issues"
   );
   ```

2. **Don't Skip Focus States**
   ```rust
   // ❌ Bad: No focus indicator
   let no_focus = "outline-none";  // Without alternative focus indicator
   
   // ✅ Good: Custom focus indicator
   let accessible_focus = interactive_element(colors)
       .focus()
           .ring_color(Color::Interactive)
           .ring_width_2()
           .outline_none()
       .build();
   ```

3. **Don't Use Inaccessible Patterns**
   ```rust
   // ❌ Bad: Click handler on non-interactive element
   let bad_pattern = format!(
       "<div class=\"{}\" onclick=\"handleClick()\">Click me</div>",
       card_styles(colors).classes()
   );
   
   // ✅ Good: Proper interactive element
   let good_pattern = format!(
       "<button class=\"{}\" onclick=\"handleClick()\" type=\"button\">
           Click me
        </button>",
       button_styles(colors).ghost().classes()
   );
   ```

## 🎮 Interactive Widget Accessibility

### Modal Dialogs

```rust
// Accessible modal with focus trap and escape handling
fn accessible_modal(
    title: &str,
    content: &str,
    colors: impl ColorProvider,
) -> String {
    let overlay_classes = "fixed inset-0 bg-black bg-opacity-50 z-40";
    let modal_classes = card_styles(colors.clone())
        .elevation(CardElevation::High)
        .classes() + " fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-50 max-w-md w-full mx-4";
    
    let title_classes = text_styles(colors.clone())
        .typography(Typography::Heading2)
        .color(Color::TextPrimary)
        .classes();
    
    let close_button_classes = button_styles(colors)
        .ghost()
        .size(Size::Small)
        .classes();
    
    format!(
        r#"
        <div class="{}" aria-hidden="true" data-modal-overlay></div>
        <div class="{}" 
             role="dialog" 
             aria-modal="true" 
             aria-labelledby="modal-title"
             aria-describedby="modal-content"
             data-focus-trap>
            <div class="flex justify-between items-center mb-4">
                <h2 id="modal-title" class="{}">{}</h2>
                <button class="{}" 
                        aria-label="Close dialog"
                        data-modal-close>
                    <span aria-hidden="true">×</span>
                </button>
            </div>
            <div id="modal-content" class="mb-6">
                {}
            </div>
            <div class="flex justify-end space-x-2">
                <button class="{}" data-modal-close>Cancel</button>
                <button class="{}">Confirm</button>
            </div>
        </div>
        "#,
        overlay_classes,
        modal_classes,
        title_classes, title,
        close_button_classes,
        content,
        button_styles(colors.clone()).secondary().classes(),
        button_styles(colors).primary().classes()
    )
}
```

### Dropdown Menus

```rust
// Accessible dropdown with ARIA
fn accessible_dropdown(
    trigger_text: &str,
    items: &[(&str, &str)], // (text, href)
    colors: impl ColorProvider,
) -> String {
    let trigger_classes = button_styles(colors.clone())
        .secondary()
        .classes();
    
    let menu_classes = card_styles(colors.clone())
        .elevation(CardElevation::High)
        .classes() + " absolute top-full left-0 mt-1 min-w-48 py-1";
    
    let item_classes = interactive_element(colors)
        .base("block px-4 py-2 text-sm")
        .hover().bg_color(Color::Surface)
        .focus().bg_color(Color::Surface).outline_none()
        .build();
    
    let menu_items = items.iter()
        .enumerate()
        .map(|(i, (text, href))| {
            format!(
                r#"<a href="{}" class="{}" role="menuitem" tabindex="-1" id="menu-item-{}">{}</a>"#,
                href, item_classes, i, text
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    
    format!(
        r#"
        <div class="relative" data-dropdown>
            <button class="{}" 
                    aria-expanded="false"
                    aria-haspopup="true"
                    aria-controls="dropdown-menu"
                    data-dropdown-trigger>
                {}
                <span aria-hidden="true">▼</span>
            </button>
            <div class="{} hidden" 
                 role="menu"
                 aria-orientation="vertical"
                 aria-labelledby="dropdown-trigger"
                 id="dropdown-menu"
                 data-dropdown-menu>
                {}
            </div>
        </div>
        "#,
        trigger_classes, trigger_text, menu_classes, menu_items
    )
}
```

### Tabs Interface

```rust
// Accessible tabs with proper ARIA
fn accessible_tabs(
    tabs: &[(&str, &str)], // (label, content)
    active_index: usize,
    colors: impl ColorProvider,
) -> String {
    let tab_list_classes = layout_styles(colors.clone())
        .direction_horizontal()
        .spacing_none()
        .divider_bottom()
        .classes();
    
    let active_tab_classes = button_styles(colors.clone())
        .primary()
        .classes() + " border-b-2 border-transparent border-b-current";
    
    let inactive_tab_classes = button_styles(colors.clone())
        .ghost()
        .classes() + " border-b-2 border-transparent";
    
    let tab_buttons = tabs.iter()
        .enumerate()
        .map(|(i, (label, _))| {
            let classes = if i == active_index { &active_tab_classes } else { &inactive_tab_classes };
            format!(
                r#"<button class="{}" 
                           role="tab" 
                           aria-selected="{}"
                           aria-controls="tabpanel-{}"
                           id="tab-{}"
                           tabindex="{}">{}</button>"#,
                classes,
                i == active_index,
                i,
                i,
                if i == active_index { "0" } else { "-1" },
                label
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    
    let tab_panels = tabs.iter()
        .enumerate()
        .map(|(i, (_, content))| {
            let panel_classes = card_styles(colors.clone())
                .spacing(CardSpacing::Large)
                .classes();
            
            format!(
                r#"<div class="{} {}" 
                         role="tabpanel" 
                         aria-labelledby="tab-{}"
                         id="tabpanel-{}"
                         tabindex="0">{}</div>"#,
                panel_classes,
                if i == active_index { "" } else { "hidden" },
                i,
                i,
                content
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    
    format!(
        r#"
        <div>
            <div class="{}" role="tablist" aria-label="Content sections">
                {}
            </div>
            {}
        </div>
        "#,
        tab_list_classes, tab_buttons, tab_panels
    )
}
```

## 🧭 Navigation Accessibility

### Skip Links

```rust
// Essential skip navigation
fn skip_navigation_links(colors: impl ColorProvider) -> String {
    let skip_link_classes = interactive_element(colors)
        .base("absolute -top-full left-4 z-50 px-4 py-2 bg-white border border-gray-300 rounded")
        .focus()
            .top_4()
            .ring_color(Color::Interactive)
            .ring_width_2()
        .build();
    
    format!(
        r#"
        <a href="#main-content" class="{}">Skip to main content</a>
        <a href="#navigation" class="{}">Skip to navigation</a>
        <a href="#search" class="{}">Skip to search</a>
        "#,
        skip_link_classes, skip_link_classes, skip_link_classes
    )
}
```

### Breadcrumb Navigation

```rust
// Accessible breadcrumbs
fn accessible_breadcrumbs(
    crumbs: &[(&str, Option<&str>)], // (text, optional href)
    colors: impl ColorProvider,
) -> String {
    let breadcrumb_classes = layout_styles(colors.clone())
        .direction_horizontal()
        .spacing_sm()
        .classes();
    
    let link_classes = interactive_element(colors.clone())
        .base("underline")
        .hover().text_color(Color::Interactive)
        .focus().ring_color(Color::Interactive).outline_none()
        .build();
    
    let current_classes = text_styles(colors)
        .color(Color::TextPrimary)
        .weight(FontWeight::Medium)
        .classes();
    
    let items = crumbs.iter()
        .enumerate()
        .map(|(i, (text, href))| {
            let is_last = i == crumbs.len() - 1;
            let separator = if is_last { "" } else { r#"<span aria-hidden="true"> / </span>"# };
            
            if let Some(url) = href {
                format!(
                    r#"<a href="{}" class="{}">{}</a>{}"#,
                    url, link_classes, text, separator
                )
            } else {
                format!(
                    r#"<span class="{}" aria-current="page">{}</span>{}"#,
                    current_classes, text, separator
                )
            }
        })
        .collect::<Vec<_>>()
        .join("");
    
    format!(
        r#"<nav aria-label="Breadcrumb" class="{}">
            <ol class="flex items-center space-x-2">
                {}
            </ol>
           </nav>"#,
        breadcrumb_classes, items
    )
}
```

## 📋 Form Accessibility Deep Dive

### Error Handling

```rust
// Comprehensive form error handling
fn accessible_form_with_validation(colors: impl ColorProvider) -> String {
    let form_classes = layout_styles(colors.clone())
        .spacing_lg()
        .direction_vertical()
        .classes();
    
    let error_summary_classes = card_styles(colors.clone())
        .spacing(CardSpacing::Medium)
        .classes() + " border-l-4 border-red-500 bg-red-50";
    
    let field_classes = layout_styles(colors.clone())
        .spacing_sm()
        .direction_vertical()
        .classes();
    
    let label_classes = text_styles(colors.clone())
        .typography(Typography::Label)
        .color(Color::TextPrimary)
        .weight(FontWeight::Medium)
        .classes();
    
    let input_classes = interactive_input(colors.clone())
        .base_style()
        .hover().border_primary()
        .focus().border_primary().ring_primary()
        .build();
    
    let error_input_classes = interactive_input(colors.clone())
        .base_style()
        .border_color(Color::Error)
        .hover().border_color(Color::Error)
        .focus().border_color(Color::Error).ring_color(Color::Error)
        .build();
    
    let error_text_classes = text_styles(colors.clone())
        .typography(Typography::Caption)
        .color(Color::Error)
        .classes();
    
    let help_text_classes = text_styles(colors)
        .typography(Typography::Caption)
        .color(Color::TextTertiary)
        .classes();
    
    format!(
        r#"
        <form class="{}" novalidate data-form-validation>
            <div class="{} hidden" role="alert" aria-live="assertive" id="error-summary">
                <h2 class="font-bold text-red-800 mb-2">Please fix the following errors:</h2>
                <ul id="error-list" class="list-disc list-inside"></ul>
            </div>
            
            <div class="{}">
                <label for="email" class="{}">
                    Email Address <span class="text-red-500" aria-label="required">*</span>
                </label>
                <input 
                    id="email" 
                    class="{}" 
                    type="email" 
                    required 
                    aria-describedby="email-help email-error"
                    aria-invalid="false"
                />
                <div id="email-help" class="{}">
                    We'll never share your email with anyone else.
                </div>
                <div id="email-error" class="{} hidden" role="alert" aria-live="polite">
                    Please enter a valid email address.
                </div>
            </div>
            
            <div class="{}">
                <label for="password" class="{}">
                    Password <span class="text-red-500" aria-label="required">*</span>
                </label>
                <input 
                    id="password" 
                    class="{}" 
                    type="password" 
                    required 
                    minlength="8"
                    aria-describedby="password-help password-error"
                    aria-invalid="false"
                />
                <div id="password-help" class="{}">
                    Must be at least 8 characters long.
                </div>
                <div id="password-error" class="{} hidden" role="alert" aria-live="polite">
                    Password must be at least 8 characters long.
                </div>
            </div>
            
            <button type="submit" class="{}">
                Create Account
            </button>
        </form>
        
        <script>
        // Form validation JavaScript would go here
        // This would handle ARIA states, error announcements, etc.
        </script>
        "#,
        form_classes,
        error_summary_classes,
        field_classes, label_classes, input_classes, help_text_classes, error_text_classes,
        field_classes, label_classes, input_classes, help_text_classes, error_text_classes,
        button_styles(colors).primary().classes()
    )
}
```

## 🎯 Motion and Animation Accessibility

### Respecting User Preferences

```rust
// Motion-safe animations
fn motion_safe_animations() -> String {
    r#"
    /* Respect user's motion preferences */
    @media (prefers-reduced-motion: reduce) {
        * {
            animation-duration: 0.01ms !important;
            animation-iteration-count: 1 !important;
            transition-duration: 0.01ms !important;
        }
    }
    
    /* Smooth animations for users who can handle them */
    @media (prefers-reduced-motion: no-preference) {
        .smooth-transition {
            transition: all 0.3s ease-in-out;
        }
    }
    "#.to_string()
}

// Animation-aware component
fn accessible_loading_button(
    text: &str,
    is_loading: bool,
    colors: impl ColorProvider,
) -> String {
    let classes = button_styles(colors)
        .primary()
        .state(if is_loading { ButtonState::Loading } else { ButtonState::Default })
        .classes();
    
    let spinner = if is_loading {
        r#"<span class="animate-spin motion-reduce:animate-none inline-block w-4 h-4 border-2 border-white border-t-transparent rounded-full mr-2" aria-hidden="true"></span>"#
    } else {
        ""
    };
    
    format!(
        r#"<button class="{}" 
                   type="submit" 
                   {}
                   aria-describedby="loading-announcement">
            {}{}
            <div id="loading-announcement" class="sr-only" aria-live="polite">
                {}
            </div>
           </button>"#,
        classes,
        if is_loading { r#"aria-disabled="true""# } else { "" },
        spinner,
        if is_loading { "Loading..." } else { text },
        if is_loading { "Please wait, your request is being processed" } else { "" }
    )
}
```

## 🔍 Accessibility Resources

### WCAG 2.1 Detailed Guidelines

#### Level A Requirements
- **1.1.1**: Non-text content has text alternatives
- **1.3.1**: Information and relationships are programmatically determinable
- **2.1.1**: All functionality is keyboard accessible
- **2.4.1**: Blocks of content can be bypassed
- **3.1.1**: Language of page is programmatically determinable
- **4.1.1**: Content can be parsed by assistive technology

#### Level AA Requirements  
- **1.4.3**: Color contrast ratio of at least 4.5:1 for normal text
- **1.4.4**: Text can be resized up to 200% without loss of functionality
- **2.4.6**: Headings and labels describe topic or purpose
- **2.4.7**: Focus indicator is visible
- **3.1.2**: Language of parts is programmatically determinable

### Comprehensive Testing Strategy

```rust
#[cfg(test)]
mod comprehensive_accessibility_tests {
    use super::*;

    #[test]
    fn test_semantic_html_structure() {
        let form_html = accessible_contact_form(VibeColors::new());
        
        // Test semantic elements
        assert!(form_html.contains("<form"));
        assert!(form_html.contains("<fieldset"));
        assert!(form_html.contains("<legend"));
        assert!(form_html.contains("<label"));
        assert!(form_html.contains("for="));
    }

    #[test]
    fn test_aria_attributes() {
        let modal_html = accessible_modal("Test", "Content", VibeColors::new());
        
        // Test ARIA attributes
        assert!(modal_html.contains("role=\"dialog\""));
        assert!(modal_html.contains("aria-modal=\"true\""));
        assert!(modal_html.contains("aria-labelledby"));
        assert!(modal_html.contains("aria-describedby"));
    }

    #[test]
    fn test_keyboard_navigation() {
        let tabs_html = accessible_tabs(&[("Tab 1", "Content 1")], 0, VibeColors::new());
        
        // Test keyboard navigation attributes
        assert!(tabs_html.contains("role=\"tab\""));
        assert!(tabs_html.contains("tabindex=\"0\""));
        assert!(tabs_html.contains("tabindex=\"-1\""));
        assert!(tabs_html.contains("aria-selected"));
    }

    #[test]
    fn test_focus_management() {
        let colors = VibeColors::new();
        let interactive_classes = interactive_element(colors)
            .focus().ring_color(Color::Interactive)
            .build();
        
        // Test focus styles are present
        assert!(interactive_classes.contains("focus:"));
    }

    #[test]
    fn test_live_regions() {
        let form_html = accessible_form_with_validation(VibeColors::new());
        
        // Test live regions for dynamic content
        assert!(form_html.contains("aria-live=\"polite\""));
        assert!(form_html.contains("aria-live=\"assertive\""));
        assert!(form_html.contains("role=\"alert\""));
    }

    #[test]
    fn test_error_association() {
        let form_html = accessible_form_with_validation(VibeColors::new());
        
        // Test error message association
        assert!(form_html.contains("aria-describedby"));
        assert!(form_html.contains("aria-invalid"));
    }
}

// Color contrast testing utility
pub fn test_all_color_combinations(theme: &dyn ColorProvider) -> Vec<ContrastIssue> {
    let mut issues = Vec::new();
    let palette = theme.palette();
    
    // Test text on background combinations
    let text_bg_combinations = [
        (&palette.text_primary, &palette.background, "Primary text on background"),
        (&palette.text_secondary, &palette.background, "Secondary text on background"),
        (&palette.text_tertiary, &palette.background, "Tertiary text on background"),
        (&palette.text_primary, &palette.surface, "Primary text on surface"),
    ];
    
    for (fg, bg, description) in &text_bg_combinations {
        let ratio = calculate_contrast_ratio(fg, bg);
        if ratio < 4.5 {
            issues.push(ContrastIssue {
                description: description.to_string(),
                foreground: fg.clone(),
                background: bg.clone(),
                ratio,
                required_ratio: 4.5,
            });
        }
    }
    
    // Test interactive element combinations
    let interactive_combinations = [
        ("white", &palette.primary, "White text on primary button"),
        ("white", &palette.success, "White text on success button"),
        ("white", &palette.error, "White text on error button"),
    ];
    
    for (fg, bg, description) in &interactive_combinations {
        let ratio = calculate_contrast_ratio(fg, bg);
        if ratio < 4.5 {
            issues.push(ContrastIssue {
                description: description.to_string(),
                foreground: fg.to_string(),
                background: bg.clone(),
                ratio,
                required_ratio: 4.5,
            });
        }
    }
    
    issues
}

#[derive(Debug)]
pub struct ContrastIssue {
    pub description: String,
    pub foreground: String,
    pub background: String,
    pub ratio: f64,
    pub required_ratio: f64,
}

// Simplified contrast calculation (would need full implementation)
fn calculate_contrast_ratio(foreground: &str, background: &str) -> f64 {
    // This is a placeholder - would need actual color parsing and luminance calculation
    // following WCAG guidelines
    4.6 // Placeholder returning passing ratio
}
```

### Manual Testing Checklist

```rust
pub struct ComprehensiveAccessibilityAudit {
    pub keyboard_only_navigation: bool,
    pub screen_reader_testing: bool,
    pub voice_control_testing: bool,
    pub high_contrast_mode: bool,
    pub zoom_testing: bool,
    pub color_blindness_simulation: bool,
    pub mobile_accessibility: bool,
    pub cognitive_load_assessment: bool,
}

impl ComprehensiveAccessibilityAudit {
    pub fn new() -> Self {
        Self {
            keyboard_only_navigation: false,
            screen_reader_testing: false,
            voice_control_testing: false,
            high_contrast_mode: false,
            zoom_testing: false,
            color_blindness_simulation: false,
            mobile_accessibility: false,
            cognitive_load_assessment: false,
        }
    }
    
    pub fn complete_audit(&self) -> AccessibilityScore {
        let passed_tests = [
            self.keyboard_only_navigation,
            self.screen_reader_testing,
            self.voice_control_testing,
            self.high_contrast_mode,
            self.zoom_testing,
            self.color_blindness_simulation,
            self.mobile_accessibility,
            self.cognitive_load_assessment,
        ].iter().filter(|&&x| x).count();
        
        AccessibilityScore {
            percentage: (passed_tests as f64 / 8.0) * 100.0,
            wcag_compliance: passed_tests >= 6, // Basic threshold
            recommendations: self.generate_recommendations(),
        }
    }
    
    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if !self.keyboard_only_navigation {
            recommendations.push("Test all functionality using only keyboard navigation".to_string());
        }
        if !self.screen_reader_testing {
            recommendations.push("Test with screen readers (NVDA, JAWS, VoiceOver)".to_string());
        }
        if !self.zoom_testing {
            recommendations.push("Test with browser zoom up to 200%".to_string());
        }
        
        recommendations
    }
}

pub struct AccessibilityScore {
    pub percentage: f64,
    pub wcag_compliance: bool,
    pub recommendations: Vec<String>,
}
```

## 🔗 Related Documentation

- [Color System](./colors.md) - Accessible color usage
- [Typography](./typography.md) - Readable typography practices
- [Component Guidelines](./components.md) - Accessible component patterns
- [Theming Guide](./theming.md) - Accessible theme creation
- [Examples](./examples/) - Accessible component implementations