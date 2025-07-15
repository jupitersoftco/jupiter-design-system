# Changelog

All notable changes to the Jupiter Design System will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Interactive Builders System**: New fluent API for building interactive components with pseudo-class states
  - `interactive_input()` - Specialized builder for form inputs
  - `interactive_button()` - Specialized builder for buttons with variants
  - `interactive_element()` - Generic builder for any interactive element
  - Support for hover, focus, active, and disabled states
  - Type-safe chaining with order independence
  - Clean CSS output with grouped pseudo-classes

### Changed

- Updated `LlasiTheme` to use new interactive builders for:
  - `input_field()` - Now uses clean fluent API instead of format strings
  - `input_textarea_field()` - Clean pseudo-class handling
  - `social_icon_button()` - Type-safe interactive states

### Deprecated

- Manual string formatting for pseudo-classes (still works but not recommended)

### Examples

#### Before (messy string formatting):

```rust
format!("{} {} focus:{} hover:{}",
    theme.colors.bg_class(Color::Background),
    theme.colors.border_class(Color::Border),
    theme.colors.border_class(Color::Primary),
    theme.colors.border_class(Color::Foreground)
)
```

#### After (clean fluent API):

```rust
interactive_input(colors)
    .standard_style()
    .hover().border_primary().shadow_md()
    .focus().border_primary().ring_primary().outline_none()
    .disabled().opacity_50().cursor_not_allowed()
    .build()
```

## [0.1.0] - 2024-01-15

### Added

- Core design system foundation
- Color provider trait and palette system
- Button component with full variant support
- Typography, spacing, and sizing tokens
- VibeColors default theme
- Comprehensive test suite
- Documentation and examples

### Features

- **Type Safety**: Compile-time validation of design tokens
- **Theme Support**: Extensible color provider system
- **Component Builders**: Fluent APIs for consistent component styling
- **Framework Agnostic**: Pure CSS class generation for any frontend

### Components

- **Button**: Primary, Secondary, Success, Warning, Error, Ghost, Link variants
- **Sizes**: XSmall through XLarge with consistent scaling
- **States**: Default, Hover, Active, Disabled, Loading support

### Testing

- Unit tests for all core functionality
- Integration tests for component builders
- Theme compatibility validation
