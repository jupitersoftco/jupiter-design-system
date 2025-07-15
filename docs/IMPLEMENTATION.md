# Jupiter Design System: Implementation Guide

This guide walks through how to create new patterns and builders in Jupiter Design System, based on lessons learned from implementing Typography, Card, and Button systems.

## 🏗️ **Creating New Patterns**

### **Step 1: Define Semantic Enums**

Start by identifying the **semantic concepts** your pattern needs to express:

```rust
// src/patterns/my_pattern.rs

/// Semantic size concepts (not just CSS classes)
#[derive(Debug, Clone, PartialEq)]
pub enum MyPatternSize {
    /// Compact for dense layouts
    Compact,
    /// Standard for most use cases
    Standard,
    /// Comfortable for touch interfaces
    Comfortable,
    /// Spacious for accessibility
    Spacious,
}

/// Visual style concepts
#[derive(Debug, Clone, PartialEq)]
pub enum MyPatternStyle {
    /// Subtle, background element
    Subtle,
    /// Normal prominence
    Standard,
    /// Emphasized, draws attention
    Emphasized,
    /// Critical, requires immediate attention
    Critical,
}
```

**Key Principles:**

- Use **semantic names** ("Comfortable") not syntactic ("Large")
- Include **documentation** explaining when to use each variant
- Think about **user intent**, not CSS implementation

### **Step 2: Create Pattern Struct**

```rust
/// Main pattern configuration
#[derive(Debug, Clone)]
pub struct MyPattern<T: ColorProvider> {
    pub size: MyPatternSize,
    pub style: MyPatternStyle,
    pub color_provider: T,
    // Add other semantic properties...
}

impl<T: ColorProvider> MyPattern<T> {
    /// Create with sensible defaults
    pub fn new(color_provider: T) -> Self {
        Self {
            size: MyPatternSize::Standard,
            style: MyPatternStyle::Standard,
            color_provider,
        }
    }

    /// Fluent builder methods
    pub fn size(mut self, size: MyPatternSize) -> Self {
        self.size = size;
        self
    }

    pub fn style(mut self, style: MyPatternStyle) -> Self {
        self.style = style;
        self
    }

    /// Generate CSS classes
    pub fn classes(&self) -> String {
        let mut classes = vec![];

        // Base classes (always applied)
        classes.push("my-pattern-base".to_string());

        // Size-specific classes
        classes.push(self.get_size_classes());

        // Style-specific classes
        classes.push(self.get_style_classes());

        // Join and deduplicate
        let mut all_classes: Vec<String> = classes
            .join(" ")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        all_classes.sort();
        all_classes.dedup();
        all_classes.join(" ")
    }

    /// Convert size enum to CSS classes
    fn get_size_classes(&self) -> String {
        match self.size {
            MyPatternSize::Compact => "p-1 text-sm",
            MyPatternSize::Standard => "p-2 text-base",
            MyPatternSize::Comfortable => "p-3 text-lg",
            MyPatternSize::Spacious => "p-4 text-xl",
        }.to_string()
    }

    /// Convert style enum to CSS classes
    fn get_style_classes(&self) -> String {
        match self.style {
            MyPatternStyle::Subtle => "bg-gray-50 text-gray-600",
            MyPatternStyle::Standard => "bg-white text-gray-900",
            MyPatternStyle::Emphasized => "bg-blue-50 text-blue-900 border border-blue-200",
            MyPatternStyle::Critical => "bg-red-50 text-red-900 border border-red-200",
        }.to_string()
    }
}
```

### **Step 3: Add Convenience Functions**

```rust
/// Convenience function to create pattern
pub fn my_pattern<T: ColorProvider>(color_provider: T) -> MyPattern<T> {
    MyPattern::new(color_provider)
}

/// Convenience for emphasized variant
pub fn emphasized_pattern<T: ColorProvider>(color_provider: T) -> MyPattern<T> {
    MyPattern::new(color_provider).style(MyPatternStyle::Emphasized)
}
```

### **Step 4: Export from Module**

```rust
// src/patterns/mod.rs
pub mod my_pattern;
pub use my_pattern::*;
```

## 🔧 **Creating New Builders**

### **Step 1: Create Builder Struct**

```rust
// src/builders/my_builder.rs

use crate::core::color::ColorProvider;
use crate::patterns::my_pattern::{MyPattern, MyPatternSize, MyPatternStyle};

/// Builder with chainable API
#[derive(Debug, Clone)]
pub struct MyStyles<T: ColorProvider> {
    pattern: MyPattern<T>,
    custom_classes: Vec<String>,
}

impl<T: ColorProvider> MyStyles<T> {
    /// Create new builder
    pub fn new(color_provider: T) -> Self {
        Self {
            pattern: MyPattern::new(color_provider),
            custom_classes: Vec::new(),
        }
    }

    /// Set size using enum
    pub fn size(mut self, size: MyPatternSize) -> Self {
        self.pattern = self.pattern.size(size);
        self
    }

    /// Set size using string (for migration convenience)
    pub fn size_str(mut self, size: &str) -> Self {
        let size_enum = match size {
            "compact" => MyPatternSize::Compact,
            "standard" => MyPatternSize::Standard,
            "comfortable" => MyPatternSize::Comfortable,
            "spacious" => MyPatternSize::Spacious,
            _ => return self, // ignore invalid values
        };
        self.pattern = self.pattern.size(size_enum);
        self
    }

    /// Convenience methods (highly recommended)
    pub fn compact(self) -> Self {
        self.size(MyPatternSize::Compact)
    }

    pub fn comfortable(self) -> Self {
        self.size(MyPatternSize::Comfortable)
    }

    pub fn emphasized(self) -> Self {
        self.style(MyPatternStyle::Emphasized)
    }

    /// Add custom classes
    pub fn custom_classes(mut self, classes: &str) -> Self {
        if !classes.is_empty() {
            self.custom_classes.push(classes.to_string());
        }
        self
    }

    /// Generate final CSS classes
    pub fn classes(&self) -> String {
        let mut classes = vec![self.pattern.classes()];

        // Add custom classes
        for custom_class in &self.custom_classes {
            classes.push(custom_class.clone());
        }

        // Join and deduplicate
        let mut all_classes: Vec<String> = classes
            .join(" ")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        all_classes.sort();
        all_classes.dedup();
        all_classes.join(" ")
    }
}
```

### **Step 2: Add Convenience Functions**

```rust
/// Create builder
pub fn my_styles<T: ColorProvider>(color_provider: T) -> MyStyles<T> {
    MyStyles::new(color_provider)
}

/// Utility function for string-based API
pub fn my_classes_from_strings<T: ColorProvider>(
    color_provider: T,
    size: Option<&str>,
    style: Option<&str>,
    custom_classes: Option<&str>,
) -> String {
    let mut builder = my_styles(color_provider);

    if let Some(size) = size {
        builder = builder.size_str(size);
    }

    if let Some(style) = style {
        builder = builder.style_str(style);
    }

    if let Some(custom) = custom_classes {
        builder = builder.custom_classes(custom);
    }

    builder.classes()
}
```

### **Step 3: Export from Module**

```rust
// src/builders/mod.rs
pub mod my_builder;
pub use my_builder::{my_classes_from_strings, my_styles, MyStyles};
```

## 🧪 **Creating Comprehensive Tests**

### **Pattern Tests**

```rust
// src/patterns/my_pattern_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::color::VibeColors;

    #[test]
    fn test_pattern_defaults() {
        let pattern = MyPattern::new(VibeColors::default());
        let classes = pattern.classes();

        assert!(classes.contains("my-pattern-base"));
        assert!(classes.contains("p-2")); // Standard size
        assert!(classes.contains("bg-white")); // Standard style
    }

    #[test]
    fn test_pattern_size_variations() {
        let colors = VibeColors::default();

        let compact = MyPattern::new(colors.clone()).size(MyPatternSize::Compact).classes();
        assert!(compact.contains("p-1"));
        assert!(compact.contains("text-sm"));

        let spacious = MyPattern::new(colors).size(MyPatternSize::Spacious).classes();
        assert!(spacious.contains("p-4"));
        assert!(spacious.contains("text-xl"));
    }

    #[test]
    fn test_pattern_style_variations() {
        let colors = VibeColors::default();

        let subtle = MyPattern::new(colors.clone()).style(MyPatternStyle::Subtle).classes();
        assert!(subtle.contains("bg-gray-50"));
        assert!(subtle.contains("text-gray-600"));

        let critical = MyPattern::new(colors).style(MyPatternStyle::Critical).classes();
        assert!(critical.contains("bg-red-50"));
        assert!(critical.contains("border-red-200"));
    }
}
```

### **Builder Tests**

```rust
// src/builders/my_builder_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::color::VibeColors;

    fn create_styles() -> MyStyles<VibeColors> {
        my_styles(VibeColors::default())
    }

    #[test]
    fn test_chaining_api() {
        let classes = create_styles()
            .compact()
            .emphasized()
            .custom_classes("custom-class")
            .classes();

        assert!(classes.contains("p-1")); // compact
        assert!(classes.contains("bg-blue-50")); // emphasized
        assert!(classes.contains("custom-class")); // custom
    }

    #[test]
    fn test_string_api() {
        let classes = my_classes_from_strings(
            VibeColors::default(),
            Some("compact"),
            Some("emphasized"),
            Some("custom-class"),
        );

        assert!(classes.contains("p-1"));
        assert!(classes.contains("bg-blue-50"));
        assert!(classes.contains("custom-class"));
    }

    #[test]
    fn test_invalid_strings_ignored() {
        let base_classes = create_styles().classes();
        let invalid_classes = create_styles().size_str("invalid").classes();
        assert_eq!(base_classes, invalid_classes);
    }

    #[test]
    fn test_class_deduplication() {
        let classes = create_styles()
            .custom_classes("bg-white bg-white") // duplicate
            .classes();

        let count = classes.matches("bg-white").count();
        assert_eq!(count, 1);
    }
}
```

## 📏 **File Size Management**

### **Keep Files Under 300 Lines**

Follow the workspace rule: **all Rust files under 300 LOC**.

```rust
// ✅ Good: Focused pattern file (~250 lines)
// src/patterns/my_pattern.rs
pub enum MyPatternSize { /* ... */ }
pub enum MyPatternStyle { /* ... */ }
pub struct MyPattern<T> { /* ... */ }
impl<T> MyPattern<T> { /* focused implementation */ }

// ✅ Good: Focused builder file (~280 lines)
// src/builders/my_builder.rs
pub struct MyStyles<T> { /* ... */ }
impl<T> MyStyles<T> { /* focused implementation */ }

// ✅ Good: Separate test files
// src/patterns/my_pattern_test.rs (< 300 lines)
// src/builders/my_builder_test.rs (< 300 lines)
```

### **Split Large Implementations**

If a pattern gets too complex:

```rust
// Split into focused modules
src/patterns/layout/
├── mod.rs           # Public interface
├── grid.rs          # Grid layout patterns
├── flex.rs          # Flexbox patterns
├── spacing.rs       # Spacing patterns
└── responsive.rs    # Responsive patterns
```

## 🎯 **Best Practices**

### **1. Semantic Naming**

```rust
// ❌ Implementation-focused
pub enum ButtonSize { Small12px, Medium16px, Large20px }

// ✅ Purpose-focused
pub enum ButtonSize { Compact, Standard, Comfortable }
```

### **2. Comprehensive String APIs**

```rust
// Support both type-safe and string APIs
pub fn size(mut self, size: ButtonSize) -> Self { /* ... */ }
pub fn size_str(mut self, size: &str) -> Self { /* ... */ }

// Convenience methods for common cases
pub fn small(self) -> Self { self.size(ButtonSize::Compact) }
pub fn large(self) -> Self { self.size(ButtonSize::Comfortable) }
```

### **3. Graceful Degradation**

```rust
pub fn size_str(mut self, size: &str) -> Self {
    let size_enum = match size {
        "compact" => ButtonSize::Compact,
        "standard" => ButtonSize::Standard,
        "comfortable" => ButtonSize::Comfortable,
        _ => return self, // ✅ Ignore invalid, don't panic
    };
    self.pattern = self.pattern.size(size_enum);
    self
}
```

### **4. Class Deduplication**

```rust
pub fn classes(&self) -> String {
    // ... generate classes ...

    // Always deduplicate
    let mut all_classes: Vec<String> = classes
        .join(" ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    all_classes.sort();
    all_classes.dedup();
    all_classes.join(" ")
}
```

### **5. Comprehensive Testing**

Test **every public method** and **edge case**:

```rust
#[test]
fn test_all_size_variants() {
    for size in [MyPatternSize::Compact, MyPatternSize::Standard, /* ... */] {
        let classes = MyPattern::new(colors.clone()).size(size).classes();
        assert!(!classes.is_empty(), "Size {:?} should generate classes", size);
    }
}
```

## 🔗 **Integration with Components**

### **Dioxus Component Usage**

```rust
// Easy integration in components
#[component]
pub fn MyComponent(props: MyProps) -> Element {
    let classes = my_styles(VibeColors::default())
        .size_str(&props.size)
        .style_str(&props.style)
        .custom_classes(&props.class.unwrap_or_default())
        .classes();

    rsx! {
        div { class: "{classes}", {props.children} }
    }
}
```

### **Utility Function Pattern**

```rust
impl MyComponentUtils {
    pub fn classes(size: &str, style: &str, custom: Option<&str>) -> String {
        my_classes_from_strings(
            VibeColors::default(),
            Some(size),
            Some(style),
            custom,
        )
    }
}
```

## 🚀 **Migration Strategy**

### **1. Create Patterns First**

Define semantic meaning before implementation

### **2. Build Parallel Builders**

Create new builders alongside existing code

### **3. Test Comprehensively**

Ensure new system matches old output

### **4. Migrate Incrementally**

Update components one at a time

### **5. Maintain Backward Compatibility**

Keep old APIs working during transition

This implementation approach has proven successful for Typography, Card, and Button systems, delivering **type safety**, **developer experience**, and **maintainability** while maintaining **zero breaking changes** during migration.
