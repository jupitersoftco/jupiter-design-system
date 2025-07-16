# WORKFLOW RULE: Test Discovery Patterns

## Problem This Prevents
Missing test files, incorrect test counts, and undiscovered testing patterns that lead to inaccurate documentation.

## The Rule
**Follow a systematic discovery workflow BEFORE documenting any testing infrastructure.**

## Test Discovery Workflow

### Phase 1: Initial Discovery (2 minutes)
```bash
# 1. Find all test files
find . -name "*test*" -type f -name "*.rs" | sort

# 2. Count test files
find . -name "*test*" -type f -name "*.rs" | wc -l

# 3. Show test file structure
find . -name "*test*" -type f -name "*.rs" | head -20

# 4. Check for test directories
find . -type d -name "*test*" | sort
```

### Phase 2: Deep Analysis (5 minutes)
```bash
# 1. Count actual test functions
grep -r "#\[test\]" . --include="*.rs" | wc -l

# 2. Count tests per file
for f in $(find . -name "*test*" -name "*.rs"); do
  echo "$f: $(grep -c "#\[test\]" "$f") tests"
done

# 3. Identify test patterns
grep -r "mod tests" . --include="*.rs" -A 5 | head -30

# 4. Find helper functions
grep -r "fn create_\|fn assert_\|fn test_helper" . --include="*.rs" | grep -v "#\[test\]"
```

### Phase 3: Pattern Recognition (3 minutes)
```bash
# 1. Identify test organization
grep -r "#\[cfg(test)\]" . --include="*.rs" -B 2 -A 10

# 2. Find common test utilities
grep -r "use super::\*" . --include="*test*.rs" | sort | uniq

# 3. Discover assertion patterns
grep -r "assert" . --include="*test*.rs" | cut -d: -f3- | sort | uniq -c | sort -nr | head -20

# 4. Check for integration tests
ls -la tests/ 2>/dev/null || echo "No integration test directory"
```

## Real Example: Jupiter Design System Discovery

### What I Should Have Found First
```bash
$ find . -name "*test*" -type f -name "*.rs" | wc -l
8

$ for f in $(find . -name "*test*" -name "*.rs"); do
    echo "$f: $(grep -c "#\[test\]" "$f") tests"
  done
./src/core/color_test.rs: 11 tests
./src/builders/button_test.rs: 23 tests
./src/builders/text_test.rs: 24 tests
./src/builders/product_test.rs: 29 tests
./src/builders/selection_test.rs: 18 tests
./src/builders/layout_test.rs: 16 tests
./src/builders/state_test.rs: 18 tests
./src/builders/card_test.rs: 18 tests
# Total: 157 tests (not 180+!)
```

### Pattern Discovery Results
```bash
# Test organization pattern
$ grep -r "#\[cfg(test)\]" . --include="*.rs" | wc -l
8  # All tests use same pattern

# Helper functions
$ grep -r "fn create_" . --include="*test*.rs" | grep -v "#\[test\]"
./src/builders/text_test.rs:    fn create_text_styles() -> TextStyles<VibeColors> {
./src/builders/card_test.rs:    fn create_card_styles() -> CardStyles<VibeColors> {
# Pattern: Each test file has its own create_*_styles() helper

# No shared utilities found
$ find . -name "*util*" | grep -i test
# (empty - no test utilities exist!)
```

## Workflow for Different Project Types

### Rust Projects
```bash
# Cargo test summary
cargo test --no-run 2>&1 | tail -20

# Find test modules
grep -r "mod.*test" . --include="*.rs"

# Check for criterion benchmarks
find . -name "*.rs" -exec grep -l "criterion" {} \;
```

### JavaScript/TypeScript Projects
```bash
# Find test files
find . \( -name "*.test.js" -o -name "*.spec.js" -o -name "*.test.ts" -o -name "*.spec.ts" \) | wc -l

# Check test framework
grep -h "from ['\"].*test\|from ['\"].*jest\|from ['\"].*mocha" . -r --include="*.js" --include="*.ts" | sort | uniq

# Count test suites
grep -r "describe(" . --include="*.test.js" --include="*.spec.js" | wc -l
```

### Python Projects
```bash
# Find test files
find . -name "test_*.py" -o -name "*_test.py" | wc -l

# Check for pytest
find . -name "*.py" -exec grep -l "pytest\|unittest" {} \; | head -10

# Count test functions
grep -r "def test_" . --include="*.py" | wc -l
```

## Discovery Output Template

Create a discovery summary BEFORE writing documentation:

```markdown
## Test Infrastructure Discovery Results

### File Structure
- Test files found: 8
- Test directories: 0 (no separate test directory)
- Organization: Co-located with implementation (*_test.rs)

### Test Counts
- Total test functions: 157
- Distribution:
  - color_test.rs: 11 tests
  - button_test.rs: 23 tests
  - [... complete list ...]

### Patterns Identified
- Test structure: #[cfg(test)] mod tests
- Helper pattern: Individual create_*_styles() per file
- Shared utilities: None found
- Integration tests: Not implemented

### Architecture
- ❌ Two-layer testing (claimed) - NOT FOUND
- ✅ Single-layer builder testing (actual)
- ❌ Pattern tests - NOT FOUND
- ✅ Builder API tests only
```

## Time Impact
- Following this workflow: 10 minutes discovery
- Not following: Multiple rounds of corrections
- Accuracy improvement: 100%

## Key Insight
**"You can't document what you haven't discovered."**

Always complete the discovery workflow before writing a single line of documentation. The 10 minutes spent on discovery saves hours of corrections later.