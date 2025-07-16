# Rust Design System Patterns Rule

## Context
Rust design systems have unique patterns that differ from JavaScript/CSS frameworks. Documentation must reflect Rust-specific idioms.

## The Rule
**When documenting Rust design systems, emphasize Rust's unique strengths and patterns:**

### Core Rust Design System Patterns

1. **Trait-Based Composition**
   ```rust
   // Document trait hierarchies
   pub trait ColorProvider: Clone + Send + Sync {
       fn palette(&self) -> ColorPalette;
   }
   
   // Show how traits enable composition
   pub trait Themeable {
       type ColorProvider: ColorProvider;
       fn with_theme(provider: Self::ColorProvider) -> Self;
   }
   ```

2. **Builder Pattern with Type Safety**
   ```rust
   // Emphasize compile-time guarantees
   impl ButtonBuilder<NoVariant> {
       pub fn primary(self) -> ButtonBuilder<WithVariant> {
           // Type system prevents missing required fields
       }
   }
   ```

3. **Zero-Cost Abstractions**
   ```rust
   // Document performance benefits
   #[inline(always)]
   pub fn button_styles(provider: impl ColorProvider) -> ButtonStyles {
       // Inlined at compile time, no runtime overhead
   }
   ```

### Rust-Specific Documentation Topics

#### Memory Safety in UI Components
```markdown
## Memory Safety Benefits

The Jupiter Design System leverages Rust's ownership model to prevent:
- Use-after-free bugs in event handlers
- Data races in concurrent rendering
- Memory leaks from circular references

Example: Theme providers use Arc<T> for safe sharing across threads.
```

#### Compile-Time CSS Validation
```markdown
## Type-Safe Styling

Unlike runtime CSS-in-JS solutions, Jupiter validates styles at compile time:
- Invalid color combinations won't compile
- Missing required properties are caught by the type system
- No runtime style calculation overhead
```

#### Lifetime Considerations
```rust
// Document when lifetimes matter
pub struct ThemedComponent<'a, T: ColorProvider> {
    theme: &'a T,
    // Explain: Lifetime ensures theme outlives component
}
```

### Common Rust Patterns to Document

1. **Derive Macros**
   ```rust
   #[derive(Clone, Copy, Debug, PartialEq)]
   pub enum Size {
       Small,
       Medium,
       Large,
   }
   // Document: Why these derives are important for design systems
   ```

2. **Const Generics for Performance**
   ```rust
   pub struct Grid<const COLS: usize> {
       // Document: Compile-time grid sizing for zero overhead
   }
   ```

3. **Feature Flags**
   ```toml
   [features]
   default = ["vibe-theme"]
   all-themes = ["vibe-theme", "minimal-theme", "dark-theme"]
   ```

## Example from Today

**JavaScript-Style Documentation (Wrong):**
```markdown
Create a button with: `new Button({ variant: 'primary' })`
```

**Rust-Style Documentation (Right):**
```markdown
Create a button using the type-safe builder pattern:

```rust
let button = button_styles(theme)
    .primary()     // Returns ButtonBuilder<WithVariant>
    .large()       // Compile-time size validation
    .classes();    // Generate final CSS classes

// This won't compile - type safety prevents errors:
// let button = button_styles(theme).classes(); // Error: missing variant
```
```

## Rust Design System Checklist

- [ ] Document trait hierarchies and composition
- [ ] Explain lifetime parameters where used
- [ ] Show zero-cost abstraction benefits
- [ ] Include memory safety advantages
- [ ] Document const generics usage
- [ ] Explain derive macro choices
- [ ] Show compile-time validation examples
- [ ] Include concurrent usage patterns
- [ ] Document feature flag combinations

## Key Rust Idioms for Design Systems

1. **Prefer Traits over Inheritance**
   - Document how traits enable flexible composition
   - Show trait bounds for generic components

2. **Use Enums for Variants**
   - Exhaustive matching prevents missing cases
   - Copy derives for cheap variant passing

3. **Builder Pattern for Complex Components**
   - Type states prevent invalid configurations
   - Method chaining for ergonomic API

4. **Const Functions for Compile-Time Computation**
   - Calculate styles at compile time where possible
   - Document performance benefits

## Key Principle
Rust design systems offer compile-time safety and zero-cost abstractions that JS systems cannot. Always highlight these unique benefits in documentation.