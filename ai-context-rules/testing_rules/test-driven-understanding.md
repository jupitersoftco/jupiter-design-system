# Test-Driven Understanding Rule

## Context
Tests reveal actual system behavior and usage patterns, but they can also lie or be incomplete. Use tests as a starting point, not the final authority.

## The Rule
**Use tests to understand actual behavior, then validate test assumptions against implementation:**

### Test Analysis Process

1. **Find All Tests**
   ```bash
   # Locate all test files
   find . -name "*test*" -type f
   
   # Count test functions
   rg "#\[test\]" --type rust --count
   
   # Find test modules
   rg "mod.*test" --type rust -n
   ```

2. **Extract Test Patterns**
   ```bash
   # Find common test patterns
   rg "assert_eq|assert|expect" --type rust -A 2 -B 2
   
   # Find setup patterns
   rg "setup|before|init" --type rust -i -A 5
   ```

3. **Validate Test Assumptions**
   ```bash
   # For each test assertion, find the actual implementation
   TEST_CLAIM="Color::Primary"
   rg "$TEST_CLAIM" --type rust src/
   
   # Verify the test isn't testing fantasy behavior
   rg "Color::Primary" --type rust --count
   ```

### Test Validation Patterns

#### Pattern 1: Enum Variant Testing
```rust
// Test might show:
#[test]
fn test_color_variants() {
    assert_eq!(Color::Primary.to_string(), "primary");
    assert_eq!(Color::Accent.to_string(), "accent");  // <- This might not exist in src/
}
```

**Validation Commands:**
```bash
# Check if all tested variants exist in source
rg "Color::" --type rust tests/ | grep -o "Color::[A-Za-z]*" | sort | uniq
rg "Color::" --type rust src/ | grep -o "Color::[A-Za-z]*" | sort | uniq
```

#### Pattern 2: Builder Pattern Testing
```rust
// Test might show:
#[test]
fn test_button_builder() {
    let btn = button_styles(theme)
        .primary()
        .large()
        .disabled()  // <- This method might not exist
        .classes();
}
```

**Validation Commands:**
```bash
# Check if all chained methods exist
rg "\.primary\(\)" --type rust src/
rg "\.large\(\)" --type rust src/
rg "\.disabled\(\)" --type rust src/
```

### Example from Today

**Test-Revealed Pattern:**
```rust
// Found in tests/color_test.rs
#[test]
fn test_color_variants() {
    assert_eq!(Color::Primary.to_string(), "primary");
    assert_eq!(Color::Accent.to_string(), "accent");
}
```

**Validation Revealed:**
```bash
# Check source implementation
rg "Color::" --type rust src/
# Result: Color::Accent NOT found in src/, only in tests

# Check enum definition
rg "enum Color" --type rust src/ -A 20
# Result: No Accent variant in actual enum
```

**Conclusion:** Test was testing future/planned behavior, not current implementation.

### Test Truth vs Implementation Truth

#### Tests Can Lie About:
- Features that are planned but not implemented
- Variants that exist in test mocks but not real code
- API methods that are stubbed for testing
- Performance characteristics that are simulated

#### Tests Are Reliable For:
- Integration patterns that actually work
- Error conditions that are actually handled
- API signatures that are actually implemented
- Usage patterns that are actually supported

### Documentation Strategy

1. **Use Tests for Usage Patterns**
   ```rust
   // Extract real usage from tests
   let theme = VibeColors::new();
   let button = button_styles(theme)
       .primary()
       .large()
       .classes();
   ```

2. **Validate Against Source**
   ```bash
   # Ensure every documented method exists
   rg "\.primary\(\)" --type rust src/
   rg "\.large\(\)" --type rust src/
   ```

3. **Document Discrepancies**
   ```markdown
   ## Known Issues
   - Color::Accent appears in tests but is not yet implemented (see issue #45)
   - Button .disabled() method is planned but not available
   ```

### Test-Driven Documentation Workflow

```bash
# 1. Extract all test patterns
rg "#\[test\]" --type rust -A 20 | grep -E "(assert|expect|Color::|button_styles)"

# 2. Validate each pattern against source
for pattern in $(rg "Color::" --type rust tests/ | grep -o "Color::[A-Za-z]*" | sort | uniq); do
    echo "Checking $pattern in source:"
    rg "$pattern" --type rust src/ --count
done

# 3. Document only validated patterns
# 4. Note discrepancies in "Known Issues" section
```

## Key Principle
Tests show you how the system is intended to be used, but not necessarily how it currently works. Always validate test assumptions against actual implementation before documenting.