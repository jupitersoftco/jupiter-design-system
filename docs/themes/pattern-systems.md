# Pattern Systems Documentation

This document provides comprehensive documentation for all pattern systems in Jupiter Design System.

## Overview

Patterns in Jupiter Design System are abstract, composable concepts that define behavior and semantics independent of visual styling. They work with any `ColorProvider` implementation.

## 1. Action Patterns (`patterns/actions.rs`)

Action patterns define the semantic meaning and hierarchy of user actions.

### Core Concepts

#### ActionIntent
Defines what an action does semantically:
```rust
pub enum ActionIntent {
    Primary,      // Main action user should take
    Secondary,    // Alternative or supplementary action
    Constructive, // Creates, adds, confirms
    Destructive,  // Deletes, removes, cancels
    Navigation,   // Moves between contexts
    Informational,// Shows details, help, etc.
}
```

#### ActionHierarchy
Defines how prominent an action should be:
```rust
pub enum ActionHierarchy {
    Hero,     // Most important action on page
    Primary,  // Important action in section
    Secondary,// Less important but visible
    Tertiary, // Subtle, supporting action
    Minimal,  // Very subtle, almost invisible
}
```

#### ActionContext
Defines where/how the action is used:
```rust
pub enum ActionContext {
    Standalone, // Normal buttons
    Form,       // Submit, cancel buttons
    Navigation, // Menu items, tabs
    Inline,     // Links, embedded actions
    Toolbar,    // Action bars
    Floating,   // FABs, overlays
}
```

### Usage Example
```rust
use jupiter_design_system::patterns::*;

let colors = VibeColors::default();

// Create primary action
let primary_action = action_semantics(colors.clone())
    .primary()
    .classes();

// Create hero CTA with urgency
let hero_cta = action_semantics(colors.clone())
    .hero()
    .urgent()
    .classes();

// Create destructive action in form context
let delete_button = action_semantics(colors)
    .destructive()
    .context(ActionContext::Form)
    .classes();
```

## 2. Button Pattern (`patterns/button.rs`)

The button pattern is a composite pattern that combines action semantics, interactive behavior, and focus management.

### Complete Button Abstraction
```rust
pub struct ButtonPattern<C: ColorProvider + Clone> {
    disabled: bool,
    loading: bool,
    selected: bool,
    action_semantics: ActionSemantics<C>,
    interactive_element: InteractiveElement<C>,
    focus_management: FocusManagement<C>,
    custom_classes: Vec<String>,
}
```

### Convenience Functions
```rust
// Primary button with standard styling
let primary = primary_button(colors.clone()).classes();

// Secondary button
let secondary = secondary_button(colors.clone()).classes();

// Destructive button
let destructive = destructive_button(colors.clone()).classes();

// Hero CTA button
let hero = hero_button(colors.clone()).classes();

// Navigation button (menu item style)
let nav = navigation_button(colors.clone()).classes();

// Link that acts like a button
let link_button = button_link(colors).classes();
```

### Advanced Composition
```rust
let custom_button = button_pattern(colors)
    .primary_action()           // Set action intent
    .hero_prominence()          // Set hierarchy
    .prominent_interaction()    // Set interaction style
    .prominent_focus()          // Set focus behavior
    .disabled(false)            // Set state
    .loading(false)
    .selected(false)
    .custom("my-custom-class")  // Add custom classes
    .classes();

// Get accessibility attributes
let attrs = button_pattern(colors.clone())
    .primary_action()
    .loading(true)
    .accessibility_attributes();
// Returns: [("aria-busy", "true"), ("tabindex", "0"), ...]

// Get semantic information
let info = destructive_button(colors)
    .disabled(true)
    .semantic_info();
// Returns: ButtonSemanticInfo { 
//   action_intent: Destructive,
//   is_destructive: true,
//   is_disabled: true,
//   ...
// }
```

## 3. Card Pattern (`patterns/card.rs`)

Card patterns define container semantics for grouped content.

### Card Configuration Options

#### CardElevation
```rust
pub enum CardElevation {
    Flat,      // No shadow
    Raised,    // Small shadow
    Elevated,  // Medium shadow
    Floating,  // Large shadow
}
```

#### CardSpacing
```rust
pub enum CardSpacing {
    Compact,   // Minimal padding
    Default,   // Standard padding
    Spacious,  // Generous padding
}
```

#### CardSurface
```rust
pub enum CardSurface {
    Default,   // Standard surface
    Muted,     // Subdued background
    Accent,    // Accent background
    Interactive,// Hoverable surface
}
```

#### CardInteraction
```rust
pub enum CardInteraction {
    Static,    // Non-interactive
    Hoverable, // Hover effects
    Clickable, // Full click area
    Selectable,// Can be selected
}
```

### Usage Example
```rust
// Basic card
let card = card_pattern(colors.clone())
    .elevation(CardElevation::Raised)
    .spacing(CardSpacing::Default)
    .surface(CardSurface::Default)
    .interaction(CardInteraction::Static)
    .classes();

// Interactive product card
let product_card = card_pattern(colors.clone())
    .elevation(CardElevation::Elevated)
    .spacing(CardSpacing::Spacious)
    .surface(CardSurface::Interactive)
    .interaction(CardInteraction::Clickable)
    .hover_effect(true)
    .classes();

// Selectable card with custom styling
let selectable_card = card_pattern(colors)
    .elevation(CardElevation::Flat)
    .interaction(CardInteraction::Selectable)
    .selected(true)
    .custom("border-2")
    .classes();
```

## 4. Focus Management (`patterns/focus.rs`)

Focus patterns ensure proper keyboard navigation and accessibility.

### FocusBehavior
```rust
pub enum FocusBehavior {
    Default,   // Standard focus ring
    Subtle,    // Minimal focus indicator
    Prominent, // Strong focus indicator
    Custom(String), // Custom focus classes
}
```

### KeyboardPattern
```rust
pub enum KeyboardPattern {
    Button,    // Space/Enter activation
    Link,      // Enter activation only
    Toggle,    // Space toggles state
    MenuItem,  // Arrow navigation
    Tab,       // Tab panel behavior
}
```

### Usage Example
```rust
// Button focus management
let button_focus = focus_management(colors.clone())
    .button()
    .classes();

// Menu item with arrow navigation
let menu_focus = focus_management(colors.clone())
    .menu_item()
    .focus_behavior(FocusBehavior::Subtle)
    .classes();

// Custom focus with screen reader support
let custom_focus = focus_management(colors)
    .keyboard_pattern(KeyboardPattern::Toggle)
    .screen_reader_pattern(ScreenReaderPattern::Announcement)
    .focus_behavior(FocusBehavior::Prominent)
    .classes();
```

## 5. Interactive Elements (`patterns/interactions.rs`)

Interactive patterns define how elements respond to user interaction.

### InteractiveState
```rust
pub enum InteractiveState {
    Default,  // Normal state
    Hover,    // Mouse hover
    Active,   // Pressed/clicked
    Focused,  // Keyboard focus
    Disabled, // Non-interactive
    Loading,  // Processing
}
```

### InteractionIntensity
```rust
pub enum InteractionIntensity {
    Gentle,    // Subtle effects
    Standard,  // Normal effects
    Prominent, // Strong effects
}
```

### Usage Example
```rust
// Standard button interaction
let button_interaction = interactive_element(colors.clone())
    .hoverable()
    .focusable()
    .pressable()
    .standard_interaction()
    .classes();

// Gentle card interaction
let card_interaction = interactive_element(colors.clone())
    .hoverable()
    .gentle_interaction()
    .classes();

// Custom interaction with specific state
let custom_interaction = interactive_element(colors)
    .state(InteractiveState::Hover)
    .custom("transform scale-105")
    .classes();
```

## 6. Typography Pattern (`patterns/typography.rs`)

Typography patterns provide semantic text styling.

### Typography Hierarchy
```rust
pub enum TypographyHierarchy {
    Title,      // Main page title (h1)
    Heading,    // Section heading (h2)
    Subheading, // Sub-section (h3)
    H4, H5, H6, // Minor headings
    Body,       // Regular text
    BodyLarge,  // Emphasized body
    BodySmall,  // De-emphasized body
    Caption,    // Image captions
    Overline,   // Category labels
    Code,       // Monospace code
}
```

### Usage Example
```rust
// Page title
let title = title_typography(colors.clone())
    .color(TypographyColor::Primary)
    .alignment(TypographyAlignment::Center)
    .classes();

// Body text with line clamping
let body = body_typography(colors.clone())
    .size(TypographySize::LG)
    .weight(TypographyWeight::Medium)
    .overflow(TypographyOverflow::Clamp(3))
    .classes();

// Custom typography composition
let custom_text = typography_pattern(colors)
    .hierarchy(TypographyHierarchy::Caption)
    .color(TypographyColor::Muted)
    .alignment(TypographyAlignment::Right)
    .element(TypographyElement::Span)
    .classes();

// Get HTML element
let element = title_typography(colors.clone()).get_element();
// Returns: "h1"

// Get line clamp styles
let clamp_style = body_typography(colors)
    .overflow(TypographyOverflow::Clamp(2))
    .get_clamp_style();
// Returns: CSS style string for line clamping
```

## 7. Product Pattern (`patterns/product.rs`)

Product patterns provide e-commerce specific abstractions.

### ProductDisplayPattern
```rust
pub enum ProductDisplayPattern {
    ListItem,  // Standard listing
    Featured,  // Enhanced visibility
    Tile,      // Compact display
    Showcase,  // Detailed view
    Preview,   // Quick view
}
```

### ProductImagePattern
```rust
pub enum ProductImagePattern {
    Thumbnail,    // Small image
    Gallery,      // Multiple images
    Zoom,         // Zoomable image
    Placeholder,  // Loading state
}
```

### ProductPricePattern
```rust
pub enum ProductPricePattern {
    Regular,      // Standard price
    Sale,         // Discounted price
    Range,        // Price range
    Subscription, // Recurring price
}
```

### Usage Example
```rust
// Product listing item
let product_item = product_pattern(colors.clone())
    .display_pattern(ProductDisplayPattern::ListItem)
    .image_pattern(ProductImagePattern::Thumbnail)
    .price_pattern(ProductPricePattern::Regular)
    .availability(ProductAvailabilityState::Available)
    .classes();

// Featured product showcase
let featured = product_pattern(colors.clone())
    .display_pattern(ProductDisplayPattern::Featured)
    .image_pattern(ProductImagePattern::Gallery)
    .price_pattern(ProductPricePattern::Sale)
    .variant_pattern(Some(ProductVariantPattern::Dropdown))
    .interactive(true)
    .classes();

// Product with loading state
let loading_product = product_pattern(colors)
    .display_pattern(ProductDisplayPattern::Tile)
    .interaction_state(ProductInteractionState::Loading)
    .classes();
```

## 8. Selection Pattern (`patterns/selection.rs`)

Selection patterns handle single and multi-select behaviors.

### SelectionState
```rust
pub enum SelectionState {
    Unselected,    // Not selected
    Selected,      // Selected
    Indeterminate, // Partially selected
}
```

### SelectionBehavior
```rust
pub enum SelectionBehavior {
    Single,    // Radio button style
    Multiple,  // Checkbox style
    Toggle,    // Switch style
}
```

### Usage Example
```rust
// Checkbox style selection
let checkbox = selection_pattern(colors.clone())
    .behavior(SelectionBehavior::Multiple)
    .state(SelectionState::Selected)
    .classes();

// Radio button style
let radio = selection_pattern(colors.clone())
    .behavior(SelectionBehavior::Single)
    .state(SelectionState::Unselected)
    .disabled(false)
    .classes();

// Toggle switch
let toggle = selection_pattern(colors)
    .behavior(SelectionBehavior::Toggle)
    .state(SelectionState::Selected)
    .label_position(LabelPosition::Right)
    .classes();
```

## 9. State Pattern (`patterns/states.rs`)

State patterns manage component states and transitions.

### ComponentState
```rust
pub enum ComponentState {
    Default,   // Normal state
    Active,    // Currently active
    Success,   // Operation succeeded
    Warning,   // Needs attention
    Error,     // Has errors
}
```

### LoadingState
```rust
pub enum LoadingState {
    Idle,      // Not loading
    Loading,   // In progress
    Skeleton,  // Skeleton loader
    Progress(u8), // With percentage
}
```

### ValidationState
```rust
pub enum ValidationState {
    Valid,     // Passes validation
    Invalid,   // Fails validation
    Pending,   // Being validated
}
```

### Usage Example
```rust
// Success state
let success = state_pattern(colors.clone())
    .component_state(ComponentState::Success)
    .with_icon(true)
    .classes();

// Loading with skeleton
let loading = state_pattern(colors.clone())
    .loading_state(LoadingState::Skeleton)
    .classes();

// Form field with validation
let field = state_pattern(colors)
    .validation_state(ValidationState::Invalid)
    .with_message("Email is required")
    .classes();
```

## 10. Layout Pattern (`patterns/layout.rs`)

Layout patterns provide structural composition.

### CardSectionLayout
Manages layout within card sections:
```rust
let header = CardSectionLayout::new(colors.clone())
    .header()
    .with_divider(true)
    .classes();

let content = CardSectionLayout::new(colors.clone())
    .content()
    .spacing(Spacing::Large)
    .classes();

let footer = CardSectionLayout::new(colors)
    .footer()
    .alignment(Alignment::Right)
    .classes();
```

### LayoutBuilder
General purpose layout composition:
```rust
let layout = LayoutBuilder::new(colors)
    .direction(Direction::Row)
    .gap(Spacing::Medium)
    .align(Alignment::Center)
    .justify(Justification::SpaceBetween)
    .wrap(true)
    .classes();
```

## Pattern Composition

Patterns are designed to work together:

```rust
// E-commerce product card combining multiple patterns
let product_card = {
    let card_classes = card_pattern(colors.clone())
        .elevation(CardElevation::Raised)
        .interaction(CardInteraction::Clickable)
        .classes();
    
    let product_classes = product_pattern(colors.clone())
        .display_pattern(ProductDisplayPattern::Tile)
        .price_pattern(ProductPricePattern::Sale)
        .classes();
    
    let interactive_classes = interactive_element(colors.clone())
        .hoverable()
        .standard_interaction()
        .classes();
    
    let focus_classes = focus_management(colors)
        .button()
        .classes();
    
    format!("{} {} {} {}", 
        card_classes, 
        product_classes, 
        interactive_classes, 
        focus_classes
    )
};
```

## Best Practices

1. **Compose patterns** rather than creating monolithic components
2. **Use semantic patterns** to convey meaning, not just styling
3. **Apply appropriate patterns** based on the element's purpose
4. **Maintain consistency** by reusing pattern combinations
5. **Test accessibility** using the built-in focus and screen reader patterns