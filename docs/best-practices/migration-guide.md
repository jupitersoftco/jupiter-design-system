# Migration Guide

## 🔄 Migration Philosophy

This guide helps teams migrate to the Jupiter Design System from existing design systems or ad-hoc styling approaches while maintaining design consistency and minimizing disruption.

## 📋 Pre-Migration Assessment

### Current State Analysis

```rust
// Assessment tool for existing codebase
pub struct DesignSystemAudit {
    pub css_files: Vec<String>,
    pub component_files: Vec<String>,
    pub color_usage: ColorUsageReport,
    pub typography_usage: TypographyUsageReport,
    pub spacing_patterns: SpacingPatternReport,
}

impl DesignSystemAudit {
    pub fn analyze_codebase(root_path: &str) -> Self {
        // Scan codebase for design patterns
        let css_files = find_css_files(root_path);
        let component_files = find_component_files(root_path);
        
        Self {
            css_files,
            component_files,
            color_usage: analyze_color_usage(&css_files),
            typography_usage: analyze_typography_usage(&css_files),
            spacing_patterns: analyze_spacing_patterns(&css_files),
        }
    }
    
    pub fn generate_migration_plan(&self) -> MigrationPlan {
        MigrationPlan {
            priority_components: self.identify_high_impact_components(),
            color_mapping: self.suggest_color_mappings(),
            breaking_changes: self.identify_breaking_changes(),
            estimated_effort: self.estimate_migration_effort(),
        }
    }
}

#[derive(Debug)]
pub struct ColorUsageReport {
    pub unique_colors: Vec<String>,
    pub color_frequency: std::collections::HashMap<String, usize>,
    pub semantic_opportunities: Vec<String>,
}

#[derive(Debug)]
pub struct MigrationPlan {
    pub priority_components: Vec<ComponentMigration>,
    pub color_mapping: std::collections::HashMap<String, Color>,
    pub breaking_changes: Vec<BreakingChange>,
    pub estimated_effort: MigrationEffort,
}
```

### Compatibility Matrix

| Current System | Migration Complexity | Recommended Approach |
|---------------|---------------------|---------------------|
| **Tailwind CSS** | Low | Direct mapping, use Jupiter tokens |
| **Bootstrap** | Medium | Component-by-component replacement |
| **Material-UI** | Medium | Theme override, gradual adoption |
| **Custom CSS** | High | Full audit and systematic replacement |
| **Styled Components** | Medium | Wrapper approach with Jupiter classes |
| **CSS Modules** | Medium | Module-by-module replacement |

## 🎯 Migration Strategies

### Strategy 1: Gradual Migration (Recommended)

```rust
// Phase 1: Introduce Jupiter alongside existing system
pub mod migration_phase1 {
    use jupiter_design_system::prelude::*;
    
    // Wrapper functions for gradual adoption
    pub fn legacy_button_to_jupiter(legacy_variant: &str, legacy_size: &str) -> String {
        let colors = VibeColors::new();
        
        let variant = match legacy_variant {
            "btn-primary" => ButtonVariant::Primary,
            "btn-secondary" => ButtonVariant::Secondary,
            "btn-success" => ButtonVariant::Success,
            "btn-danger" => ButtonVariant::Error,
            _ => ButtonVariant::Secondary,
        };
        
        let size = match legacy_size {
            "btn-sm" => Size::Small,
            "btn-lg" => Size::Large,
            _ => Size::Medium,
        };
        
        button_styles(colors)
            .variant(variant)
            .size(size)
            .classes()
    }
    
    // Compatibility layer for existing components
    pub fn migrate_legacy_card(legacy_classes: &str) -> String {
        let colors = VibeColors::new();
        
        let elevation = if legacy_classes.contains("shadow-lg") {
            CardElevation::High
        } else if legacy_classes.contains("shadow") {
            CardElevation::Medium
        } else {
            CardElevation::Low
        };
        
        card_styles(colors)
            .elevation(elevation)
            .spacing(CardSpacing::Medium)
            .classes()
    }
}

// Phase 2: Component-by-component replacement
pub mod migration_phase2 {
    use jupiter_design_system::prelude::*;
    
    // Replace existing components with Jupiter equivalents
    pub struct MigratedButton {
        pub jupiter_classes: String,
        pub legacy_classes: String, // Keep for fallback
        pub migration_complete: bool,
    }
    
    impl MigratedButton {
        pub fn new(variant: ButtonVariant, size: Size, feature_flag: bool) -> Self {
            let colors = VibeColors::new();
            let jupiter_classes = button_styles(colors)
                .variant(variant)
                .size(size)
                .classes();
            
            let legacy_classes = match variant {
                ButtonVariant::Primary => "btn btn-primary",
                ButtonVariant::Secondary => "btn btn-secondary",
                _ => "btn btn-default",
            }.to_string();
            
            Self {
                jupiter_classes,
                legacy_classes,
                migration_complete: feature_flag,
            }
        }
        
        pub fn get_classes(&self) -> &str {
            if self.migration_complete {
                &self.jupiter_classes
            } else {
                &self.legacy_classes
            }
        }
    }
}
```

### Strategy 2: Theme-Based Migration

```rust
// Create a bridge theme that maps existing design tokens
pub struct BridgeTheme {
    legacy_colors: LegacyColorPalette,
    jupiter_colors: ColorPalette,
}

impl BridgeTheme {
    pub fn from_legacy_system(legacy_tokens: LegacyDesignTokens) -> Self {
        Self {
            legacy_colors: legacy_tokens.colors,
            jupiter_colors: ColorPalette {
                primary: legacy_tokens.colors.brand_primary,
                secondary: legacy_tokens.colors.brand_secondary,
                success: legacy_tokens.colors.semantic_success,
                warning: legacy_tokens.colors.semantic_warning,
                error: legacy_tokens.colors.semantic_error,
                // Map other colors...
                ..Default::default()
            },
        }
    }
}

// Legacy token structure (example)
#[derive(Debug, Clone)]
pub struct LegacyDesignTokens {
    pub colors: LegacyColorPalette,
    pub spacing: LegacySpacing,
    pub typography: LegacyTypography,
}

#[derive(Debug, Clone)]
pub struct LegacyColorPalette {
    pub brand_primary: String,
    pub brand_secondary: String,
    pub semantic_success: String,
    pub semantic_warning: String,
    pub semantic_error: String,
    // Other legacy colors...
}
```

## 🔧 Component Migration Patterns

### Button Migration

```rust
// Before: Legacy button component
pub struct LegacyButton {
    pub variant: String,     // "primary", "secondary", etc.
    pub size: String,        // "small", "medium", "large"
    pub disabled: bool,
    pub loading: bool,
    pub full_width: bool,
}

impl LegacyButton {
    pub fn get_css_classes(&self) -> String {
        format!("btn btn-{} btn-{} {} {} {}",
            self.variant,
            self.size,
            if self.disabled { "btn-disabled" } else { "" },
            if self.loading { "btn-loading" } else { "" },
            if self.full_width { "btn-block" } else { "" }
        ).trim().to_string()
    }
}

// After: Jupiter-based button
pub struct MigratedButton {
    colors: VibeColors,
}

impl MigratedButton {
    pub fn new() -> Self {
        Self {
            colors: VibeColors::new(),
        }
    }
    
    pub fn get_css_classes(
        &self,
        variant: ButtonVariant,
        size: Size,
        state: ButtonState,
        full_width: bool,
    ) -> String {
        button_styles(self.colors.clone())
            .variant(variant)
            .size(size)
            .state(state)
            .full_width(full_width)
            .classes()
    }
}

// Migration mapping function
pub fn migrate_legacy_button_props(legacy: &LegacyButton) -> (ButtonVariant, Size, ButtonState, bool) {
    let variant = match legacy.variant.as_str() {
        "primary" => ButtonVariant::Primary,
        "secondary" => ButtonVariant::Secondary,
        "success" => ButtonVariant::Success,
        "danger" | "error" => ButtonVariant::Error,
        "warning" => ButtonVariant::Warning,
        _ => ButtonVariant::Secondary,
    };
    
    let size = match legacy.size.as_str() {
        "small" | "sm" => Size::Small,
        "large" | "lg" => Size::Large,
        _ => Size::Medium,
    };
    
    let state = if legacy.disabled {
        ButtonState::Disabled
    } else if legacy.loading {
        ButtonState::Loading
    } else {
        ButtonState::Default
    };
    
    (variant, size, state, legacy.full_width)
}
```

### Card Migration

```rust
// Legacy card structure
pub struct LegacyCard {
    pub shadow: String,      // "none", "sm", "md", "lg"
    pub padding: String,     // "sm", "md", "lg"
    pub border: bool,
    pub rounded: bool,
}

// Migration function
pub fn migrate_card_component(legacy: &LegacyCard, colors: impl ColorProvider) -> String {
    let elevation = match legacy.shadow.as_str() {
        "lg" | "xl" => CardElevation::High,
        "md" => CardElevation::Medium,
        "sm" => CardElevation::Low,
        _ => CardElevation::None,
    };
    
    let spacing = match legacy.padding.as_str() {
        "sm" => CardSpacing::Small,
        "lg" => CardSpacing::Large,
        "xl" => CardSpacing::XLarge,
        _ => CardSpacing::Medium,
    };
    
    card_styles(colors)
        .elevation(elevation)
        .spacing(spacing)
        .classes()
}
```

### Typography Migration

```rust
// Legacy typography mapping
pub fn migrate_typography_classes(legacy_class: &str, colors: impl ColorProvider) -> String {
    let (typography, color) = match legacy_class {
        "h1" | "heading-1" => (Typography::Heading1, Color::TextPrimary),
        "h2" | "heading-2" => (Typography::Heading2, Color::TextPrimary),
        "h3" | "heading-3" => (Typography::Heading3, Color::TextPrimary),
        "body" | "paragraph" => (Typography::Body, Color::TextSecondary),
        "small" | "caption" => (Typography::Caption, Color::TextTertiary),
        "label" | "form-label" => (Typography::Label, Color::TextPrimary),
        _ => (Typography::Body, Color::TextSecondary),
    };
    
    text_styles(colors)
        .typography(typography)
        .color(color)
        .classes()
}

// Batch migration tool
pub fn migrate_typography_across_files(file_patterns: &[&str]) -> MigrationReport {
    let mut report = MigrationReport::new();
    
    for pattern in file_patterns {
        let files = glob::glob(pattern).unwrap();
        
        for file_path in files {
            let file_path = file_path.unwrap();
            let content = std::fs::read_to_string(&file_path).unwrap();
            
            // Find and replace legacy typography classes
            let updated_content = replace_legacy_typography(&content);
            
            if content != updated_content {
                report.add_file_change(file_path, content.len(), updated_content.len());
                std::fs::write(&file_path, updated_content).unwrap();
            }
        }
    }
    
    report
}
```

## 🧪 Testing During Migration

### Compatibility Testing

```rust
#[cfg(test)]
mod migration_tests {
    use super::*;

    #[test]
    fn test_legacy_button_mapping_accuracy() {
        let legacy_buttons = vec![
            LegacyButton {
                variant: "primary".to_string(),
                size: "large".to_string(),
                disabled: false,
                loading: false,
                full_width: true,
            },
            LegacyButton {
                variant: "secondary".to_string(),
                size: "small".to_string(),
                disabled: true,
                loading: false,
                full_width: false,
            },
        ];
        
        let migrated_button = MigratedButton::new();
        
        for legacy in &legacy_buttons {
            let (variant, size, state, full_width) = migrate_legacy_button_props(legacy);
            let jupiter_classes = migrated_button.get_css_classes(variant, size, state, full_width);
            let legacy_classes = legacy.get_css_classes();
            
            // Both should generate valid CSS classes
            assert!(!jupiter_classes.is_empty());
            assert!(!legacy_classes.is_empty());
            
            // Verify semantic equivalence
            if legacy.variant == "primary" {
                assert!(jupiter_classes.contains("jupiter-blue") || jupiter_classes.contains("primary"));
            }
            
            if legacy.disabled {
                assert!(jupiter_classes.contains("disabled") || jupiter_classes.contains("opacity"));
            }
        }
    }

    #[test]
    fn test_visual_parity_between_systems() {
        // This would integrate with visual regression testing
        let legacy_card = LegacyCard {
            shadow: "md".to_string(),
            padding: "lg".to_string(),
            border: true,
            rounded: true,
        };
        
        let jupiter_classes = migrate_card_component(&legacy_card, VibeColors::new());
        
        // Verify Jupiter card has equivalent visual properties
        assert!(jupiter_classes.contains("shadow") || jupiter_classes.contains("elevation"));
        assert!(jupiter_classes.contains("p-") || jupiter_classes.contains("padding"));
    }

    #[test]
    fn test_accessibility_preservation() {
        let legacy_button = LegacyButton {
            variant: "primary".to_string(),
            size: "medium".to_string(),
            disabled: true,
            loading: false,
            full_width: false,
        };
        
        let (variant, size, state, full_width) = migrate_legacy_button_props(&legacy_button);
        let jupiter_classes = MigratedButton::new().get_css_classes(variant, size, state, full_width);
        
        // Disabled state should be preserved and accessible
        assert!(state == ButtonState::Disabled);
        assert!(jupiter_classes.contains("disabled") || jupiter_classes.contains("opacity"));
    }
}
```

### A/B Testing Framework

```rust
pub struct MigrationABTest {
    pub component_name: String,
    pub legacy_classes: String,
    pub jupiter_classes: String,
    pub test_percentage: f32,
}

impl MigrationABTest {
    pub fn should_use_jupiter(&self, user_id: &str) -> bool {
        // Simple hash-based A/B testing
        let hash = calculate_hash(user_id);
        (hash % 100) as f32 / 100.0 < self.test_percentage
    }
    
    pub fn get_classes(&self, user_id: &str) -> &str {
        if self.should_use_jupiter(user_id) {
            &self.jupiter_classes
        } else {
            &self.legacy_classes
        }
    }
}

fn calculate_hash(input: &str) -> u32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish() as u32
}
```

## 📊 Migration Monitoring

### Progress Tracking

```rust
#[derive(Debug, Clone)]
pub struct MigrationProgress {
    pub total_components: usize,
    pub migrated_components: usize,
    pub in_progress_components: usize,
    pub legacy_css_removed: usize,
    pub jupiter_css_added: usize,
}

impl MigrationProgress {
    pub fn completion_percentage(&self) -> f32 {
        if self.total_components == 0 {
            return 100.0;
        }
        (self.migrated_components as f32 / self.total_components as f32) * 100.0
    }
    
    pub fn generate_report(&self) -> String {
        format!(
            "Migration Progress Report\n\
             ========================\n\
             Total Components: {}\n\
             Migrated: {} ({:.1}%)\n\
             In Progress: {}\n\
             Remaining: {}\n\
             Legacy CSS Lines Removed: {}\n\
             Jupiter CSS Lines Added: {}",
            self.total_components,
            self.migrated_components,
            self.completion_percentage(),
            self.in_progress_components,
            self.total_components - self.migrated_components - self.in_progress_components,
            self.legacy_css_removed,
            self.jupiter_css_added
        )
    }
}

// Automated progress tracking
pub fn track_migration_progress(project_root: &str) -> MigrationProgress {
    let mut progress = MigrationProgress {
        total_components: 0,
        migrated_components: 0,
        in_progress_components: 0,
        legacy_css_removed: 0,
        jupiter_css_added: 0,
    };
    
    // Scan codebase for migration markers
    let files = find_component_files(project_root);
    
    for file_path in files {
        let content = std::fs::read_to_string(&file_path).unwrap_or_default();
        
        progress.total_components += 1;
        
        if content.contains("// MIGRATION: COMPLETE") {
            progress.migrated_components += 1;
        } else if content.contains("// MIGRATION: IN_PROGRESS") {
            progress.in_progress_components += 1;
        }
        
        // Count legacy vs Jupiter usage
        progress.legacy_css_removed += count_legacy_css_usage(&content);
        progress.jupiter_css_added += count_jupiter_usage(&content);
    }
    
    progress
}
```

### Performance Impact Analysis

```rust
#[derive(Debug)]
pub struct PerformanceComparison {
    pub legacy_bundle_size: usize,
    pub jupiter_bundle_size: usize,
    pub legacy_load_time: std::time::Duration,
    pub jupiter_load_time: std::time::Duration,
    pub legacy_memory_usage: usize,
    pub jupiter_memory_usage: usize,
}

impl PerformanceComparison {
    pub fn analyze_migration_impact(&self) -> MigrationImpact {
        let bundle_size_diff = self.jupiter_bundle_size as i64 - self.legacy_bundle_size as i64;
        let load_time_diff = self.jupiter_load_time.saturating_sub(self.legacy_load_time);
        let memory_diff = self.jupiter_memory_usage as i64 - self.legacy_memory_usage as i64;
        
        MigrationImpact {
            bundle_size_change: bundle_size_diff,
            load_time_change: load_time_diff,
            memory_usage_change: memory_diff,
            overall_recommendation: self.get_recommendation(bundle_size_diff, memory_diff),
        }
    }
    
    fn get_recommendation(&self, bundle_diff: i64, memory_diff: i64) -> String {
        match (bundle_diff > 0, memory_diff > 0) {
            (false, false) => "Migration improves both bundle size and memory usage".to_string(),
            (false, true) => "Migration reduces bundle size but increases memory usage".to_string(),
            (true, false) => "Migration increases bundle size but reduces memory usage".to_string(),
            (true, true) => "Migration increases both bundle size and memory usage - review carefully".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct MigrationImpact {
    pub bundle_size_change: i64,
    pub load_time_change: std::time::Duration,
    pub memory_usage_change: i64,
    pub overall_recommendation: String,
}
```

## 🚨 Common Migration Pitfalls

### Pitfall 1: Inconsistent Color Mapping

```rust
// ❌ Bad: Inconsistent color usage during migration
pub fn bad_color_migration() {
    let colors = VibeColors::new();
    
    // Some components use Jupiter colors
    let button1 = button_styles(colors.clone()).primary().classes();
    
    // Others still use legacy hardcoded colors
    let button2 = "bg-blue-500 text-white px-4 py-2"; // Hardcoded legacy
    
    // This creates visual inconsistency!
}

// ✅ Good: Consistent color mapping
pub fn good_color_migration() {
    let colors = VibeColors::new();
    
    // All components use Jupiter color system
    let button1 = button_styles(colors.clone()).primary().classes();
    let button2 = button_styles(colors.clone()).primary().classes();
    
    // Consistent visual appearance across all components
}
```

### Pitfall 2: Breaking Accessibility

```rust
// ❌ Bad: Losing accessibility during migration
pub fn bad_accessibility_migration() {
    // Legacy button had proper ARIA attributes
    let legacy_html = r#"<button class="btn btn-primary" aria-label="Submit form">Submit</button>"#;
    
    // Migration only focuses on styling, loses accessibility
    let jupiter_classes = button_styles(VibeColors::new()).primary().classes();
    let migrated_html = format!(r#"<button class="{}">Submit</button>"#, jupiter_classes);
    
    // Lost aria-label!
}

// ✅ Good: Preserving and enhancing accessibility
pub fn good_accessibility_migration() {
    let jupiter_classes = button_styles(VibeColors::new()).primary().classes();
    
    // Preserve original accessibility attributes
    let migrated_html = format!(
        r#"<button class="{}" aria-label="Submit form" type="submit">Submit</button>"#,
        jupiter_classes
    );
    
    // Better: Use Jupiter's built-in accessibility patterns
    let accessible_html = render_accessible_button("Submit", ButtonVariant::Primary, VibeColors::new());
}
```

### Pitfall 3: Performance Regression

```rust
// ❌ Bad: Creating performance issues during migration
pub fn bad_performance_migration() {
    // Don't recreate color providers repeatedly
    for _ in 0..1000 {
        let colors = VibeColors::new(); // Expensive recreation
        let classes = button_styles(colors).primary().classes();
    }
}

// ✅ Good: Optimized migration
pub fn good_performance_migration() {
    // Reuse color provider instance
    let colors = VibeColors::new();
    
    for _ in 0..1000 {
        let classes = button_styles(colors.clone()).primary().classes();
    }
    
    // Even better: Use caching for repeated patterns
    lazy_static! {
        static ref CACHED_COLORS: VibeColors = VibeColors::new();
        static ref PRIMARY_BUTTON: String = button_styles(CACHED_COLORS.clone()).primary().classes();
    }
}
```

## 📋 Migration Checklist

### Pre-Migration
- [ ] Audit current design system usage
- [ ] Identify all components requiring migration
- [ ] Map legacy design tokens to Jupiter tokens
- [ ] Plan migration phases and timeline
- [ ] Set up A/B testing framework
- [ ] Establish performance baselines

### During Migration
- [ ] Migrate components in dependency order
- [ ] Maintain visual parity during transition
- [ ] Preserve all accessibility features
- [ ] Update documentation and examples
- [ ] Monitor performance impact
- [ ] Test with real user data

### Post-Migration
- [ ] Remove legacy CSS and dependencies
- [ ] Update build processes and tooling
- [ ] Train team on Jupiter Design System
- [ ] Establish design system governance
- [ ] Set up automated testing and monitoring
- [ ] Plan for future Jupiter updates

## 🔧 Migration Tools

### Automated Migration Scripts

```rust
// Command-line migration tool
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Jupiter Migration Tool")
        .version("1.0")
        .about("Migrates legacy design systems to Jupiter")
        .subcommand(
            SubCommand::with_name("audit")
                .about("Audits codebase for migration opportunities")
                .arg(Arg::with_name("path").required(true))
        )
        .subcommand(
            SubCommand::with_name("migrate")
                .about("Performs automated migration")
                .arg(Arg::with_name("component").required(true))
                .arg(Arg::with_name("dry-run").long("dry-run"))
        )
        .get_matches();
    
    match matches.subcommand() {
        ("audit", Some(audit_matches)) => {
            let path = audit_matches.value_of("path").unwrap();
            let audit = DesignSystemAudit::analyze_codebase(path);
            println!("{:#?}", audit.generate_migration_plan());
        }
        ("migrate", Some(migrate_matches)) => {
            let component = migrate_matches.value_of("component").unwrap();
            let dry_run = migrate_matches.is_present("dry-run");
            
            if dry_run {
                println!("Dry run: Would migrate {}", component);
            } else {
                perform_migration(component);
            }
        }
        _ => {
            println!("Use --help for usage information");
        }
    }
}

fn perform_migration(component: &str) {
    match component {
        "buttons" => migrate_all_buttons(),
        "cards" => migrate_all_cards(),
        "typography" => migrate_all_typography(),
        _ => println!("Unknown component: {}", component),
    }
}
```

This migration guide provides a systematic approach to adopting the Jupiter Design System while maintaining design consistency and minimizing disruption to existing workflows.