# TODO: Qdrant Import - Jupiter Design System Testing Session

## Overview
This document lists all knowledge and rules that need to be imported into the MCP/Qdrant vector database from today's Jupiter Design System testing documentation session.

## Critical Rules Collection

### 1. CRITICAL-verify-before-documenting
- **Vector Collection**: `critical_rules`
- **Rule ID**: `verify-before-documenting`
- **Content**: Complete verification protocol before writing documentation
- **Key Commands**: `find . -name "*test*" | wc -l`, `grep -r "#\[test\]" . | wc -l`
- **Time Impact**: Saves 2-3 hours of rework
- **Status**: ❌ Not imported

### 2. CRITICAL-separate-current-future-state
- **Vector Collection**: `critical_rules`
- **Rule ID**: `separate-current-future-state`
- **Content**: Clear separation of current vs future features with ✅/❌/🚧 markers
- **Key Template**: Current Implementation ✅ / Not Yet Implemented ❌ / Future Roadmap 🚧
- **Status**: ❌ Not imported

## Workflow Rules Collection

### 3. WORKFLOW-test-discovery-patterns
- **Vector Collection**: `workflow_rules`
- **Rule ID**: `test-discovery-patterns`
- **Content**: Systematic 3-phase test discovery workflow
- **Key Commands**: Discovery phase, analysis phase, pattern recognition
- **Time Impact**: 10 minutes discovery prevents hours of corrections
- **Status**: ❌ Not imported

### 4. WORKFLOW-documentation-audit-process
- **Vector Collection**: `workflow_rules`
- **Rule ID**: `documentation-audit-process`
- **Content**: 4-phase audit process before publishing documentation
- **Key Script**: Automated audit script for verification
- **Time Impact**: 10-15 minutes audit prevents credibility damage
- **Status**: ❌ Not imported

## Testing Rules Collection

### 5. TESTING-accurate-test-counting
- **Vector Collection**: `testing_rules`
- **Rule ID**: `accurate-test-counting`
- **Content**: Multiple methods for verifying test counts and coverage claims
- **Key Commands**: Multiple counting methods, per-file breakdown, coverage verification
- **Prevents**: False test coverage claims, inflated test counts
- **Status**: ❌ Not imported

### 6. TESTING-example-validation
- **Vector Collection**: `testing_rules`
- **Rule ID**: `example-validation`
- **Content**: 3-step protocol for validating code examples
- **Key Commands**: Extract examples, verify imports, test compilation
- **Prevents**: Broken examples in documentation
- **Status**: ❌ Not imported

## Debugging Rules Collection

### 7. DEBUGGING-claim-verification
- **Vector Collection**: `debugging_rules`
- **Rule ID**: `claim-verification`
- **Content**: Process for verifying every claim with specific commands
- **Key Protocol**: Challenge every claim, document verification commands
- **Prevents**: False statements about codebase features
- **Status**: ❌ Not imported

## Rust Knowledge Collection

### 8. RUST-test-organization-patterns
- **Vector Collection**: `rust_knowledge`
- **Rule ID**: `rust-test-organization-patterns`
- **Content**: Understanding actual Rust test organization patterns
- **Key Commands**: Pattern detection commands for different Rust test styles
- **Prevents**: Assumptions about standard Rust testing patterns
- **Status**: ❌ Not imported

## Session-Specific Knowledge

### 9. Jupiter Design System Testing Reality
- **Vector Collection**: `project_knowledge`
- **Rule ID**: `jupiter-design-system-testing-reality`
- **Content**: Actual test infrastructure vs documentation claims
- **Key Facts**: 157 tests (not 180+), single-layer builder testing, no integration tests
- **File Structure**: 8 test files, co-located pattern (*_test.rs)
- **Status**: ❌ Not imported

### 10. Documentation Accuracy Lessons
- **Vector Collection**: `workflow_rules`
- **Rule ID**: `documentation-accuracy-lessons`
- **Content**: Lessons learned from aspirational vs actual documentation
- **Key Insight**: User asked "are you sure you've been thorough?" 3 times
- **Pattern**: Each question revealed more inaccuracies
- **Status**: ❌ Not imported

## Testing Session Specific Data

### 11. Actual Test Count Data
- **Vector Collection**: `project_knowledge`
- **Rule ID**: `jupiter-test-count-verification`
- **Content**: Verified test counts per module
- **Data**: 
  - color_test.rs: 11 tests
  - button_test.rs: 23 tests
  - text_test.rs: 24 tests
  - product_test.rs: 29 tests
  - selection_test.rs: 18 tests
  - layout_test.rs: 16 tests
  - state_test.rs: 18 tests
  - card_test.rs: 18 tests
  - **Total**: 157 tests
- **Status**: ❌ Not imported

### 12. Common Documentation Pitfalls
- **Vector Collection**: `debugging_rules`
- **Rule ID**: `common-documentation-pitfalls`
- **Content**: Specific pitfalls observed in session
- **Examples**: 
  - Counting test modules instead of test functions
  - Including non-test items in counts
  - Missing test categories
  - Aspirational present tense language
- **Status**: ❌ Not imported

## Command Reference Knowledge

### 13. Test Discovery Commands
- **Vector Collection**: `workflow_rules`
- **Rule ID**: `test-discovery-commands`
- **Content**: Comprehensive command reference for test discovery
- **Languages**: Rust, JavaScript/TypeScript, Python
- **Commands**: Language-specific test counting, pattern discovery, architecture verification
- **Status**: ❌ Not imported

### 14. Verification Command Library
- **Vector Collection**: `testing_rules`
- **Rule ID**: `verification-command-library`
- **Content**: Reusable commands for verifying documentation claims
- **Categories**: Count verification, architecture verification, feature verification
- **Format**: Copy-paste ready bash commands
- **Status**: ❌ Not imported

## Error Pattern Knowledge

### 15. False Architecture Claims
- **Vector Collection**: `debugging_rules`
- **Rule ID**: `false-architecture-claims`
- **Content**: Common false claims about software architecture
- **Examples**: Two-layer testing, comprehensive utilities, advanced features
- **Detection**: Commands to verify architectural claims
- **Status**: ❌ Not imported

### 16. Color Mapping Errors
- **Vector Collection**: `project_knowledge`
- **Rule ID**: `jupiter-color-mapping-errors`
- **Content**: Incorrect color mappings found in documentation
- **Errors**: 
  - secondary → text-gray-600 (wrong) vs text-jupiter-green-500 (correct)
  - warning → text-yellow-600 (wrong) vs text-amber-500 (correct)
  - success → text-green-600 (wrong) vs text-green-500 (correct)
- **Status**: ❌ Not imported

## Meta-Knowledge

### 17. Session Pattern Recognition
- **Vector Collection**: `workflow_rules`
- **Rule ID**: `session-pattern-recognition`
- **Content**: Pattern of user questions revealing documentation issues
- **Pattern**: "Are you sure you've been thorough?" → More issues found
- **Insight**: Verification should be proactive, not reactive
- **Status**: ❌ Not imported

### 18. Documentation Quality Metrics
- **Vector Collection**: `testing_rules`
- **Rule ID**: `documentation-quality-metrics`
- **Content**: How to measure documentation quality
- **Metrics**: Claim verification rate, example compilation rate, user confusion incidents
- **Goal**: Zero false claims, 100% working examples
- **Status**: ❌ Not imported

## Import Priority

### High Priority (Critical for preventing major errors)
1. CRITICAL-verify-before-documenting
2. CRITICAL-separate-current-future-state  
3. TESTING-accurate-test-counting
4. DEBUGGING-claim-verification

### Medium Priority (Workflow improvements)
5. WORKFLOW-test-discovery-patterns
6. WORKFLOW-documentation-audit-process
7. TESTING-example-validation
8. RUST-test-organization-patterns

### Low Priority (Supporting knowledge)
9. Jupiter Design System Testing Reality
10. Documentation Accuracy Lessons
11. Actual Test Count Data
12. Common Documentation Pitfalls
13. Test Discovery Commands
14. Verification Command Library
15. False Architecture Claims
16. Color Mapping Errors
17. Session Pattern Recognition
18. Documentation Quality Metrics

## Import Commands Template

For each rule, use this template:

```bash
# Import to critical_rules collection
rules_and_context --operation add --collection critical_rules --rule-id "verify-before-documenting" --content "$(cat CRITICAL-verify-before-documenting.md)"

# Import to workflow_rules collection
rules_and_context --operation add --collection workflow_rules --rule-id "test-discovery-patterns" --content "$(cat WORKFLOW-test-discovery-patterns.md)"

# Import to testing_rules collection
rules_and_context --operation add --collection testing_rules --rule-id "accurate-test-counting" --content "$(cat TESTING-accurate-test-counting.md)"

# Import to debugging_rules collection
rules_and_context --operation add --collection debugging_rules --rule-id "claim-verification" --content "$(cat DEBUGGING-claim-verification.md)"

# Import to rust_knowledge collection
rules_and_context --operation add --collection rust_knowledge --rule-id "rust-test-organization-patterns" --content "$(cat RUST-test-organization-patterns.md)"
```

## Success Metrics

After import, the system should:
- ✅ Prevent aspirational documentation
- ✅ Ensure accurate test counting
- ✅ Require verification of all claims
- ✅ Separate current vs future state clearly
- ✅ Validate all code examples
- ✅ Understand actual Rust test patterns
- ✅ Provide systematic discovery workflows
- ✅ Enable comprehensive documentation audits

## Notes

This import represents lessons learned from a session where initial documentation was 70% inaccurate, requiring multiple rounds of corrections after user questioning. The rules and knowledge captured here transform reactive debugging into proactive verification, ensuring documentation accuracy from the start.

**Total Items to Import**: 18 rules/knowledge pieces across 5 vector collections