# WORKFLOW RULE: Documentation Audit Process

## Problem This Prevents
Shipping inaccurate documentation that leads to repeated user questions and credibility damage.

## The Rule
**Always audit documentation against actual codebase before presenting it to users.**

## Documentation Audit Workflow

### Phase 1: Pre-Audit Setup (1 minute)
```bash
# Create audit workspace
mkdir -p /tmp/doc-audit
cd /tmp/doc-audit

# Note the claims you're about to verify
echo "CLAIMS TO VERIFY:" > audit-checklist.txt
echo "- Test count: [number]" >> audit-checklist.txt
echo "- Architecture: [description]" >> audit-checklist.txt
echo "- Features: [list]" >> audit-checklist.txt
```

### Phase 2: Systematic Verification (5-10 minutes)
```bash
# 1. Count verification
echo "=== COUNT VERIFICATION ===" >> audit-results.txt
echo "Test files: $(find . -name "*test*.rs" | wc -l)" >> audit-results.txt
echo "Test functions: $(grep -r "#\[test\]" . --include="*.rs" | wc -l)" >> audit-results.txt
echo "Integration tests: $(find . -path "*/tests/*.rs" | wc -l)" >> audit-results.txt

# 2. Architecture verification
echo "=== ARCHITECTURE VERIFICATION ===" >> audit-results.txt
echo "Test directories: $(find . -type d -name "*test*" | wc -l)" >> audit-results.txt
echo "Test modules: $(grep -r "mod tests" . --include="*.rs" | wc -l)" >> audit-results.txt
echo "Test organization: $(ls -la src/ | grep -E "test|spec" | wc -l)" >> audit-results.txt

# 3. Feature verification
echo "=== FEATURE VERIFICATION ===" >> audit-results.txt
echo "Test utilities: $(find . -name "*util*" | grep -i test | wc -l)" >> audit-results.txt
echo "Helper functions: $(grep -r "fn.*helper\|fn.*assert_" . --include="*test*.rs" | wc -l)" >> audit-results.txt
echo "Benchmarks: $(grep -r "#\[bench\]" . --include="*.rs" | wc -l)" >> audit-results.txt
```

### Phase 3: Claim Validation (3-5 minutes)
```bash
# Compare claims vs reality
echo "=== CLAIM VALIDATION ===" >> audit-results.txt

# For each claim in your documentation:
# Claim: "180+ tests"
ACTUAL_TESTS=$(grep -r "#\[test\]" . --include="*.rs" | wc -l)
echo "Claim: 180+ tests | Actual: $ACTUAL_TESTS tests" >> audit-results.txt

# Claim: "Two-layer architecture"
PATTERN_TESTS=$(find . -name "*pattern*test*" | wc -l)
echo "Claim: Two-layer architecture | Actual: $PATTERN_TESTS pattern tests" >> audit-results.txt

# Claim: "Comprehensive utilities"
UTIL_FILES=$(find . -name "*util*" | grep -i test | wc -l)
echo "Claim: Comprehensive utilities | Actual: $UTIL_FILES utility files" >> audit-results.txt
```

### Phase 4: Example Verification (2-3 minutes)
```bash
# Test that code examples actually work
echo "=== EXAMPLE VERIFICATION ===" >> audit-results.txt

# Extract code examples from documentation
grep -A 10 -B 2 "```rust" your-documentation.md > examples.txt

# Check if imports exist
grep -r "use.*TextStyles" . --include="*.rs" > /dev/null && echo "✅ TextStyles import exists" || echo "❌ TextStyles import missing"

# Check if functions exist
grep -r "fn create_text_styles" . --include="*.rs" > /dev/null && echo "✅ create_text_styles exists" || echo "❌ create_text_styles missing"
```

## Real Example: Jupiter Design System Audit

### Pre-Audit Claims
```markdown
Total: 180+ tests ✅
Architecture: Two-layer testing (Pattern + Builder) ✅
Integration Tests: Comprehensive cross-pattern testing ✅
Test Utilities: Advanced assertion helpers ✅
```

### Audit Results
```bash
$ bash audit-script.sh
=== COUNT VERIFICATION ===
Test files: 8
Test functions: 157
Integration tests: 0

=== ARCHITECTURE VERIFICATION ===
Test directories: 0
Test modules: 8
Test organization: 0

=== FEATURE VERIFICATION ===
Test utilities: 0
Helper functions: 8
Benchmarks: 0

=== CLAIM VALIDATION ===
Claim: 180+ tests | Actual: 157 tests ❌
Claim: Two-layer architecture | Actual: 0 pattern tests ❌
Claim: Comprehensive utilities | Actual: 0 utility files ❌
```

### Post-Audit Corrections
```markdown
Total: 157 tests ✅ (verified: grep -r "#\[test\]" . | wc -l)
Architecture: Single-layer builder testing ✅ (verified: no pattern tests found)
Integration Tests: None ❌ (verified: no tests/ directory)
Test Utilities: Basic helpers only ✅ (verified: 8 create_*_styles functions)
```

## Audit Checklist Template

Create this checklist for every documentation project:

```markdown
# Documentation Audit Checklist

## Quantitative Claims
- [ ] Test count verified with command
- [ ] File count verified with command  
- [ ] Architecture verified with ls/find
- [ ] Feature count verified with grep

## Qualitative Claims
- [ ] All "comprehensive" claims backed by evidence
- [ ] All "advanced" claims verified with examples
- [ ] All "extensive" claims quantified
- [ ] All performance claims measured

## Code Examples
- [ ] All imports exist in codebase
- [ ] All functions exist in codebase
- [ ] All examples compile/run
- [ ] All file paths are correct

## User Experience
- [ ] Current vs future state clearly separated
- [ ] No aspirational language in current sections
- [ ] Working examples provided for all current features
- [ ] Verification commands included
```

## Automated Audit Script

Create a reusable audit script:

```bash
#!/bin/bash
# doc-audit.sh - Automated documentation audit

echo "=== DOCUMENTATION AUDIT REPORT ==="
echo "Date: $(date)"
echo "Project: $(basename $(pwd))"
echo

echo "=== COUNTS ==="
echo "Rust files: $(find . -name "*.rs" -not -path "*/target/*" | wc -l)"
echo "Test files: $(find . -name "*test*.rs" -not -path "*/target/*" | wc -l)"
echo "Test functions: $(grep -r "#\[test\]" . --include="*.rs" | wc -l)"
echo "Integration tests: $(find . -path "*/tests/*.rs" -not -path "*/target/*" | wc -l)"
echo

echo "=== ARCHITECTURE ==="
echo "Test directories: $(find . -type d -name "*test*" -not -path "*/target/*" | wc -l)"
echo "Test modules: $(grep -r "mod tests" . --include="*.rs" | wc -l)"
echo "Cargo.toml exists: $(ls Cargo.toml 2>/dev/null && echo "✅" || echo "❌")"
echo

echo "=== FEATURES ==="
echo "Test utilities: $(find . -name "*util*" -not -path "*/target/*" | grep -i test | wc -l)"
echo "Helper functions: $(grep -r "fn.*helper\|fn.*create_.*\|fn.*assert_" . --include="*test*.rs" | wc -l)"
echo "Benchmarks: $(grep -r "#\[bench\]" . --include="*.rs" | wc -l)"
echo "Coverage config: $(find . -name "tarpaulin.toml" -o -name ".coverage*" | wc -l)"
echo

echo "=== VALIDATION ==="
echo "Run this audit before publishing any documentation about this project."
```

## Language-Specific Audit Commands

### JavaScript/TypeScript
```bash
# Test audit
echo "Jest tests: $(find . -name "*.test.js" -o -name "*.spec.js" | wc -l)"
echo "Test functions: $(grep -r "it(\|test(" . --include="*.test.js" | wc -l)"
echo "Coverage config: $(ls jest.config.js coverage/ 2>/dev/null | wc -l)"

# Framework audit
echo "React components: $(find . -name "*.jsx" -o -name "*.tsx" | wc -l)"
echo "Storybook: $(ls .storybook/ 2>/dev/null && echo "✅" || echo "❌")"
echo "E2E tests: $(find . -name "*.e2e.js" -o -name "cypress/" -type d | wc -l)"
```

### Python
```bash
# Test audit
echo "Test files: $(find . -name "test_*.py" -o -name "*_test.py" | wc -l)"
echo "Test functions: $(grep -r "def test_" . --include="*.py" | wc -l)"
echo "Coverage config: $(ls .coveragerc setup.cfg 2>/dev/null | wc -l)"

# Framework audit
echo "Flask apps: $(grep -r "from flask" . --include="*.py" | wc -l)"
echo "Django apps: $(find . -name "settings.py" | wc -l)"
echo "Requirements: $(ls requirements*.txt 2>/dev/null | wc -l)"
```

## Time Impact
- Running audit: 10-15 minutes
- Fixing issues found by audit: 30-60 minutes
- User frustration from inaccurate docs: High
- Credibility damage from repeated errors: Significant

## Key Principle
**"Audit ruthlessly, ship confidently."**

The audit process transforms uncertain documentation into verified, trustworthy information that users can depend on.