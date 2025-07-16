# Jupiter Design System - Theme System Documentation

This documentation covers the trait-based theme system in Jupiter Design System and best practices for implementing concrete trait implementations.

## Documentation Structure

### Core Documentation
- **[Architecture Overview](./architecture.md)** - Understanding the theme system design
- **[Core Traits](./core-traits.md)** - Essential traits and their purposes
- **[Implementation Guide](./implementation-guide.md)** - How to implement your own traits
- **[Best Practices](./best-practices.md)** - Design patterns and conventions

### Pattern & Builder Systems
- **[Pattern Systems](./pattern-systems.md)** - Complete documentation of all pattern abstractions
- **[Builder System](./builder-system.md)** - CSS class generation builders
- **[API Reference](./api-reference.md)** - Complete function and type reference
- **[API Inventory](./api-inventory.md)** - Full inventory of public APIs

### Implementation & Integration
- **[Examples](./examples.md)** - Complete implementation examples
- **[Integration Examples](./integration-examples.md)** - Framework-specific integration
- **[Testing Guide](./testing-guide.md)** - Comprehensive testing strategies
- **[Migration Guide](./migration.md)** - Real-world migration scenarios

## Quick Start

```rust
use jupiter_design_system::prelude::*;
use jupiter_design_system::themes::VibeColors;

// Create a custom color provider
let colors = VibeColors::with_overrides(|palette| {
    palette.primary = "blue-600".to_string();
    palette.secondary = "green-500".to_string();
});

// Use with patterns
let button = primary_button(colors)
    .hero_prominence()
    .classes();
```

For detailed implementation guidance, see the individual documentation files.