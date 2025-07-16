# Discovery-First Development Rule

## Overview
**Always perform exhaustive codebase discovery before writing any code or documentation.** This rule prevents hours of rework caused by incomplete understanding of existing systems.

## When It Applies
- Starting any new feature or documentation task
- Working with unfamiliar codebases or modules
- Responding to "implement X" or "document Y" requests
- Beginning any task that touches existing code

## Why It's Critical
Incomplete discovery leads to:
- **Cascading failures**: Wrong assumptions compound into larger mistakes
- **Wasted effort**: Code that duplicates existing functionality
- **Breaking changes**: Modifications that violate established patterns
- **Documentation drift**: Docs that don't match actual implementation

## Specific Actions to Take

### 1. Exhaustive Pattern Search
```bash
# Count occurrences to understand scope
grep -r "FeatureName" . --include="*.rs" | wc -l

# Find all implementations
rg "impl.*FeatureName" --type rust

# Discover usage patterns
rg "FeatureName::" --type rust -A 2 -B 2

# Check for builder patterns
rg "FeatureNameBuilder|with_|build\(\)" --type rust
```

### 2. Discover Hidden Systems
```bash
# Find related modules
find . -name "*.rs" -exec grep -l "FeatureName" {} \;

# Check for trait implementations
rg "impl.*for.*FeatureName" --type rust

# Look for macro usage
rg "#\[derive.*FeatureName|feature_name!" --type rust
```

### 3. Validate Test Coverage
```bash
# Find all test files
find . -name "*test*.rs" -o -name "*_test.rs"

# Check test completeness
rg "fn test_" --type rust | grep -i feature

# Look for integration tests
find tests examples -name "*.rs" -exec grep -l "FeatureName" {} \;
```

## Examples from Today's Work

### Example 1: Color System Discovery Failures
**Initial assumption**: "Simple Color enum with basic variants"
```rust
// What we thought existed
enum Color {
    Primary, Secondary, Success, Warning, Error
}
```

**Reality after proper discovery**:
```rust
// What actually existed
enum Color {
    Primary, Secondary, Success, Warning, Error, Info,
    Background, Surface, Muted, Accent, Destructive,
    Ghost, Link, // ... 19 total variants
}

// Plus hidden features
struct ButtonStyleBuilder { /* gradient support */ }
type ColorClasses = HashMap<String, String>; // 233 references
```

### Example 2: Missing Dependencies
**Failed approach**: Started documenting CSS classes
```rust
colors.text() // Returns "text-primary"
```

**Proper discovery revealed**:
- Custom jupiter-navy colors requiring Tailwind config
- CSS prefix system with brand-specific naming
- Build-time class generation requirements

## Consequences of Not Following

### Time Impact
- **Initial work**: 30 minutes
- **Rework after discovery**: 2+ hours
- **Total time wasted**: 4x the original estimate

### Quality Impact
- Incomplete documentation missing 50% of features
- Test suite bugs (missing Color::Accent variant)
- User confusion from undocumented requirements

### Trust Impact
- Multiple "are you sure?" validations required
- Loss of confidence in initial analysis
- Frustration from repeated corrections

## Implementation Checklist

Before starting any development task:

- [ ] Run comprehensive searches for the feature/module name
- [ ] Count total references to understand scope
- [ ] Check examples/ directory for real usage patterns
- [ ] Validate test files match implementation
- [ ] Look for builder patterns and advanced APIs
- [ ] Search for configuration requirements
- [ ] Check for macro-generated code
- [ ] Review integration examples

## Red Flags Requiring Deeper Discovery

1. **Too few search results** - You're missing something
2. **No examples found** - Check different directories
3. **Simple implementation** - Look for builders/factories
4. **Standard patterns only** - Search for custom prefixes
5. **Clean test suite** - Verify against actual implementation

## Remember

> "The codebase is always more complex than it appears. Every 'simple' feature has hidden depths."

**Time spent on discovery is never wasted. Time spent on rework always is.**