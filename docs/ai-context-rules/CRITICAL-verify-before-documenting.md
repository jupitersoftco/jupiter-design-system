# CRITICAL RULE: Verify Before Documenting

## Problem This Prevents
Writing aspirational documentation that doesn't match implementation reality, leading to user confusion and 2-3 hours of rework.

## The Rule
**ALWAYS verify actual implementation before writing ANY documentation. Never assume standard patterns exist.**

## Required Verification Protocol

### Step 1: Count and Measure
```bash
# Count test files
find . -name "*test*" -type f | wc -l

# Count actual tests
grep -r "#\[test\]" . --include="*.rs" | wc -l

# For other languages:
grep -r "test\(" . --include="*.js" | wc -l  # JavaScript
grep -r "@Test" . --include="*.java" | wc -l  # Java
```

### Step 2: Verify Architecture
```bash
# Check if assumed directories exist
ls -la src/tests/ src/integration/ 2>/dev/null || echo "Directories don't exist"

# Verify module structure
grep -r "mod tests" . --include="*.rs" | head -20
```

### Step 3: Validate Patterns
```bash
# Check for utilities/helpers
find . -name "*util*" -o -name "*helper*" | grep -i test

# Verify specific patterns exist
grep -r "test_utils::" . --include="*.rs"
grep -r "assert_contains_all" . --include="*.rs"
```

## Real Example: Jupiter Design System Failure

### What I Documented (Wrong)
```markdown
Total: 180+ tests ✅
Architecture: Two-layer testing (Pattern + Builder)
Integration Tests: Comprehensive cross-pattern testing
Test Utilities: Advanced assertion helpers
```

### What Actually Existed
```markdown
Total: 157 tests
Architecture: Single-layer builder testing only
Integration Tests: None
Test Utilities: Basic create_*_styles() helpers only
```

### Impact
- User asked 3 times: "Are you sure you've been thorough?"
- Complete documentation rewrite required
- Lost credibility with repeated corrections

## Prevention Checklist

Before writing documentation:
- [ ] Run actual count commands and record output
- [ ] Open and read at least 3 test files completely
- [ ] Verify every architectural claim with `ls` or `find`
- [ ] Test that code examples actually compile
- [ ] Check if utilities/helpers actually exist

## Documentation Template

Always structure documentation with verified sections:

```markdown
# Feature Documentation

## Current Implementation (Verified)
[Only what you've confirmed exists]
<!-- Verification command: grep -r "feature" . -->

## Not Yet Implemented
[What doesn't exist but might be expected]

## Future Roadmap
[Planned improvements clearly marked as future]
```

## Time Impact
- Following this rule: 10 minutes of verification
- Not following this rule: 2-3 hours of rework
- User frustration: Significant
- Credibility damage: High

## Key Commands for Different Languages

### Rust
```bash
cargo test --no-run 2>&1 | grep -c "test result"
find . -name "*.rs" -exec grep -l "#\[test\]" {} \;
```

### JavaScript/TypeScript
```bash
find . -name "*.test.js" -o -name "*.spec.js" | wc -l
grep -r "describe\|it\(" . --include="*.js" | wc -l
```

### Python
```bash
find . -name "test_*.py" -o -name "*_test.py" | wc -l
grep -r "def test_" . --include="*.py" | wc -l
```

## Remember
**"Documentation should describe reality, not aspirations."**

Every claim must be verifiable with a command. If you can't verify it, don't document it as existing.