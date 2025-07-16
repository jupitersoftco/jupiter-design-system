# Workflow Rule: Documentation Audit Workflow

## Problem This Prevents
- **Documentation drift** where docs no longer match implementation
- **Accumulated inaccuracies** that compound over time
- **Developer confusion** from outdated information
- **Wasted investigation time** on documented but non-existent features

## Time/Effort Saved
- Saves 3-4 hours per developer onboarding
- Prevents 1-2 hours of confusion per documentation discrepancy
- Eliminates need for emergency documentation updates

## Actionable Workflow Steps

### 1. Pre-Audit Preparation
```bash
# Create audit workspace
mkdir -p docs/audit/$(date +%Y-%m-%d)
cd docs/audit/$(date +%Y-%m-%d)

# Generate current state snapshot
find ../../src -name "*.rs" > source_files.txt
find ../../src -name "*test*.rs" > test_files.txt
grep -r "pub struct\|pub enum\|pub trait" ../../src > public_api.txt
```

### 2. Systematic Documentation Review

#### Phase 1: Inventory Check
```markdown
## Documentation Inventory Checklist
- [ ] List all documentation files
- [ ] Map docs to source modules
- [ ] Identify orphaned documentation
- [ ] Find undocumented modules
```

#### Phase 2: Claim Verification
```bash
# For each documentation claim about tests:
claim="Button has 5 tests"
file="button_test.rs"
actual=$(grep -c "#\[test\]" src/builders/$file)
echo "Claim: $claim | Actual: $actual tests in $file"
```

#### Phase 3: Cross-Reference Matrix
```markdown
| Documentation File | Source Module | Claims | Verified | Discrepancies |
|-------------------|---------------|---------|----------|---------------|
| button.md | builders/button.rs | 5 tests | 3 tests | Overclaimed by 2 |
| card.md | builders/card.rs | "tests" | 0 tests | No tests exist |
```

### 3. Discrepancy Resolution

#### For Each Discrepancy:
1. **Classify the error type:**
   - Outdated information
   - Aspirational documentation
   - Factual error
   - Missing context

2. **Determine the fix:**
   ```markdown
   ## Discrepancy: "Card has interaction tests"
   **Type**: Aspirational documentation
   **Reality**: card_test.rs is empty
   **Fix**: Update to "Card tests are planned but not yet implemented"
   ```

3. **Apply the correction:**
   ```bash
   # Document the change
   echo "Fixed: card.md line 45 - clarified test status" >> corrections.log
   ```

## Examples from Today's Session

### Example 1: Test Documentation Audit
**Process:**
```bash
# 1. Found claim in TESTING.md
"Each builder has comprehensive test coverage"

# 2. Verified actual state
$ for f in src/builders/*_test.rs; do
    echo "$f: $(grep -c '#\[test\]' $f) tests"
  done

button_test.rs: 3 tests
card_test.rs: 0 tests
layout_test.rs: 0 tests
state_test.rs: 7 tests

# 3. Updated documentation
"Test coverage varies by builder:
- button: 3 tests (basic coverage)
- state: 7 tests (comprehensive)
- card, layout: 0 tests (pending implementation)"
```

### Example 2: API Documentation Verification
**Found:** "GridBuilder supports 12-column layouts"
**Verification:**
```rust
// Checked src/builders/layout.rs
// Found: GridBuilder exists but column support not implemented
// No tests for grid functionality
```
**Correction:** "GridBuilder structure exists but column support is not yet implemented"

## Audit Automation Script
```bash
#!/bin/bash
# doc-audit.sh - Documentation audit automation

echo "Documentation Audit Report - $(date)"
echo "===================================="

# Check test claims
echo -e "\n## Test Coverage Verification"
for doc in docs/**/*.md; do
    # Extract test claims
    grep -n "test\|Test" "$doc" | while read -r line; do
        # Extract numbers from claims
        if echo "$line" | grep -E "[0-9]+ tests?"; then
            echo "Found claim in $doc: $line"
        fi
    done
done

# Verify public API documentation
echo -e "\n## Public API Documentation Check"
grep -h "pub struct\|pub enum" src/**/*.rs | while read -r api; do
    name=$(echo "$api" | awk '{print $3}')
    if ! grep -r "$name" docs/ > /dev/null; then
        echo "Undocumented: $api"
    fi
done

# Check for outdated examples
echo -e "\n## Example Code Verification"
find docs -name "*.md" -exec grep -l "```rust" {} \; | while read -r file; do
    echo "Checking examples in: $file"
    # Extract and verify rust code blocks
done
```

## Audit Schedule Template
```markdown
## Documentation Audit Schedule

### Weekly Quick Audit (15 mins)
- [ ] Verify test counts in main modules
- [ ] Check for new undocumented APIs
- [ ] Update "last verified" dates

### Monthly Deep Audit (2 hours)
- [ ] Full claim verification
- [ ] Example code compilation check
- [ ] Cross-reference all documentation
- [ ] Update architecture diagrams

### Per-Release Audit (4 hours)
- [ ] Complete documentation inventory
- [ ] Verify all public API docs
- [ ] Test all example code
- [ ] Update migration guides
```

## Quality Metrics
```markdown
## Documentation Quality Score
- Accuracy: X/Y claims verified correctly
- Coverage: X% of public APIs documented  
- Freshness: X% updated in last 30 days
- Examples: X% with working code samples
```

## Red Flags During Audit
1. Documentation without "last updated" dates
2. Absolute statements without verification
3. Example code that doesn't compile
4. Test counts that don't match reality
5. Features documented but not implemented

## Remember
**An audit is only valuable if discrepancies are fixed immediately.**