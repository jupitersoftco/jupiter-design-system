# Rust Knowledge: Design System Implementation Patterns

## Overview
Rust design systems have unique patterns and pitfalls. Understanding these patterns helps identify missing pieces and potential issues quickly.

## Common Rust Design System Patterns

### 1. Trait-Based Theming
```rust
// The pattern
pub trait ColorProvider {
    fn resolve_color(&self, color: Color) -> &str;
    fn text_class(&self, color: Color) -> String {
        format!("text-{}", self.resolve_color(color))
    }
}

// What to check
// 1. Are trait methods actually usable?
// 2. Do generated strings work in target environment?
// 3. Is there a CSS pipeline for the strings?
```

### 2. Builder Pattern for Components
```rust
// Common pattern
pub struct ButtonStyles<C: ColorProvider> {
    variant: ButtonVariant,
    size: ButtonSize,
    color_provider: C,
}

impl<C: ColorProvider> ButtonStyles<C> {
    pub fn new(provider: C) -> Self { ... }
    pub fn variant(mut self, v: ButtonVariant) -> Self { ... }
    pub fn build(self) -> String { ... }
}

// What to verify
// 1. Is builder documented?
// 2. Are all builder methods tested?
// 3. Does output actually render?
```

### 3. CSS Class Generation
```rust
// The anti-pattern
fn generate_classes(&self) -> String {
    format!("bg-{} text-{} border-{}", 
        self.bg_color,    // "jupiter-blue-500"
        self.text_color,  // "white"
        self.border_color // "jupiter-blue-600"
    )
}

// Problems:
// 1. Assumes CSS framework (Tailwind)
// 2. Assumes configuration exists
// 3. No validation of output
// 4. Mixes custom and standard classes
```

## Rust-Specific Validation

### 1. Check for CSS Dependencies
```bash
# Rust design systems often hide CSS dependencies
grep -r "format!" src --include="*.rs" | grep -E "text-|bg-|border-" | head -20

# If found, check for CSS pipeline
ls tailwind.config.js package.json || echo "❌ Missing CSS pipeline"
```

### 2. Verify Builder Patterns
```rust
// Look for builder patterns
grep -r "impl.*Builder\|Styles" src --include="*.rs"

// Check if documented
grep -r "Builder\|builder" docs --include="*.md" || echo "❌ Builders undocumented"

// Verify in examples
grep -r "::new(" examples --include="*.rs" | grep -v "Vec::new"
```

### 3. Find Hidden Traits
```rust
// Design systems love traits
grep -r "pub trait" src --include="*.rs" | while read trait; do
    trait_name=$(echo $trait | awk '{print $3}')
    echo "Trait: $trait_name"
    echo "Implementations:"
    grep -r "impl.*$trait_name.*for" src --include="*.rs" | wc -l
    echo "Usage in examples:"
    grep -r "$trait_name" examples --include="*.rs" | wc -l
    echo
done
```

## Common Pitfalls

### 1. String-Based CSS Without Pipeline
```rust
// Code generates strings
pub fn button_class(&self) -> String {
    "px-4 py-2 bg-jupiter-blue-500 text-white rounded".to_string()
}

// But no:
// - tailwind.config.js
// - postcss.config.js  
// - build process
// = Useless strings
```

### 2. Phantom Type Complexity
```rust
// Advanced but undocumented
pub struct Component<T, State = DefaultState> {
    _phantom: PhantomData<(T, State)>,
}

// Check: Are phantom types documented?
// Check: Do examples show usage?
```

### 3. Macro Magic
```rust
// Hidden in macros
component_styles! {
    Button {
        primary: "bg-blue-500",
        secondary: "bg-gray-500",
    }
}

// Check: Where is macro defined?
// Check: Is macro documented?
// Check: Can users extend it?
```

## Validation Checklist for Rust Design Systems

### Build Configuration
- [ ] Check for `build.rs` - might generate CSS
- [ ] Look for `OUT_DIR` usage - generated files
- [ ] Check features in `Cargo.toml` - conditional compilation
- [ ] Verify `[dev-dependencies]` - missing from release?

### Pattern Detection
```bash
# Find all public builders
grep -r "pub struct.*Builder\|pub struct.*Styles" src --include="*.rs"

# Find CSS generation
grep -r "format!.*class\|format!.*style" src --include="*.rs"

# Find trait bounds on generics
grep -r "where.*:" src --include="*.rs" | grep Provider
```

### Integration Points
```bash
# How does Rust integrate with web?
grep -r "wasm\|web_sys\|dioxus\|yew\|leptos" src --include="*.rs"

# Check examples for web usage
find examples -name "*.html" -o -name "*.css" -o -name "index.html"
```

## Today's Rust-Specific Issues

### Issue 1: Trait Methods Generate Non-Functional Output
```rust
fn text_class(&self, color: Color) -> String {
    format!("text-{}", self.resolve_color(color))
    // Returns: "text-jupiter-blue-500"
    // Problem: No CSS definition exists
}
```

### Issue 2: Builder Pattern Hidden in Examples
```rust
// Not discovered until late:
ButtonStyles::new(provider)
    .variant(ButtonVariant::Primary)
    .size(ButtonSize::Large)
    .classes()
```

### Issue 3: No Web Integration Examples
```rust
// Examples show:
println!("{}", button_classes);

// Should show:
dioxus::rsx! {
    button { class: "{button_classes}", "Click me" }
}
```

## Best Practices for Rust Design Systems

### 1. Validate Output Early
```rust
#[test]
fn test_generated_classes_exist() {
    let css_file = include_str!("../dist/output.css");
    let generated_class = theme.text_class(Color::Primary);
    assert!(css_file.contains(&generated_class), 
        "Generated class {} not in CSS", generated_class);
}
```

### 2. Document the Full Pipeline
```rust
//! # CSS Pipeline Required
//! 
//! This crate generates Tailwind CSS classes that require:
//! 1. Node.js and npm
//! 2. Tailwind CSS configuration
//! 3. Build process: `npm run build-css`
//! 
//! See `examples/complete-setup/` for full integration.
```

### 3. Provide Integration Examples
```rust
// examples/with_dioxus/
// examples/with_yew/
// examples/vanilla_web/
// Each with complete setup including CSS pipeline
```

## Detection Commands

```bash
# Quick Rust design system audit
echo "=== Checking for CSS generation ==="
grep -r "format!" src --include="*.rs" | grep -E "class|style" | wc -l

echo "=== Checking for builders ==="
grep -r "Builder\|Styles" src --include="*.rs" | grep "pub struct" | wc -l

echo "=== Checking for web integration ==="
find examples -name "*.html" | wc -l

echo "=== Checking for CSS pipeline ==="
ls package.json tailwind.config.js 2>/dev/null || echo "❌ No CSS pipeline"

echo "=== Checking for complete examples ==="
find examples -name "Cargo.toml" | while read cargo; do
    dir=$(dirname "$cargo")
    echo -n "$dir: "
    ls "$dir"/*.html "$dir"/package.json 2>/dev/null || echo "❌ Incomplete"
done
```

## Key Lesson

**Rust design systems that generate CSS have implicit JavaScript/CSS dependencies.**

The type safety and elegance of Rust can hide the messy reality that web styling requires a complete CSS pipeline. Always verify the full stack works, not just the Rust parts.