# Comprehensive Search Patterns Rule

## Context
Superficial searches miss critical information. Systematic search patterns ensure complete discovery.

## The Rule
**Use progressive search patterns from basic to exhaustive discovery:**

### Level 1: Basic Discovery
```bash
# Quick overview
find . -name "*.rs" -type f | head -20
rg "pub struct|pub enum|pub trait" --type rust --count
```

### Level 2: Feature Discovery
```bash
# Find all public APIs
rg "^pub " --type rust -n | head -50

# Find all implementations
rg "impl.*\{" --type rust -n

# Find all tests
rg "#\[test\]" --type rust -n
```

### Level 3: Deep Pattern Discovery
```bash
# Find builders and patterns
rg "Builder|builder" --type rust -i -n
rg "new\(\)|default\(\)" --type rust -n

# Find trait implementations
rg "impl.*for" --type rust -n

# Find error types
rg "Error|Result" --type rust -n
```

### Level 4: Complete System Analysis
```bash
# Statistical analysis
rg "pub " --type rust --stats
rg "impl " --type rust --stats
rg "fn " --type rust --stats

# Feature usage analysis
rg "use.*::" --type rust | cut -d: -f1 | sort | uniq -c | sort -nr
```

## Design System Specific Patterns

### Color System Discovery
```bash
# Find all color-related code
rg "color|Color" --type rust -i --stats

# Find color definitions
rg "Color::|rgb|hsl|#[0-9a-f]{3,6}" --type rust -i

# Find color usage in tests
rg "Color::" --type rust tests/
```

### Component Discovery
```bash
# Find all component builders
rg "styles\(|_styles|Builder" --type rust -i

# Find component variants
rg "enum.*\{" --type rust -A 20 | grep -E "(Primary|Secondary|Success|Error)"

# Find component states
rg "State|state|Hover|Focus|Active" --type rust -i
```

### Theme Discovery
```bash
# Find theme implementations
rg "theme|Theme" --type rust -i --stats

# Find theme providers
rg "Provider|provider" --type rust -i

# Find theme customization
rg "custom|Custom|override|Override" --type rust -i
```

## Example from Today

**Basic Search (Level 1):**
```bash
rg "Color" --type rust
# Found: Basic Color enum usage
```

**Missed with Basic Search:**
- Color::Accent variant (only in tests)
- Gradient color functions
- Color mixing utilities
- Theme color mappings

**Comprehensive Search (Level 4):**
```bash
# Complete color system analysis
rg "Color::" --type rust -A 2 -B 2 | grep -v "^--$" | head -50
rg "impl.*Color" --type rust -A 10
rg "mod.*color|color.*mod" --type rust -i
find . -name "*color*" -type f
```

**Found with Comprehensive Search:**
- All color variants including test-only ones
- Color utility functions
- Color validation logic
- Color conversion helpers

## Search Strategy by Task

### Documentation Task
```bash
# 1. Find all public APIs
rg "^pub " --type rust -n

# 2. Find all examples
rg "example|Example" --type rust -i -n

# 3. Find all tests
find . -name "*test*" -type f

# 4. Find all documentation
find . -name "*.md" -type f | grep -v target
```

### Bug Investigation
```bash
# 1. Find error messages
rg "Error|panic|unwrap|expect" --type rust -n

# 2. Find test failures
rg "assert|should|fail" --type rust -i -n

# 3. Find workarounds
rg "TODO|FIXME|HACK|workaround" --type rust -i -n
```

### Feature Development
```bash
# 1. Find similar features
rg "similar_feature_name" --type rust -i -n

# 2. Find patterns to follow
rg "impl.*for.*\{" --type rust -n

# 3. Find test patterns
rg "test.*similar" --type rust -i -n
```

## Time Investment Guidelines

- **Basic Search:** 2-3 minutes for quick overview
- **Feature Discovery:** 5-10 minutes for moderate depth
- **Deep Pattern Discovery:** 10-15 minutes for thorough investigation
- **Complete Analysis:** 20-30 minutes for comprehensive understanding

## Key Principle
Invest more time in search patterns upfront to avoid multiple revision cycles. A 20-minute comprehensive search saves hours of documentation revisions.