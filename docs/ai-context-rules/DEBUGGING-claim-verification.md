# DEBUGGING RULE: Claim Verification Process

## Problem This Prevents
Making false claims about codebase features, leading to documentation that misleads users and wastes their time.

## The Rule
**Every claim in documentation must be verifiable with a specific command. If you can't prove it, don't claim it.**

## Red Flags That Indicate Unverified Claims

### 1. Vague Quantifiers
```markdown
# 🚨 RED FLAG - Unverifiable
- "Comprehensive testing infrastructure"
- "Extensive test coverage"
- "High-performance class generation"
- "Advanced testing utilities"

# ✅ GOOD - Specific and verifiable
- "157 unit tests across 8 test files"
- "Builder API testing only (no integration tests)"
- "Basic helper functions: create_*_styles()"
```

### 2. Assumed Standard Patterns
```markdown
# 🚨 RED FLAG - Assumptions
- "Two-layer testing architecture"
- "Integration test suite"
- "Performance benchmarks"
- "Visual regression testing"

# ✅ GOOD - Verified reality
- "Single-layer builder testing (verified with ls src/)"
- "No integration tests (no tests/ directory found)"
- "No performance measurement (no #[bench] attributes)"
```

### 3. Aspirational Language
```markdown
# 🚨 RED FLAG - Future state presented as current
- "The system provides comprehensive testing"
- "Tests ensure bulletproof reliability"
- "Performance optimization included"

# ✅ GOOD - Current state clearly marked
- "Current testing: Builder APIs only"
- "Future: Integration testing (not implemented)"
- "Performance: No measurement configured"
```

## Claim Verification Protocol

### Step 1: Challenge Every Claim
For each claim, ask: "What command proves this?"

```bash
# Claim: "100% test coverage"
# Verification: Find coverage measurement
find . -name "*.toml" -exec grep -l "coverage\|tarpaulin" {} \;
ls -la .coverage/ coverage/ 2>/dev/null || echo "No coverage data"

# Claim: "180+ tests"
# Verification: Count actual tests
grep -r "#\[test\]" . --include="*.rs" | wc -l

# Claim: "Advanced test utilities"
# Verification: Find utility functions
find . -name "*util*" -o -name "*helper*" | grep -i test
grep -r "assert_contains\|test_helper" . --include="*.rs"
```

### Step 2: Document Verification Commands
Always include the commands used to verify claims:

```markdown
## Test Infrastructure

### Current Implementation
- **157 unit tests** (verified: `grep -r "#\[test\]" . --include="*.rs" | wc -l`)
- **8 test files** (verified: `find . -name "*test*.rs" | wc -l`)
- **No integration tests** (verified: `ls tests/ 2>/dev/null || echo "No tests dir"`)

### Verification Commands
```bash
# Test count verification
grep -r "#\[test\]" . --include="*.rs" | wc -l

# Architecture verification
find . -name "mod.rs" -exec grep -l "mod tests" {} \;
```

## Real Example: Jupiter Design System Debug Process

### Initial False Claims
```markdown
❌ "Two-layer testing strategy (Pattern + Builder)"
❌ "180+ tests with 100% coverage"
❌ "Comprehensive integration testing"
❌ "Advanced test utilities and helpers"
```

### Verification Process
```bash
# Claim: Two-layer testing
$ find . -name "*pattern*test*" -o -name "*integration*"
# Result: No files found - claim is false

# Claim: 180+ tests
$ grep -r "#\[test\]" . --include="*.rs" | wc -l
157
# Result: 157 tests, not 180+ - claim is false

# Claim: Comprehensive integration testing
$ ls tests/ 2>/dev/null || echo "No tests directory"
No tests directory
# Result: No integration tests exist - claim is false

# Claim: Advanced test utilities
$ find . -name "*util*" | grep -i test
# Result: No test utilities found - claim is false
```

### Corrected Claims
```markdown
✅ "Single-layer builder testing (verified: no pattern tests found)"
✅ "157 unit tests (verified: grep -r '#\[test\]' . | wc -l)"
✅ "No integration testing (verified: tests/ directory doesn't exist)"
✅ "Basic helpers only (verified: only create_*_styles() functions found)"
```

## Common Claim Categories to Verify

### 1. Architecture Claims
```bash
# Verify test organization
find . -type d -name "*test*" | sort
grep -r "mod.*test" . --include="*.rs" | cut -d: -f1 | sort | uniq

# Verify module structure
ls -la src/ | grep -E "(test|spec|integration)"
```

### 2. Performance Claims
```bash
# Verify benchmarks exist
grep -r "#\[bench\]" . --include="*.rs" | wc -l

# Verify performance measurement
find . -name "*.toml" -exec grep -l "criterion\|bench" {} \;

# Check for timing in CI
grep -r "time\|performance" .github/workflows/ 2>/dev/null
```

### 3. Coverage Claims
```bash
# Verify coverage tooling
cargo tarpaulin --version 2>/dev/null || echo "No coverage tool"

# Check for coverage files
find . -name ".coverage*" -o -name "coverage.*" -o -name "*.lcov"

# Verify CI coverage
grep -r "coverage\|codecov" .github/workflows/ 2>/dev/null
```

### 4. Feature Claims
```bash
# Verify specific features exist
grep -r "feature_name" . --include="*.rs" | head -5

# Check for feature flags
grep -r "cfg(feature" . --include="*.rs" | head -5

# Verify dependencies
grep -A 20 "\[dependencies\]" Cargo.toml | grep "feature_name"
```

## Debugging Inaccurate Documentation

When documentation doesn't match reality:

### 1. Identify the Discrepancy
```bash
# What was claimed vs what exists
echo "Claimed: Two-layer testing"
echo "Reality: $(find . -name "*pattern*test*" | wc -l) pattern tests found"
```

### 2. Find the Source of Truth
```bash
# Check actual implementation
find . -name "*.rs" -exec grep -l "pattern\|builder" {} \; | head -10

# Read the actual code
head -50 src/builders/text.rs | grep -E "struct|impl|mod"
```

### 3. Update Documentation with Facts
```markdown
## Corrected Architecture

### What Actually Exists
- Builder APIs only (TextStyles, CardStyles, etc.)
- Co-located unit tests (*_test.rs files)
- Basic helper functions per test file

### What Was Incorrectly Claimed
- ❌ Pattern layer (no separate pattern tests found)
- ❌ Integration testing (no tests/ directory)
- ❌ Shared test utilities (each file has its own helpers)
```

## Time Impact
- Verifying claims upfront: 5-10 minutes
- Debugging false claims later: 1-2 hours
- User trust damage: Significant

## Key Debugging Commands

### Quick Reality Check
```bash
# Count everything
find . -name "*.rs" | wc -l  # Total files
find . -name "*test*.rs" | wc -l  # Test files
grep -r "#\[test\]" . --include="*.rs" | wc -l  # Test functions
find . -type d | grep -E "(test|spec|integration)" | wc -l  # Test directories
```

### Architecture Verification
```bash
# Show actual structure
tree src/ -I "target" | head -20

# Show test organization
find . -name "*test*.rs" -exec basename {} \; | sort

# Show module patterns
grep -r "mod " . --include="*.rs" | grep -v "target" | head -10
```

## Remember
**"Trust but verify" - every claim needs a command that proves it.**

The goal is documentation that accurately reflects reality, not documentation that makes the project look better than it is.