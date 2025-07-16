# Rust-Specific Knowledge Rules

## Rule 1: Public API Discovery Commands

**When to Apply**: Documenting Rust crates

**Essential Commands**:
```bash
# Count all public items
find src -name "*.rs" | xargs grep -E "pub (fn|struct|enum|trait|const|static)" | wc -l

# List all public functions
find src -name "*.rs" | xargs grep -E "pub fn" | grep -v "test"

# Find trait implementations
find src -name "*.rs" | xargs grep -E "impl.*for"

# Find convenience functions
find src -name "*.rs" | xargs grep -E "pub fn [a-z_]+\(" | head -20

# Find builder patterns
find src -name "*.rs" | xargs grep -E "pub struct.*Builder"

# Find pattern functions
find src -name "*.rs" | xargs grep -E "pub fn.*pattern"
```

**Why Important**:
- Rust's module system can hide APIs
- Re-exports make discovery complex
- Generic types need special attention

---

## Rule 2: Trait Documentation Pattern

**When to Apply**: Documenting Rust traits

**Pattern**:
```rust
/// Trait for providing semantic colors
pub trait ColorProvider {
    /// Get the color palette for this provider
    fn palette(&self) -> &ColorPalette;
    
    /// Resolve semantic color to CSS class
    fn resolve_color(&self, color: Color) -> &str { 
        // Default implementation
    }
}
```

**Documentation Should Include**:
- Purpose of trait
- Required vs provided methods
- Generic constraints
- Usage examples
- Implementation examples

**Why Important**:
- Traits are core to Rust design
- Default implementations are easily missed
- Generic constraints affect usage

---

## Rule 3: Builder Pattern Recognition

**When to Apply**: Documenting Rust APIs

**Common Patterns**:
```rust
// Method chaining builders
let button = button_styles(colors)
    .variant(Primary)
    .size(Large)
    .classes();

// Fluent configuration
let typography = typography_pattern(colors)
    .hierarchy(Title)
    .color(Primary)
    .alignment(Center)
    .classes();
```

**Documentation Focus**:
- Method chaining order
- Terminal methods (`.classes()`, `.build()`)
- Required vs optional configuration
- State transitions

---

## Rule 4: Generic Type Documentation

**When to Apply**: Documenting generic Rust functions

**Pattern**:
```rust
/// Creates button pattern with any color provider
pub fn button_pattern<C: ColorProvider + Clone>(
    color_provider: C
) -> ButtonPattern<C>
```

**Must Document**:
- Generic constraints (`ColorProvider + Clone`)
- Why constraints are needed
- Usage with different concrete types
- Lifetime parameters if present

**Example**:
```rust
// Works with any ColorProvider
let button1 = button_pattern(VibeColors::default());
let button2 = button_pattern(CustomTheme::new());
```

---

## Rule 5: Module Re-export Discovery

**When to Apply**: Understanding Rust crate structure

**Commands**:
```bash
# Find re-exports
grep -r "pub use" src/

# Check prelude modules
find src -name "prelude.rs" -o -name "mod.rs" | xargs grep "pub use"

# Find convenience re-exports
grep -r "pub use.*::" src/lib.rs
```

**Why Important**:
- Items can be accessible through multiple paths
- Prelude modules change import patterns
- Documentation should match actual usage

---

## Rule 6: Cargo.toml Feature Discovery

**When to Apply**: Documenting Rust crates

**Check For**:
```toml
[features]
default = ["theme-vibe"]
theme-vibe = []
theme-dark = []
serde = ["dep:serde"]
```

**Document**:
- Feature-gated APIs
- Default features
- Optional dependencies
- Feature combinations

---

## Rule 7: Error Handling Patterns

**When to Apply**: Documenting fallible operations

**Common Patterns**:
```rust
// Result-based APIs
pub fn from_hex(hex: &str) -> Result<Color, ColorError>

// Option-based APIs  
pub fn resolve_color(&self, color: Color) -> Option<&str>

// Panic-based APIs (document!)
pub fn resolve_color(&self, color: Color) -> &str // panics if invalid
```

**Document**:
- When functions can fail
- Error types and meanings
- Panic conditions
- Recovery strategies

---

## Rule 8: Macro Documentation

**When to Apply**: Documenting Rust macros

**Pattern**:
```rust
/// Creates theme with compile-time color validation
/// 
/// # Examples
/// ```rust
/// let theme = theme! {
///     primary: "blue-500",
///     secondary: "gray-600",
/// };
/// ```
macro_rules! theme { ... }
```

**Must Include**:
- Input syntax
- Expansion example
- Compile-time vs runtime behavior
- Hygiene considerations

---

## Rule 9: Testing Documentation

**When to Apply**: Documenting Rust code

**Include**:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_theme_creation() {
        let theme = MyTheme::new();
        assert_eq!(theme.name(), "My Theme");
    }
}
```

**Document**:
- How to run tests
- Test organization
- Integration test location
- Benchmark setup

---

## Rule 10: Performance Considerations

**When to Apply**: Documenting performance-sensitive APIs

**Document**:
- Allocation patterns
- Clone costs
- Compilation impact
- Runtime characteristics

**Example**:
```rust
/// This function clones the color provider for each call.
/// Consider caching the result for repeated use.
pub fn expensive_operation<C: ColorProvider + Clone>(colors: C) -> String
```