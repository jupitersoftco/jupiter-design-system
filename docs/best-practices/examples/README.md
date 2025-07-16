# Jupiter Design System Examples

This directory contains comprehensive examples demonstrating best practices for using the Jupiter Design System across different scenarios and frameworks.

## 📁 Directory Structure

### Core Examples
- [**Basic Usage**](./basic-usage.md) - Getting started with fundamental components
- [**Complete Components**](./complete-components.md) - Full-featured component implementations
- [**Theme Customization**](./theme-customization.md) - Custom theme creation and usage
- [**Interactive Patterns**](./interactive-patterns.md) - Advanced interactive component patterns

### Framework Integration
- [**Dioxus Integration**](./dioxus-integration.md) - Using with Dioxus web framework
- [**Yew Integration**](./yew-integration.md) - Integration with Yew framework
- [**Leptos Integration**](./leptos-integration.md) - Working with Leptos framework

### Application Patterns
- [**E-commerce Application**](./ecommerce-app.md) - Complete e-commerce interface
- [**Dashboard Application**](./dashboard-app.md) - Admin dashboard patterns
- [**Form Applications**](./form-applications.md) - Complex form implementations
- [**Accessibility Showcase**](./accessibility-showcase.md) - Comprehensive accessibility examples

### Advanced Techniques
- [**Performance Optimization**](./performance-optimization.md) - Real-world performance techniques
- [**Custom Components**](./custom-components.md) - Building custom design system extensions
- [**Testing Strategies**](./testing-strategies.md) - Component and theme testing examples

## 🚀 Quick Start Examples

### Simple Button
```rust
use jupiter_design_system::prelude::*;

let colors = VibeColors::new();
let button_classes = button_styles(colors).primary().classes();
// Result: "inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-md bg-jupiter-blue-500 text-white hover:bg-jupiter-blue-600 focus:outline-none focus:ring-2 focus:ring-jupiter-blue-500 focus:ring-offset-2"
```

### Interactive Card
```rust
let card_classes = interactive_element(colors)
    .base("p-6 rounded-lg bg-white border border-gray-200 cursor-pointer")
    .hover()
        .shadow_lg()
        .border_color(Color::Interactive)
    .focus()
        .ring_color(Color::Interactive)
        .ring_offset_2()
    .build();
```

### Form Field
```rust
let input_classes = interactive_input(colors)
    .base_style()
    .hover().border_primary()
    .focus().border_primary().ring_primary().outline_none()
    .disabled().opacity_50().cursor_not_allowed()
    .build();
```

## 📖 Usage Guidelines

### How to Use These Examples

1. **Start with Basic Usage** if you're new to the design system
2. **Reference Complete Components** for production-ready implementations
3. **Check Framework Integration** for your specific frontend framework
4. **Study Application Patterns** for real-world usage scenarios
5. **Apply Advanced Techniques** for optimization and customization

### Example Format

Each example includes:
- **Overview**: What the example demonstrates
- **Code**: Complete, runnable code examples
- **Explanation**: Step-by-step breakdown
- **Best Practices**: Key principles illustrated
- **Common Pitfalls**: What to avoid
- **Variations**: Alternative approaches

### Running Examples

Most examples are designed to be:
- **Copy-pasteable** into your project
- **Framework-agnostic** where possible
- **Production-ready** with proper error handling
- **Accessible** following WCAG guidelines
- **Performant** using optimization techniques

## 🎯 Learning Path

### Beginner (Start Here)
1. [Basic Usage](./basic-usage.md)
2. [Complete Components](./complete-components.md)
3. [Framework Integration](./dioxus-integration.md) (choose your framework)

### Intermediate
1. [Theme Customization](./theme-customization.md)
2. [Interactive Patterns](./interactive-patterns.md)
3. [Form Applications](./form-applications.md)

### Advanced
1. [Performance Optimization](./performance-optimization.md)
2. [Custom Components](./custom-components.md)
3. [Accessibility Showcase](./accessibility-showcase.md)

### Specialized Applications
1. [E-commerce Application](./ecommerce-app.md)
2. [Dashboard Application](./dashboard-app.md)
3. [Testing Strategies](./testing-strategies.md)

## 🛠️ Development Tools

### Example Testing
```bash
# Run example tests
cargo test --examples

# Run specific example
cargo run --example basic_usage

# Build all examples
cargo build --examples
```

### Validation Tools
```bash
# Check accessibility
cargo run --example accessibility_validation

# Performance benchmarks
cargo bench --examples

# CSS output analysis
cargo run --example css_analysis
```

## 🔗 Related Resources

- [Jupiter Design System Documentation](../../README.md)
- [Best Practices Overview](../README.md)
- [API Reference](../../IMPLEMENTATION.md)
- [Contributing Guidelines](../../CONTRIBUTING.md)

---

**Need help?** Check the [troubleshooting guide](../troubleshooting.md) or [create an issue](https://github.com/jupiter-design-system/issues).