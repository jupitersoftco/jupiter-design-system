# Workflow Rule: Comprehensive Search Pattern Sequences

## Overview
This rule provides specific command sequences for comprehensive codebase discovery. Design systems require systematic search patterns that go beyond simple grep commands to uncover hidden relationships, dependencies, and usage patterns.

## Core Search Pattern Library

### 1. Enum and Variant Discovery
```bash
# Find all enum definitions
grep -r "enum.*{" . --include="*.rs" | grep -v "test"

# Extract enum variants (handles multi-line)
ENUM_NAME="Color"
grep -A 50 "enum $ENUM_NAME" . --include="*.rs" | grep -E "^\s+[A-Z][a-zA-Z]*" | sed 's/,$//' | sort -u

# Find all enum usage patterns
grep -r "$ENUM_NAME::" . --include="*.rs" | grep -o "$ENUM_NAME::[A-Za-z]*" | sort | uniq -c | sort -nr

# Find match arms for enum
grep -r "match.*$ENUM_NAME" . --include="*.rs" -A 30 | grep -E "=>\|[A-Z][a-z]+"
```

### 2. Builder Pattern Discovery
```bash
# Find all builder structs
grep -r "Builder\|builder" . --include="*.rs" | grep -E "struct|impl" | grep -v test

# Extract builder method chains
grep -r "::new\|::default\|::builder" . --include="*.rs" -A 10 | grep -E "\.[a-z_]+\(" | sort -u

# Find fluent interface patterns
grep -r "fn.*mut self.*->" . --include="*.rs" | grep "Self"

# Real usage in examples
find examples -name "*.rs" -exec grep -l "Builder\|builder" {} \; | xargs cat | grep -B 2 -A 10 "builder"
```

### 3. CSS Class and Styling Patterns
```bash
# Extract all CSS classes from code
grep -r "class\|className" . --include="*.rs" | grep -o '"[^"]*"' | tr ' ' '\n' | grep -v '^$' | sort -u > all_classes.txt

# Categorize by prefix
cat all_classes.txt | grep -E "^(text|bg|border|ring|shadow|gradient)-" | cut -d- -f1 | sort | uniq -c

# Find custom prefixes (jupiter-, brand-, etc)
cat all_classes.txt | grep -E "^[a-z]+-[a-z]+-[0-9]" | cut -d- -f1 | sort -u

# Find dynamic class generation
grep -r "format!\|push_str\|concat!" . --include="*.rs" | grep -E "text-|bg-|border-"

# Extract color values
grep -r "Color::" . --include="*.rs" | grep -o '"[^"]*"' | grep -E "text-|bg-|border-" | sort -u
```

### 4. Trait Implementation Mapping
```bash
# Find all traits
grep -r "trait.*{" . --include="*.rs" | grep "pub" | cut -d' ' -f3 | sort -u > traits.txt

# Map implementations
while read trait; do
  echo "=== $trait implementations ==="
  grep -r "impl.*$trait.*for" . --include="*.rs" | grep -v test | cut -d' ' -f4
done < traits.txt

# Find blanket implementations
grep -r "impl<.*>.*for.*where" . --include="*.rs"

# Find derive macros
grep -r "#\[derive(" . --include="*.rs" | grep -o "([^)]*)" | tr ',' '\n' | sort | uniq -c
```

### 5. Public API Surface Mapping
```bash
# Extract all public items
grep -r "pub fn\|pub struct\|pub enum\|pub trait\|pub type\|pub const" src --include="*.rs" | grep -v test > public_api.txt

# Categorize by type
echo "=== API Surface ==="
echo "Functions: $(grep "pub fn" public_api.txt | wc -l)"
echo "Structs: $(grep "pub struct" public_api.txt | wc -l)"
echo "Enums: $(grep "pub enum" public_api.txt | wc -l)"
echo "Traits: $(grep "pub trait" public_api.txt | wc -l)"
echo "Types: $(grep "pub type" public_api.txt | wc -l)"
echo "Constants: $(grep "pub const" public_api.txt | wc -l)"

# Find most used APIs
grep -r "use.*::" . --include="*.rs" | cut -d: -f3 | sort | uniq -c | sort -nr | head -20
```

## Comprehensive Discovery Workflow

### Phase 1: Structure Discovery
```bash
# Map codebase structure
find . -name "*.rs" -type f | grep -v target | cut -d/ -f2-3 | sort -u

# Find main modules
find . -name "mod.rs" -o -name "lib.rs" | grep -v target | sort

# Identify feature boundaries
grep -r "pub mod" . --include="*.rs" | grep -v test
```

### Phase 2: Dependency Analysis
```bash
# Internal dependencies
grep -r "use crate::\|use super::" . --include="*.rs" | cut -d: -f3 | sort | uniq -c | sort -nr

# External dependencies
grep -r "use.*::" . --include="*.rs" | grep -v "crate::\|super::\|self::" | cut -d: -f3 | cut -d: -f1 | sort -u

# Feature flags
grep -r "#\[cfg(" . --include="*.rs" | grep -o "(.*)" | sort -u

# Conditional compilation
grep -r "cfg!\|cfg_attr!" . --include="*.rs"
```

### Phase 3: Pattern Extraction
```bash
# String literal patterns
find . -name "*.rs" -exec cat {} \; | grep -o '"[^"]*"' | sort | uniq -c | sort -nr > string_frequency.txt

# Common method names
grep -r "fn.*(" . --include="*.rs" | grep -o "fn [a-z_]*" | cut -d' ' -f2 | sort | uniq -c | sort -nr | head -20

# Error patterns
grep -r "Error\|error" . --include="*.rs" | grep -E "struct|enum|type" | sort -u

# Common patterns in examples
find examples -name "*.rs" -exec cat {} \; | grep -E "let|match|if let|for" | sort | uniq -c | sort -nr
```

## Today's Search Evolution Example

### Initial Naive Search
```bash
# What we started with
grep -r "ColorProvider" src/core/color.rs
# Found: Basic trait definition
```

### Progressive Discovery
```bash
# Round 2: Wider net
grep -r "Color" . --include="*.rs" | wc -l
# Found: 500+ references

# Round 3: Pattern extraction  
grep -r "Color::" . --include="*.rs" | grep -o "Color::[A-Za-z]*" | sort -u
# Found: 19 variants (not 18 as tests claimed)

# Round 4: CSS dependencies
grep -r "jupiter-" . --include="*.rs" | grep -o '"[^"]*"' | sort -u
# Found: jupiter-navy-300 through jupiter-navy-900

# Round 5: Builder patterns
grep -r "Styles.*::" . --include="*.rs" | grep -v test
# Found: ButtonStyles builder with 15+ methods

# Round 6: Usage statistics
find . -name "*.rs" -exec grep -l "Color::" {} \; | wc -l
# Found: 47 files using Color
```

## Advanced Pattern Sequences

### Finding Hidden Features
```bash
# Look for undocumented public APIs
comm -23 <(grep -r "pub " src --include="*.rs" | sort) <(grep -r "///" src --include="*.rs" | sort)

# Find macro-generated code
grep -r "macro_rules!\|derive(" . --include="*.rs"

# Find conditional features
grep -r "feature =\|cfg(feature" . --include="*.rs" | cut -d'"' -f2 | sort -u
```

### Cross-Reference Validation
```bash
# Compare tests vs implementation
echo "=== Implementation Coverage ==="
IMPL_COUNT=$(grep -r "pub fn" src --include="*.rs" | grep -v test | wc -l)
TEST_COUNT=$(grep -r "#\[test\]" . --include="*.rs" | wc -l)
echo "Public functions: $IMPL_COUNT"
echo "Test functions: $TEST_COUNT"
echo "Coverage ratio: $(( TEST_COUNT * 100 / IMPL_COUNT ))%"

# Find untested code
grep -r "pub fn" src --include="*.rs" | grep -v test | cut -d' ' -f3 | cut -d'(' -f1 > impl_funcs.txt
grep -r "test.*fn" . --include="*.rs" | grep -o "[a-z_]*(" | cut -d'(' -f1 > test_funcs.txt
comm -23 <(sort impl_funcs.txt) <(sort test_funcs.txt) > untested_funcs.txt
```

## Time-Saving Command Combinations

### Quick Feature Analysis
```bash
# One-liner for feature overview
FEATURE="Color"; echo "=== $FEATURE Analysis ===" && echo "Files: $(find . -name "*.rs" -exec grep -l "$FEATURE" {} \; | wc -l)" && echo "Usage: $(grep -r "$FEATURE::" . --include="*.rs" | wc -l)" && echo "Variants: $(grep -r "$FEATURE::" . --include="*.rs" | grep -o "$FEATURE::[A-Za-z]*" | sort -u | wc -l)"
```

### Batch CSS Class Analysis
```bash
# Extract and categorize all CSS classes
grep -r "class" . --include="*.rs" | grep -o '"[^"]*"' | tr ' ' '\n' | grep -v '^$' | while read class; do echo "$class" | grep -E "^([a-z]+)-" | sed 's/\(^[a-z]*\)-.*/\1/'; done | sort | uniq -c | sort -nr
```

### Complete Module Analysis
```bash
# Analyze a module completely
MODULE="color"; find . -path "*/$MODULE/*" -name "*.rs" | while read file; do echo "=== $file ==="; grep -E "pub |impl |trait |enum |struct " "$file" | grep -v "//"; done
```

## Critical Success Factors

1. **Always search examples directory** - Real usage patterns live there
2. **Extract patterns, not just occurrences** - `sort | uniq -c | sort -nr` is your friend
3. **Cross-reference multiple searches** - Combine results for complete picture
4. **Use context flags** - `-A` and `-B` reveal relationships
5. **Save intermediate results** - Pipe to files for comparison

## Validation Metrics

After running comprehensive search:
```bash
# Completeness check
echo "Search completeness:"
echo "✓ Core implementation files: $(find src -name "*.rs" | wc -l)"
echo "✓ Test files: $(find . -name "*test*.rs" | wc -l)"
echo "✓ Example files: $(find examples -name "*.rs" | wc -l)"
echo "✓ Total patterns found: $(cat all_patterns.txt | wc -l)"
echo "✓ Unique CSS classes: $(cat all_classes.txt | wc -l)"
echo "✓ Public API items: $(cat public_api.txt | wc -l)"
```

## Key Learning

The difference between a basic search and comprehensive discovery:
- Basic: `grep -r "Feature" .` (finds 10% of system)
- Comprehensive: This workflow (finds 100% of system)

Today's Color system documentation required all these patterns to discover:
- 19 enum variants (not 18)
- 4 color families (not 3)
- Builder pattern with 15+ methods
- 233 total usage points
- CSS framework dependency
- Gradient and ring utilities

**Time invested in comprehensive search saves hours of revision later.**