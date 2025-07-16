# Critical Rule: Documentation Validation Checklist

## Overview
When creating technical documentation, especially for design systems or APIs, thorough validation against the actual implementation is critical. Today's color system documentation required 4 separate validation passes, each revealing critical missing information.

## The Problem
Initial documentation often misses:
- Hidden implementation details
- Configuration requirements
- Edge cases and special patterns
- Test suite bugs
- Undocumented dependencies

## Validation Process

### 1. Source Code Verification
```bash
# Find ALL references to the feature
grep -r "ColorProvider" . --include="*.rs" --include="*.ts" --include="*.js"
grep -r "Color::" . --include="*.rs" | wc -l  # Count usage

# Find unique patterns
grep -r "jupiter-" . --include="*.rs" | cut -d: -f2 | sort -u
```

### 2. Test Suite Analysis
**Always read the actual test files!**

Example bug found today:
```rust
// color_test.rs - Missing enum variant in test
let colors = [
    Color::Primary,
    Color::Secondary,  // ❌ Color::Accent missing here!
    Color::Success,
    // ... 15 more variants
];
assert_eq!(colors.len(), 18); // Should be 19!
```

### 3. Usage Pattern Discovery
```bash
# Find real usage in examples
find examples -name "*.rs" -exec cat {} \; | grep -A5 -B5 "Color::"

# Find builder patterns
grep -r "ButtonStyles" . --include="*.rs" -A 10
```

### 4. Configuration Dependencies
Look for references to external systems:
```bash
# Today's miss: Tailwind config requirement
grep -r "jupiter-blue-500" .  # Found CSS class
# But no Tailwind config exists to define it!
```

### 5. Edge Case Hunting
```bash
# Complex patterns often hide in specific files
grep -r "gradient" . --include="*.rs"
# Found: from-jupiter-navy-900/80 to-jupiter-blue-900/80

# Selection states
grep -r "ring-" . --include="*.rs"  
# Found: ring-jupiter-blue-300 (undocumented)
```

## Critical Validation Checklist

### Before First Draft
- [ ] Read the main implementation file completely
- [ ] Read the test file completely
- [ ] Run the examples
- [ ] Search for hardcoded values

### During Documentation
- [ ] Every enum variant is listed
- [ ] Every public method is documented
- [ ] Configuration requirements are explicit
- [ ] Usage examples compile and run

### Final Validation
- [ ] grep for all documented values - do they exist?
- [ ] grep for all existing values - are they documented?
- [ ] Do the examples match actual usage patterns?
- [ ] Are there builder patterns or convenience functions?

## Lesson from Today
What seemed like a simple color system had:
- 19 tokens (not 18 as tests claimed)
- 4 Jupiter color families (not 3)
- Required Tailwind configuration (not standalone)
- Selection ring colors (not just standard colors)
- Branded gradients with opacity
- ButtonStyles builder pattern

Each validation pass revealed critical information that would have caused user implementation failures.

## Command Patterns for Validation

```bash
# Full validation sweep
echo "=== Checking enum variants ==="
grep -r "enum Color" . --include="*.rs" -A 30

echo "=== Checking test coverage ==="
grep -r "Color::" . --include="*test.rs" | sort | uniq -c

echo "=== Checking unique values ==="
grep -r "jupiter-" . --include="*.rs" | grep -v test | cut -d'"' -f2 | sort -u

echo "=== Checking usage patterns ==="
find . -name "*.rs" -exec grep -l "ColorProvider" {} \; | xargs -I {} echo "File: {}"

echo "=== Checking for TODOs/FIXMEs ==="
grep -r "TODO\|FIXME" . --include="*.rs" | grep -i color
```

## Remember
**Never trust initial understanding**. The codebase always has more complexity than it initially appears. Validate thoroughly, then validate again.