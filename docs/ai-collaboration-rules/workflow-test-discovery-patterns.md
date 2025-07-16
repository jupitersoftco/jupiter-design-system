# Workflow Rule: Test Discovery Patterns

## Problem This Prevents
- **Missing test files** in documentation or coverage reports
- **Hidden test modules** that aren't properly tracked
- **Incomplete test inventory** leading to coverage gaps
- **Duplicate test effort** from not knowing tests exist

## Time/Effort Saved
- Saves 1-2 hours of manual test searching
- Prevents duplicate test implementation
- Ensures accurate test coverage metrics
- Speeds up test failure debugging

## Test Discovery Patterns

### 1. Separate Test File Pattern
```bash
# Pattern: {module}_test.rs files
find src -name "*_test.rs" -type f

# Verify they contain actual tests
for f in $(find src -name "*_test.rs"); do
    count=$(grep -c "^\s*#\[test\]" "$f")
    if [ $count -gt 0 ]; then
        echo "$f: $count tests"
    else
        echo "$f: EMPTY (no tests)"
    fi
done
```

### 2. Inline Test Module Pattern
```bash
# Pattern: mod tests {} within source files
grep -n "mod tests" src/**/*.rs

# Count tests in inline modules
for f in $(grep -l "mod tests" src/**/*.rs); do
    # Extract test module and count tests
    awk '/mod tests/,/^}/' "$f" | grep -c "#\[test\]"
done
```

### 3. Integration Test Pattern
```bash
# Pattern: tests/ directory files
find tests -name "*.rs" -type f 2>/dev/null || echo "No tests/ directory"

# Check for integration test functions
find tests -name "*.rs" -exec grep -l "^\s*#\[test\]" {} \; 2>/dev/null
```

### 4. Documentation Test Pattern
```bash
# Pattern: ``` rust code blocks with assertions
grep -r "```rust" docs/ | while read -r line; do
    file=$(echo "$line" | cut -d: -f1)
    # Check if doc has testable code
    if grep -A10 "```rust" "$file" | grep -q "assert"; then
        echo "$file has testable documentation"
    fi
done

# Check for #[doc] tests in source
grep -r "/// ```" src/ | grep -B1 -A5 "assert"
```

## Discovery Workflow

### Step 1: Complete Test Inventory
```bash
#!/bin/bash
# test-discovery.sh

echo "=== Test Discovery Report ==="
echo "Generated: $(date)"
echo

echo "1. Separate Test Files:"
find src -name "*_test.rs" | while read -r f; do
    tests=$(grep -c "#\[test\]" "$f")
    lines=$(wc -l < "$f")
    echo "   $f - $tests tests, $lines lines"
done

echo -e "\n2. Inline Test Modules:"
grep -l "mod tests" src/**/*.rs | while read -r f; do
    # Count tests within the module
    tests=$(awk '/mod tests/,/^}/' "$f" | grep -c "#\[test\]")
    echo "   $f - $tests tests in 'mod tests'"
done

echo -e "\n3. Integration Tests:"
if [ -d "tests" ]; then
    find tests -name "*.rs" | while read -r f; do
        tests=$(grep -c "#\[test\]" "$f")
        echo "   $f - $tests tests"
    done
else
    echo "   No tests/ directory found"
fi

echo -e "\n4. Example Tests:"
if [ -d "examples" ]; then
    examples=$(find examples -name "*.rs" | wc -l)
    echo "   Found $examples example files"
fi
```

### Step 2: Test Organization Matrix
```markdown
| Pattern | Location | Example | Tests Found | Notes |
|---------|----------|---------|-------------|-------|
| Separate files | src/builders/*_test.rs | button_test.rs | 3 | Preferred pattern |
| Inline modules | src/**/*.rs#mod tests | color.rs | 5 | For tightly coupled tests |
| Integration | tests/*.rs | theme_integration.rs | 0 | Not yet used |
| Doc tests | /// ``` | typography.rs | 2 | In documentation |
```

### Step 3: Test Naming Convention Check
```bash
# Verify test naming patterns
echo "=== Test Naming Patterns ==="

# Common patterns to check
patterns=(
    "test_*"
    "*_test"
    "should_*"
    "it_*"
    "*_when_*"
)

for pattern in "${patterns[@]}"; do
    count=$(grep -r "#\[test\]" src/ -A1 | grep "fn $pattern" | wc -l)
    echo "$pattern: $count tests"
done
```

## Examples from Today's Session

### Example 1: Hidden Test Discovery
**Initial search:**
```bash
$ ls src/builders/*_test.rs
# Found: button_test.rs, card_test.rs, layout_test.rs, state_test.rs
```

**Deep search revealed:**
```bash
$ grep -r "mod tests" src/
src/core/color.rs:mod tests {
# Found additional inline tests not in separate files
```

### Example 2: Empty Test File Detection
**Discovery process:**
```bash
$ find src -name "*_test.rs" -size 0
src/builders/card_test.rs
src/builders/layout_test.rs

$ find src -name "*_test.rs" -exec sh -c '[ $(grep -c "#\[test\]" "$1") -eq 0 ] && echo "$1"' _ {} \;
src/builders/card_test.rs
src/builders/layout_test.rs
src/builders/product_test.rs  # Not empty but has 0 tests
```

## Advanced Discovery Techniques

### 1. Test Attribute Variations
```bash
# Standard test attribute
grep -r "^\s*#\[test\]" src/

# Async tests
grep -r "^\s*#\[tokio::test\]" src/
grep -r "^\s*#\[async_std::test\]" src/

# Conditional tests
grep -r "^\s*#\[cfg_attr.*test\]" src/

# Property-based tests
grep -r "^\s*#\[quickcheck\]" src/
grep -r "^\s*#\[proptest\]" src/
```

### 2. Test Helper Discovery
```bash
# Find test utilities and helpers
grep -r "^\s*#\[cfg(test)\]" src/ -B2 -A5

# Find test-only modules
grep -r "^\s*#\[cfg(test)\]\s*mod" src/

# Find test fixtures
find . -path "*/testdata/*" -o -path "*/fixtures/*" -o -path "*/test_*"
```

### 3. Coverage Gap Analysis
```bash
# Find public functions without corresponding tests
for f in src/**/*.rs; do
    # Skip test files
    [[ $f == *_test.rs ]] && continue
    
    # Extract public function names
    grep "^\s*pub fn" "$f" | while read -r fn; do
        fn_name=$(echo "$fn" | sed 's/.*fn \([a-z_]*\).*/\1/')
        # Check if test exists
        if ! grep -r "test_$fn_name\|${fn_name}_test" src/**/*test*.rs > /dev/null; then
            echo "No test found for: $fn_name in $f"
        fi
    done
done
```

## Test Discovery Checklist
- [ ] Check for *_test.rs files
- [ ] Search for inline `mod tests` blocks
- [ ] Look for integration tests in tests/
- [ ] Find documentation tests in comments
- [ ] Identify test helper modules
- [ ] Check for alternative test frameworks
- [ ] Verify example files are runnable
- [ ] Find test data and fixtures

## Red Flags in Test Discovery
1. Test files with 0 tests but >0 lines (dead code)
2. Test modules without #[cfg(test)]
3. Test functions not marked with #[test]
4. Inconsistent test file naming
5. Tests in source files without module isolation

## Remember
**Tests can hide in many places - systematic discovery prevents gaps.**