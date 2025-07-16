# Critical Rule: Verify Before Documenting

## Problem This Prevents
- **False documentation claims** about test existence or implementation details
- **Misleading information** that causes wasted time investigating non-existent tests
- **Confusion** between what is planned vs what actually exists
- **Trust erosion** when documentation doesn't match reality

## Time/Effort Saved
- Saves 30-60 minutes per incorrect claim that needs to be investigated
- Prevents cascading documentation errors
- Eliminates need for documentation audits and corrections

## Actionable Steps

### 1. Always Read Source Files First
```bash
# Before claiming tests exist, physically verify:
- Read the actual test file
- Count the test functions
- Verify test names match your claims
```

### 2. Use Explicit Verification Commands
```bash
# Good: Verify test count
grep -c "#\[test\]" src/builders/button_test.rs

# Good: List all test functions
grep -A 1 "#\[test\]" src/builders/button_test.rs

# Good: Check for test modules
grep -n "mod tests" src/builders/button.rs
```

### 3. Document What You Actually See
```markdown
# Good Documentation
"The button_test.rs file contains 3 tests:
- test_button_new()
- test_button_color()
- test_button_action()
(Verified by reading the file)"

# Bad Documentation
"The button module has comprehensive tests"
(Vague claim without verification)
```

## Examples from Today's Session

### Example 1: Button Test Miscount
**Wrong Approach:**
```markdown
"Button component has 5 tests covering all functionality"
```

**Correct Approach:**
```markdown
"After reading button_test.rs, found 3 tests:
1. test_button_new() - Tests basic instantiation
2. test_button_color() - Tests color variants
3. test_button_action() - Tests action variants"
```

### Example 2: Non-existent Test Claims
**Wrong Approach:**
```markdown
"The layout system has comprehensive grid tests"
```

**Correct Approach:**
```markdown
"Checked layout_test.rs - Currently empty (0 tests).
Grid functionality exists but tests are not yet implemented."
```

## Verification Checklist
- [ ] Did I read the actual source file?
- [ ] Did I count the exact number of tests?
- [ ] Did I verify test names and what they test?
- [ ] Did I distinguish between "exists" vs "should exist"?
- [ ] Did I note the verification method used?

## Red Flags to Avoid
1. Using words like "comprehensive", "extensive", or "full coverage" without verification
2. Claiming tests exist based on function names alone
3. Assuming test files contain tests without checking
4. Extrapolating from partial information

## Remember
**When in doubt, read the file and quote directly from it.**