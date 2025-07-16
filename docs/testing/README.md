# Jupiter Design System Testing Documentation

## 🚨 Important Note

This testing documentation includes both **current implementation** and **future recommendations**. Please note:

- ✅ **Unit Testing Guide** - Reflects actual current implementation
- ⚠️ **Integration Testing Guide** - Future roadmap (not yet implemented)
- ⚠️ **Visual Testing Guide** - Theoretical approach (not implemented)
- ⚠️ **Test Utilities** - Mix of current basic utilities and future recommendations

## Current State (What Actually Exists)

### ✅ Implemented
- **157 total tests** across 8 test files
- **Builder-focused testing** (TextStyles, CardStyles, ButtonStyles, etc.)
- **Co-located test files** (`*_test.rs` alongside implementation)
- **String API testing** and **enum convenience method testing**
- **Override behavior testing** (size, weight, color overrides)
- **Basic helper functions** (`create_text_styles()`, etc.)

### ❌ Not Yet Implemented
- **Integration testing infrastructure**
- **Visual regression testing**
- **Advanced test utilities** (helper functions, constants, assertions)
- **Cross-pattern compatibility testing**
- **Performance benchmarking**
- **Browser/component testing**

## Quick Start

To run the existing tests:

```bash
# Run all tests
cargo test

# Run specific builder tests
cargo test text_test
cargo test button_test
cargo test card_test
```

## Test Structure

Current tests follow this pattern:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::themes::VibeColors;

    fn create_text_styles() -> TextStyles<VibeColors> {
        text_styles(VibeColors::default())
    }

    #[test]
    fn test_hierarchy_str() {
        let classes = create_text_styles().hierarchy_str("title").classes();
        assert!(classes.contains("text-4xl"));
        assert!(classes.contains("font-bold"));
    }
}
```

## Documentation Files

1. **[overview.md](overview.md)** - Testing philosophy and current state
2. **[unit-testing.md](unit-testing.md)** - Current testing patterns and examples
3. **[integration-testing.md](integration-testing.md)** - Future roadmap
4. **[visual-testing.md](visual-testing.md)** - Theoretical approach
5. **[best-practices.md](best-practices.md)** - Testing conventions
6. **[test-utilities.md](test-utilities.md)** - Current vs recommended utilities

## Contributing

When adding new tests:

1. Follow the existing co-location pattern
2. Use the established helper functions
3. Test both string APIs and enum convenience methods
4. Include override behavior testing
5. Add descriptive error messages

## Future Work

The testing infrastructure could be enhanced with:
- Integration testing framework
- Advanced test utilities
- Performance benchmarking
- Cross-pattern compatibility testing
- Visual regression testing

This documentation serves as both a guide for current testing practices and a roadmap for future improvements.