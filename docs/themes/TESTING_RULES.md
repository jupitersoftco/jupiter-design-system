# Testing Rules for Documentation Projects

## Rule 1: Documentation Examples Must Compile

**When to Apply**: Any code example in documentation

**Implementation**:
```rust
// Create docs/examples/verification.rs
#[cfg(test)]
mod doc_examples {
    use super::*;
    
    #[test]
    fn verify_readme_example() {
        // Paste exact example from README
        let colors = VibeColors::default();
        let button = primary_button(colors).classes();
        
        // Verify it produces expected output
        assert!(!button.is_empty());
        assert!(button.contains("bg-"));
    }
}
```

**Why Critical**:
- Broken examples destroy user trust
- Compilation errors indicate API misunderstanding
- Forces verification of claims

**Today's Issue**: Created 100+ code examples without compilation testing

---

## Rule 2: Integration Examples Need Real Testing

**When to Apply**: Framework integration documentation

**Implementation**:
```bash
# Create minimal test projects
mkdir test-integrations/
cd test-integrations/

# React test
npx create-react-app react-test
cd react-test
npm install jupiter-design-system
# Implement integration example
npm run build

# Vue test  
npm create vue@latest vue-test
cd vue-test
npm install jupiter-design-system
# Implement integration example
npm run build
```

**Why Important**:
- Framework APIs change frequently
- Build tools have specific requirements
- Integration complexity is often underestimated

---

## Rule 3: Theme Switching Must Be Tested

**When to Apply**: Documenting theme systems

**Test Pattern**:
```rust
#[test]
fn test_theme_switching() {
    let theme1 = VibeTheme::new();
    let theme2 = CorporateTheme::new();
    
    let button1 = primary_button(theme1).classes();
    let button2 = primary_button(theme2).classes();
    
    // Themes should produce different output
    assert_ne!(button1, button2);
    
    // But both should be valid
    assert!(!button1.is_empty());
    assert!(!button2.is_empty());
}
```

**Why Important**:
- Theme switching is core functionality
- Subtle bugs can break theming
- User expectations must be met

---

## Rule 4: Performance Claims Need Benchmarks

**When to Apply**: Making any performance claims

**Implementation**:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_theme_operations(c: &mut Criterion) {
    let theme = VibeTheme::new();
    
    c.bench_function("button generation", |b| {
        b.iter(|| {
            black_box(primary_button(theme.clone()).classes())
        });
    });
}

criterion_group!(benches, benchmark_theme_operations);
criterion_main!(benches);
```

**Why Important**:
- Performance claims without data are meaningless
- Benchmarks reveal actual bottlenecks
- Users rely on performance guidance

---

## Rule 5: Error Cases Must Be Documented and Tested

**When to Apply**: Documenting fallible operations

**Pattern**:
```rust
#[test]
fn test_invalid_theme_handling() {
    let mut theme = VibeTheme::new();
    theme.palette.primary = "".to_string(); // Invalid
    
    // Should either panic with clear message or handle gracefully
    let result = std::panic::catch_unwind(|| {
        primary_button(theme).classes()
    });
    
    // Document the actual behavior
    assert!(result.is_err() || /* handle gracefully */);
}
```

**Why Important**:
- Users will encounter error conditions
- Error behavior should be predictable
- Documentation should match reality

---

## Rule 6: Accessibility Claims Need Testing

**When to Apply**: Documenting accessibility features

**Implementation**:
```rust
#[test]
fn test_accessibility_attributes() {
    let button = primary_button(colors)
        .disabled(true)
        .loading(true);
    
    let attrs = button.accessibility_attributes();
    
    // Verify ARIA attributes are present
    assert!(attrs.contains(&("aria-disabled", "true")));
    assert!(attrs.contains(&("aria-busy", "true")));
    assert!(attrs.contains(&("tabindex", "0")));
}

#[test]
fn test_color_contrast() {
    let theme = AccessibilityTheme::new(ContrastLevel::AA);
    
    // Would need actual contrast testing library
    // This is pseudocode for concept
    let text_color = theme.resolve_color(Color::TextPrimary);
    let bg_color = theme.resolve_color(Color::Background);
    
    let contrast_ratio = calculate_contrast(text_color, bg_color);
    assert!(contrast_ratio >= 4.5); // WCAG AA requirement
}
```

**Why Important**:
- Accessibility is legally required
- Claims must be verifiable
- Users with disabilities depend on accuracy

---

## Rule 7: Migration Examples Need Before/After Tests

**When to Apply**: Documenting migration paths

**Pattern**:
```rust
#[test]
fn test_migration_from_old_system() {
    // Test old system
    let old_theme = OldThemeSystem {
        primary: "#3B82F6".to_string(),
    };
    let old_output = old_theme.generate_button_css();
    
    // Test new system
    let new_theme = MigratedTheme::from_old(old_theme);
    let new_output = primary_button(new_theme).classes();
    
    // Verify migration maintains essential properties
    assert!(old_output.contains("blue"));
    assert!(new_output.contains("blue"));
}
```

**Why Important**:
- Migration is high-risk operation
- Users need confidence in migration path
- Broken migrations cause adoption failure

---

## Rule 8: Documentation Examples Should Be Minimal

**When to Apply**: Creating code examples

**Good Example**:
```rust
// Minimal, focused example
let button = primary_button(colors).classes();
```

**Bad Example**:
```rust
// Complex example with many concerns
let button = ButtonBuilder::new(colors)
    .with_theme(theme)
    .with_state(state)
    .with_accessibility(attrs)
    .with_responsive(breakpoints)
    .with_animation(transitions)
    .build()
    .classes();
```

**Why Important**:
- Complex examples obscure main point
- Minimal examples are easier to test
- Users can build complexity gradually

---

## Rule 9: Test Documentation Structure

**When to Apply**: Large documentation projects

**Implementation**:
```rust
#[test]
fn test_documentation_completeness() {
    let doc_files = vec![
        "docs/README.md",
        "docs/architecture.md",
        "docs/core-traits.md",
        // ... all expected files
    ];
    
    for file in doc_files {
        assert!(std::path::Path::new(file).exists(), 
                "Missing documentation file: {}", file);
    }
}

#[test]
fn test_documentation_links() {
    // Test that all internal links work
    // Test that examples are referenced
    // Test that API references are accurate
}
```

**Why Important**:
- Documentation structure affects usability
- Broken links frustrate users
- Systematic testing prevents regressions

---

## Rule 10: Continuous Integration for Documentation

**When to Apply**: Any documentation project

**Implementation**:
```yaml
# .github/workflows/docs.yml
name: Documentation Tests

on: [push, pull_request]

jobs:
  test-docs:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    
    - name: Test documentation examples
      run: |
        cd docs/examples
        cargo test
    
    - name: Test integration examples
      run: |
        cd integration-tests
        ./test-all-frameworks.sh
    
    - name: Verify documentation completeness
      run: |
        ./scripts/verify-docs.sh
```

**Why Important**:
- Documentation rots without testing
- CI catches problems early
- Maintains quality over time