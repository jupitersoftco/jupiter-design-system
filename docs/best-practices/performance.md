# Performance Optimization Best Practices

## ⚡ Performance Philosophy

The Jupiter Design System is built for performance by design, with compile-time optimizations, minimal runtime overhead, and efficient CSS class generation.

### Core Principles

1. **Compile-Time Generation**: All CSS classes are resolved at compile time
2. **Zero Runtime Cost**: No JavaScript runtime for style generation
3. **Dead Code Elimination**: Unused styles are automatically removed
4. **Minimal Bundle Size**: Only used design tokens are included
5. **Caching Strategies**: Intelligent caching for repeated operations

## 🏗️ Compile-Time Optimizations

### Static CSS Generation

```rust
// All style generation happens at compile time
let button_classes = button_styles(colors)
    .primary()
    .large()
    .classes(); // Resolves to static string at compile time

// No runtime JavaScript needed - just CSS classes
// Result: "inline-flex items-center justify-center px-6 py-3 text-lg font-medium rounded-md bg-jupiter-blue-500 text-white hover:bg-jupiter-blue-600"
```

### Compile-Time Theme Resolution

```rust
// Theme colors resolved at compile time
const BUTTON_PRIMARY: &str = "bg-jupiter-blue-500 text-white hover:bg-jupiter-blue-600";
const BUTTON_SECONDARY: &str = "bg-gray-100 text-gray-900 hover:bg-gray-200";

// No runtime theme switching overhead
fn get_button_classes(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Primary => BUTTON_PRIMARY,
        ButtonVariant::Secondary => BUTTON_SECONDARY,
        // ... other variants
    }
}
```

### Macro-Based Optimization

```rust
// Compile-time macro for frequently used patterns
macro_rules! common_button {
    (primary) => {
        "inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-md bg-jupiter-blue-500 text-white hover:bg-jupiter-blue-600 focus:outline-none focus:ring-2 focus:ring-jupiter-blue-500 focus:ring-offset-2"
    };
    (secondary) => {
        "inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-md bg-gray-100 text-gray-900 hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2"
    };
}

// Zero-cost abstraction
let primary_button = common_button!(primary);
```

## 🗄️ Caching Strategies

### Component-Level Caching

```rust
use std::collections::HashMap;
use std::sync::LazyLock;

// Global cache for common component combinations
static COMPONENT_CACHE: LazyLock<HashMap<ComponentKey, String>> = LazyLock::new(|| {
    let mut cache = HashMap::new();
    let colors = VibeColors::new();
    
    // Pre-populate with common combinations
    cache.insert(
        ComponentKey::Button(ButtonVariant::Primary, Size::Medium),
        button_styles(colors.clone()).primary().size(Size::Medium).classes()
    );
    
    cache.insert(
        ComponentKey::Button(ButtonVariant::Secondary, Size::Medium),
        button_styles(colors.clone()).secondary().size(Size::Medium).classes()
    );
    
    cache
});

#[derive(Hash, Eq, PartialEq)]
enum ComponentKey {
    Button(ButtonVariant, Size),
    Card(CardElevation, CardSpacing),
    Text(Typography, Color),
}

// Fast lookup for cached components
pub fn get_cached_component(key: ComponentKey) -> Option<&'static String> {
    COMPONENT_CACHE.get(&key)
}
```

### Theme-Specific Caching

```rust
use std::sync::Arc;
use dashmap::DashMap;

#[derive(Clone)]
pub struct CachedColorProvider {
    base_provider: Arc<dyn ColorProvider + Send + Sync>,
    class_cache: Arc<DashMap<(Color, ClassType), String>>,
}

#[derive(Hash, Eq, PartialEq)]
enum ClassType {
    Text,
    Background,
    Border,
}

impl CachedColorProvider {
    pub fn new(provider: impl ColorProvider + Send + Sync + 'static) -> Self {
        Self {
            base_provider: Arc::new(provider),
            class_cache: Arc::new(DashMap::new()),
        }
    }
}

impl ColorProvider for CachedColorProvider {
    fn palette(&self) -> &ColorPalette {
        // Access base provider palette
        unsafe { &*(self.base_provider.palette() as *const ColorPalette) }
    }
    
    fn text_class(&self, color: Color) -> String {
        self.class_cache
            .entry((color, ClassType::Text))
            .or_insert_with(|| self.base_provider.text_class(color))
            .clone()
    }
    
    fn bg_class(&self, color: Color) -> String {
        self.class_cache
            .entry((color, ClassType::Background))
            .or_insert_with(|| self.base_provider.bg_class(color))
            .clone()
    }
}
```

### Builder Pattern Optimization

```rust
// Optimized builder with method chaining and minimal allocations
#[derive(Clone)]
pub struct OptimizedButtonBuilder<C: ColorProvider> {
    variant: ButtonVariant,
    size: Size,
    state: ButtonState,
    full_width: bool,
    cached_result: Option<String>,
    color_provider: C,
}

impl<C: ColorProvider> OptimizedButtonBuilder<C> {
    // Implement Copy for small builders to avoid allocations
    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self.cached_result = None; // Invalidate cache
        self
    }
    
    pub fn large(mut self) -> Self {
        self.size = Size::Large;
        self.cached_result = None; // Invalidate cache
        self
    }
    
    pub fn classes(mut self) -> String {
        // Return cached result if available
        if let Some(cached) = self.cached_result {
            return cached;
        }
        
        // Generate and cache result
        let result = self.generate_classes();
        self.cached_result = Some(result.clone());
        result
    }
    
    fn generate_classes(&self) -> String {
        // Optimized class generation with string building
        let mut classes = Vec::with_capacity(10); // Pre-allocate
        
        // Base classes
        classes.push("inline-flex items-center justify-center font-medium rounded-md");
        
        // Variant classes
        match self.variant {
            ButtonVariant::Primary => {
                classes.push(self.color_provider.bg_class(Color::Primary).as_str());
                classes.push("text-white");
            }
            ButtonVariant::Secondary => {
                classes.push("bg-gray-100 text-gray-900");
            }
            // ... other variants
        }
        
        // Size classes
        match self.size {
            Size::Small => classes.push("px-3 py-2 text-sm"),
            Size::Medium => classes.push("px-4 py-2 text-sm"),
            Size::Large => classes.push("px-6 py-3 text-lg"),
            // ... other sizes
        }
        
        if self.full_width {
            classes.push("w-full");
        }
        
        classes.join(" ")
    }
}
```

## 📦 Bundle Size Optimization

### Conditional Compilation

```rust
// Only include used components
#[cfg(feature = "buttons")]
pub mod button;

#[cfg(feature = "cards")]
pub mod card;

#[cfg(feature = "forms")]
pub mod form;

// Cargo.toml
// [features]
// default = ["buttons", "cards"]
// buttons = []
// cards = []
// forms = []
// all = ["buttons", "cards", "forms", "layout", "typography"]
```

### Tree Shaking Support

```rust
// Export only used functions to enable dead code elimination
pub use crate::builders::{
    // Core components (always included)
    button_styles,
    
    // Optional components (conditionally compiled)
    #[cfg(feature = "cards")]
    card_styles,
    
    #[cfg(feature = "forms")]
    input_styles,
};

// Granular imports for better tree shaking
// Instead of: use jupiter_design_system::*;
// Use: use jupiter_design_system::{button_styles, VibeColors};
```

### Minimal Color Palettes

```rust
// Lightweight theme with only essential colors
#[derive(Debug, Clone)]
pub struct MinimalTheme {
    palette: MinimalColorPalette,
}

#[derive(Debug, Clone)]
pub struct MinimalColorPalette {
    pub primary: &'static str,
    pub background: &'static str,
    pub text: &'static str,
    pub border: &'static str,
}

impl Default for MinimalTheme {
    fn default() -> Self {
        Self {
            palette: MinimalColorPalette {
                primary: "blue-600",
                background: "white",
                text: "gray-900",
                border: "gray-300",
            },
        }
    }
}

impl ColorProvider for MinimalTheme {
    fn palette(&self) -> &ColorPalette {
        // Convert minimal palette to full palette only when needed
        &ColorPalette {
            primary: self.palette.primary.to_string(),
            background: self.palette.background.to_string(),
            text_primary: self.palette.text.to_string(),
            border: self.palette.border.to_string(),
            // Use defaults for unused colors
            ..Default::default()
        }
    }
}
```

## 🔄 Runtime Performance

### String Allocation Optimization

```rust
// Use Cow (Clone on Write) for efficient string handling
use std::borrow::Cow;

pub fn optimized_class_generation(base: &str, variants: &[&str]) -> Cow<str> {
    if variants.is_empty() {
        // No allocation needed
        Cow::Borrowed(base)
    } else {
        // Allocate only when necessary
        let mut result = String::with_capacity(base.len() + variants.iter().map(|v| v.len() + 1).sum::<usize>());
        result.push_str(base);
        for variant in variants {
            result.push(' ');
            result.push_str(variant);
        }
        Cow::Owned(result)
    }
}

// Pre-allocated string building
pub fn build_classes_efficiently(components: &[&str]) -> String {
    // Calculate exact capacity needed
    let capacity = components.iter().map(|c| c.len()).sum::<usize>() + components.len() - 1;
    let mut result = String::with_capacity(capacity);
    
    for (i, component) in components.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push_str(component);
    }
    
    result
}
```

### Memory-Efficient Builders

```rust
// Use bit flags for boolean options to reduce memory usage
#[derive(Clone, Copy)]
pub struct CompactButtonBuilder {
    variant: u8,        // 3 bits needed for 8 variants
    size: u8,          // 3 bits needed for 5 sizes  
    flags: u8,         // 8 flags for various options
    color_index: u8,   // Index into color palette
}

impl CompactButtonBuilder {
    const FULL_WIDTH_FLAG: u8 = 1 << 0;
    const WITH_ICON_FLAG: u8 = 1 << 1;
    const DISABLED_FLAG: u8 = 1 << 2;
    
    pub fn full_width(mut self) -> Self {
        self.flags |= Self::FULL_WIDTH_FLAG;
        self
    }
    
    pub fn with_icon(mut self) -> Self {
        self.flags |= Self::WITH_ICON_FLAG;
        self
    }
    
    fn has_flag(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }
}
```

### Lazy Static CSS Generation

```rust
use once_cell::sync::Lazy;

// Generate common CSS patterns once
static COMMON_BUTTON_CLASSES: Lazy<HashMap<(ButtonVariant, Size), &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Pre-generate most common combinations
    map.insert((ButtonVariant::Primary, Size::Medium), 
        "inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-md bg-jupiter-blue-500 text-white hover:bg-jupiter-blue-600 focus:outline-none focus:ring-2 focus:ring-jupiter-blue-500 focus:ring-offset-2");
    
    map.insert((ButtonVariant::Secondary, Size::Medium),
        "inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-md border border-gray-300 bg-white text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-jupiter-blue-500 focus:ring-offset-2");
    
    map
});

pub fn get_common_button_classes(variant: ButtonVariant, size: Size) -> Option<&'static str> {
    COMMON_BUTTON_CLASSES.get(&(variant, size)).copied()
}
```

## 🎯 CSS Optimization

### Utility-First Approach

```rust
// Generate atomic CSS utilities for maximum reuse
pub struct AtomicCSSGenerator;

impl AtomicCSSGenerator {
    // Generate reusable utility classes
    pub fn spacing_utilities() -> Vec<(&'static str, &'static str)> {
        vec![
            ("p-1", "padding: 0.25rem;"),
            ("p-2", "padding: 0.5rem;"),
            ("p-4", "padding: 1rem;"),
            ("p-6", "padding: 1.5rem;"),
            ("p-8", "padding: 2rem;"),
            // ... more utilities
        ]
    }
    
    pub fn color_utilities(theme: &dyn ColorProvider) -> Vec<(String, String)> {
        let palette = theme.palette();
        vec![
            (format!("text-primary"), format!("color: {};", palette.text_primary)),
            (format!("bg-primary"), format!("background-color: {};", palette.primary)),
            (format!("border-primary"), format!("border-color: {};", palette.primary)),
            // ... more color utilities
        ]
    }
}
```

### CSS Purging Integration

```rust
// Generate safelist for CSS purging tools
pub fn generate_css_safelist() -> Vec<String> {
    let mut safelist = Vec::new();
    
    // Add all possible design system classes
    for variant in [ButtonVariant::Primary, ButtonVariant::Secondary, ButtonVariant::Success] {
        for size in [Size::Small, Size::Medium, Size::Large] {
            let classes = button_styles(VibeColors::new())
                .variant(variant)
                .size(size)
                .classes();
                
            // Extract individual classes
            safelist.extend(classes.split_whitespace().map(String::from));
        }
    }
    
    // Remove duplicates
    safelist.sort();
    safelist.dedup();
    safelist
}

// Generate PurgeCSS configuration
pub fn generate_purgecss_config() -> serde_json::Value {
    serde_json::json!({
        "content": ["./src/**/*.rs", "./examples/**/*.rs"],
        "safelist": generate_css_safelist(),
        "defaultExtractor": {
            "extensions": ["rs"],
            "extractor": "([\\w\\-/:]+)"
        }
    })
}
```

## 📊 Performance Monitoring

### Build-Time Metrics

```rust
use std::time::Instant;

pub struct PerformanceMetrics {
    pub component_generation_time: std::time::Duration,
    pub cache_hit_rate: f64,
    pub memory_usage: usize,
    pub css_size: usize,
}

impl PerformanceMetrics {
    pub fn measure_component_generation<F, R>(f: F) -> (R, std::time::Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }
}

// Performance tracking during development
#[cfg(debug_assertions)]
pub fn track_performance<T>(name: &str, operation: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let result = operation();
    let duration = start.elapsed();
    
    if duration > std::time::Duration::from_millis(1) {
        println!("⚠️  Slow operation '{}': {:?}", name, duration);
    }
    
    result
}

#[cfg(not(debug_assertions))]
pub fn track_performance<T>(_name: &str, operation: impl FnOnce() -> T) -> T {
    operation()
}
```

### Memory Usage Analysis

```rust
use std::mem;

pub fn analyze_memory_usage() {
    println!("Memory usage analysis:");
    println!("  ButtonStyles: {} bytes", mem::size_of::<ButtonStyles<VibeColors>>());
    println!("  CardStyles: {} bytes", mem::size_of::<CardStyles<VibeColors>>());
    println!("  ColorPalette: {} bytes", mem::size_of::<ColorPalette>());
    println!("  VibeColors: {} bytes", mem::size_of::<VibeColors>());
}

// Track allocations in hot paths
pub struct AllocationTracker {
    allocations: std::cell::RefCell<Vec<(String, usize)>>,
}

impl AllocationTracker {
    pub fn track_allocation(&self, name: String, size: usize) {
        self.allocations.borrow_mut().push((name, size));
    }
    
    pub fn report(&self) {
        let allocations = self.allocations.borrow();
        let total: usize = allocations.iter().map(|(_, size)| size).sum();
        
        println!("Total allocations: {} bytes", total);
        for (name, size) in allocations.iter() {
            println!("  {}: {} bytes", name, size);
        }
    }
}
```

## 🧪 Performance Testing

### Benchmark Suite

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn bench_button_generation(c: &mut Criterion) {
        let colors = VibeColors::new();
        
        c.bench_function("button_primary", |b| {
            b.iter(|| {
                black_box(button_styles(colors.clone()).primary().classes())
            })
        });
        
        c.bench_function("button_cached", |b| {
            b.iter(|| {
                black_box(get_common_button_classes(ButtonVariant::Primary, Size::Medium))
            })
        });
    }

    fn bench_theme_operations(c: &mut Criterion) {
        let colors = VibeColors::new();
        let cached_colors = CachedColorProvider::new(colors);
        
        c.bench_function("theme_resolve_uncached", |b| {
            b.iter(|| {
                black_box(colors.text_class(Color::Primary))
            })
        });
        
        c.bench_function("theme_resolve_cached", |b| {
            b.iter(|| {
                black_box(cached_colors.text_class(Color::Primary))
            })
        });
    }

    fn bench_complex_component(c: &mut Criterion) {
        let colors = VibeColors::new();
        
        c.bench_function("complex_card", |b| {
            b.iter(|| {
                black_box({
                    let header = card_header_styles(colors.clone()).classes();
                    let content = card_content_styles(colors.clone()).classes();
                    let footer = card_footer_styles(colors.clone()).classes();
                    format!("{} {} {}", header, content, footer)
                })
            })
        });
    }

    criterion_group!(benches, bench_button_generation, bench_theme_operations, bench_complex_component);
    criterion_main!(benches);
}
```

### Load Testing

```rust
use std::sync::Arc;
use std::thread;

pub fn stress_test_component_generation() {
    let colors = Arc::new(VibeColors::new());
    let mut handles = Vec::new();
    
    // Spawn multiple threads generating components
    for i in 0..10 {
        let colors = Arc::clone(&colors);
        let handle = thread::spawn(move || {
            for j in 0..1000 {
                let _ = button_styles(colors.as_ref())
                    .variant(match j % 3 {
                        0 => ButtonVariant::Primary,
                        1 => ButtonVariant::Secondary,
                        _ => ButtonVariant::Success,
                    })
                    .size(match j % 3 {
                        0 => Size::Small,
                        1 => Size::Medium,
                        _ => Size::Large,
                    })
                    .classes();
            }
            println!("Thread {} completed", i);
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Stress test completed successfully");
}
```

## 🚦 Performance Guidelines

### Do's ✅

1. **Use Caching for Repeated Operations**
   ```rust
   // ✅ Good: Cache frequently used combinations
   static CACHED_BUTTONS: Lazy<HashMap<ButtonVariant, String>> = Lazy::new(|| {
       let colors = VibeColors::new();
       let mut cache = HashMap::new();
       cache.insert(ButtonVariant::Primary, button_styles(colors.clone()).primary().classes());
       cache.insert(ButtonVariant::Secondary, button_styles(colors).secondary().classes());
       cache
   });
   ```

2. **Pre-allocate String Capacity**
   ```rust
   // ✅ Good: Pre-allocate with known capacity
   let mut classes = String::with_capacity(100);
   classes.push_str("base-classes ");
   classes.push_str("variant-classes");
   ```

3. **Use Static Strings When Possible**
   ```rust
   // ✅ Good: Static strings for immutable content
   const BUTTON_BASE: &str = "inline-flex items-center justify-center font-medium rounded-md";
   ```

### Don'ts ❌

1. **Don't Generate Classes in Hot Loops**
   ```rust
   // ❌ Bad: Repeated generation
   for item in items {
       let classes = button_styles(colors.clone()).primary().classes(); // Regenerated each time
   }
   
   // ✅ Good: Generate once, reuse
   let button_classes = button_styles(colors).primary().classes();
   for item in items {
       // Use button_classes
   }
   ```

2. **Don't Clone Heavy Objects Unnecessarily**
   ```rust
   // ❌ Bad: Unnecessary cloning
   fn process_items(items: Vec<Item>, colors: VibeColors) {
       for item in items {
           let classes = button_styles(colors.clone()).primary().classes();
       }
   }
   
   // ✅ Good: Use references
   fn process_items(items: &[Item], colors: &VibeColors) {
       for item in items {
           let classes = button_styles(colors).primary().classes();
       }
   }
   ```

3. **Don't Ignore Compile-Time Optimizations**
   ```rust
   // ❌ Bad: Runtime string building
   let classes = format!("bg-{} text-{}", primary_color, text_color);
   
   // ✅ Good: Compile-time resolution
   let classes = button_styles(colors).primary().classes();
   ```

## 📈 Performance Metrics

### Target Performance Goals

- **Component Generation**: < 1μs per component
- **Theme Resolution**: < 100ns per color lookup
- **Memory Usage**: < 1KB per component instance
- **CSS Bundle Size**: < 50KB for complete design system
- **Cache Hit Rate**: > 90% for repeated operations

### Monitoring and Alerts

```rust
pub struct PerformanceMonitor {
    generation_times: Vec<std::time::Duration>,
    cache_hits: usize,
    cache_misses: usize,
}

impl PerformanceMonitor {
    pub fn record_generation_time(&mut self, duration: std::time::Duration) {
        self.generation_times.push(duration);
        
        // Alert if generation takes too long
        if duration > std::time::Duration::from_micros(10) {
            eprintln!("⚠️  Slow component generation: {:?}", duration);
        }
    }
    
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
        
        // Alert if cache hit rate is too low
        let total = self.cache_hits + self.cache_misses;
        if total > 100 && (self.cache_hits as f64 / total as f64) < 0.8 {
            eprintln!("⚠️  Low cache hit rate: {:.2}%", 
                (self.cache_hits as f64 / total as f64) * 100.0);
        }
    }
    
    pub fn report(&self) {
        let total_cache = self.cache_hits + self.cache_misses;
        let hit_rate = if total_cache > 0 {
            (self.cache_hits as f64 / total_cache as f64) * 100.0
        } else {
            0.0
        };
        
        println!("Performance Report:");
        println!("  Average generation time: {:?}", 
            self.generation_times.iter().sum::<std::time::Duration>() / self.generation_times.len() as u32);
        println!("  Cache hit rate: {:.2}%", hit_rate);
        println!("  Total operations: {}", self.generation_times.len());
    }
}
```

## 🔗 Related Documentation

- [Overview](./overview.md) - Understanding the architecture for performance
- [Component Guidelines](./components.md) - Efficient component usage patterns
- [Theming Guide](./theming.md) - Performance-optimized theme creation
- [Examples](./examples/) - Real-world performance optimization examples