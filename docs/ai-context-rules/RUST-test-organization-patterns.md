# RUST RULE: Test Organization Patterns

## Problem This Prevents
Misunderstanding Rust test organization, leading to incorrect documentation of test architecture and missing test patterns.

## The Rule
**Understand actual Rust test organization patterns before documenting testing infrastructure.**

## Common Rust Test Organization Patterns

### 1. Co-located Unit Tests (Most Common)
```rust
// src/lib.rs or src/module.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

**Detection Command:**
```bash
grep -r "#\[cfg(test)\]" . --include="*.rs" | wc -l
```

### 2. Separate Test Files (Less Common)
```rust
// src/calculator.rs
pub fn add(a: i32, b: i32) -> i32 { a + b }

// src/calculator_test.rs
#[cfg(test)]
mod tests {
    use super::super::calculator::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

**Detection Command:**
```bash
find . -name "*_test.rs" -not -path "*/target/*" | wc -l
```

### 3. Integration Tests Directory
```rust
// tests/integration_test.rs
use my_crate::add;

#[test]
fn test_add_integration() {
    assert_eq!(add(2, 3), 5);
}
```

**Detection Command:**
```bash
find . -path "*/tests/*.rs" -not -path "*/target/*" | wc -l
```

## Jupiter Design System: Actual Pattern Analysis

### What I Found
```bash
$ find . -name "*test*.rs" -not -path "*/target/*" | sort
./src/builders/button_test.rs
./src/builders/card_test.rs
./src/builders/layout_test.rs
./src/builders/product_test.rs
./src/builders/selection_test.rs
./src/builders/state_test.rs
./src/builders/text_test.rs
./src/core/color_test.rs

# Pattern: Separate test files alongside implementation
```

### Test Module Structure
```bash
$ head -10 src/builders/text_test.rs
#[cfg(test)]
mod tests {
    use crate::builders::text::{
        text_clamp_style, text_classes_from_strings, text_element_from_hierarchy, text_styles,
        TextStyles,
    };
    use crate::themes::VibeColors;

    // Helper function to create text styles with default color provider
    fn create_text_styles() -> TextStyles<VibeColors> {
```

**Pattern:** Separate test files with `#[cfg(test)]` modules

### Helper Function Pattern
```bash
$ grep -r "fn create_" . --include="*test*.rs" | grep -v "#\[test\]"
./src/builders/text_test.rs:    fn create_text_styles() -> TextStyles<VibeColors> {
./src/builders/card_test.rs:    fn create_card_styles() -> CardStyles<VibeColors> {
./src/builders/button_test.rs:    fn create_button_styles() -> ButtonStyles<VibeColors> {
```

**Pattern:** Each test file has its own helper functions (no shared utilities)

## Rust Test Organization Verification Commands

### 1. Identify Test Organization Pattern
```bash
# Check for co-located tests (in same file)
grep -r "#\[cfg(test)\]" . --include="*.rs" -l | while read file; do
    echo "$file has embedded tests"
done

# Check for separate test files
find . -name "*_test.rs" -o -name "*test.rs" | grep -v target

# Check for integration tests
ls -la tests/ 2>/dev/null || echo "No integration test directory"
```

### 2. Analyze Test Module Structure
```bash
# Show test module declarations
grep -r "#\[cfg(test)\]" . --include="*.rs" -A 5 | grep -E "(mod|use|fn)" | head -20

# Find test helper functions
grep -r "fn [a-zA-Z_][a-zA-Z0-9_]*(" . --include="*test*.rs" | grep -v "#\[test\]" | head -10
```

### 3. Check Test Dependencies
```bash
# Show test imports
grep -r "use.*test" . --include="*.rs" | head -10

# Check for test-only dependencies
grep -A 10 "\[dev-dependencies\]" Cargo.toml
```

## What I Incorrectly Assumed vs Reality

### My Assumptions (Wrong)
```markdown
❌ "Two-layer testing strategy (Pattern + Builder)"
❌ "Pattern testing in separate modules"
❌ "Shared test utilities across files"
❌ "Standard Rust unit test organization"
```

### Actual Reality (Verified)
```markdown
✅ "Separate test files alongside implementation"
✅ "Single-layer builder testing only"
✅ "Individual helper functions per test file"
✅ "Custom test organization pattern"
```

## Rust Test Discovery Commands

### Quick Assessment
```bash
# Test file count and pattern
echo "=== Test File Analysis ==="
echo "Co-located tests: $(grep -r "#\[cfg(test)\]" . --include="*.rs" | grep -v _test.rs | wc -l)"
echo "Separate test files: $(find . -name "*_test.rs" -not -path "*/target/*" | wc -l)"
echo "Integration tests: $(find . -path "*/tests/*.rs" -not -path "*/target/*" | wc -l)"

# Test function count
echo "=== Test Function Counts ==="
grep -r "#\[test\]" . --include="*.rs" | wc -l
```

### Detailed Analysis
```bash
# Per-file test breakdown
echo "=== Per-File Test Count ==="
for f in $(find . -name "*test*.rs" -not -path "*/target/*"); do
    count=$(grep -c "#\[test\]" "$f")
    echo "$f: $count tests"
done

# Test module pattern analysis
echo "=== Test Module Patterns ==="
grep -r "mod tests" . --include="*.rs" | wc -l
grep -r "mod test" . --include="*.rs" | wc -l
```

## Common Rust Test Patterns to Look For

### 1. Test Attributes
```bash
# Standard test attribute
grep -r "#\[test\]" . --include="*.rs" | wc -l

# Async tests
grep -r "#\[tokio::test\]" . --include="*.rs" | wc -l
grep -r "#\[async_std::test\]" . --include="*.rs" | wc -l

# Ignored tests
grep -r "#\[ignore\]" . --include="*.rs" | wc -l
```

### 2. Test Macros and Utilities
```bash
# Custom test macros
grep -r "macro_rules!" . --include="*.rs" | grep -i test

# Test helper functions
grep -r "fn.*test.*helper\|fn.*assert_\|fn.*create_test" . --include="*.rs"

# Property-based testing
grep -r "proptest\|quickcheck" . --include="*.rs"
```

### 3. Benchmark Tests
```bash
# Criterion benchmarks
grep -r "#\[bench\]" . --include="*.rs" | wc -l
find . -name "benches" -type d | wc -l
grep -r "criterion" . --include="*.rs" | wc -l
```

## Documentation Template for Rust Tests

Based on verification, document actual patterns:

```markdown
## Test Organization

### Pattern Analysis
- **Test Files**: 8 separate test files (`*_test.rs`)
- **Co-located Tests**: 0 (no `#[cfg(test)]` in implementation files)
- **Integration Tests**: 0 (no `tests/` directory)
- **Test Organization**: Custom pattern (separate files with test modules)

### Verification Commands
```bash
# Test file pattern verification
find . -name "*_test.rs" -not -path "*/target/*" | wc -l

# Test module verification
grep -r "#\[cfg(test)\]" . --include="*.rs" | wc -l

# Integration test verification
ls -la tests/ 2>/dev/null || echo "No integration tests"
```

### Test Structure
Each test file follows this pattern:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::themes::VibeColors;

    fn create_builder_styles() -> BuilderStyles<VibeColors> {
        builder_styles(VibeColors::default())
    }

    #[test]
    fn test_functionality() {
        // Test implementation
    }
}
```
```

## Time Impact
- Understanding Rust patterns: 5 minutes
- Documenting wrong patterns: 1+ hours rework
- Accuracy improvement: Prevents architectural misunderstandings

## Key Insight
**"Rust projects don't always follow 'standard' patterns - verify the actual organization."**

Never assume a Rust project follows typical patterns. Always verify with commands and read actual test files to understand the organization.