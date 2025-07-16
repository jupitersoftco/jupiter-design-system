# Troubleshooting Guide

## 🔧 Common Issues and Solutions

This guide covers the most common issues encountered when using the Jupiter Design System and provides practical solutions.

## 🎨 Color and Theming Issues

### Issue: Colors Not Appearing Correctly

**Symptoms:**
- Components show default/fallback colors instead of theme colors
- Colors appear as class names in the DOM instead of actual colors

**Common Causes:**
```rust
// ❌ Problem: Using raw color strings instead of theme resolution
let button_classes = "bg-jupiter-blue-500 text-white"; // Hard-coded

// ❌ Problem: Theme not properly configured
let incomplete_theme = ColorPalette {
    primary: "".to_string(), // Empty color value
    // Missing other required colors...
    ..Default::default()
};
```

**Solutions:**
```rust
// ✅ Solution: Use proper theme resolution
let colors = VibeColors::new();
let button_classes = button_styles(colors).primary().classes();

// ✅ Solution: Ensure complete theme configuration
let custom_theme = VibeColors::with_overrides(|palette| {
    palette.primary = "purple-600".to_string();
    palette.interactive = "purple-600".to_string();
    palette.interactive_hover = "purple-700".to_string();
    // Ensure all related colors are updated
});

// ✅ Solution: Validate theme completeness
fn validate_theme(theme: &dyn ColorProvider) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let palette = theme.palette();
    
    if palette.primary.is_empty() {
        errors.push("Primary color is not defined".to_string());
    }
    if palette.text_primary.is_empty() {
        errors.push("Primary text color is not defined".to_string());
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

### Issue: Theme Changes Not Reflecting

**Symptoms:**
- Components continue showing old theme colors after theme updates
- Inconsistent colors across different components

**Solutions:**
```rust
// ✅ Solution: Ensure theme instance consistency
lazy_static! {
    static ref GLOBAL_THEME: VibeColors = VibeColors::new();
}

// Use the same theme instance across components
let button_classes = button_styles(GLOBAL_THEME.clone()).primary().classes();
let card_classes = card_styles(GLOBAL_THEME.clone()).elevation(CardElevation::Medium).classes();

// ✅ Solution: Theme update propagation
pub struct ThemeManager {
    current_theme: Arc<dyn ColorProvider + Send + Sync>,
    subscribers: Vec<Box<dyn Fn(&dyn ColorProvider) + Send + Sync>>,
}

impl ThemeManager {
    pub fn update_theme(&mut self, new_theme: impl ColorProvider + Send + Sync + 'static) {
        self.current_theme = Arc::new(new_theme);
        
        // Notify all subscribers of theme change
        for subscriber in &self.subscribers {
            subscriber(self.current_theme.as_ref());
        }
    }
}
```

## 🏗️ Component Building Issues

### Issue: Builder Methods Not Chaining

**Symptoms:**
- Compilation errors when chaining builder methods
- Methods not available on builder instances

**Common Causes:**
```rust
// ❌ Problem: Not importing the correct traits
use jupiter_design_system::ButtonStyles; // Missing traits

// ❌ Problem: Trying to call methods on final output
let classes = button_styles(colors).primary().classes();
let more_classes = classes.size(Size::Large); // Error: classes is String, not builder
```

**Solutions:**
```rust
// ✅ Solution: Import prelude for all traits
use jupiter_design_system::prelude::*;

// ✅ Solution: Chain methods before calling .classes()
let button_classes = button_styles(colors)
    .primary()
    .size(Size::Large)
    .full_width()
    .state(ButtonState::Default)
    .classes(); // Only call .classes() at the end

// ✅ Solution: Use builder pattern correctly
let builder = button_styles(colors)
    .primary()
    .size(Size::Large);

// Can continue building
let final_classes = builder.full_width().classes();
```

### Issue: Interactive States Not Working

**Symptoms:**
- Hover/focus effects not appearing
- Interactive elements not responding to user interaction

**Common Causes:**
```rust
// ❌ Problem: Using static state instead of CSS pseudo-classes
let hover_button = button_styles(colors)
    .primary()
    .state(ButtonState::Hover) // This sets a static hover appearance
    .classes();

// ❌ Problem: Missing interactive styles
let button_classes = button_styles(colors).primary().classes();
// Missing hover, focus, active states
```

**Solutions:**
```rust
// ✅ Solution: Use interactive builder for dynamic states
use jupiter_design_system::builders::interactive::*;

let interactive_button = interactive_button(colors)
    .primary()
    .hover()
        .darken()
        .scale_105()
    .focus()
        .ring_primary()
        .outline_none()
    .active()
        .scale_95()
    .build();

// ✅ Solution: Ensure CSS pseudo-classes are supported
// Make sure your CSS framework (Tailwind) includes hover: focus: prefixes
// and that PurgeCSS doesn't remove them

// ✅ Solution: For static preview of states
let preview_hover = button_styles(colors)
    .primary()
    .state(ButtonState::Hover) // For showcasing hover state
    .classes();
```

## 📝 CSS Integration Issues

### Issue: Classes Not Applied in Browser

**Symptoms:**
- Generated class names appear in DOM but no styling is applied
- Missing styles in browser developer tools

**Common Causes:**
- CSS framework not loaded or configured correctly
- PurgeCSS removing required classes
- Class names not matching CSS framework conventions

**Solutions:**
```rust
// ✅ Solution: Verify CSS framework integration
// Ensure Tailwind CSS (or your CSS framework) is properly loaded

// ✅ Solution: Configure PurgeCSS safelist
pub fn generate_purgecss_safelist() -> Vec<String> {
    let mut safelist = Vec::new();
    let colors = VibeColors::new();
    
    // Generate all possible design system classes
    for variant in [ButtonVariant::Primary, ButtonVariant::Secondary, ButtonVariant::Success] {
        for size in [Size::Small, Size::Medium, Size::Large] {
            let classes = button_styles(colors.clone())
                .variant(variant)
                .size(size)
                .classes();
            
            safelist.extend(classes.split_whitespace().map(String::from));
        }
    }
    
    safelist.sort();
    safelist.dedup();
    safelist
}

// ✅ Solution: Debug class generation
pub fn debug_component_classes(component_name: &str, classes: &str) {
    #[cfg(debug_assertions)]
    {
        println!("DEBUG: {} classes: {}", component_name, classes);
        
        // Verify no empty classes
        let class_list: Vec<&str> = classes.split_whitespace().collect();
        for class in &class_list {
            if class.is_empty() {
                eprintln!("WARNING: Empty class found in {}", component_name);
            }
        }
        
        println!("DEBUG: {} generated {} classes", component_name, class_list.len());
    }
}
```

### Issue: Conflicting CSS Classes

**Symptoms:**
- Styles not appearing as expected
- Some properties being overridden unexpectedly

**Solutions:**
```rust
// ✅ Solution: CSS class validation
pub fn validate_css_classes(classes: &str) -> Vec<String> {
    let mut conflicts = Vec::new();
    let class_list: Vec<&str> = classes.split_whitespace().collect();
    
    // Check for display conflicts
    let display_classes: Vec<&str> = class_list.iter()
        .filter(|c| c.starts_with("block") || c.starts_with("inline") || c.starts_with("flex"))
        .cloned()
        .collect();
    
    if display_classes.len() > 1 {
        conflicts.push(format!("Display conflict: {:?}", display_classes));
    }
    
    // Check for position conflicts
    let position_classes: Vec<&str> = class_list.iter()
        .filter(|c| c.starts_with("absolute") || c.starts_with("relative") || c.starts_with("fixed"))
        .cloned()
        .collect();
    
    if position_classes.len() > 1 {
        conflicts.push(format!("Position conflict: {:?}", position_classes));
    }
    
    conflicts
}

// ✅ Solution: Use class merging utility
use std::collections::HashSet;

pub fn merge_classes(base_classes: &str, additional_classes: &str) -> String {
    let mut class_set = HashSet::new();
    
    // Add base classes
    for class in base_classes.split_whitespace() {
        class_set.insert(class);
    }
    
    // Add additional classes (may override base)
    for class in additional_classes.split_whitespace() {
        // Remove conflicting classes before adding new ones
        if class.starts_with("bg-") {
            class_set.retain(|c| !c.starts_with("bg-"));
        }
        if class.starts_with("text-") {
            class_set.retain(|c| !c.starts_with("text-"));
        }
        
        class_set.insert(class);
    }
    
    let mut classes: Vec<&str> = class_set.into_iter().collect();
    classes.sort();
    classes.join(" ")
}
```

## 🎭 Framework Integration Issues

### Issue: Rust Framework Compatibility

**Symptoms:**
- Compilation errors in specific frameworks
- Type conflicts or missing trait implementations

**Solutions for Dioxus:**
```rust
// ✅ Solution: Proper Dioxus integration
use dioxus::prelude::*;
use jupiter_design_system::prelude::*;

#[component]
fn Button(
    variant: ButtonVariant,
    size: Size,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    // Create theme at component level or use context
    let colors = use_context::<VibeColors>();
    
    let classes = button_styles(colors)
        .variant(variant)
        .size(size)
        .classes();
    
    rsx! {
        button {
            class: "{classes}",
            onclick: move |e| onclick.call(e),
            {children}
        }
    }
}

// ✅ Solution: Theme context provider
#[component]
fn App() -> Element {
    let theme = VibeColors::new();
    
    rsx! {
        div {
            // Provide theme to all child components
            Provider::<VibeColors> { value: theme,
                Button {
                    variant: ButtonVariant::Primary,
                    size: Size::Large,
                    onclick: |_| println!("Clicked!"),
                    "Click me"
                }
            }
        }
    }
}
```

**Solutions for Yew:**
```rust
// ✅ Solution: Yew integration
use yew::prelude::*;
use jupiter_design_system::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub size: Size,
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let theme = use_context::<VibeColors>().expect("Theme context not found");
    
    let classes = button_styles(theme)
        .variant(props.variant)
        .size(props.size)
        .classes();
    
    let onclick = props.onclick.clone();
    
    html! {
        <button class={classes} onclick={onclick}>
            {for props.children.iter()}
        </button>
    }
}
```

## 🔍 Performance Issues

### Issue: Slow Component Generation

**Symptoms:**
- Noticeable delay when generating component classes
- High CPU usage during style generation

**Solutions:**
```rust
// ✅ Solution: Use caching for repeated patterns
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref COMPONENT_CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn cached_button_styles(
    variant: ButtonVariant,
    size: Size,
    colors: &VibeColors,
) -> String {
    let cache_key = format!("{:?}-{:?}", variant, size);
    
    let mut cache = COMPONENT_CACHE.lock().unwrap();
    
    if let Some(cached) = cache.get(&cache_key) {
        return cached.clone();
    }
    
    let classes = button_styles(colors.clone())
        .variant(variant)
        .size(size)
        .classes();
    
    cache.insert(cache_key, classes.clone());
    classes
}

// ✅ Solution: Pre-generate common combinations
lazy_static! {
    static ref COMMON_BUTTONS: HashMap<(ButtonVariant, Size), String> = {
        let colors = VibeColors::new();
        let mut map = HashMap::new();
        
        for variant in [ButtonVariant::Primary, ButtonVariant::Secondary, ButtonVariant::Success] {
            for size in [Size::Small, Size::Medium, Size::Large] {
                let classes = button_styles(colors.clone())
                    .variant(variant)
                    .size(size)
                    .classes();
                map.insert((variant, size), classes);
            }
        }
        
        map
    };
}

pub fn get_common_button(variant: ButtonVariant, size: Size) -> Option<&'static String> {
    COMMON_BUTTONS.get(&(variant, size))
}
```

### Issue: Memory Leaks

**Symptoms:**
- Increasing memory usage over time
- Application slowdown after extended use

**Solutions:**
```rust
// ✅ Solution: Avoid unnecessary cloning
// Instead of:
fn bad_pattern(colors: VibeColors) {
    for _ in 0..1000 {
        let cloned_colors = colors.clone(); // Unnecessary clone
        let _ = button_styles(cloned_colors).primary().classes();
    }
}

// Do this:
fn good_pattern(colors: &VibeColors) {
    for _ in 0..1000 {
        let _ = button_styles(colors.clone()).primary().classes(); // Only clone when needed
    }
}

// ✅ Solution: Use Arc for shared themes
use std::sync::Arc;

pub struct SharedTheme {
    colors: Arc<VibeColors>,
}

impl SharedTheme {
    pub fn new() -> Self {
        Self {
            colors: Arc::new(VibeColors::new()),
        }
    }
    
    pub fn button_classes(&self, variant: ButtonVariant, size: Size) -> String {
        button_styles(self.colors.as_ref().clone())
            .variant(variant)
            .size(size)
            .classes()
    }
}
```

## 🔧 Debugging Tools

### Debug Mode

```rust
#[cfg(debug_assertions)]
pub mod debug {
    use super::*;
    
    pub fn trace_component_generation(component_name: &str, classes: &str) {
        println!("[JUPITER DEBUG] {}: {}", component_name, classes);
        
        // Validate class structure
        let class_count = classes.split_whitespace().count();
        if class_count > 20 {
            println!("[JUPITER WARNING] {} has {} classes - consider optimization", 
                    component_name, class_count);
        }
        
        // Check for common issues
        if classes.contains("  ") {
            println!("[JUPITER WARNING] {} has double spaces in classes", component_name);
        }
        
        if classes.starts_with(' ') || classes.ends_with(' ') {
            println!("[JUPITER WARNING] {} has leading/trailing spaces", component_name);
        }
    }
    
    pub fn analyze_theme(theme: &dyn ColorProvider) {
        let palette = theme.palette();
        
        println!("[JUPITER DEBUG] Theme Analysis:");
        println!("  Primary: {}", palette.primary);
        println!("  Secondary: {}", palette.secondary);
        println!("  Success: {}", palette.success);
        println!("  Error: {}", palette.error);
        
        // Check for potential issues
        if palette.primary == palette.secondary {
            println!("[JUPITER WARNING] Primary and secondary colors are identical");
        }
        
        if palette.text_primary.is_empty() {
            println!("[JUPITER ERROR] Primary text color is not defined");
        }
    }
}

// Usage in development
pub fn debug_button(variant: ButtonVariant, size: Size, colors: &VibeColors) -> String {
    let classes = button_styles(colors.clone())
        .variant(variant)
        .size(size)
        .classes();
    
    #[cfg(debug_assertions)]
    debug::trace_component_generation("Button", &classes);
    
    classes
}
```

### Validation Tools

```rust
pub struct ValidationReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
}

pub fn validate_component_usage(component_type: &str, classes: &str) -> ValidationReport {
    let mut report = ValidationReport {
        errors: Vec::new(),
        warnings: Vec::new(),
        suggestions: Vec::new(),
    };
    
    // Basic validation
    if classes.is_empty() {
        report.errors.push("Component generated empty classes".to_string());
        return report;
    }
    
    // Component-specific validation
    match component_type {
        "button" => {
            if !classes.contains("inline-flex") && !classes.contains("flex") {
                report.warnings.push("Button should typically use flex display".to_string());
            }
            
            if !classes.contains("px-") {
                report.warnings.push("Button should have horizontal padding".to_string());
            }
            
            if !classes.contains("focus:") {
                report.suggestions.push("Consider adding focus styles for accessibility".to_string());
            }
        }
        "card" => {
            if !classes.contains("bg-") {
                report.warnings.push("Card should have background color".to_string());
            }
            
            if !classes.contains("rounded") && !classes.contains("border-radius") {
                report.suggestions.push("Consider adding border radius for modern appearance".to_string());
            }
        }
        _ => {}
    }
    
    // Check for conflicts
    let conflicts = validate_css_classes(classes);
    for conflict in conflicts {
        report.errors.push(conflict);
    }
    
    report
}
```

## 📞 Getting Help

### Error Messages

When encountering issues, look for these common error patterns:

1. **"trait bound not satisfied"** - Usually means missing imports from prelude
2. **"method not found"** - Check if you're calling methods on the right type
3. **"cannot borrow as mutable"** - Review ownership and borrowing patterns
4. **"lifetime parameter required"** - May need to adjust theme provider usage

### Community Resources

1. **Documentation**: Check the [API documentation](../IMPLEMENTATION.md)
2. **Examples**: Review [examples directory](./examples/) for patterns
3. **Issues**: Search existing issues in the project repository
4. **Discussions**: Join community discussions for design system best practices

### Creating Bug Reports

When reporting issues, include:

```rust
// Minimal reproduction case
use jupiter_design_system::prelude::*;

fn reproduce_issue() {
    let colors = VibeColors::new();
    
    // Exact code that demonstrates the problem
    let classes = button_styles(colors)
        .primary()
        .classes();
    
    println!("Generated classes: {}", classes);
    println!("Expected: contains bg-jupiter-blue-500");
    println!("Actual: {}", classes.contains("bg-jupiter-blue-500"));
}
```

Include:
- Rust version (`rustc --version`)
- Jupiter Design System version
- Operating system
- CSS framework being used
- Complete error messages
- Expected vs actual behavior

This troubleshooting guide covers the most common issues. For additional help, refer to the project documentation or create an issue with a detailed reproduction case.