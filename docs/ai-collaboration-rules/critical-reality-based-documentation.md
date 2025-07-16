# Reality-Based Documentation Rule

## Overview
**Documentation must reflect actual implementation, not idealized design.** Never trust tests, existing docs, or assumptions—always validate against source code.

## When It Applies
- Writing any technical documentation
- Creating API references or guides
- Documenting existing features
- Updating outdated documentation
- Explaining system behavior

## Why It's Critical
Documentation based on assumptions or incomplete information:
- **Misleads users**: They implement against non-existent APIs
- **Creates support burden**: Users report "bugs" that are documentation errors
- **Damages credibility**: One wrong example undermines entire docs
- **Compounds confusion**: Future developers build on false foundations

## Specific Actions to Take

### 1. Source Code Validation
```bash
# Never document without seeing the actual implementation
rg "struct FeatureName" --type rust -A 20
rg "impl FeatureName" --type rust -A 50

# Verify every method exists
rg "fn method_name" --type rust

# Check actual return types
rg "-> ResultType" --type rust
```

### 2. Test Reality Check
```rust
// WRONG: Trusting test assertions
#[test]
fn test_color_variants() {
    assert_eq!(Color::VARIANTS.len(), 18); // Test says 18
}

// RIGHT: Count actual variants
enum Color {
    Primary, Secondary, /*...*/ Accent // Actually 19!
}
```

### 3. Example Verification
```bash
# Run every example before documenting
cargo run --example feature_demo

# Test code snippets in isolation
echo "YOUR_CODE_SNIPPET" | cargo run --

# Verify output matches documentation
cargo test --doc
```

## Examples from Today's Work

### Example 1: Test Suite Lies
**What the test claimed**:
```rust
const VARIANTS: [Color; 18] = [/* ... */];
assert_eq!(Color::VARIANTS.len(), 18);
```

**What actually existed**:
```rust
enum Color {
    // ... 19 variants including Color::Accent
}
// Test was missing Accent variant entirely
```

**Impact**: Documentation would have missed a critical variant users need

### Example 2: Hidden API Complexity
**Initial documentation**:
```rust
// Simple color usage
let color = Color::Primary;
let class = color.to_class();
```

**Reality discovered**:
```rust
// Actual API includes:
- ButtonStyleBuilder with gradients
- ColorClasses with 233 string mappings  
- Custom prefix systems
- Theme-specific overrides
- Tailwind configuration requirements
```

### Example 3: Dependency Requirements
**Documented**: "Use text-primary class"
**Reality**: Requires jupiter-specific Tailwind configuration that wasn't documented

## Consequences of Not Following

### User Impact
```rust
// User tries documented API
let btn = Button::new(Color::Primary);
// FAILS: Missing Tailwind config

// User tries test example
Color::VARIANTS.iter() // PANICS: Accent not in array
```

### Maintenance Burden
- Bug reports for "missing features" that exist
- PRs to "add" functionality that's already there
- Confusion when examples don't compile
- Time spent debugging documentation vs code mismatches

### Trust Erosion
- Users learn to distrust documentation
- They read source code instead of docs
- Documentation becomes actively harmful

## Implementation Checklist

For every piece of documentation:

- [ ] Located actual implementation in source
- [ ] Verified every method/property exists
- [ ] Tested code examples compile and run
- [ ] Checked for builder patterns or factories
- [ ] Validated against real usage in examples/
- [ ] Confirmed all dependencies documented
- [ ] Tested with fresh environment
- [ ] Compared test assertions to reality

## Validation Patterns

### Pattern 1: Enum Validation
```bash
# Count actual variants
rg "enum ColorName" -A 50 | grep -E "^\s+\w+,"

# Find ALL constant arrays
rg "const.*\[.*Color.*\]" --type rust

# Check derive macros for generation
rg "#\[derive.*" -B 2 | grep enum
```

### Pattern 2: API Surface Discovery
```bash
# All public methods
rg "pub fn" --type rust | grep FeatureName

# Trait implementations
rg "impl.*for.*FeatureName" --type rust

# Associated functions
rg "impl FeatureName" -A 100 | grep "pub"
```

### Pattern 3: Usage Validation
```bash
# How it's actually used
find examples -name "*.rs" -exec grep -l "FeatureName" {} \;

# Common patterns
rg "FeatureName::" --type rust | sort | uniq -c | sort -nr
```

## Red Flags

1. **"The test says..."** - Tests can be wrong
2. **"It should work like..."** - Verify it actually does
3. **"Standard pattern..."** - Check for customizations
4. **"Obviously it..."** - Nothing is obvious
5. **"The type suggests..."** - Implementation may differ

## Documentation Quality Gates

Before publishing any documentation:

1. **Compilation Test**: Every code example must compile
2. **Execution Test**: Every example must run successfully  
3. **Output Verification**: Results must match documented behavior
4. **Dependency Check**: All requirements must be listed
5. **Fresh Environment**: Test in clean setup

## Remember

> "Tests can lie. Documentation can be wrong. Only source code tells the truth."

**Document what IS, not what SHOULD BE.**