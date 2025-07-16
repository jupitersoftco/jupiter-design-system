# TESTING RULE: Accurate Test Counting and Coverage Claims

## Problem This Prevents
False test coverage claims, inflated test counts, and missing test categories that mislead users about project quality.

## The Rule
**Always verify test counts with multiple methods and document the verification commands used.**

## Test Counting Protocol

### Step 1: Multiple Counting Methods
```bash
# Method 1: Direct test attribute count
grep -r "#\[test\]" . --include="*.rs" | wc -l

# Method 2: Cargo test output
cargo test 2>&1 | grep -E "^test result:|running \d+ test"

# Method 3: Per-file breakdown
for f in $(find . -name "*test*.rs"); do
  count=$(grep -c "#\[test\]" "$f")
  echo "$f: $count tests"
  total=$((total + count))
done
echo "Total: $total tests"

# Method 4: Test module verification
grep -r "mod tests" . --include="*.rs" | wc -l
```

### Step 2: Verify Test Categories
```bash
# Check for different test types
echo "=== Test Category Breakdown ==="
echo "Unit tests: $(find . -path "*/src/*" -name "*test*.rs" | wc -l) files"
echo "Integration tests: $(find . -path "*/tests/*" -name "*.rs" | wc -l) files"
echo "Doc tests: $(grep -r "/// ```" . --include="*.rs" | wc -l) examples"
echo "Benchmark tests: $(grep -r "#\[bench\]" . --include="*.rs" | wc -l) benchmarks"
```

### Step 3: Coverage Verification
```bash
# Check if coverage is actually measured
ls -la .coverage/ target/coverage/ 2>/dev/null || echo "No coverage data found"

# Look for coverage configuration
find . -name ".coveragerc" -o -name "codecov.yml" -o -name "tarpaulin.toml"

# Check CI for coverage runs
grep -r "coverage\|tarpaulin\|codecov" .github/workflows/ .gitlab-ci.yml 2>/dev/null
```

## Real Example: Jupiter Design System Miscount

### Initial Wrong Claims
```markdown
Total: 180+ tests ✅
Coverage: 100% ✅
Performance: < 3s ✅
```

### Actual Verification Results
```bash
$ grep -r "#\[test\]" . --include="*.rs" | wc -l
157

$ for f in $(find . -name "*test*.rs"); do
    echo "$f: $(grep -c "#\[test\]" "$f")"
  done
./src/core/color_test.rs: 11
./src/builders/button_test.rs: 23
./src/builders/text_test.rs: 24
./src/builders/product_test.rs: 29
./src/builders/selection_test.rs: 18
./src/builders/layout_test.rs: 16
./src/builders/state_test.rs: 18
./src/builders/card_test.rs: 18
Total: 157 (not 180+!)

$ find . -name "tarpaulin.toml" -o -name ".coverage"
(nothing - no coverage measurement exists!)

$ cargo test 2>&1 | tail -1
test result: ok. 157 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

## Test Count Verification Template

Always document counts with verification:

```markdown
## Test Coverage

### Verified Test Counts
```bash
# Verification command used:
$ grep -r "#\[test\]" . --include="*.rs" | wc -l
157
```

### Test Distribution
| Module | Test Count | Verified |
|--------|------------|----------|
| color_test.rs | 11 | ✅ |
| button_test.rs | 23 | ✅ |
| text_test.rs | 24 | ✅ |
| ... | ... | ... |
| **Total** | **157** | **✅** |

### Coverage Claims
- Code Coverage: Not measured (no tooling configured)
- Test Categories: Unit tests only
- Integration Tests: 0 (no tests/ directory)
- Performance Tests: 0 (no #[bench] found)
```

## Common Test Count Pitfalls

### 1. Counting Test Modules Instead of Tests
```bash
# Wrong: Counts modules, not individual tests
$ grep -r "mod tests" . | wc -l
8  # This is not the test count!

# Right: Count actual test functions
$ grep -r "#\[test\]" . | wc -l
157  # This is the actual count
```

### 2. Including Non-Test Items
```bash
# Wrong: Includes comments and strings
$ grep -r "test" . --include="*.rs" | wc -l
423  # Includes every mention of "test"

# Right: Only test attributes
$ grep -r "^[[:space:]]*#\[test\]" . --include="*.rs" | wc -l
157  # Only actual test functions
```

### 3. Missing Test Categories
```bash
# Check all test types
find . -name "*.rs" -exec grep -l "#\[test\]\|#\[tokio::test\]\|#\[async_std::test\]" {} \; | wc -l
```

## Language-Specific Counting

### JavaScript/TypeScript
```bash
# Jest/Mocha tests
grep -r "it(\|test(\|describe(" . --include="*.test.js" --include="*.spec.js" | grep -v "skip\|todo" | wc -l

# Detailed breakdown
echo "Test files: $(find . -name "*.test.js" -o -name "*.spec.js" | wc -l)"
echo "Test suites: $(grep -r "describe(" . --include="*.test.js" | wc -l)"
echo "Test cases: $(grep -r "it(\|test(" . --include="*.test.js" | wc -l)"
```

### Python
```bash
# Count test functions
grep -r "def test_" . --include="*.py" | wc -l

# Count test classes
grep -r "class Test" . --include="*.py" | wc -l

# pytest specific
pytest --collect-only -q 2>/dev/null | tail -1
```

## Coverage Reality Check

Never claim coverage without measurement:

```bash
# Rust coverage check
cargo tarpaulin --version 2>/dev/null || echo "No coverage tool installed"

# JavaScript coverage check
npm run coverage 2>/dev/null || echo "No coverage script defined"

# Python coverage check
coverage --version 2>/dev/null || echo "No coverage tool installed"
```

## Time Impact
- Accurate counting: 5 minutes
- Fixing wrong counts later: 30+ minutes
- Credibility damage from inflated numbers: High

## Key Principle
**"If you can't show the command that proves it, don't claim it."**

Every test count, coverage percentage, or performance metric must be verifiable with a specific command that anyone can run.