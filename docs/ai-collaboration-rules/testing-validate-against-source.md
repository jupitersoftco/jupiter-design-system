# Testing Rule: Always Validate Tests Against Source Implementation

## Overview
Tests can contain bugs that make them unreliable sources of truth. Always cross-reference test assertions against the actual implementation.

## The Problem
Tests are often treated as the source of truth for documentation, but they can contain:
- Missing test cases
- Incorrect assertions
- Outdated values
- Copy-paste errors
- Incomplete coverage

## Today's Example: The Missing Enum Variant

### The Bug
```rust
// src/core/color.rs - The actual enum
pub enum Color {
    Primary,
    Secondary,
    Accent,      // This variant exists!
    Success,
    // ... 15 more variants (19 total)
}

// src/core/color_test.rs - The broken test
#[test]
fn test_color_enum_variants() {
    let colors = [
        Color::Primary,
        Color::Secondary,
        // ❌ Color::Accent is missing!
        Color::Success,
        // ... other variants
    ];
    
    assert_eq!(colors.len(), 18); // Wrong! Should be 19
}
```

### Impact
- Documentation claimed 18 color tokens
- Implementation has 19 color tokens
- Users would get compile errors using the missing variant

## Validation Process

### 1. Count Source Implementation
```bash
# Count enum variants
grep -A 50 "enum Color" src/core/color.rs | grep -E "^\s+\w+," | wc -l

# Or more precisely
awk '/enum Color/,/^}/' src/core/color.rs | grep -E "^\s+\w+," | wc -l
```

### 2. Count Test Coverage
```bash
# Count what test actually tests
grep "Color::" src/core/color_test.rs | grep -v "assert" | sort -u | wc -l
```

### 3. Find Discrepancies
```rust
// Create a validation test
#[test]
fn validate_all_variants_tested() {
    use strum::IntoEnumIterator; // If using strum
    
    let all_variants = Color::iter().count();
    let tested_variants = TEST_COLORS.len();
    
    assert_eq!(all_variants, tested_variants, 
        "Test array has {} variants but enum has {}", 
        tested_variants, all_variants
    );
}
```

## Common Test Bugs to Check

### 1. Hardcoded Array Sizes
```rust
// Bad: Hardcoded size
assert_eq!(items.len(), 18);

// Good: Dynamic check
assert_eq!(items.len(), Color::variant_count());
```

### 2. Missing Edge Cases
```rust
// Test might only check common cases
test_color(Color::Primary);
test_color(Color::Secondary);
// But misses: Color::InteractiveDisabled, etc.
```

### 3. Outdated Values
```rust
// Test written when color was blue-500
assert_eq!(theme.primary(), "blue-500");
// But implementation changed to jupiter-blue-500
```

## Validation Commands

### For Enums
```bash
# Get all enum variants from source
ENUM_NAME="Color"
echo "=== Enum Variants in Source ==="
grep -A 100 "enum $ENUM_NAME" src/**/*.rs | grep -E "^\s+\w+," | sed 's/,$//' | sort

echo "=== Enum Variants in Tests ==="
grep "$ENUM_NAME::" src/**/*test.rs | grep -o "$ENUM_NAME::\w*" | sort -u

echo "=== Comparing ==="
diff <(grep -A 100 "enum $ENUM_NAME" src/**/*.rs | grep -E "^\s+\w+," | sed 's/,$//' | sort) \
     <(grep "$ENUM_NAME::" src/**/*test.rs | grep -o "$ENUM_NAME::\w*" | sed "s/$ENUM_NAME:://" | sort -u)
```

### For Values
```bash
# Compare test assertions with actual values
echo "=== Test Assertions ==="
grep "assert_eq!" src/**/*test.rs | grep -o '"[^"]*"' | sort -u

echo "=== Implementation Values ==="
grep -r "to_string()" src --include="*.rs" | grep -o '"[^"]*"' | sort -u

# Find mismatches
comm -3 <(grep "assert_eq!" src/**/*test.rs | grep -o '"[^"]*"' | sort -u) \
        <(grep -r "to_string()" src --include="*.rs" | grep -o '"[^"]*"' | sort -u)
```

## Best Practices

### 1. Read Implementation First
```rust
// Don't assume test is correct
// First: Read src/core/feature.rs
// Then: Read src/core/feature_test.rs
// Finally: Compare
```

### 2. Validate Test Completeness
```bash
# Check test coverage
cargo tarpaulin --out Html
# Look for untested code paths
```

### 3. Create Meta-Tests
```rust
#[test]
fn test_all_variants_have_test_cases() {
    let source_variants = get_all_enum_variants();
    let tested_variants = get_tested_variants();
    
    for variant in source_variants {
        assert!(tested_variants.contains(&variant), 
            "Variant {:?} is not tested", variant);
    }
}
```

## Red Flags in Tests

Watch for:
- `assert_eq!(array.len(), HARDCODED_NUMBER)`
- Test arrays built manually instead of programmatically
- No test for "all variants" or "all methods"
- Tests that haven't been updated with implementation
- Copy-pasted test blocks with minimal changes

## Today's Lesson
A test that claims to verify "all color variants exist" but actually only tests 18 of 19 variants is worse than no test at all - it provides false confidence.

**Always validate tests against implementation, never trust them blindly.**