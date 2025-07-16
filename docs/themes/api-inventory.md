# Jupiter Design System - Complete API Inventory

This document provides a complete inventory of all public APIs in the Jupiter Design System.

## Core Module (`src/core/`)

### color.rs
- **Enums**: `Color` (semantic color tokens)
- **Structs**: `ColorPalette` 
- **Traits**: `ColorProvider`
- **Functions**: None

### sizing.rs  
- **Enums**: `Size`, `Breakpoint`
- **Traits**: `SizeProvider`
- **Functions**: None

### spacing.rs
- **Enums**: `Spacing`
- **Traits**: `SpacingProvider`
- **Functions**: None

### typography.rs
- **Enums**: `Typography`, `FontWeight`, `FontFamily`
- **Traits**: `TypographyProvider`
- **Functions**: None

## Patterns Module (`src/patterns/`)

### actions.rs
- **Enums**: `ActionIntent`, `ActionHierarchy`, `ActionContext`
- **Structs**: `ActionSemantics<C>`
- **Functions**: `action_semantics<C>`

### button.rs
- **Structs**: `ButtonPattern<C>`, `ButtonSemanticInfo`
- **Functions**: 
  - `button_pattern<C>`
  - `primary_button<C>`
  - `secondary_button<C>`
  - `destructive_button<C>`
  - `hero_button<C>`
  - `navigation_button<C>`
  - `button_link<C>`

### card.rs
- **Enums**: `CardElevation`, `CardSpacing`, `CardSurface`, `CardInteraction`
- **Structs**: `CardPattern<C>`
- **Functions**: `card_pattern<C>`

### focus.rs
- **Enums**: `FocusBehavior`, `KeyboardPattern`, `ScreenReaderPattern`
- **Structs**: `FocusManagement<C>`
- **Functions**: `focus_management<C>`

### interactions.rs
- **Enums**: `InteractiveState`, `InteractionIntensity`
- **Structs**: `InteractiveElement<C>`
- **Functions**: `interactive_element<C>`

### layout.rs
- **Structs**: `CardSectionLayout<C>`, `LayoutBuilder<C>`
- **Functions**: None

### product.rs
- **Enums**: 
  - `ProductDisplayPattern`
  - `ProductInteractionState`
  - `ProductAvailabilityState`
  - `ProductImagePattern`
  - `ProductPricePattern`
  - `ProductVariantPattern`
- **Structs**: `ProductPattern<C>`
- **Functions**: `product_pattern<C>`

### selection.rs
- **Enums**: `SelectionState`, `SelectionBehavior`
- **Structs**: `SelectionPattern<C>`
- **Functions**: `selection_pattern<C>`

### states.rs
- **Enums**: `ComponentState`, `LoadingState`, `ValidationState`
- **Structs**: `StatePattern<C>`
- **Functions**: `state_pattern<C>`

### typography.rs
- **Enums**: 
  - `TypographyHierarchy`
  - `TypographySize`
  - `TypographyWeight`
  - `TypographyColor`
  - `TypographyAlignment`
  - `TypographyOverflow`
  - `TypographyElement`
- **Structs**: `TypographyPattern<T>`
- **Functions**:
  - `typography_pattern<T>`
  - `title_typography<T>`
  - `heading_typography<T>`
  - `body_typography<T>`
  - `caption_typography<T>`
  - `code_typography<T>`

## Builders Module (`src/builders/`)

### button.rs
- **Enums**: `ButtonVariant`, `ButtonState`
- **Structs**: `ButtonStyles<C>`, `ButtonClasses`
- **Functions**: 
  - `button_styles<C>`
  - `button_classes_from_strings`

### card.rs
- **Structs**: `CardStyles<C>`
- **Functions**: `card_styles<C>`

### interactive.rs
- **Structs**: 
  - `InteractiveBase<C>`
  - `HoverBuilder<C>`
  - `FocusBuilder<C>`
  - `ActiveBuilder<C>`
  - `DisabledBuilder<C>`
  - `InputBuilder<C>`
  - `ButtonBuilder<C>`
- **Functions**: None (builders created through other APIs)

### layout.rs
- **Structs**: `LayoutStyles<C>`
- **Functions**: `layout_styles<C>`

### product.rs
- **Structs**: `ProductBuilder<C>`
- **Functions**: `product_builder<C>`

### selection.rs
- **Structs**: `SelectionStyles<C>`, `SelectionClasses`
- **Functions**: 
  - `selection_styles<C>`
  - `selection_classes_from_strings`

### state.rs
- **Structs**: `StateStyles<C>`
- **Functions**: `state_styles<C>`

### text.rs
- **Structs**: `TextStyles<T>`, `TextClasses`
- **Functions**: 
  - `text_styles<T>`
  - `text_classes_from_strings`

## Themes Module (`src/themes/`)

### mod.rs
- **Traits**: `Theme`
- **Structs**: `VibeColors`, `VibeTheme`
- **Functions**: None

## Utils Module (`src/utils/`)

### mod.rs
- **Structs**: `DesignSystem`
- **Functions**: Various utility functions

## Prelude Exports

The prelude (`lib.rs`) re-exports commonly used items for convenience.