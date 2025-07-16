# AI Collaboration Rules

This directory contains critical rules and patterns learned from documenting the Jupiter Design System color system. These rules would have saved hours of rework if known beforehand.

## Overview

During the documentation of the Jupiter Design System's color system, multiple validation passes were required due to incomplete initial analysis. This collection captures the lessons learned to improve future AI collaboration efficiency.

## Rule Categories

### 🚨 Critical Rules
- **[Functional Validation First](./critical-functional-validation.md)** - Validate system works before documenting features
- **[Design System Completeness](./critical-design-system-completeness.md)** - Verify CSS/build pipeline before API documentation
- **[Integration-First Documentation](./critical-integration-first-documentation.md)** - Document how to use before what it does
- **[Output Validation](./critical-output-validation.md)** - Test generated output in target environment
- **[Documentation Validation Checklist](./critical-documentation-validation.md)** - Always validate documentation against actual implementation
- **[Assume Incomplete Tests](./critical-assume-incomplete-tests.md)** - Test suites often have gaps and bugs

### 🔄 Workflow Rules  
- **[Example-First Development](./workflow-example-first-development.md)** - Start with examples to understand real usage
- **[User Journey Validation](./workflow-user-journey-validation.md)** - Test complete user experience from discovery to integration
- **[Build Pipeline Discovery](./workflow-build-pipeline-discovery.md)** - Find and document complete build process
- **[Systematic Feature Discovery](./workflow-systematic-discovery.md)** - Use systematic patterns before starting work
- **[Iterative Validation](./workflow-iterative-validation.md)** - Each "are you sure?" requires deeper analysis
- **[Exhaustive Search Patterns](./workflow-exhaustive-search-patterns.md)** - Design systems require complete discovery

### 🧪 Testing Rules
- **[Validate Against Source](./testing-validate-against-source.md)** - Tests can lie; always verify with implementation

### 🔍 Debugging Rules
- **[Trace Generated Values](./debugging-trace-generated-values.md)** - Follow values from generation to consumption
- **[Find Hidden Dependencies](./debugging-find-hidden-dependencies.md)** - CSS frameworks and build tools are often required
- **[CSS Framework Dependencies](./debugging-css-framework-dependencies.md)** - Design systems have implicit CSS requirements

### 🦀 Rust Knowledge
- **[Design System Patterns](./rust-design-system-patterns.md)** - Rust-specific design system patterns and pitfalls
- **[Enum Validation Patterns](./rust-knowledge-enum-validation.md)** - Rust-specific patterns for complete validation

## Key Discoveries from Today

1. **Multiple Validation Passes Required**
   - Initial: Found basic implementation
   - Pass 2: Found test bugs (missing Color::Accent)
   - Pass 3: Found jupiter-navy colors, gradients
   - Pass 4: Found ButtonStyles builder, 233 usages

2. **Hidden Dependencies**
   - Jupiter colors require Tailwind configuration
   - CSS classes don't work without setup
   - No documentation of requirements

3. **Test Suite Bugs**
   - Test claimed 18 enum variants, actually 19
   - Missing variant would break user code
   - Tests gave false confidence

## Quick Reference

### Before Starting Documentation
```bash
# Systematic discovery
grep -r "Feature" . --include="*.rs" | wc -l  # Scope check
find examples -name "*.rs" -exec cat {} \;    # Real usage
grep -r "test" . --include="*.rs" | grep Feature  # Test coverage
```

### When User Asks "Are You Sure?"
1. First time: You missed something obvious
2. Second time: You missed entire categories  
3. Third time: Fundamental misunderstanding
4. Fourth time: Start over with exhaustive search

### CSS Dependencies Check
```bash
# Extract all CSS classes
grep -r "text-\|bg-\|border-" . --include="*.rs" | grep -o '"[^"]*"' | sort -u

# Check for custom prefixes
grep -E "jupiter-|brand-|custom-" generated-classes.txt
```

## Lessons Learned

### What Went Wrong
- Trusted test suite without verification
- Didn't check examples directory initially
- Assumed standard Tailwind colors
- Missed builder patterns and advanced usage

### What Should Have Happened
1. Exhaustive search before documentation
2. Test validation against source
3. Example code analysis
4. Configuration requirement check

### Time Impact
- Initial documentation: 30 minutes
- Four validation passes: 2+ hours
- Could have been avoided with 15-minute systematic discovery

## Using These Rules

1. **Start with systematic discovery** - Don't trust initial findings
2. **Validate everything** - Tests, documentation, assumptions  
3. **Look for patterns** - Builders, gradients, prefixes
4. **Check dependencies** - CSS, build tools, configuration
5. **Read examples** - They show real usage patterns

## Remember

> "The codebase always has more complexity than it initially appears. Validate thoroughly, then validate again."

These rules are derived from actual mistakes and discoveries. Following them will prevent similar issues in future documentation efforts.