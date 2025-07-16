# Workflow Rule: Iterative Validation Patterns

## Overview
This rule establishes command sequences for thorough validation at each iteration level. When documentation requires multiple passes, each iteration should use progressively more comprehensive patterns.

## Command Pattern Progression

### Initial Discovery Pattern
```bash
# Basic feature search
grep -r "FeatureName" src --include="*.rs"
find src -name "*.rs" -exec grep -l "FeatureName" {} \;

# Read primary implementation
cat src/core/feature.rs
cat src/lib.rs

# Quick usage count
echo "Direct usage: $(grep -r "Feature::" . --include="*.rs" | wc -l)"
```

### First Validation Iteration
```bash
# Expand search to tests and examples
find . -path "*/test*" -name "*.rs" -exec cat {} \; | grep -A 10 -B 10 "Feature"
find examples -name "*.rs" -exec cat {} \;

# Check for variant spellings
grep -r "feature\|Feature\|FEATURE" . --include="*.rs" -i

# Extract method signatures
grep -r "impl.*Feature" . --include="*.rs" -A 20 | grep "pub fn"
```

### Second Validation Iteration  
```bash
# Pattern extraction and categorization
grep -r "Feature::" . --include="*.rs" | grep -o "Feature::[A-Za-z]*" | sort | uniq -c

# Find all string literals related to feature
grep -r "Feature" . --include="*.rs" | grep -o '"[^"]*"' | sort -u > feature_strings.txt

# Check for builder patterns
grep -r "Builder\|builder" . --include="*.rs" | grep -B 2 -A 5 "Feature"

# CSS class extraction
grep -r "class\|classes" . --include="*.rs" | grep -o '"[^"]*"' | tr ' ' '\n' | sort -u
```

### Third Validation Iteration
```bash
# Complete statistical analysis
echo "=== Feature Usage Statistics ==="
echo "Total files: $(find . -name "*.rs" -exec grep -l "Feature" {} \; | wc -l)"
echo "Total references: $(grep -r "Feature" . --include="*.rs" | wc -l)"
echo "Unique patterns: $(grep -r "Feature" . --include="*.rs" | cut -d: -f2 | sort -u | wc -l)"

# Find indirect usage through traits
grep -r "impl.*for.*Feature" . --include="*.rs"
grep -r "where.*Feature" . --include="*.rs"

# Configuration and dependency check
find . -name "*.toml" -exec grep -H "feature\|Feature" {} \;
find . -name "*.json" -exec grep -H "feature\|Feature" {} \;
```

### Final Exhaustive Pattern
```bash
# Nuclear option - complete codebase analysis
find . -name "*.rs" -o -name "*.toml" | 
  grep -v target | 
  xargs grep -l "Feature\|feature" | 
  while read file; do
    echo "=== $file ==="
    cat "$file" | grep -C 10 "feature\|Feature"
  done > complete_analysis.txt

# Cross-reference all findings
grep -r "pub " src --include="*.rs" | grep -v test > public_api.txt
diff -u public_api.txt documented_api.txt || echo "Documentation gaps found"
```

## Practical Example from Today's Work

### Color System Discovery Evolution

#### Initial Pattern (Found 15% of system)
```bash
grep -r "Color" src/core/color.rs
cat src/core/color.rs
# Result: Found basic Color enum with 18 variants
```

#### First Iteration (Found 40% of system)
```bash
find . -name "*test*.rs" -exec grep -H "Color::" {} \;
# Result: Discovered test had wrong count (missing Color::Accent)
```

#### Second Iteration (Found 70% of system)
```bash
grep -r "jupiter-" . --include="*.rs" | cut -d'"' -f2 | sort -u
grep -r "ring-\|gradient-" . --include="*.rs" | grep -o '"[^"]*"' | sort -u
# Result: Found jupiter-navy family, selection rings, gradients
```

#### Third Iteration (Found 95% of system)
```bash
find examples -name "*.rs" -exec cat {} \; | grep -B 5 -A 10 "Color"
grep -r "Styles" . --include="*.rs" | grep struct
# Result: Found ButtonStyles builder pattern
```

#### Final Iteration (Found 100% of system)
```bash
echo "Total Color usage: $(grep -r "Color::" . --include="*.rs" | wc -l)"
find . -name "*.rs" -exec grep -l "jupiter-\|brand-" {} \; | xargs cat > full_jupiter_usage.txt
# Result: 233 total usages, complete CSS dependency map
```

## Validation Checkpoints

### After Each Iteration, Verify:
```bash
# 1. New patterns discovered?
echo "New patterns this iteration: $(comm -13 previous_patterns.txt current_patterns.txt | wc -l)"

# 2. Usage count increased?
echo "Previous count: $(cat previous_count.txt)"
echo "Current count: $(grep -r "Feature" . --include="*.rs" | wc -l)"

# 3. New file types found?
find . -name "*" -exec grep -l "Feature" {} \; | xargs file | cut -d: -f2 | sort -u

# 4. Documentation coverage?
echo "Documented features: $(grep -c "^###" documentation.md)"
echo "Actual features: $(grep -r "pub fn\|pub struct\|pub enum" src | wc -l)"
```

## Time Allocation Pattern

### Recommended Time Per Iteration
- Initial Discovery: 5 minutes
- First Validation: 10 minutes  
- Second Validation: 15 minutes
- Third Validation: 20 minutes
- Exhaustive Analysis: As needed

### Early Exit Criteria
```bash
# Can exit early if these all return 0
[ $(grep -r "TODO\|FIXME\|unimplemented!" . --include="*.rs" | grep -i feature | wc -l) -eq 0 ] && \
[ $(find examples -name "*.rs" -exec grep -l "Feature" {} \; | wc -l) -eq 0 ] && \
[ $(grep -r "feature" . --include="*.toml" | wc -l) -eq 0 ] && \
echo "Feature appears simple - may exit early"
```

## Key Commands to Save Time

### Create Search Aliases
```bash
# Add to your workflow
alias feature_search='grep -r "$1" . --include="*.rs" | wc -l'
alias pattern_extract='grep -r "$1" . --include="*.rs" | grep -o "$1::[A-Za-z]*" | sort -u'
alias css_extract='grep -r "class" . --include="*.rs" | grep -o "\"[^\"]*\"" | tr " " "\n" | sort -u'
```

### Batch Analysis Script
```bash
# Save as analyze_feature.sh
#!/bin/bash
FEATURE=$1
echo "=== Analyzing $FEATURE ==="
echo "Files: $(find . -name "*.rs" -exec grep -l "$FEATURE" {} \; | wc -l)"
echo "Total usage: $(grep -r "$FEATURE" . --include="*.rs" | wc -l)"  
echo "Patterns: $(grep -r "$FEATURE::" . --include="*.rs" | grep -o "$FEATURE::[A-Za-z]*" | sort -u | wc -l)"
echo "Examples: $(find examples -name "*.rs" -exec grep -l "$FEATURE" {} \; | wc -l)"
```

## Critical Insight

**Each validation iteration must use DIFFERENT search patterns.** Repeating the same searches wastes time. The goal is to uncover new aspects:

1. **Iteration 1**: Find what you missed in the obvious places
2. **Iteration 2**: Find categories you didn't know existed  
3. **Iteration 3**: Find patterns and relationships
4. **Iteration 4**: Find configuration and dependencies

## Remember

When asked to validate again:
- Don't defend previous work
- Use new search patterns
- Look in new locations
- Question assumptions
- Measure progress quantitatively

The difference between initial discovery and complete understanding is often 5-10x in scope.