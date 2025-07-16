# Workflow Rule: Iterative Validation with User Pushback

## Overview
When a user repeatedly asks "are you sure?", it's a signal that deeper validation is needed. Each iteration should dig deeper into the codebase rather than just reviewing the same surface-level information.

## Today's Pattern: The Four "Are You Sure?" Iterations

### Iteration 1: Surface Level
**User**: "are you sure you've been thorough?"
**AI Response**: Reviewed obvious files, found one issue (interactive colors)
**Problem**: Only looked at direct references

### Iteration 2: Test Discovery  
**User**: "are you sure you've been thorough?"
**AI Response**: Found test bug (missing Color::Accent)
**Problem**: Still missing usage patterns

### Iteration 3: Deep Search
**User**: "are you sure you've been thorough?"
**AI Response**: Found jupiter-navy colors, gradients, selection rings
**Problem**: Still missing builder patterns and examples

### Iteration 4: Exhaustive Analysis
**User**: "are you sure you've been thorough?"
**AI Response**: Found ButtonStyles builder, 233 usages, configuration requirements
**Finally**: Complete picture

## The Escalating Validation Protocol

### Level 1: Direct References
```bash
# What you check first time
grep -r "FeatureName" . --include="*.rs"
cat main_implementation_file.rs
```

### Level 2: Test and Edge Cases
```bash
# What you check on first "are you sure?"
find . -name "*test.rs" -exec cat {} \;
grep -r "FeatureName" . --include="*.rs" -B 5 -A 5
```

### Level 3: Pattern Extraction
```bash
# What you check on second "are you sure?"
grep -r "feature-prefix" . | cut -d: -f2 | sort -u
find examples -name "*.rs" -exec cat {} \;
grep -r "edge-case-pattern\|special-pattern" .
```

### Level 4: Statistical Analysis
```bash
# What you check on third "are you sure?"
echo "Total usages: $(grep -r "Feature::" . | wc -l)"
echo "Files affected: $(find . -exec grep -l "Feature" {} \; | wc -l)"
echo "Unique patterns: $(grep -r "pattern" . | cut -d: -f2 | sort -u | wc -l)"

# Read EVERYTHING
find . -name "*.rs" -exec grep -l "Feature" {} \; | xargs cat | less
```

## The "Are You Sure?" Checklist

When user asks "are you sure?" for the:

### First Time
- [ ] Have I read ALL test files?
- [ ] Have I searched for variant spellings (Color, color, colour)?
- [ ] Have I checked the examples directory?
- [ ] Have I looked for hardcoded values?

### Second Time  
- [ ] Have I extracted ALL unique patterns?
- [ ] Have I found ALL prefixed values (jupiter-*, brand-*, custom-*)?
- [ ] Have I checked for builder patterns or convenience functions?
- [ ] Have I searched for gradients, animations, complex CSS?

### Third Time
- [ ] Have I counted total usage across the codebase?
- [ ] Have I read files I haven't looked at yet?
- [ ] Have I checked for configuration files?
- [ ] Have I looked for external dependencies?

### Fourth Time
Stop and do EVERYTHING:
```bash
# Nuclear option - read everything
find . -name "*.rs" -o -name "*.toml" -o -name "*.json" | 
  grep -v target | 
  grep -v ".git" | 
  xargs grep -l "Feature\|feature" | 
  xargs cat > full_analysis.txt

# Review the entire analysis
less full_analysis.txt
```

## Pattern Recognition

### User Says "are you sure?" Once
- They've noticed something wrong
- Surface-level analysis insufficient
- Dig deeper into tests and examples

### User Says "are you sure?" Twice  
- Multiple things are wrong
- You're missing entire categories
- Look for patterns and variants

### User Says "are you sure?" Three Times
- Fundamental misunderstanding
- Major features undiscovered  
- Do exhaustive analysis

### User Says "are you sure?" Four Times
- Start over with systematic discovery
- Read every relevant file
- Question all assumptions

## Time Management

Each iteration should be MORE thorough:
- Iteration 1: 5 minutes
- Iteration 2: 10 minutes  
- Iteration 3: 20 minutes
- Iteration 4: Complete analysis

## Key Insight

**"Are you sure?" is not asking for the same validation again.** It's asking for:
1. Deeper analysis
2. Broader search patterns
3. Questioning assumptions
4. Finding what you missed

## The Mantra

When asked "are you sure?":
1. I missed something significant
2. Look where I haven't looked
3. Use different search patterns
4. Question my assumptions

## Today's Lesson

It took FOUR iterations to find:
- Missing enum variant in tests
- Fourth Jupiter color family
- Selection ring colors
- Branded gradients
- ButtonStyles builder pattern
- 233 total usages
- Configuration requirements

Each "are you sure?" revealed major gaps. The final documentation was 5x larger than the initial version.

**Never defend your work when asked "are you sure?" - dig deeper instead.**