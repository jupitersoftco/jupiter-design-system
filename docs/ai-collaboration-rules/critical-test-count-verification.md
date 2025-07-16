# Critical Rule: Test Count Verification

## Problem This Prevents
- **Inflated test coverage claims** that mislead about actual protection
- **False confidence** in untested code
- **Resource misallocation** based on incorrect coverage metrics
- **Broken CI/CD assumptions** about test suite completeness

## Time/Effort Saved
- Saves 2-3 hours debugging "missing" tests that never existed
- Prevents test coverage report discrepancies
- Avoids emergency test writing when gaps are discovered late

## Actionable Steps

### 1. Count Tests Programmatically
```bash
# Count actual test functions
grep -c "^\s*#\[test\]" src/builders/button_test.rs

# List all test functions with line numbers
grep -n "^\s*#\[test\]" src/builders/button_test.rs

# Find all test files and count tests in each
find src -name "*test*.rs" -exec sh -c 'echo "$1: $(grep -c "^\s*#\[test\]" "$1")"' _ {} \;
```

### 2. Verify Test Modules
```bash
# Check for test modules within source files
grep -n "mod tests" src/**/*.rs

# Count tests in inline test modules
grep -B1 "^\s*#\[test\]" src/builders/button.rs | grep -c "#\[test\]"
```

### 3. Document with Exact Counts
```markdown
## Test Coverage Summary (2024-01-16)
| Module | Test File | Test Count | Verified Method |
|--------|-----------|------------|-----------------|
| button | button_test.rs | 3 | grep count |
| card | card_test.rs | 0 | file read |
| layout | layout_test.rs | 0 | file read |
| state | state_test.rs | 7 | grep count |
```

## Examples from Today's Session

### Example 1: Button Test Verification
**Initial Claim:** "Button has comprehensive tests"

**Verification Process:**
```bash
$ grep -c "#\[test\]" src/builders/button_test.rs
3

$ grep "#\[test\]" src/builders/button_test.rs
    #[test]
    #[test]
    #[test]
```

**Accurate Documentation:**
```markdown
Button component has exactly 3 tests in button_test.rs:
1. test_button_new() - line 67
2. test_button_color() - line 75  
3. test_button_action() - line 90
```

### Example 2: Empty Test File Discovery
**Initial Assumption:** "card_test.rs contains card tests"

**Verification:**
```bash
$ wc -l src/builders/card_test.rs
0 src/builders/card_test.rs

$ grep -c "#\[test\]" src/builders/card_test.rs
0
```

**Accurate Documentation:**
```markdown
card_test.rs exists but is empty (0 lines, 0 tests)
```

## Test Counting Best Practices

### 1. Multiple Verification Methods
```bash
# Method 1: Direct grep
grep -c "#\[test\]" file.rs

# Method 2: Cargo test list (if available)
cargo test --lib -- --list | grep -c "test"

# Method 3: Manual file inspection
cat file.rs | grep -A2 "#\[test\]"
```

### 2. Track Test Types
```markdown
## StateBuilder Tests (Total: 7)
### Initialization Tests: 2
- test_state_new()
- test_state_default()

### State Transition Tests: 3  
- test_loading_state()
- test_success_state()
- test_error_state()

### Builder Pattern Tests: 2
- test_state_chaining()
- test_state_build()
```

### 3. Include Negative Findings
```markdown
## Test Gap Analysis
### Files with 0 tests:
- card_test.rs (empty file)
- layout_test.rs (empty file)
- interactive.rs (no test module)

### Modules without test files:
- themes/
- patterns/ (tests may be in individual pattern files)
```

## Verification Script Template
```bash
#!/bin/bash
# Test count verification script

echo "Test Count Verification Report"
echo "Generated: $(date)"
echo "================================"

for file in $(find src -name "*test*.rs"); do
    count=$(grep -c "^\s*#\[test\]" "$file")
    lines=$(wc -l < "$file")
    echo "$file: $count tests in $lines lines"
done

echo "================================"
echo "Total test files: $(find src -name "*test*.rs" | wc -l)"
echo "Total tests: $(find src -name "*test*.rs" -exec grep -c "^\s*#\[test\]" {} \; | paste -sd+ | bc)"
```

## Red Flags to Avoid
1. Claiming "multiple tests" without counting
2. Assuming test files contain tests
3. Conflating test functions with test cases
4. Including non-test functions in counts
5. Counting commented-out tests

## Remember
**Zero tests is valid data - document it accurately rather than hide it.**