# Critical Rule: Assume Tests Are Incomplete Until Proven Otherwise

## Overview
Test suites often have gaps, bugs, or outdated assertions. Never trust test completeness without verification.

## The Trust But Verify Approach

### 1. Test Coverage != Correctness
```rust
// Test exists and passes
#[test]
fn test_all_colors() {
    let colors = [...]; // 18 items
    assert_eq!(colors.len(), 18); // ✅ Passes!
}

// But implementation has 19 variants
pub enum Color {
    Primary,
    Secondary, 
    Accent,  // This one is missing from test!
    // ... 16 more
}
```

### 2. Common Test Incompleteness Patterns

#### Missing Variants
```rust
// Test missing enum variants
match color {
    Color::Primary => test_primary(),
    Color::Secondary => test_secondary(),
    // Forgot Color::Accent, Color::InteractiveDisabled, etc.
}
```

#### Missing Edge Cases
```rust
// Tests common cases
test_button_variant(ButtonVariant::Primary);
test_button_variant(ButtonVariant::Secondary);

// But misses
// - ButtonVariant::Ghost
// - ButtonVariant::Link  
// - Combinations with disabled state
```

#### Outdated Assertions
```rust
// Test written for old implementation
assert_eq!(color_value, "blue-500");

// But current implementation returns  
"jupiter-blue-500" // Custom prefix added later
```

## Verification Protocol

### Step 1: Count Everything
```bash
# Count enum variants in source
ENUM="Color"
echo "Source variants:"
grep -A 100 "enum $ENUM" src/**/*.rs | grep -E "^\s+\w+," | wc -l

echo "Test coverage:"
grep "$ENUM::" src/**/*test.rs | grep -o "$ENUM::\w*" | sort -u | wc -l

echo "Difference:"
# If these don't match, tests are incomplete
```

### Step 2: Extract Test Arrays
```rust
// Find test arrays
grep -B 2 -A 20 "\[.*Color::" src/**/*test.rs

// Look for:
// - Hardcoded array sizes
// - Manual construction
// - Missing items
```

### Step 3: Cross-Reference Implementations
```bash
# Get all method implementations
grep -r "fn.*Color" src --include="*.rs" | grep -v test

# Check if all methods have tests
for method in resolve_color text_class bg_class border_class; do
    echo "Method: $method"
    echo "Implementation calls: $(grep -r "$method" src --include="*.rs" | grep -v test | wc -l)"
    echo "Test calls: $(grep -r "$method" src --include="*test.rs" | wc -l)"
done
```

## Red Flags in Test Suites

### 1. Hardcoded Counts
```rust
// 🚩 Red flag
assert_eq!(items.len(), 18);

// ✅ Better
assert_eq!(items.len(), Color::VARIANT_COUNT);
// or
assert_eq!(items.len(), expected_items.len());
```

### 2. Manual Test Arrays
```rust
// 🚩 Red flag - manually maintained
let test_colors = [
    Color::Primary,
    Color::Secondary,
    // Easy to forget to update
];

// ✅ Better - generated
let test_colors: Vec<_> = Color::iter().collect();
```

### 3. No "All Variants" Test
```rust
// 🚩 Missing comprehensive test
// Individual tests exist but no test that validates ALL variants

// ✅ Should have
#[test]
fn test_all_enum_variants_have_values() {
    for variant in Color::iter() {
        let value = resolve_color(variant);
        assert!(!value.is_empty());
    }
}
```

### 4. Copy-Paste Test Blocks
```rust
// 🚩 Red flag - copy-pasted tests
#[test]
fn test_primary() {
    assert_eq!(resolve(Color::Primary), "jupiter-blue-500");
}

#[test]
fn test_secondary() {
    assert_eq!(resolve(Color::Secondary), "jupiter-green-500");
}
// Missing tests for other variants
```

## Validation Commands

### Quick Completeness Check
```bash
#!/bin/bash
# check-test-completeness.sh

ENUM=$1
echo "Checking completeness for enum: $ENUM"

# Source variants
SOURCE_COUNT=$(grep -A 100 "enum $ENUM" src/**/*.rs | grep -E "^\s+\w+," | wc -l)
echo "Source variants: $SOURCE_COUNT"

# Test coverage
TEST_COUNT=$(grep "$ENUM::" src/**/*test.rs | grep -o "$ENUM::\w*" | sort -u | wc -l)
echo "Tested variants: $TEST_COUNT"

# Missing coverage
if [ $SOURCE_COUNT -ne $TEST_COUNT ]; then
    echo "⚠️  WARNING: Missing test coverage for $((SOURCE_COUNT - TEST_COUNT)) variants!"
    
    # Show what's missing
    echo "Missing variants:"
    comm -23 <(grep -A 100 "enum $ENUM" src/**/*.rs | grep -E "^\s+\w+," | sed 's/,$//' | sed 's/^\s*//' | sort) \
             <(grep "$ENUM::" src/**/*test.rs | grep -o "$ENUM::\w*" | sed "s/$ENUM:://" | sort -u)
fi
```

## The Verification Mindset

### Before Documentation
1. **Assume tests are incomplete**
2. **Verify with source code**
3. **Count everything**
4. **Look for test array construction**

### During Documentation  
1. **Document what exists, not what tests claim**
2. **Note discrepancies found**
3. **Include verification steps**
4. **Warn about potential gaps**

### Example Documentation Note
```markdown
> ⚠️ **Note**: The test suite contains a bug where only 18 of 19 Color variants 
> are tested. The `Color::Accent` variant is missing from the test array in 
> `color_test.rs`. Always verify enum completeness against the source implementation.
```

## Today's Lesson

A test called `test_color_enum_variants()` that supposedly validates "all color variants exist" was missing one variant. This led to:
- Documentation claiming 18 tokens (wrong)
- Missing `Color::Accent` from examples
- Incomplete implementation guidance

**The test suite actively misled the documentation effort.**

## Key Principle

**Tests are documentation of what the developers THINK the code does, not what it ACTUALLY does.**

Always verify against source implementation.