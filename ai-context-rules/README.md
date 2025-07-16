# AI Context Rules for Jupiter Design System

## Overview
This directory contains AI collaboration rules derived from extensive documentation work on the Jupiter Design System. These rules address specific efficiency gaps and critical considerations that emerged during the development process.

## Rule Categories

### Critical Rules (🔴 High Priority)
Project-breaking issues and essential patterns that must be followed:

1. **[Discovery-First Documentation](./critical_rules/discovery-first-documentation.md)** - Comprehensive discovery prevents revision cycles
2. **[Anticipate Documentation Scope](./critical_rules/anticipate-documentation-scope.md)** - Include testing, migration, security, and governance from the start

### Workflow Rules (🟡 Process Improvement)
Development patterns and systematic approaches:

3. **[Iterative Validation Pattern](./workflow_rules/iterative-validation-pattern.md)** - Three-pass validation approach
4. **[Comprehensive Search Patterns](./workflow_rules/comprehensive-search-patterns.md)** - Progressive search from basic to exhaustive
5. **[User Feedback Signals](./workflow_rules/user-feedback-signals.md)** - Recognize when to expand scope by 2-3x

### Testing Rules (🟢 Quality Assurance)
Validation and testing strategies:

6. **[Documentation-Code Alignment](./testing_rules/documentation-code-alignment.md)** - Verify every claim against actual code
7. **[Test-Driven Understanding](./testing_rules/test-driven-understanding.md)** - Use tests to understand behavior, validate against implementation

### Debugging Rules (🟠 Problem Resolution)
Error patterns and troubleshooting:

8. **[Reality-Based Documentation](./debugging_rules/reality-based-documentation.md)** - Document limitations, workarounds, and actual state

### Rust Knowledge (🔵 Language-Specific)
Rust-specific patterns and idioms:

9. **[Design System Patterns](./rust_knowledge/design-system-patterns.md)** - Trait-based composition, zero-cost abstractions, type safety

## Key Insights from Today's Work

### The Discovery Problem
- **Issue:** Created documentation without thorough discovery
- **Impact:** Required 4 revision cycles
- **Solution:** Comprehensive search patterns before any documentation

### The Scope Problem  
- **Issue:** User asked "Are you sure?" twice - signal of insufficient scope
- **Impact:** Had to add 6 additional comprehensive guides
- **Solution:** Anticipate full documentation suite (testing, migration, security, governance)

### The Reality Problem
- **Issue:** Documentation didn't match actual implementation
- **Impact:** Features documented that didn't exist or were test-only
- **Solution:** Validate every claim against source code

## Usage Guidelines

### For New Projects
1. Start with **Discovery-First Documentation** rule
2. Apply **Comprehensive Search Patterns** for investigation
3. Use **Anticipate Documentation Scope** for planning

### For Documentation Tasks
1. Follow **Reality-Based Documentation** for accuracy
2. Use **Documentation-Code Alignment** for verification
3. Apply **Test-Driven Understanding** for behavior analysis

### For User Feedback
1. Recognize **User Feedback Signals** patterns
2. Use **Iterative Validation Pattern** for improvements
3. Apply **Comprehensive Search Patterns** for gap analysis

## Time Investment Guidelines

- **Discovery Phase:** 20-30 minutes saves 2-3 hours of revisions
- **Comprehensive Scope:** 3 hours upfront saves multiple feedback cycles
- **Reality Validation:** 15 minutes per documented feature prevents user confusion

## Success Metrics

Following these rules should result in:
- ✅ Single-pass documentation completion
- ✅ No "Are you sure?" feedback loops
- ✅ Documentation that matches actual implementation
- ✅ Comprehensive coverage without user prompting
- ✅ Reduced revision cycles

## Integration with Rules and Context MCP Tool

These rules are designed to be stored in Qdrant using the `rules_and_context` MCP tool with the following vector categories:

- `critical_rules` - Rules 1-2
- `workflow_rules` - Rules 3-5  
- `testing_rules` - Rules 6-7
- `debugging_rules` - Rule 8
- `rust_knowledge` - Rule 9

Each rule includes practical commands, real examples from today's work, and specific guidance for immediate application.

---

*Generated from analysis of Jupiter Design System documentation work session*