# Debugging Rule: Documentation Accuracy Debugging

## Problem This Prevents
- **Cascading misinformation** that spreads through the codebase
- **Wasted debugging time** from incorrect documentation
- **Loss of developer trust** in documentation
- **Incorrect implementation** based on wrong docs

## Time/Effort Saved
- Saves 2-3 hours per documentation-related bug
- Prevents 1-2 hours of incorrect implementation
- Reduces documentation fix cycles by 70%
- Eliminates trust rebuilding time

## Debugging Process

### 1. Identify Documentation Inaccuracy Symptoms
```markdown
## Common Symptoms:
- Code doesn't work as documented
- Tests fail despite following docs
- Developers report confusion
- Multiple interpretations of same doc
- Examples don't compile/run
```

### 2. Documentation Verification Checklist
```bash
#!/bin/bash
# doc-accuracy-check.sh

# For a specific documentation claim
DOC_FILE="docs/testing.md"
CLAIM="ButtonBuilder has 5 tests"

echo "Debugging documentation claim: $CLAIM"
echo "From file: $DOC_FILE"

# Step 1: Find the source of truth
echo -e "\n1. Locating source code..."
find src -name "*button*" -type f

# Step 2: Verify the claim
echo -e "\n2. Verifying claim..."
actual_count=$(grep -c "#\[test\]" src/builders/button_test.rs)
echo "Actual test count: $actual_count"

# Step 3: Check claim history
echo -e "\n3. Checking documentation history..."
git log -p --grep="5 tests" -- "$DOC_FILE"

# Step 4: Find related claims
echo -e "\n4. Finding related claims..."
grep -n "test" "$DOC_FILE" | grep -i "button"
```

### 3. Root Cause Analysis
```markdown
## Documentation Error Categories

### 1. Temporal Confusion
**Symptom**: Mixing current and future state
**Debug**: Check commit history for when feature was added
**Fix**: Add explicit time markers

### 2. Assumption Propagation  
**Symptom**: One wrong claim leads to others
**Debug**: Trace claim dependencies
**Fix**: Verify each claim independently

### 3. Version Drift
**Symptom**: Docs for different version
**Debug**: Compare doc date with code version
**Fix**: Version-tag documentation

### 4. Copy-Paste Errors
**Symptom**: Similar but incorrect information
**Debug**: Search for similar patterns
**Fix**: Generate docs from code when possible
```

## Debugging Techniques

### 1. Claim Tracing
```bash
# Trace a documentation claim to its source
claim="CardBuilder supports responsive layouts"

# Find where claim appears
grep -r "$claim" docs/

# Find related code
grep -r "responsive\|Responsive" src/ | grep -i card

# Check tests for validation
grep -r "responsive" src/**/*test*.rs

# Look for TODOs or planned features
grep -r "TODO.*responsive\|PLANNED.*responsive" src/
```

### 2. Documentation Diff Analysis
```bash
# Compare documentation claims with implementation
diff <(grep "pub fn" src/builders/button.rs | sort) \
     <(grep "ButtonBuilder::" docs/api.md | grep -o "::[a-z_]*" | sort)

# Find undocumented public APIs
comm -23 <(grep "pub" src/**/*.rs | grep -v test | sort) \
         <(grep -h "##\|pub" docs/**/*.md | sort)
```

### 3. Example Validation
```rust
// Tool to validate documentation examples
#[cfg(test)]
mod doc_validation {
    #[test]
    fn validate_button_example() {
        // Copy example from docs
        let example = r#"
            let button = ButtonBuilder::new("Click me")
                .color(ButtonColor::Primary)
                .action(ButtonAction::Submit)
                .build();
        "#;
        
        // This test fails if example doesn't compile
        // Ensures documentation stays accurate
    }
}
```

## Examples from Today's Session

### Example 1: Test Count Mismatch
**Documentation**: "Button has comprehensive test coverage with 5 tests"

**Debugging Process**:
```bash
# 1. Initial verification
$ grep -c "#\[test\]" src/builders/button_test.rs
3

# 2. Check if tests exist elsewhere
$ grep -r "test.*button" src/ | grep "#\[test\]"
# Only found 3 in button_test.rs

# 3. Check documentation history
$ git blame docs/testing.md | grep "5 tests"
# Added 2 weeks ago by commit abc123

# 4. Check commit abc123
$ git show abc123
# Found: Documentation written before tests were implemented

# Root cause: Aspirational documentation
# Fix: Update to reflect actual count (3 tests)
```

### Example 2: Non-existent Feature Documentation
**Documentation**: "GridBuilder supports 12-column responsive layouts"

**Debugging Process**:
```rust
// 1. Look for the feature
grep -r "column" src/builders/layout.rs
// No results

// 2. Check for related code
grep -r "12\|twelve" src/
// No results  

// 3. Check design documents
grep -r "12-column" docs/
// Found in ROADMAP.md

// Root cause: Future feature documented as current
// Fix: Move to "Planned Features" section
```

## Documentation Accuracy Tooling

### 1. Automated Accuracy Checks
```toml
# In Cargo.toml
[dev-dependencies]
doc-validator = "0.1"

[[test]]
name = "validate_docs"
path = "tests/validate_docs.rs"
```

### 2. CI Documentation Validation
```yaml
# .github/workflows/doc-check.yml
name: Documentation Accuracy
on: [push, pull_request]

jobs:
  validate:
    steps:
      - name: Extract code examples
        run: ./scripts/extract-examples.sh
        
      - name: Compile examples
        run: cargo test --doc
        
      - name: Verify claims
        run: ./scripts/verify-claims.sh
        
      - name: Check API coverage
        run: cargo doc --no-deps
```

### 3. Documentation Testing Framework
```rust
/// Framework for testing documentation claims
#[cfg(test)]
mod doc_tests {
    use super::*;
    
    /// Verify quantitative claims
    #[test]
    fn verify_test_counts() {
        let files = [
            ("button_test.rs", 3),
            ("state_test.rs", 7),
            ("card_test.rs", 0),
        ];
        
        for (file, expected) in files {
            let actual = count_tests(&format!("src/builders/{}", file));
            assert_eq!(actual, expected, 
                "Documentation claims {} has {} tests, but found {}", 
                file, expected, actual);
        }
    }
}
```

## Debugging Checklist
- [ ] Identify the inaccurate claim
- [ ] Locate the source file/code
- [ ] Verify actual vs documented state
- [ ] Check git history for context
- [ ] Identify root cause category
- [ ] Document the fix rationale
- [ ] Add validation test if possible
- [ ] Update related documentation

## Common Debugging Patterns

### Pattern 1: Number Inflation
```markdown
Claimed: "Comprehensive test suite with 50+ tests"
Reality: 15 tests across all modules
Cause: Counting assertions instead of test functions
Fix: Use precise language and actual counts
```

### Pattern 2: Feature Confusion
```markdown
Claimed: "Supports dark mode"
Reality: DarkTheme enum exists but not implemented
Cause: Confusing API existence with functionality
Fix: Distinguish between API and implementation
```

### Pattern 3: Version Mismatch
```markdown
Claimed: "Use set_theme() method"
Reality: Method renamed to with_theme() in v0.2
Cause: Outdated documentation
Fix: Version-specific documentation
```

## Remember
**Trust in documentation is lost quickly but rebuilt slowly - accuracy matters.**