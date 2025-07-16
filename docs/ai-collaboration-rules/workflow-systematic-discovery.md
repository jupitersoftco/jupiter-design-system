# Workflow Rule: Systematic Feature Discovery

## Overview
When documenting or implementing features in an existing codebase, use systematic discovery patterns to uncover all related functionality before starting work.

## The Discovery Workflow

### Phase 1: Broad Search
```bash
# Start with the most general search
grep -r "feature_name" . --include="*.rs" --include="*.ts" --include="*.js"

# Count occurrences to understand scope
grep -r "FeatureName" . | wc -l

# Find all files mentioning the feature
find . -type f -name "*.rs" -exec grep -l "FeatureName" {} \; | sort
```

### Phase 2: Pattern Extraction
```bash
# Extract unique values/patterns
grep -r "prefix-" . --include="*.rs" | cut -d: -f2 | sort -u > unique_values.txt

# Find usage contexts (5 lines before/after)
grep -r "FeatureName" . --include="*.rs" -B 5 -A 5 > usage_contexts.txt
```

### Phase 3: Relationship Mapping
```bash
# Find related traits/interfaces
grep -r "trait.*Feature\|impl.*Feature" . --include="*.rs"

# Find builder patterns
grep -r "Builder\|Styles\|Config" . --include="*.rs" | grep -i feature

# Find test patterns
find . -name "*test.rs" -exec grep -l "Feature" {} \;
```

## Today's Example: Color System Discovery

### What We Did Wrong
Started documenting based on initial findings:
- Found `Color` enum with 18 variants
- Found `ColorProvider` trait
- Found basic usage in one file

### What We Should Have Done
```bash
# 1. Count total Color usage
grep -r "Color::" . --include="*.rs" | wc -l
# Result: 233 occurrences - this is a major feature!

# 2. Find ALL color prefixes
grep -r "jupiter-" . --include="*.rs" | cut -d'"' -f2 | sort -u
# Found: jupiter-blue, jupiter-green, jupiter-orange, jupiter-navy (4, not 3!)

# 3. Find complex patterns
grep -r "gradient\|ring-\|from-.*to-" . --include="*.rs"
# Found: gradients, selection rings - advanced features!

# 4. Check examples directory
cat examples/*.rs | grep -A 10 -B 10 "Color"
# Found: ButtonStyles builder pattern - completely different API!

# 5. Read test files completely
cat src/core/color_test.rs
# Found: Test bug - missing Color::Accent variant
```

## Systematic Discovery Checklist

### Before Starting Documentation
1. **Scope Assessment**
   ```bash
   find . -type f -name "*.rs" -exec grep -l "Feature" {} \; | wc -l
   ```
   - < 5 files: Small feature
   - 5-20 files: Medium feature  
   - > 20 files: Major feature (needs thorough discovery)

2. **Pattern Collection**
   ```bash
   # Create a findings file
   echo "=== Feature Discovery ===" > feature_findings.md
   
   # Add all unique patterns
   echo "## Unique Values" >> feature_findings.md
   grep -r "pattern" . | cut -d: -f2 | sort -u >> feature_findings.md
   
   # Add file list
   echo "## Files Using Feature" >> feature_findings.md
   find . -name "*.rs" -exec grep -l "Feature" {} \; >> feature_findings.md
   ```

3. **Dependency Mapping**
   ```bash
   # Find what depends on this feature
   grep -r "use.*Feature\|import.*Feature" . --include="*.rs"
   
   # Find what this feature depends on
   cat feature_implementation.rs | grep "^use\|^import"
   ```

## Discovery Command Templates

### For Rust Projects
```bash
# Complete feature analysis
FEATURE="ColorProvider"
echo "Analyzing $FEATURE..."

# Usage count
echo "Total usage: $(grep -r "$FEATURE" . --include="*.rs" | wc -l)"

# File distribution  
echo "Files using $FEATURE:"
find . -name "*.rs" -exec grep -l "$FEATURE" {} \; | sort

# Method calls
echo "Methods called:"
grep -r "$FEATURE" . --include="*.rs" | grep -o "\.$FEATURE::[a-zA-Z_]*" | sort -u

# Test coverage
echo "Test files:"
find . -name "*test.rs" -exec grep -l "$FEATURE" {} \;
```

### For Design Systems
```bash
# Find all design tokens
grep -r "token\|Token" . --include="*.rs" | grep -v test

# Find all CSS class generation
grep -r "class\|Class" . --include="*.rs" | grep "format!\|concat!\|push_str"

# Find hardcoded styles
grep -r "bg-\|text-\|border-" . --include="*.rs" | grep '"'
```

## Key Lesson
**Discovery before documentation!** Today we spent hours correcting documentation because we didn't invest 15 minutes in systematic discovery first.

## Time Investment
- Systematic discovery: 15-30 minutes
- Fixing incomplete documentation: 2-4 hours
- User frustration from bad docs: Immeasurable

Always choose discovery first.