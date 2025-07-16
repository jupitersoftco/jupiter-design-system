# Workflow Rule: Exhaustive Search Patterns for Design Systems

## Overview
Design systems have interconnected components that require specific search patterns to discover completely. Standard searches miss critical relationships and patterns.

## Search Pattern Progression

### Level 1: Basic Search
```bash
# What most people do first
grep -r "ColorProvider" . --include="*.rs"
```
**Finds**: Direct usage
**Misses**: Builder patterns, trait implementations, examples

### Level 2: Pattern-Based Search  
```bash
# Broader pattern matching
grep -r "Color\|color" . --include="*.rs" -i
grep -r "provider\|Provider" . --include="*.rs"
```
**Finds**: Related code
**Misses**: CSS classes, prefixed values

### Level 3: Value Extraction
```bash
# Extract all unique values
grep -r "jupiter-\|brand-\|custom-" . --include="*.rs" | 
  cut -d'"' -f2 | 
  sort -u
```
**Finds**: All custom prefixes
**Misses**: Usage context

### Level 4: Statistical Analysis
```bash
# Count and categorize
echo "Total Color:: usage: $(grep -r "Color::" . --include="*.rs" | wc -l)"
echo "Files using Color: $(grep -r "Color::" . --include="*.rs" | cut -d: -f1 | sort -u | wc -l)"
echo "Unique patterns: $(grep -r "Color::" . --include="*.rs" | grep -o "Color::[A-Za-z]*" | sort -u | wc -l)"
```

## Design System Specific Searches

### 1. Token Discovery
```bash
# Find all token types
grep -r "enum.*{" . --include="*.rs" -A 20 | grep -E "^\s+[A-Z][a-zA-Z]+,"

# Find token values  
grep -r "pub const\|const.*:" . --include="*.rs" | grep -i "color\|size\|space"
```

### 2. Builder Pattern Discovery
```bash
# Find builder structs
grep -r "Builder\|Styles" . --include="*.rs" | grep "struct"

# Find fluent methods
grep -r "fn.*self\)" . --include="*.rs" | grep -A 1 "Self"

# Find method chains
grep -r "\." . --include="*.rs" | grep -E "\.([a-z_]+\(\)\.){2,}"
```

### 3. CSS Class Generation
```bash
# Find format! patterns
grep -r "format!" . --include="*.rs" | grep -E "text-|bg-|border-"

# Find string concatenation
grep -r "push_str\|concat!\|+" . --include="*.rs" | grep -E "text-|bg-|border-"

# Find CSS class templates
grep -r '".*{.*}"' . --include="*.rs" | grep class
```

### 4. Trait Implementation Map
```bash
# Find all trait implementations
grep -r "impl.*for.*{" . --include="*.rs" | grep -v test

# Map trait to implementors
TRAIT="ColorProvider"
echo "=== $TRAIT implementors ==="
grep -r "impl.*$TRAIT.*for" . --include="*.rs" | cut -d' ' -f4
```

## Today's Search Evolution

### Initial Search (Incomplete)
```bash
grep -r "Color" src/core/color.rs
# Found: Basic enum and trait
```

### Second Pass (Better)
```bash
grep -r "Color::" . --include="*.rs"
# Found: 233 usages across many files
```

### Third Pass (Revealing)
```bash
grep -r "jupiter-" . --include="*.rs" | cut -d'"' -f2 | sort -u
# Found: jupiter-navy colors, ring classes, gradients
```

### Final Pass (Complete)
```bash
find examples -name "*.rs" -exec cat {} \;
# Found: ButtonStyles builder pattern, real usage examples
```

## The Exhaustive Search Protocol

### Step 1: Core Discovery
```bash
# Find main implementation
find . -name "*.rs" -path "*/core/*" -o -path "*/lib/*" | xargs grep -l "feature"

# Read primary files
cat src/core/feature.rs src/lib.rs
```

### Step 2: Usage Mapping
```bash
# Find all direct usage
grep -r "use.*feature\|Feature::" . --include="*.rs" > usage_map.txt

# Categorize by directory
cat usage_map.txt | cut -d: -f1 | xargs dirname | sort | uniq -c
```

### Step 3: Pattern Extraction
```bash
# Extract all patterns
grep -r "feature" . --include="*.rs" -o | sort | uniq -c | sort -nr > patterns.txt

# Find unique values
grep -r '"[^"]*feature[^"]*"' . --include="*.rs" -o | sort -u > values.txt
```

### Step 4: Example Analysis
```bash
# Never skip examples!
find examples -name "*.rs" | while read file; do
  echo "=== $file ==="
  cat "$file" | grep -A 10 -B 10 "feature"
done
```

### Step 5: Test Verification
```bash
# Compare implementation vs tests
echo "=== Implementation ==="
grep -r "pub fn\|pub trait\|pub struct\|pub enum" src --include="*.rs" | grep -v test | wc -l

echo "=== Test coverage ==="
grep -r "#\[test\]" src --include="*.rs" | wc -l
```

## Search Pattern Templates

### For Enums
```bash
ENUM="Color"
# Get all variants
grep -A 50 "enum $ENUM" . --include="*.rs" | grep -E "^\s+[A-Z]"

# Find all usage
grep -r "$ENUM::" . --include="*.rs" | grep -o "$ENUM::[A-Za-z]*" | sort -u

# Find match expressions
grep -r "match.*$ENUM" . --include="*.rs" -A 20
```

### For Builders
```bash
BUILDER="Styles"
# Find builder methods
grep -r "impl.*$BUILDER" . --include="*.rs" -A 50 | grep "pub fn"

# Find usage chains
grep -r "$BUILDER::new" . --include="*.rs" -A 5

# Find all method combinations
grep -r "\.$BUILDER" . --include="*.rs" | grep -o "\.[a-z_]*(" | sort -u
```

### For CSS Classes
```bash
# Find all CSS class patterns
grep -r "class" . --include="*.rs" | grep -o '"[^"]*"' | tr ' ' '\n' | grep -E "^(text|bg|border)-" | sort -u

# Group by prefix
grep -r "class" . --include="*.rs" | grep -o '"[^"]*"' | tr ' ' '\n' | grep -E "^(text|bg|border)-" | cut -d- -f1 | sort | uniq -c

# Find custom prefixes
grep -r "class" . --include="*.rs" | grep -o '"[^"]*"' | tr ' ' '\n' | grep -E "-[a-z]+-[0-9]+" | cut -d- -f2 | sort -u
```

## Completeness Verification

After exhaustive search, verify:

```bash
# 1. All public APIs documented?
grep -r "pub fn\|pub trait\|pub struct\|pub enum" src --include="*.rs" | grep -v test > public_api.txt

# 2. All patterns found?
grep -r "fn main\|example" examples --include="*.rs" -B 5 -A 20 > example_patterns.txt

# 3. All values captured?
find . -name "*.rs" -exec cat {} \; | grep -o '"[^"]*"' | sort -u > all_strings.txt

# 4. Cross-reference
echo "Checking if documentation covers all findings..."
```

## Key Lesson

**Exhaustive search is not optional for design systems.** They have:
- Hidden patterns (gradients, rings, states)
- Builder APIs not obvious from core types
- Configuration dependencies
- Test gaps

Today's documentation would have been 80% incomplete without exhaustive search patterns.