# Current Limitations

This document provides a transparent overview of the current limitations in the Jupiter Design System, helping developers understand what's available today versus what's planned for the future.

## Testing Infrastructure

### ✅ What Exists
- **157 unit tests** for builder APIs
- **Co-located test files** (`*_test.rs`) with implementations
- **Basic helper functions** in each test module
- **100% pass rate** on existing tests

### ❌ What's Missing
- **No integration testing** - Cross-builder compatibility is untested
- **No visual regression testing** - Only CSS class validation exists
- **No performance benchmarking** - No systematic performance measurement
- **No shared test utilities** - Each test file has its own helpers
- **No test constants** - CSS classes are hardcoded strings
- **Limited error messages** - Most assertions lack descriptive context

## Design System Architecture

### ✅ What Exists
- **Builder pattern APIs** - Fluent interfaces for generating CSS classes
- **String-based convenience methods** - e.g., `hierarchy_str("title")`
- **Enum-based methods** - e.g., `.title()`, `.primary()`
- **Override behavior** - Size, weight, color overrides work correctly
- **Two themes** - VibeColors and PsychedelicColors

### ❌ What's Missing
- **No pattern layer testing** - Only builder APIs are tested
- **No component framework integration** - Builders exist but no actual components
- **No runtime CSS generation** - Only class strings are produced
- **No design tokens export** - Colors/spacing are embedded in code
- **No accessibility utilities** - Focus states must be manually added
- **No responsive design helpers** - Breakpoints not systematically handled

## Documentation Gaps

### ✅ What Exists
- **Comprehensive API documentation** - All public methods documented
- **Usage examples** - Basic examples in docs
- **Testing guide** - Covers current testing approach

### ❌ What's Missing
- **No interactive documentation** - No live component preview
- **No design guidelines** - When/how to use different patterns
- **No migration guides** - How to adopt the design system
- **No contribution guidelines** - How to add new patterns
- **No changelog** - Version history and breaking changes

## Color System Limitations

### Current Issues
- **Incorrect documentation** - Some color mappings in docs don't match implementation:
  - `secondary` maps to `jupiter-green-500` (not `gray-600`)
  - `warning` maps to `amber-500` (not `yellow-600`)
- **Limited color scales** - Only specific color values, not full scales
- **No dark mode support** - Colors are hardcoded for light mode
- **No color contrast validation** - Accessibility not enforced

## Builder Limitations

### TextStyles
- **No line height customization** - Fixed at `leading-relaxed`
- **No letter spacing for body text** - Only titles have tracking options
- **Limited text decoration** - No underline, strikethrough options
- **No gradient text support** - Despite theme capabilities

### CardStyles
- **No border radius options** - Fixed rounded corners
- **No gradient backgrounds** - Despite psychedelic theme
- **Limited shadow options** - Only predefined elevations
- **No hover state customization** - Fixed hover behaviors

### ButtonStyles
- **No icon support** - Text-only buttons
- **No size variants for icons** - When icons are added
- **No button groups** - Individual buttons only
- **No toggle states** - No active/selected state

## Testing Documentation vs Reality

### Documentation Claims vs Actual Implementation
- **Claimed: Two-layer testing** → Reality: Builder-only testing
- **Claimed: Integration tests** → Reality: Not implemented
- **Claimed: Test utilities** → Reality: Basic helpers only
- **Claimed: Performance tests** → Reality: No benchmarks
- **Claimed: 180+ tests** → Reality: 157 tests

## Technical Debt

### Code Organization
- **Duplicate helper functions** - Same helpers in each test file
- **No shared constants** - CSS classes repeated throughout
- **Inconsistent test patterns** - Different approaches in different files
- **No test categorization** - All tests in flat structure

### Missing Infrastructure
- **No CI/CD configuration** - Tests run locally only
- **No coverage reporting** - Unknown actual coverage percentage
- **No performance monitoring** - No regression detection
- **No automated releases** - Manual version management

## Browser/Framework Support

### Current Limitations
- **Rust-only implementation** - No JavaScript/TypeScript bindings
- **No React/Vue/Angular adapters** - Raw CSS classes only
- **No Dioxus component library** - Despite Dioxus dependency
- **No SSR considerations** - Client-side only design
- **No CSS-in-JS integration** - Static classes only

## Recommendations for Contributors

### Immediate Improvements Needed
1. **Extract common test helpers** into shared utilities
2. **Add integration tests** for cross-builder compatibility
3. **Fix color documentation** to match implementation
4. **Add descriptive test error messages**
5. **Create CSS class constants** to reduce magic strings

### Medium-term Goals
1. **Implement component library** using builders
2. **Add dark mode support** with color system updates
3. **Create design tokens** for better portability
4. **Add accessibility validation** and ARIA helpers
5. **Implement responsive design system**

### Long-term Vision
1. **Visual regression testing** infrastructure
2. **Interactive documentation** with live examples
3. **Multi-framework support** (React, Vue, etc.)
4. **Design system versioning** and migration tools
5. **Comprehensive accessibility** compliance

## Using the Design System Today

Despite these limitations, the Jupiter Design System provides:
- ✅ **Consistent styling** through builder patterns
- ✅ **Type-safe APIs** with Rust's type system
- ✅ **Reliable class generation** with 100% test pass rate
- ✅ **Flexible theming** with two distinct themes
- ✅ **Good developer experience** with fluent APIs

The system is production-ready for Rust applications that can work with CSS class strings, but teams should be aware of the current limitations and plan accordingly.

## Contributing

To help address these limitations:
1. Check the [Testing Documentation](testing/README.md) for current patterns
2. Follow existing builder patterns when adding features
3. Add tests for any new functionality
4. Update documentation to reflect actual implementation
5. Consider opening issues for missing features you need

Remember: It's better to have a small, well-tested design system than a large, unreliable one. The current limitations represent opportunities for focused, meaningful contributions.