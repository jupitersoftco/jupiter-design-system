# Spacing & Layout Best Practices

## 📐 Spacing Philosophy

The Jupiter Design System's spacing system creates visual harmony through consistent, proportional spacing that enhances readability and establishes clear relationships between elements.

### Core Principles

1. **Systematic Scale**: Proportional spacing system for visual harmony
2. **Semantic Purpose**: Spacing conveys relationships and hierarchy
3. **Responsive Adaptation**: Spacing adjusts appropriately across breakpoints
4. **Accessibility Focus**: Adequate spacing for touch targets and readability

## 📏 Spacing System

### Spacing Scale

The system provides a systematic spacing scale based on a consistent ratio:

```rust
Spacing::None      // 0rem (0px) - No spacing
Spacing::XSmall    // 0.25rem (4px) - Minimal spacing
Spacing::Small     // 0.5rem (8px) - Tight spacing
Spacing::Medium    // 1rem (16px) - Standard spacing
Spacing::Large     // 1.5rem (24px) - Generous spacing
Spacing::XLarge    // 2rem (32px) - Large spacing
Spacing::XXLarge   // 3rem (48px) - Maximum spacing
```

### Layout Spacing Types

Specialized spacing for layout components:

```rust
LayoutSpacing::None  // No spacing
LayoutSpacing::XS    // 0.25rem - Minimal layout spacing
LayoutSpacing::SM    // 0.5rem - Small layout spacing
LayoutSpacing::MD    // 1rem - Medium layout spacing
LayoutSpacing::LG    // 1.5rem - Large layout spacing
LayoutSpacing::XL    // 2rem - Extra large spacing
LayoutSpacing::XL2   // 3rem - Maximum layout spacing
```

### Spacing Application Types

Different ways to apply spacing:

```rust
// Padding (internal spacing)
let padded_element = colors.padding_class(Spacing::Medium);  // "p-4"

// Margin (external spacing)
let spaced_element = colors.margin_class(Spacing::Large);    // "m-6"

// Directional spacing
let top_spaced = "pt-4";     // Padding top
let bottom_spaced = "pb-4";  // Padding bottom
let left_spaced = "pl-4";    // Padding left
let right_spaced = "pr-4";   // Padding right
```

## 🏗️ Layout System

### Layout Direction

```rust
LayoutDirection::Vertical    // Stack elements vertically
LayoutDirection::Horizontal  // Arrange elements horizontally
```

### Layout Alignment

```rust
LayoutAlignment::Start    // Align to start (left/top)
LayoutAlignment::Center   // Center alignment
LayoutAlignment::End      // Align to end (right/bottom)
LayoutAlignment::Between  // Space between elements
LayoutAlignment::Around   // Space around elements
LayoutAlignment::Evenly   // Even spacing distribution
```

### Layout Dividers

```rust
LayoutDivider::None     // No divider
LayoutDivider::Top      // Top border divider
LayoutDivider::Bottom   // Bottom border divider
LayoutDivider::Left     // Left border divider
LayoutDivider::Right    // Right border divider
```

## 🛠️ Implementation Patterns

### Basic Spacing Usage

```rust
use jupiter_design_system::prelude::*;

let colors = VibeColors::new();

// Card with internal spacing
let card_classes = card_styles(colors)
    .spacing(CardSpacing::Large)  // Generous internal padding
    .classes();

// Button with appropriate spacing
let button_classes = button_styles(colors)
    .size(Size::Medium)  // Includes built-in spacing
    .classes();

// Text with reading-friendly spacing
let text_classes = text_styles(colors)
    .typography(Typography::Body)
    .spacing(Spacing::Medium)  // Line height and letter spacing
    .classes();
```

### Layout Component Building

```rust
use jupiter_design_system::builders::layout::LayoutStyles;

// Card header with divider
let header_layout = LayoutStyles::new(colors)
    .divider_bottom()
    .spacing_md()
    .direction_horizontal()
    .alignment_between()
    .classes();

// Card content area
let content_layout = LayoutStyles::new(colors)
    .spacing_lg()
    .direction_vertical()
    .alignment_start()
    .classes();

// Card footer with top divider
let footer_layout = LayoutStyles::new(colors)
    .divider_top()
    .spacing_md()
    .direction_horizontal()
    .alignment_end()
    .classes();
```

### Responsive Spacing

```rust
// Spacing that adapts to screen size
let responsive_card = card_styles(colors)
    .spacing(CardSpacing::Medium)      // Base spacing
    .responsive_spacing(Breakpoint::Tablet, CardSpacing::Large)
    .responsive_spacing(Breakpoint::Desktop, CardSpacing::XLarge)
    .classes();

// Layout with responsive direction
let responsive_layout = LayoutStyles::new(colors)
    .direction_vertical()              // Mobile: stack vertically
    .responsive_direction(Breakpoint::Tablet, LayoutDirection::Horizontal)
    .spacing_md()
    .classes();
```

## 📋 Spacing Guidelines

### Visual Hierarchy Through Spacing

```rust
// Page-level spacing hierarchy
let page_title_spacing = Spacing::XXLarge;    // Maximum space around titles
let section_spacing = Spacing::XLarge;        // Large space between sections
let paragraph_spacing = Spacing::Medium;      // Medium space between paragraphs
let inline_spacing = Spacing::Small;          // Small space between inline elements

// Component spacing hierarchy
let card_container_spacing = Spacing::Large;  // Space around cards
let card_internal_spacing = Spacing::Medium;  // Space inside cards
let button_spacing = Spacing::Small;          // Space around buttons
```

### Content Relationships

```rust
// Related content grouping
let related_group = LayoutStyles::new(colors)
    .spacing_sm()        // Tight spacing for related items
    .direction_vertical()
    .classes();

// Separated content sections
let separated_section = LayoutStyles::new(colors)
    .spacing_xl()        // Large spacing for distinct sections
    .divider_top()       // Visual separator
    .classes();

// Interactive element spacing
let interactive_area = LayoutStyles::new(colors)
    .spacing_md()        // Adequate touch target spacing
    .direction_horizontal()
    .alignment_center()
    .classes();
```

### Form Layout Patterns

```rust
// Form field spacing
let form_field_spacing = LayoutStyles::new(colors)
    .spacing_md()        // Standard field spacing
    .direction_vertical()
    .alignment_start()
    .classes();

// Form section spacing
let form_section_spacing = LayoutStyles::new(colors)
    .spacing_lg()        // Section separation
    .divider_bottom()    // Visual grouping
    .classes();

// Button group spacing
let button_group_spacing = LayoutStyles::new(colors)
    .spacing_sm()        // Related button spacing
    .direction_horizontal()
    .alignment_end()
    .classes();
```

## 🎨 Spacing in Component Design

### Card Component Spacing

```rust
// Card with proper spacing hierarchy
let card_header = LayoutStyles::new(colors)
    .spacing_md()
    .divider_bottom()
    .classes();

let card_content = LayoutStyles::new(colors)
    .spacing_lg()        // Generous reading space
    .direction_vertical()
    .classes();

let card_actions = LayoutStyles::new(colors)
    .spacing_md()
    .divider_top()
    .direction_horizontal()
    .alignment_end()
    .classes();
```

### Navigation Spacing

```rust
// Navigation bar spacing
let navbar_spacing = LayoutStyles::new(colors)
    .spacing_md()
    .direction_horizontal()
    .alignment_between()
    .classes();

// Navigation menu spacing
let menu_spacing = LayoutStyles::new(colors)
    .spacing_sm()        // Compact menu spacing
    .direction_vertical()
    .alignment_start()
    .classes();

// Navigation item spacing
let nav_item_spacing = LayoutStyles::new(colors)
    .spacing_sm()
    .direction_horizontal()
    .alignment_center()
    .classes();
```

### Grid and List Spacing

```rust
// Grid container spacing
let grid_spacing = LayoutStyles::new(colors)
    .spacing_lg()        // Space between grid items
    .classes();

// List item spacing
let list_item_spacing = LayoutStyles::new(colors)
    .spacing_md()
    .divider_bottom()    // Subtle separation
    .classes();

// Data table spacing
let table_cell_spacing = LayoutStyles::new(colors)
    .spacing_sm()        // Compact data presentation
    .classes();
```

## 📱 Responsive Layout Patterns

### Mobile-First Spacing

```rust
// Start with mobile spacing
let mobile_first_layout = LayoutStyles::new(colors)
    .spacing_sm()        // Compact on mobile
    .direction_vertical() // Stack on mobile
    
    // Expand for tablets
    .responsive_spacing(Breakpoint::Tablet, LayoutSpacing::MD)
    .responsive_direction(Breakpoint::Tablet, LayoutDirection::Horizontal)
    
    // Generous spacing on desktop
    .responsive_spacing(Breakpoint::Desktop, LayoutSpacing::LG)
    .classes();
```

### Breakpoint-Specific Layouts

```rust
// Different layouts per breakpoint
fn get_responsive_layout(colors: impl ColorProvider) -> String {
    LayoutStyles::new(colors)
        // Mobile: vertical, tight spacing
        .direction_vertical()
        .spacing_sm()
        
        // Tablet: horizontal, medium spacing
        .responsive_direction(Breakpoint::Tablet, LayoutDirection::Horizontal)
        .responsive_spacing(Breakpoint::Tablet, LayoutSpacing::MD)
        .responsive_alignment(Breakpoint::Tablet, LayoutAlignment::Between)
        
        // Desktop: expanded spacing
        .responsive_spacing(Breakpoint::Desktop, LayoutSpacing::LG)
        .classes()
}
```

### Container Patterns

```rust
// Page container with responsive spacing
let page_container = LayoutStyles::new(colors)
    .spacing_md()                     // Base container spacing
    .responsive_spacing(Breakpoint::Tablet, LayoutSpacing::LG)
    .responsive_spacing(Breakpoint::Desktop, LayoutSpacing::XL)
    .classes();

// Content container with max width
let content_container = LayoutStyles::new(colors)
    .spacing_lg()
    .direction_vertical()
    .alignment_center()
    .classes();
```

## 🚦 Spacing Usage Guidelines

### Do's ✅

1. **Use Systematic Spacing**
   ```rust
   // ✅ Good: System-based spacing
   let spaced_card = card_styles(colors)
       .spacing(CardSpacing::Large)
       .classes();
   
   let layout = LayoutStyles::new(colors)
       .spacing_md()
       .classes();
   ```

2. **Create Visual Hierarchy**
   ```rust
   // ✅ Good: Clear spacing hierarchy
   let title_spacing = Spacing::XXLarge;      // Most space
   let section_spacing = Spacing::XLarge;     // Less space
   let paragraph_spacing = Spacing::Medium;   // Standard space
   let inline_spacing = Spacing::Small;       // Minimal space
   ```

3. **Consider Touch Targets**
   ```rust
   // ✅ Good: Adequate interactive spacing
   let button_layout = LayoutStyles::new(colors)
       .spacing_md()       // 44px minimum touch target
       .direction_horizontal()
       .alignment_center()
       .classes();
   ```

### Don'ts ❌

1. **Don't Use Arbitrary Spacing**
   ```rust
   // ❌ Bad: Custom spacing values
   let custom_spacing = "p-7 m-3";  // Not in system
   
   // ✅ Good: System spacing
   let system_spacing = LayoutStyles::new(colors)
       .spacing_lg()
       .classes();
   ```

2. **Don't Ignore Relationships**
   ```rust
   // ❌ Bad: Same spacing for unrelated elements
   let title_spacing = Spacing::Small;
   let paragraph_spacing = Spacing::Small;
   
   // ✅ Good: Hierarchical spacing
   let title_spacing = Spacing::XLarge;
   let paragraph_spacing = Spacing::Medium;
   ```

3. **Don't Forget Responsive Behavior**
   ```rust
   // ❌ Bad: Fixed spacing on all screens
   let fixed_layout = LayoutStyles::new(colors)
       .spacing_xl()  // Too much on mobile
       .classes();
   
   // ✅ Good: Responsive spacing
   let responsive_layout = LayoutStyles::new(colors)
       .spacing_sm()
       .responsive_spacing(Breakpoint::Desktop, LayoutSpacing::XL)
       .classes();
   ```

## ♿ Accessibility Considerations

### Touch Target Spacing

```rust
// Minimum 44px touch targets
let accessible_button = button_styles(colors)
    .size(Size::Medium)  // Built-in accessible sizing
    .classes();

let accessible_layout = LayoutStyles::new(colors)
    .spacing_md()        // Ensures adequate spacing between interactive elements
    .classes();
```

### Reading Flow

```rust
// Comfortable reading spacing
let reading_content = LayoutStyles::new(colors)
    .spacing_lg()        // Generous line spacing
    .direction_vertical()
    .alignment_start()
    .classes();

// Scannable content spacing
let scannable_list = LayoutStyles::new(colors)
    .spacing_md()        // Clear item separation
    .direction_vertical()
    .classes();
```

### Focus Management

```rust
// Clear focus separation
let focus_area = LayoutStyles::new(colors)
    .spacing_md()        // Space for focus indicators
    .classes();
```

## 🧪 Testing Spacing and Layout

### Visual Consistency Tests

```rust
#[cfg(test)]
mod spacing_tests {
    use super::*;

    #[test]
    fn test_spacing_scale_consistency() {
        let colors = VibeColors::new();
        
        // Test that spacing values follow expected progression
        let small_spacing = colors.margin_class(Spacing::Small);
        let medium_spacing = colors.margin_class(Spacing::Medium);
        let large_spacing = colors.margin_class(Spacing::Large);
        
        assert!(small_spacing.contains("m-2"));
        assert!(medium_spacing.contains("m-4"));
        assert!(large_spacing.contains("m-6"));
    }

    #[test]
    fn test_layout_direction_classes() {
        let colors = VibeColors::new();
        
        let vertical_layout = LayoutStyles::new(colors.clone())
            .direction_vertical()
            .classes();
            
        let horizontal_layout = LayoutStyles::new(colors)
            .direction_horizontal()
            .classes();
        
        assert!(vertical_layout.contains("flex-col"));
        assert!(horizontal_layout.contains("flex-row"));
    }
}
```

### Responsive Behavior Tests

```rust
#[test]
fn test_responsive_spacing() {
    let colors = VibeColors::new();
    
    let responsive_layout = LayoutStyles::new(colors)
        .spacing_sm()
        .responsive_spacing(Breakpoint::Tablet, LayoutSpacing::MD)
        .classes();
    
    // Should contain both base and responsive classes
    assert!(responsive_layout.contains("p-2"));
    assert!(responsive_layout.contains("md:p-4"));
}
```

## 📊 Performance Considerations

### Layout Caching

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CachedLayoutProvider {
    cache: HashMap<(LayoutSpacing, LayoutDirection, LayoutAlignment), String>,
}

impl CachedLayoutProvider {
    pub fn get_layout(&mut self, 
        spacing: LayoutSpacing,
        direction: LayoutDirection,
        alignment: LayoutAlignment,
        colors: impl ColorProvider,
    ) -> &String {
        self.cache.entry((spacing, direction, alignment)).or_insert_with(|| {
            LayoutStyles::new(colors)
                .spacing(spacing)
                .direction(direction)
                .alignment(alignment)
                .classes()
        })
    }
}
```

### Common Layout Patterns

```rust
// Pre-defined common layouts for performance
pub mod common_layouts {
    use super::*;
    
    pub fn card_header(colors: impl ColorProvider) -> String {
        LayoutStyles::new(colors)
            .spacing_md()
            .direction_horizontal()
            .alignment_between()
            .divider_bottom()
            .classes()
    }
    
    pub fn button_group(colors: impl ColorProvider) -> String {
        LayoutStyles::new(colors)
            .spacing_sm()
            .direction_horizontal()
            .alignment_end()
            .classes()
    }
    
    pub fn form_field(colors: impl ColorProvider) -> String {
        LayoutStyles::new(colors)
            .spacing_md()
            .direction_vertical()
            .alignment_start()
            .classes()
    }
}
```

## 🔗 Related Documentation

- [Color System](./colors.md) - Color integration in layouts
- [Typography](./typography.md) - Text spacing and hierarchy
- [Component Guidelines](./components.md) - Component-specific spacing
- [Accessibility Guide](./accessibility.md) - Accessible spacing practices
- [Examples](./examples/) - Real-world layout patterns