# TODO: Qdrant Import List - Session 2

This document lists everything from today's session that needs to be imported into Qdrant via the MCP rules_and_context tool.

## 🚨 Critical Rules (6 items)

### 1. Functional Validation First
- **Vector Collection**: `critical_rules`
- **Rule Name**: `functional_validation_first`
- **Source File**: `/docs/ai-collaboration-rules/critical-functional-validation.md`
- **Key Insight**: Always test system works before documenting features (5-minute test vs hours of wasted docs)

### 2. Design System Completeness Check
- **Vector Collection**: `critical_rules`
- **Rule Name**: `design_system_completeness_check`
- **Source File**: `/docs/ai-collaboration-rules/critical-design-system-completeness.md`
- **Key Insight**: Verify CSS/build pipeline exists before documenting APIs (Jupiter generates non-functional CSS classes)

### 3. Integration-First Documentation
- **Vector Collection**: `critical_rules`
- **Rule Name**: `integration_first_documentation`
- **Source File**: `/docs/ai-collaboration-rules/critical-integration-first-documentation.md`
- **Key Insight**: Document "how to use" before "what it does" (integration docs prevent abandonment)

### 4. Output Validation
- **Vector Collection**: `critical_rules`
- **Rule Name**: `output_validation_before_documentation`
- **Source File**: `/docs/ai-collaboration-rules/critical-output-validation.md`
- **Key Insight**: Test generated output in target environment (CSS classes that don't work are worse than no output)

### 5. Documentation Validation Checklist
- **Vector Collection**: `critical_rules`
- **Rule Name**: `documentation_validation_checklist`
- **Source File**: `/docs/ai-collaboration-rules/critical-documentation-validation.md`
- **Key Insight**: Always validate docs against implementation (found 4 validation passes needed)

### 6. Assume Incomplete Tests
- **Vector Collection**: `critical_rules`
- **Rule Name**: `assume_incomplete_tests`
- **Source File**: `/docs/ai-collaboration-rules/critical-assume-incomplete-tests.md`
- **Key Insight**: Test suites have gaps/bugs (test claimed 18 variants but enum has 19)

## 🔄 Workflow Rules (6 items)

### 7. Example-First Development
- **Vector Collection**: `workflow_rules`
- **Rule Name**: `example_first_development`
- **Source File**: `/docs/ai-collaboration-rules/workflow-example-first-development.md`
- **Key Insight**: Start with examples to understand real usage (found ButtonStyles builder pattern late)

### 8. User Journey Validation
- **Vector Collection**: `workflow_rules`
- **Rule Name**: `user_journey_validation`
- **Source File**: `/docs/ai-collaboration-rules/workflow-user-journey-validation.md`
- **Key Insight**: Test complete user experience from discovery to integration (multiple friction points found)

### 9. Build Pipeline Discovery
- **Vector Collection**: `workflow_rules`
- **Rule Name**: `build_pipeline_discovery`
- **Source File**: `/docs/ai-collaboration-rules/workflow-build-pipeline-discovery.md`
- **Key Insight**: Find complete build process first (missing Tailwind config made system unusable)

### 10. Systematic Feature Discovery
- **Vector Collection**: `workflow_rules`
- **Rule Name**: `systematic_feature_discovery`
- **Source File**: `/docs/ai-collaboration-rules/workflow-systematic-discovery.md`
- **Key Insight**: Use systematic search patterns before starting work (233 Color:: usages found late)

### 11. Iterative Validation
- **Vector Collection**: `workflow_rules`
- **Rule Name**: `iterative_validation_patterns`
- **Source File**: `/docs/ai-collaboration-rules/workflow-iterative-validation.md`
- **Key Insight**: Each "are you sure?" requires deeper analysis (4 iterations needed)

### 12. Exhaustive Search Patterns
- **Vector Collection**: `workflow_rules`
- **Rule Name**: `exhaustive_search_patterns`
- **Source File**: `/docs/ai-collaboration-rules/workflow-exhaustive-search-patterns.md`
- **Key Insight**: Design systems require complete discovery (gradients, rings, builder patterns missed)

## 🧪 Testing Rules (1 item)

### 13. Validate Against Source
- **Vector Collection**: `testing_rules`
- **Rule Name**: `validate_tests_against_source`
- **Source File**: `/docs/ai-collaboration-rules/testing-validate-against-source.md`
- **Key Insight**: Tests can lie; always verify with implementation (Color::Accent missing from test)

## 🔍 Debugging Rules (3 items)

### 14. Trace Generated Values
- **Vector Collection**: `debugging_rules`
- **Rule Name**: `trace_generated_values_end_to_end`
- **Source File**: `/docs/ai-collaboration-rules/debugging-trace-generated-values.md`
- **Key Insight**: Follow values from generation to consumption (found broken CSS class pipeline)

### 15. Find Hidden Dependencies
- **Vector Collection**: `debugging_rules`
- **Rule Name**: `find_hidden_dependencies`
- **Source File**: `/docs/ai-collaboration-rules/debugging-find-hidden-dependencies.md`
- **Key Insight**: CSS frameworks and build tools often required but undocumented

### 16. CSS Framework Dependencies
- **Vector Collection**: `debugging_rules`
- **Rule Name**: `css_framework_dependencies`
- **Source File**: `/docs/ai-collaboration-rules/debugging-css-framework-dependencies.md`
- **Key Insight**: Design systems have implicit CSS requirements (Tailwind config needed)

## 🦀 Rust Knowledge (2 items)

### 17. Rust Design System Patterns
- **Vector Collection**: `rust_knowledge`
- **Rule Name**: `rust_design_system_patterns`
- **Source File**: `/docs/ai-collaboration-rules/rust-design-system-patterns.md`
- **Key Insight**: Rust-specific design system patterns and pitfalls (CSS generation without pipeline)

### 18. Enum Validation Patterns
- **Vector Collection**: `rust_knowledge`
- **Rule Name**: `enum_validation_patterns`
- **Source File**: `/docs/ai-collaboration-rules/rust-knowledge-enum-validation.md`
- **Key Insight**: Rust-specific patterns for complete validation (use strum for exhaustiveness)

## 📋 Documentation Artifacts (4 items)

### 19. Jupiter Color System Documentation
- **Vector Collection**: `project_knowledge`
- **Rule Name**: `jupiter_color_system_complete`
- **Source Files**: All files in `/docs/colors/`
- **Key Insight**: Complete color system documentation including 19 tokens, 4 Jupiter color families, Tailwind config requirements

### 20. Jupiter Missing Components Analysis
- **Vector Collection**: `project_knowledge`
- **Rule Name**: `jupiter_missing_components`
- **Source File**: `/docs/THINGS_THAT_STILL_NEED_TO_WORK.md`
- **Key Insight**: Critical gaps for production readiness (Tailwind config, setup docs, working examples)

### 21. AI Collaboration Rules Summary
- **Vector Collection**: `workflow_rules`
- **Rule Name**: `ai_collaboration_rules_summary`
- **Source File**: `/docs/ai-collaboration-rules/README.md`
- **Key Insight**: Complete summary of all efficiency rules learned from today's session

### 22. Today's Session Lessons
- **Vector Collection**: `critical_rules`
- **Rule Name**: `jupiter_documentation_lessons`
- **Content**: 
  - 4 validation passes needed to find all issues
  - System generates non-functional CSS classes
  - Missing Tailwind configuration makes system unusable
  - ButtonStyles builder pattern discovered late
  - Test suite has bug (missing Color::Accent)
  - 233 Color:: usages found across codebase
  - jupiter-navy color family undocumented
  - Selection rings and gradients missed initially

## 📊 Session Statistics to Import

- **Total documentation files created**: 13
- **Total rules created**: 18
- **Validation passes required**: 4
- **Critical issues found**: 8
- **Time that could have been saved**: 2+ hours
- **Color tokens documented**: 19
- **Jupiter color families found**: 4
- **Missing components identified**: 50+

## 🎯 High-Priority Imports

**Import these first for maximum efficiency improvement:**

1. **Functional Validation First** - Would have saved the most time
2. **Design System Completeness** - Would have caught CSS pipeline gap
3. **Integration-First Documentation** - Would have shown unusability immediately
4. **Output Validation** - Would have caught non-functional classes
5. **Example-First Development** - Would have found builder patterns early

## 📝 Import Format Template

For each rule, use this format when importing:

```
Vector Collection: [category]
Rule Name: [snake_case_name]
Title: [Human Readable Title]
Priority: [high/medium/low]
Context: Jupiter Design System documentation session
Source: [file_path]
Key Commands: [relevant bash commands]
Time Saved: [estimated hours]
```

## ⚠️ Import Notes

1. **File paths are relative** to `/Users/anon/dev/jupiter-design-system/`
2. **All content is validated** against actual implementation
3. **Commands are tested** and functional
4. **Examples are from real codebase** analysis
5. **Time estimates are conservative** based on today's experience

## 🔗 Cross-References

Many rules reference each other:
- Functional Validation → Output Validation
- Integration-First → User Journey Validation
- Example-First → Build Pipeline Discovery
- Design System Completeness → CSS Framework Dependencies

Import with these relationships preserved for maximum effectiveness.