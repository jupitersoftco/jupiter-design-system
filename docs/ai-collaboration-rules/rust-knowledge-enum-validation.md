# Rust Knowledge: Enum and Trait Validation Patterns

## Overview
When working with Rust enums and traits in design systems, specific validation patterns can prevent common documentation errors.

## Enum Validation Patterns

### 1. Complete Enum Coverage Testing
```rust
// Bad: Manual array that can miss variants
#[test]
fn test_all_colors() {
    let colors = [Color::Primary, Color::Secondary]; // Manually maintained
    assert_eq!(colors.len(), 2); // Hardcoded count
}

// Good: Use a derive macro for completeness
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter)]
pub enum Color {
    Primary,
    Secondary,
    Accent,
}

#[test]
fn test_all_colors_complete() {
    use strum::IntoEnumIterator;
    
    let all_colors: Vec<_> = Color::iter().collect();
    assert_eq!(all_colors.len(), 3); // Still hardcoded but...
    
    // Better: Test each variant exists
    for color in Color::iter() {
        // This will fail to compile if enum changes
        match color {
            Color::Primary => {},
            Color::Secondary => {},
            Color::Accent => {},
        }
    }
}
```

### 2. Enum to String Mapping Validation
```rust
// Pattern found in Jupiter Design System
impl ColorProvider for VibeColors {
    fn resolve_color(&self, color: Color) -> &str {
        match color {
            Color::Primary => &self.palette.primary,
            Color::Secondary => &self.palette.secondary,
            // Must handle ALL variants or won't compile
        }
    }
}

// Test pattern for mappings
#[test]
fn test_all_colors_have_values() {
    let provider = VibeColors::default();
    
    // If using strum
    for color in Color::iter() {
        let value = provider.resolve_color(color);
        assert!(!value.is_empty(), "Color {:?} has empty value", color);
    }
}
```

### 3. Serialization Completeness
```rust
#[derive(Serialize, Deserialize)]
pub enum Color {
    Primary,
    Secondary,
    Accent,
}

#[test]
fn test_color_serialization_roundtrip() {
    // Test all variants serialize correctly
    for color in Color::iter() {
        let serialized = serde_json::to_string(&color).unwrap();
        let deserialized: Color = serde_json::from_str(&serialized).unwrap();
        assert_eq!(color, deserialized);
    }
}
```

## Trait Implementation Patterns

### 1. Default Trait Validation
```rust
pub trait ColorProvider {
    fn palette(&self) -> &ColorPalette;
    
    // Default implementations should be tested
    fn text_class(&self, color: Color) -> String {
        format!("text-{}", self.resolve_color(color))
    }
}

#[test]
fn test_default_trait_methods() {
    let provider = TestProvider::default();
    
    // Validate format assumptions
    let text_class = provider.text_class(Color::Primary);
    assert!(text_class.starts_with("text-"));
    assert!(!text_class.contains(' ')); // No spaces
    assert!(!text_class.is_empty());
}
```

### 2. Builder Pattern Validation
```rust
// Common in design systems
pub struct ButtonStyles<C: ColorProvider> {
    variant: ButtonVariant,
    size: ButtonSize,
    state: ButtonState,
    color_provider: C,
}

// Validate all combinations
#[test]
fn test_all_button_combinations() {
    let provider = VibeColors::default();
    
    for variant in ButtonVariant::iter() {
        for size in ButtonSize::iter() {
            for state in ButtonState::iter() {
                let button = ButtonStyles::new(provider.clone())
                    .variant(variant)
                    .size(size)
                    .state(state)
                    .build();
                
                // Validate output
                assert!(!button.is_empty());
                assert!(button.contains("button"));
            }
        }
    }
}
```

## Rust-Specific Gotchas

### 1. Non-Exhaustive Enums
```rust
#[non_exhaustive]
pub enum Color {
    Primary,
    Secondary,
    Accent,
}

// This means users MUST handle unknown variants
match color {
    Color::Primary => "blue",
    Color::Secondary => "green",  
    Color::Accent => "orange",
    _ => "gray", // Required for non_exhaustive!
}
```

### 2. Associated Constants Pattern
```rust
impl Color {
    // Document these separately!
    pub const COUNT: usize = 19;
    pub const DEFAULT: Self = Self::Primary;
}

#[test]
fn test_color_constants() {
    // Validate COUNT matches reality
    assert_eq!(Color::COUNT, Color::iter().count());
}
```

### 3. Type State Pattern
```rust
// Design systems often use phantom types
pub struct Button<State> {
    classes: String,
    _state: PhantomData<State>,
}

pub struct NormalState;
pub struct HoverState;
pub struct ActiveState;

impl Button<NormalState> {
    pub fn hover(self) -> Button<HoverState> {
        // State transition
    }
}
```

## Validation Commands for Rust

### Find All Enums
```bash
# Find enum definitions
grep -r "^pub enum\|^enum" . --include="*.rs" | grep -v "test"

# Count variants
for enum in $(grep -r "^pub enum" . --include="*.rs" | awk '{print $3}'); do
    echo "Enum: $enum"
    grep -A 50 "enum $enum" . --include="*.rs" | grep -E "^\s+\w+," | wc -l
done
```

### Trait Implementation Coverage
```bash
# Find all trait implementations
grep -r "impl.*for" . --include="*.rs" | grep -v "test"

# Find default trait methods
grep -r "fn.*default" . --include="*.rs" -A 5
```

### Match Expression Completeness
```bash
# Find all match expressions for an enum
ENUM="Color"
grep -r "match.*$ENUM" . --include="*.rs" -A 20

# Check for wildcard patterns (might miss variants)
grep -r "match.*$ENUM" . --include="*.rs" -A 20 | grep "=>"
```

## Best Practices for Rust Design Systems

1. **Use `strum` for enum iteration**
   ```toml
   [dependencies]
   strum = { version = "0.24", features = ["derive"] }
   ```

2. **Implement Display for all public enums**
   ```rust
   impl fmt::Display for Color {
       fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
           write!(f, "{}", self.as_str())
       }
   }
   ```

3. **Test macro for exhaustiveness**
   ```rust
   macro_rules! assert_all_variants {
       ($enum:ty, [$($variant:pat),+]) => {
           fn _assert_all_variants(e: $enum) {
               match e {
                   $($variant => {}),+
               }
           }
       };
   }
   
   assert_all_variants!(Color, [
       Color::Primary,
       Color::Secondary,
       Color::Accent
   ]);
   ```

## Key Lesson
Rust's type system can help ensure documentation completeness, but only if you use patterns that leverage exhaustiveness checking. Manual arrays and hardcoded counts bypass these safety features.