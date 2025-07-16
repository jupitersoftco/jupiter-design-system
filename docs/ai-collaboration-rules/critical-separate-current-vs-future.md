# Critical Rule: Separate Current vs Future State

## Problem This Prevents
- **Confusion** between what exists now vs what is planned
- **False expectations** about available functionality
- **Wasted development time** looking for non-existent features
- **Planning conflicts** when future state is mistaken for current state

## Time/Effort Saved
- Saves 1-2 hours of searching for non-existent implementations
- Prevents incorrect dependency planning
- Avoids rework when assumptions prove false

## Actionable Steps

### 1. Use Clear Temporal Language
```markdown
# Good: Clear separation
## Current State (Verified)
- Button component has 3 implemented tests
- Card builder exists but has no tests

## Future State (Planned)
- Button component will need accessibility tests
- Card builder should have rendering tests

# Bad: Ambiguous
- Button component has comprehensive test coverage
- Card builder tests rendering and interactions
```

### 2. Mark Implementation Status Explicitly
```rust
// Good: Clear status markers
/// Status: IMPLEMENTED
/// Tests: 3/3 passing
pub struct Button { ... }

/// Status: PLANNED
/// Tests: 0 (not yet implemented)
/// TODO: Add grid layout tests
pub struct GridLayout { ... }
```

### 3. Use Verification Timestamps
```markdown
## Test Coverage (Verified: 2024-01-16)
- Button: 3 tests ✓
- Card: 0 tests (planned for Q1)
- Layout: 0 tests (empty test file exists)
```

## Examples from Today's Session

### Example 1: Layout System Documentation
**Wrong Approach:**
```markdown
"The layout system includes:
- Flexbox layouts with tests
- Grid layouts with tests
- Responsive utilities with tests"
```

**Correct Approach:**
```markdown
"The layout system (as of 2024-01-16):
## Implemented:
- FlexboxBuilder struct with methods
- GridBuilder struct with methods
- Responsive enum definition

## Not Yet Implemented:
- Tests for any layout functionality (layout_test.rs is empty)
- Responsive behavior tests
- Integration with theme system"
```

### Example 2: Test Module Claims
**Wrong Approach:**
```markdown
"Each builder module has a corresponding test module"
```

**Correct Approach:**
```markdown
"Test file status (verified by reading files):
## Separate test files exist:
- button_test.rs (3 tests)
- card_test.rs (0 tests - empty)
- layout_test.rs (0 tests - empty)

## No test files found for:
- interactive.rs
- typography patterns"
```

## Documentation Templates

### For Features
```markdown
## [Feature Name]
**Status**: [Implemented|Partial|Planned]
**Verified**: [Date]

### Currently Available:
- [List actual implementations]

### Planned/TODO:
- [List future additions]

### Not Yet Implemented:
- [List missing pieces]
```

### For Tests
```markdown
## Test Coverage for [Module]
**File**: [filename]
**Verified**: [Date] by reading source

### Existing Tests (count: X):
1. test_name() - what it tests
2. test_name() - what it tests

### Missing Test Coverage:
- [Feature] - no tests yet
- [Feature] - planned for [timeframe]
```

## Verification Commands
```bash
# Check what actually exists
ls -la src/**/*test*.rs

# Verify test count
find . -name "*.rs" -exec grep -l "#\[test\]" {} \;

# Check for TODO/PLANNED markers
grep -r "TODO\|PLANNED\|FUTURE" src/
```

## Red Flags to Avoid
1. Documentation without dates or verification notes
2. Mixing "should have" with "does have"
3. Using future tense ambiguously ("will test" vs "tests")
4. Omitting implementation status markers

## Remember
**Always distinguish between aspirational design and current reality.**