# AI Context Rules - Jupiter Design System Session

## Overview

This directory contains AI context rules created from the Jupiter Design System testing documentation session. These rules address critical efficiency gaps and prevent common documentation errors.

## Rules Created

### Critical Rules (Project-Breaking Issues)
1. **[CRITICAL-verify-before-documenting.md](CRITICAL-verify-before-documenting.md)** 
   - Prevents aspirational documentation by requiring verification first
   - Saves 2-3 hours of rework from false claims

2. **[CRITICAL-separate-current-future-state.md](CRITICAL-separate-current-future-state.md)**
   - Prevents user confusion about available vs planned features
   - Requires clear ✅/❌/🚧 status markers

### Workflow Rules (Development Patterns)
3. **[WORKFLOW-test-discovery-patterns.md](WORKFLOW-test-discovery-patterns.md)**
   - Systematic approach to discovering test infrastructure
   - 10-minute discovery prevents hours of corrections

4. **[WORKFLOW-documentation-audit-process.md](WORKFLOW-documentation-audit-process.md)**
   - Comprehensive audit process before publishing documentation
   - Eliminates accuracy issues and credibility damage

### Testing Rules (Test Strategies)
5. **[TESTING-accurate-test-counting.md](TESTING-accurate-test-counting.md)**
   - Multiple methods for verifying test counts and coverage claims
   - Prevents inflated metrics and false coverage claims

6. **[TESTING-example-validation.md](TESTING-example-validation.md)**
   - Ensures all code examples compile and run
   - Prevents user frustration from broken examples

### Debugging Rules (Troubleshooting)
7. **[DEBUGGING-claim-verification.md](DEBUGGING-claim-verification.md)**
   - Process for verifying every claim with specific commands
   - Prevents false statements about codebase features

### Rust Knowledge (Language-Specific)
8. **[RUST-test-organization-patterns.md](RUST-test-organization-patterns.md)**
   - Understanding actual Rust test organization patterns
   - Prevents assumptions about standard patterns

## Key Insights from Session

### What Went Wrong
- Initial documentation claimed 180+ tests (actual: 157)
- Documented two-layer architecture that didn't exist
- Created integration testing guide for non-existent tests
- Wrong color mappings in examples
- Mixed current and future state without clear separation

### What the Rules Prevent
- Aspirational documentation without verification
- User confusion about available features
- Wasted time on non-existent functionality
- Credibility damage from repeated corrections
- False claims about test coverage and architecture

### Time Impact
- Following rules: 10-15 minutes upfront verification
- Not following rules: 2-3 hours of rework per issue
- User frustration: Eliminated
- Credibility: Maintained

## Usage

These rules should be applied whenever:
- Documenting testing infrastructure
- Making quantitative claims about codebases
- Writing examples for documentation
- Describing software architecture
- Presenting current vs future features

## Session Summary

The Jupiter Design System testing documentation session revealed critical gaps between aspirational documentation and actual implementation. The user had to ask "are you sure you've been thorough?" three times before all issues were identified and corrected.

These rules transform that reactive debugging process into a proactive verification workflow, ensuring documentation accuracy from the start.