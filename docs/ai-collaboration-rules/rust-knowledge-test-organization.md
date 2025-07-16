# Rust Knowledge: Test Organization Patterns

## Problem This Prevents
- **Test discovery confusion** from non-standard organization
- **Maintenance difficulty** with scattered tests
- **Coverage gaps** from missing test patterns
- **Compilation issues** from incorrect test setup

## Time/Effort Saved
- Saves 1-2 hours setting up test infrastructure
- Reduces test discovery time by 80%
- Prevents 30-45 minutes of compilation debugging
- Speeds up test execution with proper organization

## Rust Test Organization Patterns

### 1. Unit Test Pattern - Inline Module
```rust
// src/builders/button.rs
pub struct ButtonBuilder {
    text: String,
    color: ButtonColor,
}

impl ButtonBuilder {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            color: ButtonColor::Primary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_creates_button_with_text() {
        let button = ButtonBuilder::new("Click");
        assert_eq!(button.text, "Click");
    }
    
    #[test]
    fn test_default_color_is_primary() {
        let button = ButtonBuilder::new("Test");
        assert_eq!(button.color, ButtonColor::Primary);
    }
}
```

### 2. Unit Test Pattern - Separate File
```rust
// src/builders/mod.rs
pub mod button;

#[cfg(test)]
mod button_test;

// src/builders/button_test.rs
use super::button::*;

#[test]
fn test_button_builder_new() {
    let builder = ButtonBuilder::new("Test");
    assert_eq!(builder.text(), "Test");
}
```

### 3. Integration Test Pattern
```rust
// tests/theme_integration.rs
use jupiter_design_system::{ButtonBuilder, Theme};

#[test]
fn test_button_uses_theme_colors() {
    let theme = Theme::dark();
    let button = ButtonBuilder::new("Dark")
        .with_theme(&theme)
        .build();
        
    assert_eq!(button.background_color(), theme.primary_color());
}
```

### 4. Documentation Test Pattern
```rust
/// Creates a new ButtonBuilder with the specified text.
/// 
/// # Examples
/// 
/// ```
/// use jupiter_design_system::ButtonBuilder;
/// 
/// let button = ButtonBuilder::new("Click me").build();
/// assert_eq!(button.text(), "Click me");
/// ```
pub fn new(text: &str) -> Self {
    // implementation
}
```

## Common Rust Test Patterns

### 1. Test Module Organization
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Group related tests in nested modules
    mod creation_tests {
        use super::*;
        
        #[test]
        fn new_with_valid_text() { }
        
        #[test]
        fn new_with_empty_text() { }
    }
    
    mod builder_tests {
        use super::*;
        
        #[test]
        fn chaining_methods() { }
        
        #[test]
        fn build_creates_immutable() { }
    }
}
```

### 2. Test Utilities and Fixtures
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Test fixture creation
    fn create_test_button() -> ButtonBuilder {
        ButtonBuilder::new("Test")
            .color(ButtonColor::Primary)
            .size(ButtonSize::Medium)
    }
    
    // Common assertions
    fn assert_button_valid(button: &Button) {
        assert!(!button.text().is_empty());
        assert!(button.is_enabled());
    }
    
    #[test]
    fn test_with_fixture() {
        let button = create_test_button().build();
        assert_button_valid(&button);
    }
}
```

### 3. Parameterized Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_all_color_variants() {
        let colors = vec![
            ButtonColor::Primary,
            ButtonColor::Secondary,
            ButtonColor::Success,
            ButtonColor::Warning,
            ButtonColor::Error,
        ];
        
        for color in colors {
            let button = ButtonBuilder::new("Test")
                .color(color.clone())
                .build();
            assert_eq!(button.color(), color);
        }
    }
}
```

### 4. Property-Based Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    
    #[quickcheck]
    fn prop_button_preserves_text(text: String) -> TestResult {
        if text.is_empty() {
            return TestResult::discard();
        }
        
        let button = ButtonBuilder::new(&text).build();
        TestResult::from_bool(button.text() == text)
    }
}
```

## Test File Naming Conventions

### 1. Standard Conventions
```
src/
├── lib.rs
├── builders/
│   ├── mod.rs
│   ├── button.rs
│   ├── button_test.rs      # Separate unit tests
│   └── tests/              # Submodule tests
│       └── button_tests.rs
└── tests/                  # Integration tests
    ├── integration_test.rs
    └── e2e_test.rs
```

### 2. Test Discovery Rules
```rust
// Cargo automatically discovers:
// 1. #[cfg(test)] modules in any .rs file
// 2. Files in tests/ directory
// 3. Doc tests in /// comments
// 4. Examples in examples/ directory

// Not automatically discovered:
// - Test files in src/ must be declared with `mod`
// - Benchmark files need special configuration
```

## Examples from Today's Session

### Example 1: Mixed Test Organization
**Found Pattern**:
```
src/builders/
├── button.rs (no inline tests)
├── button_test.rs (separate test file)
├── card.rs (no inline tests)
├── card_test.rs (empty)
└── state.rs (has inline tests)
```

**Best Practice Application**:
```rust
// Recommendation: Consistent pattern
// Option 1: All separate test files
// Option 2: All inline test modules
// But not mixed
```

### Example 2: Test Module Discovery Issue
**Problem**: Tests in separate files not running

**Solution**:
```rust
// src/builders/mod.rs
pub mod button;
pub mod card;

// Add test module declarations
#[cfg(test)]
mod button_test;

#[cfg(test)]
mod card_test;
```

## Test Organization Best Practices

### 1. Module Structure Template
```rust
// src/feature/mod.rs
pub mod implementation;

#[cfg(test)]
mod tests {
    mod unit_tests;
    mod integration_tests;
    mod property_tests;
}

// Or with separate files:
#[cfg(test)]
mod implementation_test;
```

### 2. Test Categorization
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Fast unit tests
    mod unit {
        use super::*;
        
        #[test]
        fn quick_test() { }
    }
    
    // Slower integration tests
    #[test]
    #[ignore] // Run with: cargo test -- --ignored
    fn expensive_integration_test() { }
    
    // Platform-specific tests
    #[cfg(target_os = "linux")]
    #[test]
    fn linux_specific_test() { }
}
```

### 3. Test Helper Organization
```rust
// src/test_utils.rs (or tests/common/mod.rs)
#[cfg(test)]
pub mod test_utils {
    use crate::*;
    
    pub struct TestContext {
        // Shared test setup
    }
    
    pub fn create_test_theme() -> Theme {
        Theme::default()
    }
}

// Use in tests:
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    
    #[test]
    fn test_with_helpers() {
        let theme = create_test_theme();
        // test implementation
    }
}
```

## Common Pitfalls and Solutions

### 1. Test Visibility Issues
```rust
// Problem: Can't access private items
struct PrivateStruct;

// Solution: Test in same module
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_private_struct() {
        let s = PrivateStruct; // Accessible here
    }
}
```

### 2. Test Compilation Errors
```rust
// Problem: Test utilities bloat release build
// Solution: Use #[cfg(test)]

#[cfg(test)]
use mock_library::Mock;

#[cfg(not(test))]
use real_library::Real;
```

### 3. Test Discovery Problems
```rust
// Problem: Tests not found
// Solution: Explicit module declaration

// In mod.rs or lib.rs
#[cfg(test)]
mod my_tests; // Declares my_tests.rs as a test module
```

## Test Organization Checklist
- [ ] Consistent test placement (inline OR separate)
- [ ] Test modules properly declared
- [ ] Integration tests in tests/ directory
- [ ] Test utilities properly isolated with #[cfg(test)]
- [ ] Documentation tests for public APIs
- [ ] Clear test naming conventions
- [ ] Logical test grouping by functionality

## Remember
**Good test organization makes tests discoverable, maintainable, and fast to run.**