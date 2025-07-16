# Test-Driven Understanding Rule

## Overview
**Never trust test suites at face value. Always validate tests against implementation and treat test failures as learning opportunities, not just bugs to fix.**

## When It Applies
- Exploring unfamiliar codebases
- Documenting existing features
- Debugging test failures
- Understanding system behavior
- Validating assumptions about code

## Why It's Critical
Tests are often:
- **Incomplete**: Missing edge cases or entire features
- **Outdated**: Not updated when implementation changes
- **Buggy**: Containing their own logic errors
- **Misleading**: Testing idealized behavior, not reality

Trusting tests blindly leads to:
- False confidence in understanding
- Documentation that doesn't match reality
- Missing critical features
- Propagating test bugs into production

## Specific Actions to Take

### 1. Test-to-Implementation Validation
```bash
# For every test assertion, verify against source
# Test says: assert_eq!(variants.len(), 18)
# Verify: Count actual enum variants
rg "enum Color" -A 50 | grep -E "^\s+\w+," | wc -l

# Test uses: Color::VARIANTS
# Verify: Find actual constant definition
rg "const VARIANTS" --type rust -B 2 -A 20
```

### 2. Coverage Gap Analysis
```bash
# Find all enum variants
rg "enum FeatureName" -A 100 | grep -E "^\s+\w+,"

# Find which variants are tested
rg "FeatureName::" --type rust test/

# Identify gaps
# Missing variants = potential bugs
```

### 3. Test Failure Investigation
```rust
// Don't just fix the test - understand WHY it fails
#[test]
fn test_color_variants() {
    // FAILS: length mismatch
    assert_eq!(Color::VARIANTS.len(), enum_variant_count());
    
    // Investigation reveals:
    // - VARIANTS array missing Color::Accent
    // - Enum has 19 variants, array has 18
    // - This would break user code iterating variants
}
```

## Examples from Today's Work

### Example 1: The Missing Variant Bug
**Test Code**:
```rust
impl Color {
    const VARIANTS: [Color; 18] = [
        Color::Primary,
        Color::Secondary,
        // ... 16 more variants
        // Color::Accent missing!
    ];
}

#[test] 
fn test_all_variants() {
    assert_eq!(Color::VARIANTS.len(), 18); // Passes but wrong!
}
```

**Reality Check**:
```rust
enum Color {
    Primary, Secondary, /*...*/, Accent // 19 variants total
}
// Test missed that VARIANTS array was incomplete
```

**Impact**: Users iterating `Color::VARIANTS` would miss `Accent` entirely

### Example 2: Test-Driven Discovery
**Initial understanding**: "Color system seems complete based on tests"

**Test investigation revealed**:
```bash
# Searching for test gaps
rg "Color::" --type rust | grep -v test | sort -u

# Found:
- ButtonStyleBuilder using gradients (not tested)
- 233 ColorClasses mappings (minimal tests)
- Theme integration (no tests)
- CSS dependencies (undocumented in tests)
```

### Example 3: Hidden Behavior in Tests
**Test showed**:
```rust
#[test]
fn test_color_class() {
    assert_eq!(Color::Primary.class(), "text-primary");
}
```

**Deeper investigation found**:
```rust
// Reality: Complex prefix system
match self {
    Color::Primary => format!("{}-primary", prefix),
    Color::Custom(name) => format!("{}-{}", prefix, name),
    // Theme-specific overrides
    _ if theme.is_dark() => self.dark_variant().class(),
}
```

## Consequences of Not Following

### Immediate Issues
- Documentation misses critical features
- Users encounter "undocumented" behavior  
- Bug reports for "working as designed" features
- Wasted time debugging correct code

### Long-term Damage
- Test suite becomes less trusted
- Developers bypass tests entirely
- Real bugs hide among test issues
- Technical debt accumulates

### Today's Impact
- 4 validation passes required
- 2+ hours of rework
- Multiple critical features nearly missed
- User-facing bugs almost shipped

## Implementation Checklist

When using tests to understand code:

- [ ] Read test implementation, not just test names
- [ ] Verify every assertion against source code
- [ ] Count tested vs actual features/variants
- [ ] Look for commented-out or skipped tests
- [ ] Check test file dates vs implementation dates
- [ ] Run tests with verbose output
- [ ] Investigate every test failure's root cause
- [ ] Search for untested public APIs

## Test Investigation Patterns

### Pattern 1: Completeness Check
```bash
# List all public APIs
rg "pub fn|pub const|pub struct|pub enum" --type rust src/

# List all test functions
rg "fn test_|#\[test\]" --type rust

# Compare: What's not tested?
```

### Pattern 2: Assertion Validation
```bash
# Find all test assertions
rg "assert" --type rust test/ -B 2 -A 2

# For each assertion, verify the source
# assert_eq!(foo.method(), expected)
# Find: impl Foo { fn method() { ... } }
```

### Pattern 3: Test Evolution
```bash
# Check test history
git log -p -- test/feature_test.rs

# Look for:
- Removed tests (functionality might still exist)
- Changed assertions (behavior evolved)
- TODO/FIXME comments (known issues)
```

## Red Flags in Tests

1. **Magic numbers**: `assert_eq!(count, 18)` - Why 18?
2. **Commented tests**: Often hide real functionality
3. **Old dates**: Tests not updated with code
4. **Simple tests**: Complex features need complex tests
5. **No integration tests**: Unit tests miss interactions
6. **Test-only code**: Implementation differs from test setup

## Test-Driven Learning Process

1. **Run tests** - Note failures and successes
2. **Read test source** - Understand what's being tested
3. **Verify assertions** - Check against implementation
4. **Find gaps** - What's not tested?
5. **Investigate failures** - Why did it fail?
6. **Document reality** - Not test assumptions

## Remember

> "Tests are hypotheses about code behavior. Like all hypotheses, they must be validated."

**A passing test suite means the tests pass, not that the code is correct or complete.**

## Key Principle

**Use tests as a starting point for understanding, never as the final word.**