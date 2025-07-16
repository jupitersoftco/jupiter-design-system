# Documentation-Code Alignment Rule

## Context
Documentation that doesn't match actual code implementation is worse than no documentation. Every claim must be verifiable.

## The Rule
**Every documentation statement about functionality must be verified against actual code:**

### Verification Process

1. **Feature Claims**
   ```bash
   # For each documented feature, verify it exists
   FEATURE="ColorProvider trait"
   rg "$FEATURE" --type rust
   
   # Check if it's actually implemented vs just defined
   rg "impl.*$FEATURE" --type rust
   ```

2. **Test Coverage Claims**
   ```bash
   # Don't claim "comprehensive test coverage" without verification
   # Count actual tests
   rg "#\[test\]" --type rust --count
   
   # Verify test categories mentioned in docs exist
   rg "mod.*test" --type rust
   ```

3. **API Examples**
   ```bash
   # Every code example should be derived from actual usage
   # Find real usage in tests
   rg "button_styles\(" --type rust -A 5 -B 5
   
   # Verify the example pattern exists
   EXAMPLE_PATTERN="\.primary\(\)\.large\(\)"
   rg "$EXAMPLE_PATTERN" --type rust
   ```

## Example from Today

**Documentation Claim:** "The Color enum provides semantic color values"

**Required Verification:**
```bash
# Find the actual Color enum
rg "enum Color" --type rust -A 20

# Verify all documented variants exist
rg "Color::" --type rust | sort | uniq

# Check test coverage for Color
rg "Color::" --type rust tests/
```

**Finding:** Tests showed Color::Accent exists but wasn't documented initially

## Anti-Patterns to Avoid

### ❌ Aspirational Documentation
```markdown
<!-- Wrong: Documenting what should exist -->
The system provides automatic theme switching between light and dark modes.
```

### ❌ Untested Examples
```rust
// Wrong: Creating examples without verification
let theme = ThemeManager::new()
    .auto_switch(true)  // This method might not exist
    .build();
```

### ✅ Verified Documentation
```markdown
<!-- Right: Document what actually exists -->
The system provides theme customization through the ColorProvider trait,
as demonstrated in tests/theme_test.rs lines 45-67.
```

## Validation Checklist
- [ ] Every feature claim has a corresponding `rg` search result
- [ ] Every code example is derived from actual test/example files
- [ ] API signatures in docs match actual implementation
- [ ] Test coverage numbers are counted, not estimated
- [ ] Integration examples come from real integration tests

## Implementation Commands
```bash
# Before documenting any feature
FEATURE_NAME="ColorProvider"

# 1. Verify it exists
rg "struct $FEATURE_NAME|trait $FEATURE_NAME|enum $FEATURE_NAME" --type rust

# 2. Find actual usage
rg "$FEATURE_NAME" --type rust tests/ examples/

# 3. Count implementations
rg "impl.*$FEATURE_NAME" --type rust --count

# 4. Extract real examples
rg "$FEATURE_NAME" --type rust -B 5 -A 10 tests/
```

## Key Principle
If you can't find it with `rg`, don't document it. If you can't show a real example from the codebase, don't invent one.